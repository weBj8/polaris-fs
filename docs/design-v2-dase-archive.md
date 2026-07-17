# PolarisFS v2.0 —— 四方缝合怪：GPFS × Lustre × Ceph × VAST Data

> 版本：v2.0 · 2026-07-17（取代 v1.0）
> 设计目标不变：**POSIX 强兼容**、**榨干 NVMe 与网卡**、**无单点、可自愈**。
> v2.0 变更：吸收 VAST Data 的 DASE 共享一切架构，**推翻 v1.0 的"盘归属 + CRUSH 所有权"数据面**，
> 改为"无状态 IO 节点 + 全闪存共享容量层"；EC 从 8+2 升级为宽条带本地可解码码（LDC）；
> 新增 SCM 写缓冲/元数据层与 similarity 全局数据缩减。v1→v2 对照见文末附录 B。

---

## 1. 缝合清单（每个子系统只取全场最优解）

| # | 子系统 | 取自 | 机制 | 不取什么 |
|---|---|---|---|---|
| 1 | 客户端写共享一致性 | **GPFS** | 分布式 byte-range token + metanode + data-shipping 兜底 | VAST 的 close-to-open（共享写要 O_DIRECT，被独立评测点名"违反 POSIX"） |
| 2 | 并行条带布局 | **Lustre** | PFL 渐进条带（<32MB 单条带→全宽） | OST 盘归属模型（被 #5 取代） |
| 3 | 小文件 | **Lustre** | DoM：≤1MB 随元数据放 SCM 层 | — |
| 4 | 锁请求合并 | **Lustre** | intent lock 一次 RPC 拿锁+元数据 | — |
| 5 | **数据面拓扑** | **VAST** | **DASE：无状态 IO 节点经 NVMe-oF 挂载全部闪存**，任何节点可读写任何盘，无盘归属、无东西向复制流量、节点故障零重建瞬时切换 | Ceph CRUSH 的"对象→OSD 归属"间接层（保留 CRUSH 仅作纯放置函数） |
| 6 | **写路径分层** | **VAST** | 写先落 **SCM（多盘镜像）即确认**，异步聚合成大条带再刷 QLC；write-in-free-space | v1.0 的"对象直写裸 NVMe"（对 QLC 随机写是灾难） |
| 7 | **纠删码** | **VAST** | **本地可解码码 LDC 宽条带**（如 64+4 起，最大 146+4），容量开销 2.7–3%，重建只读幸存分片 1/4 且只重建在用数据 | v1.0 的 8+2（25% 开销，太贵） |
| 8 | **元数据持久层** | **VAST** | V-Tree 式宽扇出树存于**共享 SCM**，原子更新+全局 snaptime 计数器 → 元数据节点也无状态化，免 journal 重放 | v1.0 的 Raft 三副本 journal（降级为小集群选项） |
| 9 | **数据缩减** | **VAST** | similarity 全局缩减：SCM 确认后**异步**指纹+公共字典 zstd+字节级 delta，不占写延迟路径 | inline 缩减（伤延迟） |
| 10 | 稳态 QoS | **Ceph** | mClock 配额调度（client/recovery/scrub/reduction 四类队列） | VAST 无公开等价物 |
| 11 | 端到端校验 | **Ceph** | 每 4KB CRC32C 全链路（网+盘）+ 后台 scrub | VAST 未公开 scrub 机制 |
| 12 | EC 部分写算法 | **Ceph** | FastEC 思想：部分条带写/PDW 按 IO 动态选优（在 SCM 聚合层内实现） | — |
| 13 | 慢盘隔离 | **Ceph** | p99 延迟 outlier 检测自动摘除 | — |
| 14 | IO 栈 | **DAOS/工程现实** | Seastar 式 per-core 用户态 + SPDK/NVMe-oF initiator 轮询；io_uring 降级 | 内核页缓存路径 |
| 15 | 网络 | 工程现实 | RoCEv2(PFC+ECN)/IB 多轨默认开；RDMA 零拷贝；GDS | — |
| 16 | 多协议 | **VAST** | 单命名空间原生 NFSv4/S3/块/POSIX 客户端，事务统一 | 多网关拼装 |

**一句话**：VAST 重写了"服务端怎么组织盘"，GPFS 保住"客户端并发写的 POSIX 灵魂"，
Lustre 留下"大文件并行条带"，Ceph 贡献"运维稳定性三件套"（QoS/校验/慢盘隔离）。

---

## 2. 总体架构 v2.0

```
┌──────────────────── 客户端 ────────────────────┐
│ PFS-Client：页缓存 + byte-range token（GPFS 灵魂）│
│ 并行直连 IO 节点；PFL 布局本地计算；GDS 支持       │
└──────────┬──────────────────────┬──────────────┘
      元数据/token RPC           数据 RDMA
┌──────────▼───────────┐  ┌──────▼───────────────────────┐
│ MDS 集群（无状态）     │  │ IO 节点 ION×N（无状态容器）    │
│ · token 管理器(按ino散列)│  │ · per-core reactor + SPDK     │
│ · V-Tree 元数据(在SCM) │  │ · 启动挂载全部 SCM+QLC        │
│ · 任意 MDS 可服务任意树 │  │ · 写落 SCM 即确认              │
│ · 故障=换一个接着干    │  │ · 后台聚合→LDC 宽条带→QLC     │
└──────────┬───────────┘  └──────┬───────────────────────┘
           └────── NVMe-oF / RDMA 全互联（RoCEv2/IB）──────┐
┌──────────▼─────────────────────────────────▼──────────┐
│ 容量层 EBOF×M：每柜 22×QLC(E1.L 30TB) + 8×SCM(FL6)      │
│ · SCM：写缓冲(多副本镜像) + 元数据 V-Tree + token 持久状态 │
│ · QLC：LDC 宽条带数据（64+4 ~ 146+4），开销 2.7–3%       │
│ · 全部设备对全部 MDS/ION 可见（DASE 共享一切）            │
└────────────────────────────────────────────────────────┘
```

与 v1.0 的本质区别：**盘不再属于任何节点**。CRUSH 降级为纯数学放置函数
（`hash(object) → 设备列表`），任何 ION 算出同样结果并直接访问——
于是 ION 故障零重建（没有"它名下的盘"要接管）、扩容免 rebalancing 风暴、
负载天然均衡（没有热点 OSS 概念）。

---

## 3. VAST 带来的三个架构级变更（及与原设计的冲突和解）

### 3.1 DASE 共享一切 vs v1.0 盘归属 —— DASE 胜

v1.0 的"OSS 拥有 8 块本地盘、SPDK 直挂"在绝对延迟上最优，但带来三个稳定性税：
盘归属导致节点故障要重建归属关系、CRUSH 要处理 OSD 上下线的数据迁移、热点 OSS 需要 rebalancing。
2026 年的事实：RoCEv2 远端 NVMe 读额外延迟 ~5–10µs，而 QLC 读延迟 ~60–100µs——
**共享化代价 <15%，换来的是整个故障域模型的消失**。VAST 用 xAI Colossus（10 万+ GPU、EB 级）
证明了这条路的规模上限。和解方案：性能层保留"客户端页缓存+token 批量大写"吸收延迟；
对极限延迟场景（HPC checkpoint）保留可选的本地 NVMe 写直达池（v1.0 路径作为 tier 存在）。

### 3.2 SCM 写缓冲 + QLC 主存 —— 直接全盘采纳

- 写路径：`ION 收写 → RDMA 落 2–3 块 SCM（镜像，跨柜）→ 立即确认`。
  客户端看到的写延迟 = SCM 延迟（~10µs 级），QLC 的慢随机写（~25K IOPS/盘）被彻底屏蔽。
- 后台聚合器把 SCM 中的写按条带凑满（FastEC 式动态选 PDW/重构法），
  一次性整条带写入 QLC → **QLC 上只有大顺序写**，耐久度问题工程化解决
  （VAST 敢给 QLC 十年保修的底气就是这个机制）。
- Optane 已死：SCM 用 Kioxia FL6（SLC 模式，60 DWPD）或同级多源；这是 VAST 2022 验证过的替代路线。
- 这一层同时吸收了 v1.0 的"WAL 区"和"写聚合 buffer"——架构反而变简单了。

### 3.3 无状态元数据 —— 采纳，但保留 Raft 小集群模式

VAST 的 V-Tree 存共享 SCM + 原子更新，元数据节点无状态化，故障切换无需 journal 重放。
我们采纳为主体模式：**M ≥ 1 个 EBOF 时，元数据=共享 SCM 上的 V-Tree，MDS 无状态**。
但 3 节点起步的小集群没有 EBOF 怎么办？→ 提供 **Raft 模式**（v1.0 方案）作为部署选项，
两套模式共用同一份 V-Tree 逻辑结构，只是持久化后端不同（共享 SCM vs 本地盘+Raft 复制）。

---

## 4. 数据路径 v2.0（榨干 2026 年硬件的完整链路）

### 4.1 写路径
```
client (token 持有, 页缓存聚合 ≥条带单位)
  → RDMA 并行写 ION（PFL 布局决定分片目标）
  → ION per-core reactor（SPDK/NVMe-oF initiator 轮询，零中断）
  → 写落 SCM×2~3 镜像（跨 EBOF 柜）→ 立即 ACK
  → 后台: mClock 配额内的聚合器凑满 LDC 条带 → 一次大顺序写刷 QLC
  → SCM 空间释放; 异步 similarity 缩减在刷盘前完成指纹比对
```
- 客户端可见写延迟 ≈ SCM 延迟 + 一次 RDMA ≈ **20µs 级**；顺序大写由 token+聚合保证整条带。
- 小随机写：SCM 吸收突发，聚合器把同一 LDC 条带的多个 4K 合并后一次落 QLC（Ceph FastEC 的 PDW 思想，但发生在 DRAM/SCM 侧，零读改写）。

### 4.2 读路径
- 热数据（刚写入/在读缓存窗口）直接命中 SCM；冷数据直读 QLC（单盘顺序 ~7GB/s、4K 随机 ~1M IOPS 级）。
- PFL：首 32MB 单分片（小文件免扇出）；大文件扇出到全部 EBOF 的 QLC 设备——**扇出宽度不再受"OSS 数"限制，而是全池设备数**，这是 DASE 白送的带宽。

### 4.3 LDC 纠删码（容量经济学的核心）
| 参数 | v1.0 (8+2) | v2.0 (LDC) |
|---|---|---|
| 典型配置 | 8+2 | 容量层 146+4；均衡层 32+4；机柜级韧性可选叠加 29+4 |
| 容量开销 | 25% | **2.7%（146+4）/ 12.5%（32+4）** |
| 重建读取量 | 全部幸存分片 | **幸存分片的 1/4**（LDC 局部性） |
| 重建范围 | 全盘 | **仅在用数据** + declustered 全池并行 |
| MTTDL 参照 | — | VAST 官方宣称 150+4 > 4200 万年（存疑但方向正确） |

配合 write-in-free-space：条带永远写新位置，旧位置异步回收——天然免读改写，快照零拷贝。

---

## 5. POSIX 一致性：GPFS 灵魂修补 VAST 短板

VAST 官方只做 close-to-open + sync/O_DIRECT 换原子写（独立评测直指其"违反 POSIX"）；
我们的缝合点恰是对方短板。**v2.0 一致性协议与 v1.0 相同**（byte-range token + metanode +
data-shipping + 全局锁序防死锁），但有两个升级：

1. **token 持久状态放共享 SCM**（V-Tree 旁）：token 管理器随 MDS 无状态化，
   任意 MDS 可经 SCM 上的锁位+租约记录接管任意文件的 token 仲裁——去掉 v1.0 的
   "hash(ino) 固定归属 MDS"及随之而来的接管重放。
2. **区间冲突串行化借鉴 VAST 锁位**：SCM 上的原子 RDMA 锁位作为 token 仲裁的硬件加速层，
   冲突区间的 data-shipping 落点改为 SCM，冲突写也能拿到 ~20µs 确认。

语义承诺不变：read-after-write 强一致、mmap 与 token 绑定、O_APPEND 原子、跨目录原子 rename。
NFSv4 delegations / SMB lease 由网关映射到同一套 token，多协议（POSIX+NFS+S3）同一份数据强一致。

## 6. 稳定性 v2.0

| 机制 | 来源 | v2.0 状态 |
|---|---|---|
| 无状态节点故障切换 | VAST | ION/MDS 故障 = **瞬时**（状态全在共享 SCM，无重建无重放） |
| 盘故障重建 | VAST+Ceph | LDC 读 1/4 幸存分片 × 只重建在用数据 × 全池 declustered 并行；mClock 限速防冲击 |
| 端到端校验 | Ceph | 4KB CRC32C：client→RDMA→SCM→QLC→读出 全链路；后台 scrub（VAST 无公开等价物，我们保留） |
| QoS | Ceph | mClock 四类队列：client/recovery/scrub/**reduction**（缩减也纳入配额，VAST 无公开 QoS） |
| 慢盘/慢链路隔离 | Ceph | p99 outlier 3×/60s 自动摘除 |
| 机柜级韧性 | VAST | 可选 29+4 机柜层 LDC，容忍整柜掉电（约 9% 额外开销） |
| 快照 | VAST | write-in-free-space + snaptime 计数器 → 零拷贝秒级快照 |
| 脑裂防护 | 原设计 | SCM 租约记录 + quorum；客户端 token lease 30s |

## 7. 数据缩减（容量层默认开，性能层默认关）

- **异步、永在容量路径外**：写 SCM 确认后才做 similarity 指纹比对 → 命中相似块则以
  公共字典 zstd + 字节级 delta 编码；官方宣称任意数据最低 3:1（带保障条款），理性预期 2:1。
- 单 dedup 域全局生效；元数据记录 delta 引用链，读路径一次额外查找（<5µs，QLC 读延迟内可忽略）。
- HPC checkpoint/科学浮点数据缩减率低 → per-pool 开关，默认关。

---

## 8. 参考部署与性能预算 v2.0

**甜点位配置（对标 Ceres 但通用硬件）：**

| 角色 | 配置 | 数量 |
|---|---|---|
| EBOF 容量柜 | 2×BlueField-3 DPU + 22×30.72TB QLC(E1.L) + 8×800GB SCM(FL6)，4×200GbE 上行 | 5 柜 |
| ION | 64 核 / 256GB / 2×400GbE，无盘容器节点 | 8 台 |
| MDS | 32 核 / 512GB / 2×200GbE，无状态 | 4 台 |
| 交换 | 51.2T RoCE（PFC+ECN）收敛比 ≤2:1 | — |

**预算（估算值，非厂商宣称）：**
- 裸容量 5×675TB ≈ 3.4PB；LDC 146+4 后有效 ≈ **3.3PB**（v1.0 同盘位 8+2 只有 2.7PB）
- 聚合读带宽：受 ION 网卡限 8×100GB/s = 800GB/s 理论，预算 **~550GB/s**；
  QLC 池本身 110 盘×7GB/s = 770GB/s，与网络基本打平（网盘匹配原则不变）
- 写带宽：SCM 层 40 盘镜像写 ≫ 客户端需求，预算 **~200GB/s 持续**（后台刷 QLC 是长期约束：
  110 盘×~3GB/s 顺序写 ≈ 330GB/s > 200GB/s，水位安全）
- 4K 随机读（全 QLC 冷读）：110×1M = 110M IOPS 理论，预算 **~30M IOPS**（网络+reactor 限）
- 元数据：V-Tree 在 SCM 随机读 ~10µs 级，4 MDS 预算 **~100万 ops/s**；DoM 小文件免数据路径
- 对照第三方实测锚点：CoreWeave 实测 VAST 64×H200 节点聚合读 >500GiB/s（2026-06）

## 9. 不取什么 & 风险清单

**不取 VAST 的**：专有硬件绑定与订阅制（我们通用硬件+开源）、close-to-open 语义、
最小 PB 级起步（我们 3 节点 Raft 模式起步）、读写 4:1 的 WORM 倾向（SCM 层对冲）。
**风险**：
1. LDC 146+4 的实现复杂度（编解码 CPU 开销、部分条带读延迟）→ 先落地 32+4，宽条带二期；
2. SCM 成本（FL6 约 QLC 的 8–10×/GB）→ SCM:QLC 容量比按写强度定，默认 1:40；
3. NVMe-oF 全互联的 fabric 成本与 PFC 运维复杂度 → ≤64 节点用 RoCE 单层 CLOS，之上走 IB；
4. similarity 缩减的指纹 CPU 开销 → 放 ION 空闲核 + mClock 配额，禁止影响写路径。

---

## 附录 A：v1.0 → v2.0 变更对照

| 子系统 | v1.0 | v2.0 | 原因 |
|---|---|---|---|
| 数据面拓扑 | OSS 拥有本地盘 + CRUSH 归属 | **DASE 无状态 ION + 全闪共享** | VAST 证明：节点故障零重建、免 rebalancing、规模到 EB 级 |
| 写路径 | 对象直写裸 NVMe | **SCM 镜像缓冲 → 异步聚合刷 QLC** | QLC 随机写 ~25K IOPS 不可直写；SCM 缓冲让写延迟 ~20µs |
| EC | 8+2（25% 开销） | **LDC 32+4 ~ 146+4（2.7–12.5%）** | 同盘位有效容量 +22%；重建读量降 4× |
| 元数据高可用 | Raft 三副本 journal + 伙伴接管 | **共享 SCM V-Tree，MDS 无状态**（Raft 降级为小集群选项） | 切换从 <10s 到瞬时 |
| 数据缩减 | 无 | **similarity 全局异步缩减** | 容量成本再降 ~2×（官方宣称 3:1 保底） |
| 不变项 | GPFS token 一致性、Lustre PFL/DoM、Ceph QoS/校验/scrub、SPDK 用户态 | 全部保留并因 DASE 简化（token 状态也上 SCM） | — |

## 附录 B：关键事实来源（调研日 2026-07-17）

- VAST DASE/Element Store/V-Tree/SCM 写缓冲：vastdata.com 官方白皮书（2026-06）
- LDC 宽条带/2.66–3% 开销/机柜韧性：vastdata.com 博客（2024-05）
- Ceres/EBox 硬件、Optane→Kioxia FL6：storagereview.com（2022-03）、vastdata.com 博客（2022-07/2024-12）、kb.vastdata.com（2026-03）
- CoreWeave 实测 500GiB/s：coreweave.com 博客（2026-06）；xAI Colossus EB 级：blocksandfiles.com（2024-11）
- POSIX 批评（close-to-open/O_DIRECT）：hpc.social（2019）；读写 4:1 倾向：GWDG/NHR 评测（2022）
- Lustre/GPFS/Ceph/DAOS/io_uring/SPDK 来源同 v1.0（2025-11 ~ 2026-04 官方与 IO500）
