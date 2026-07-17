# PolarisFS ROADMAP v6 —— GPFS 全基本功能对齐（40 阶段）

> 2026-07-17 · 取代 v5（旧版进 git 历史）；v2.0 DASE 设计稿归档于 `docs/design-v2-dase-archive.md` 仅作参考。
> 前提：1 名人类开发者 + AI 结对。
> 一句话：**开源的、现代的 GPFS。原理照搬，功能全集对齐，技术栈全面现代化；多用开源库，代码短小精悍。**

## 0. 定位与技术栈决策

**目标**：10–50 节点小集群起步、功能对齐 GPFS 的共享 POSIX 并行文件系统。
GPFS 已经是很老的东西（kmod 客户端、90 年代内核假设、运维要专职团队、闭源绑定、百万行级代码）。
我们重写，不是兼容——**功能对标不打折，架构现代化，代码压到两个数量级之下。**

### 0.1 GPFS 基本功能对齐清单（全部进路线图，不打折）

| GPFS 功能 | 我们的阶段 | GPFS 功能 | 我们的阶段 |
|---|---|---|---|
| 分布式 byte-range token（metanode） | P14–P15 | 快照（全局/fileset） | P21–P22 |
| data-shipping 兜底 | P14 | 可写克隆 | P22 |
| 条带化 | P8 | fileset/junction | P22 |
| 2/3 副本 + 元数据副本 | P9 | user/group/fileset 三级配额 | P23 |
| failure group 放置 | P10 | NFSv4 ACL | P24 |
| mmap/O_APPEND/fcntl 强语义 | P17 | storage pool + 放置策略 | P25 |
| 在线加盘/restripe | P28 | ILM 策略引擎（mmapplypolicy） | P26 |
| 在线退盘/排水 | P29 | QoS（mmchqos） | P27 |
| declustered 重建 | P30 | 加密 at rest | P36 |
| fsck（mmfsck） | P31 | 压缩 | P37 |
| 滚动升级 | P32 | WORM/不可变（mmchattr -i） | P38 |
| 备份支持（mmbackup） | P33 | 多集群远程挂载 + AFM | P39 |
| NFS/S3 导出（CES，砍 SMB） | P40 | 性能监控（mmpmon） | P12 |

### 0.2 我们多于 GPFS 的（差异化卖点）

- **CRC32C 端到端全链路 + 内建 scrub**（GPFS 磁盘层无端到端校验）
- **现代客户端**：FUSE + io_uring + passthrough，无 kmod，不绑内核版本
- **Rust 内存安全**（数据面/控制面全量）+ Zig 热路径组件（无 GC 手工 ring）
- log-structured 落盘 → 快照零拷贝、崩溃恢复不重放
- 开源 + 通用硬件，无订阅制

### 0.3 抄与不抄

**抄 GPFS 什么（原理层）**：
- byte-range token/租约：客户端持租约才能缓存写，冲突时回调收回（P14）
- **分布式锁管理（metanode 思想，GPFS 的灵魂，原样保留）**：每文件锁仲裁者（arbitrator）
  由 hash(inode) 散列到集群各节点；锁表全内存不落盘，故障靠 租约超时 + epoch fencing +
  grace-period 重建（P14–P15）
- data-shipping 兜底、大缓冲池 + 条带化并行直读多存储节点
- failure group、storage pool、ILM、fileset 这些数据管理概念整套对齐

**不抄 GPFS 的部分**：不引入 quorum/group services 集群状态机——成员关系与 epoch 由单 MDS
颁发（namespace 仍单 MDS 串行化，这是**控制面**简化；**锁面是分布式的**，两者解耦）。
不绑 SAN 共享盘假设：走 client→chunkserver 直连（GPFS-NSD 服务端模式的现代化）。

| GPFS 的技术栈 | 我们的技术栈 | 理由 |
|---|---|---|
| 内核模块客户端（mmfs） | **FUSE**，无 kmod | 不绑内核版本，崩了不拖垮机器 |
| read/write 逐次系统调用过 /dev/fuse | **FUSE over io_uring**（内核 ≥6.15）+ **FUSE passthrough**（≥6.9 热数据直通） | 现代内核白送的性能路径 |
| 多线程 epoll 守护进程 | **io_uring 全链路**（extent store/RPC/bench） | per-core 提交批量化，榨干 NVMe |
| C + 私有守护进程 | **Rust 为主体**，**Zig 热路径组件**（P20 fuse-io_uring transport、P40 数据面） | 内存安全与零开销各取所长 |
| quorum/group services | MDS 颁发成员与 epoch | 去掉最重的集群状态机 |

**内核基线**：≥6.9（passthrough）；甜点 ≥6.15（fuse io_uring）。开发机 7.1。

### 0.4 开源库选型（能借就不造，代码短小精悍）

| 子系统 | 选用 | 不自己造的理由 |
|---|---|---|
| FUSE 客户端 | `fuser`（P20 起叠加自研 io_uring transport，Zig） | libfuse 协议的成熟 Rust 实现 |
| io_uring | `io-uring` crate（数据面零运行时依赖） | 薄绑定；自研只写 ring 提交策略 |
| 控制面 async | `tokio` + `tokio-util` codec | 生态默认，RPC/定时器全齐 |
| 数据面 runtime（可选） | `glommio`（thread-per-core on io_uring，P7 决策） | 省自建 reactor |
| 元数据持久化 | `redb`（纯 Rust ACID KV；P3 决策：redb vs 自存 extent store 二选一） | 嵌入式 B-tree+WAL 现成 |
| MDS 热备复制 | `raft-rs`（TiKV 生产级 Raft，P16 直接套小状态机） | **不写 Raft 是本路线图的纪律** |
| 序列化 | `serde`+`bincode`（RPC/元数据）；`bytemuck`（盘格式 Pod） | — |
| 校验 | `crc32c`（SSE4.2 硬件指令） | 数据路径刚需 |
| 并发结构 | `dashmap` / `parking_lot` / `crossbeam` | — |
| 日志/指标 | `tracing`；`metrics` + `metrics-exporter-prometheus` | P12 直接可用 |
| EC（P35） | `reed-solomon-simd`（或 `reed-solomon-erasure`） | 生产级编解码 |
| 压缩（P37） | `zstd` | — |
| 加密（P36） | RustCrypto：`aes-xts` / `aes-gcm` | — |
| NFS 网关（P40） | `nfsserve` crate | NFS server 骨架现成 |
| S3 网关（P40） | `s3s` crate | S3 协议层现成 |
| CLI / 错误 | `clap` derive；`thiserror`（库）/`anyhow`（bin） | — |

**代码量纪律（短小精悍）**：原则只有一条——**能用成熟开源库就不自己造**，
自研只写差异化的部分（盘格式、extent 策略、租约协议、条带映射）。
行数是参考不是上限：不为凑行数挪动代码，不设单文件/单阶段行数天花板。
**难度纪律：功能清单不打折（40 阶段全做）；唯一允许砍的是"有现成库能解决的自研冲动"。**

## 1. 目标架构

```
        ┌──── polaris-mds（单活 + 热备）────┐
        │ 目录树/inode + 文件→chunk 映射(CRUSH)│
        │ 集群成员 + epoch 颁发(控制面唯一中心) │
        └──┬──────────────┬──────────────┬───┘
      元数据 RPC      数据/租约并行直连(io_uring)
   ┌───────▼──┐   ┌──────▼─────┐  ┌─────▼──────┐
   │ FUSE 客户端│  │chunkserver1│  │chunkserver2│ ×N
   │ passthrough│─▶│ io_uring   │  │ io_uring   │
   │ writeback │   │ extent+WAL │  │ CRC32C     │
   └─────┬────┘   │ +锁仲裁者   │  │ +锁仲裁者   │
         └────────┴────────────┴───┴────────────┘
   分布式租约层：hash(inode)→每文件 arbitrator(首活者)，
   内存锁表 + 回调收回 + data-shipping + epoch/grace 恢复
```

- 一致性 v1 承诺 **close-to-open**；P14–P17 逐步补到 GPFS 级强语义。
- 冗余 v1：2 副本链式写；EC 进 P35。

## 2. 四十阶段

### 第一段：单机 MVP（P1–P6）——先证明自己能当一个文件系统用

**P1 · 磁盘格式 v0 + io_uring extent store + bench**（~2 周）✅ 已完成
交付：`docs/format.md`、`porfs-format`、`porfs-store`（追加式 extent log + CRC32C + 双 superblock + 扫描自愈 + 零拷贝 `read_batch_into`）、`porfs mkfs/info/bench`（1266+301 行自有代码）。
Gate（语义校准后）：**顺序写 ≥85% 同设备 fio 裸写（含 CRC 开销）——✅ 实测 95.5%**；
**顺序读分层考核**：io_uring 纯流水线 ≥ fio 裸读（引擎零损耗证明）——✅ 实测 111%；
带校验读 vs **fio+verify 同场基线**（fio 裸读不做任何校验，与带 CRC 读直接对比在低内存带宽
机器上是关公战秦琼：带校验+拷贝读每逻辑字节 ~4 倍总线流量 vs fio 1 倍，开发机 APU 单核
DRAM 仅 6.9GB/s < 2× 盘速，物理不可达——服务器/台式机双通道内存无此墙）；
CRC 破坏检出 100% ✅；`cargo test` 全绿（30 项）✅；clippy 零警告 ✅。

**P2 · WAL + 组提交 + 崩溃恢复**（1–2 周）
组提交 WAL、检查点、挂载免全扫；superblock 代际切换。
Gate：`kill -9` 千次循环零损坏、已确认写零丢失。

**P3 · MDS v0（单机库形态）**（2 周）
目录树/inode 表 + 文件→extent 映射；持久化方案二选一：`redb`（ACID KV 现成）或自存 extent store（P3 开工时决策，倾向 redb）；事务式 rename/create。
Gate：元数据操作崩溃后可恢复，自检通过。

**P4 · FUSE 客户端 v0**（2 周）
`fuser` 挂载：lookup/getattr/readdir/read/write/create/mkdir/unlink/rename/fsync；属性缓存超时。
Gate：pjdfstest 基础项通过。

**P5 · POSIX 补全 I**（2 周）
xattr、稀疏文件（洞）、大目录分片索引（百万级 entry 秒列）、rename 边界情形、fsync/fdatasync 语义。
Gate：pjdfstest 全量过；百万文件目录 ls/find 性能达标。

**P6 · MVP 封版（Gate 阶段）**（1 周）
`porfs mkfs + mount` 一条命令可用；真实负载 smoke：git clone、内核编译、sqlite 压测。
Gate：三个负载全跑通无数据错误。**第一个"可用"里程碑。**

### 第二段：多机并行（P7–P13）——变成"分布式"

**P7 · RPC + chunkserver 服务化**（2 周）
length-prefixed 二进制帧 over TCP（tokio-util codec + bincode；数据面评估 glommio），静态集群成员，extent 读写接口服务化。
Gate：跨机 extent 读写正确，断线重连语义明确。

**P8 · 条带化并行读**（2 周）
文件→chunk→chunkserver 映射为纯函数（CRUSH 式，无中心查表）；客户端并行直读。
Gate：4 客户端聚合读 ≥ 盘池带宽 70%。

**P9 · 并行写 + 2 副本链式写**（2 周）
主副本转发从副本，两端确认才 ACK；副本一致性校验；元数据双副本。
Gate：杀任一 chunkserver 不丢已确认写；读自动走幸存副本。

**P10 · failure group + 放置策略**（1–2 周）
节点/盘标注故障域（机架/机箱），副本强制跨 failure group；CRUSH 输入带拓扑。
Gate：整 rack 断电（模拟）不丢数据、可读。

**P11 · close-to-open 一致性**（1–2 周）
客户端属性/页缓存超时模型文档化 + 实现；open 强制重校验。
Gate：多机 open/close 交叉读写校验零错误。

**P12 · 可观测性**（1 周）
prometheus 指标（延迟直方图/带宽/副本水位）、`porfsadm` CLI、`tracing` 结构化日志规范。
Gate：仪表盘能定位"慢在哪一层"。

**P13 · 生产 v0.1 投放（Gate 阶段）**
自有集群跑"可重建数据"4 周零事故。**连自己都不用 → 降级为学习项目。**

### 第三段：分布式一致性（P14–P18）——GPFS 的灵魂

**P14 · 分布式租约管理器（DLM）核心**（6–8 周）——全路线图最难阶段
每文件 arbitrator = hash(inode, epoch) → 有序候选列表首活者（chunkserver 兼任，metanode 思想）；
内存态 byte-range 锁表；租约 30s 超时 + I/O 顺带续约；冲突 → 回调收回 → 限期 flush + 释放，
逾期视为客户端死亡；收回期间冲突写降级 data-shipping；多区间按 (inode, offset) 全局序防死锁；
fcntl 阻塞锁映射为仲裁者等待队列 + 超时兜底。
**排期预案（功能不打折）：8 周做不稳 → write-lease 移出 v1 范围延后交付，v1 先承诺 close-to-open；DLM 仍在路线图上，做完为止。**
Gate：单 arbitrator 存活时共享写交叉校验零错误；回调收回 p99 < 2× 租约超时。

**P15 · arbitrator 故障转移 + 租约恢复**（3–4 周）
MDS 颁发 epoch；arbitrator 死亡 → 候选次位者新 epoch 接管 → grace period 内客户端重报租约
重建锁表，逾期作废；epoch fencing 防脑裂（旧 epoch 消息一律拒绝）。
Gate：杀 arbitrator 恢复 <10s；恢复窗口零脏数据；旧 epoch 消息零效力。

**P16 · MDS 热备 + failover**（3–4 周）
`raft-rs` 套小状态机做 WAL 复制与选主（不写 Raft）；成员/epoch 状态随状态机重建；半自动切换。
Gate：杀 MDS → 备机接管 <30s，业务毛刺可接受。

**P17 · POSIX 强语义 II**（3–4 周）
mmap 跨机一致（与租约绑定，脏页回收）；O_APPEND 多写者原子；跨目录 rename 原子；
fcntl 跨机死锁检测（仲裁者 wait-for 图）。
Gate：并发 mmap/O_APPEND/rename 交叉校验零错误；注入死锁能被检测并解开。

**P18 · 72h 故障注入 soak（Gate 阶段）**
随机杀 MDS/arbitrator/chunkserver/客户端 + 网络分区 + 共享写校验器常驻。
Gate：72h 零数据错误、服务可恢复。

### 第四段：客户端性能（P19–P20）——现代内核特性全开

**P19 · 客户端性能 I**（2 周）
FUSE writeback cache + max_readahead/max_write 调优；条带聚合预取；大 I/O 合并；并发度调优。
Gate：fuser 框架内单客户端顺序读 ≥ 裸盘 40%。

**P20 · 客户端性能 II**（4 周）
本地 chunk 读缓存 + **FUSE passthrough**（热数据直通，≥6.9）→ **FUSE over io_uring transport**
（≥6.15；Zig 第一候选落点：无 GC、手工 ring 管理、C ABI 供 Rust 调用）。
Gate：单客户端顺序读 ≥ 裸盘 70%（FUSE 基线通常 <30%）；小 I/O 延迟 ≤ 裸设备 +50µs。

### 第五段：数据服务（P21–P27）——GPFS 数据管理全套

**P21 · COW 全局快照**（2–3 周）
log-structured 天然 write-in-free-space → 秒级零拷贝快照；保留策略（最近 N 份/按时间窗）。
Gate：快照千次无泄漏；快照内数据与打快照时刻逐字节一致。

**P22 · fileset + 可写克隆**（3 周）
独立 inode 空间的目录子树 + junction 挂载点；fileset 级快照；快照→可写克隆。
Gate：fileset 配额/快照独立生效；克隆写不污染原快照。

**P23 · 三级配额**（2 周）
user/group/fileset 配额；软/硬限制 + grace time；配额记账崩溃可恢复。
Gate：超限写入被拒且不误伤未超限用户；grace 过期软限转硬限。

**P24 · NFSv4 ACL**（2–3 周）
NFSv4 ACL 模型 + 与 mode bit 双向映射；继承规则；getfacl/setfacl 兼容。
Gate：ACL 语义测试套件过；与 mode bit 混用行为可预测。

**P25 · 存储池 + 放置策略**（2–3 周）
chunkserver 分组为 pool（如 nvme/sata）；文件/目录级 pool 指派；新写按池落盘。
Gate：同文件跨池条带正确；池满只拒该池写。

**P26 · ILM 策略引擎**（3–4 周）
类 mmapplypolicy：规则语言（大小/年龄/池/路径模式）→ 扫描 → 池间迁移/预热/驱逐；
迁移中文件可正常读写（逐 extent 搬移 + 租约保护）。
Gate：百万文件扫描迁移零错误；迁移中业务降级 <20%。

**P27 · scrub + GC + QoS**（3 周）
后台 CRC scrub（发现静默损坏→副本修复）；extent GC 回收空洞；mClock 四类队列
（client/recovery/scrub/migration）限速。
Gate：scrub/GC 期间业务降级 <25%；注入位翻转 100% 被 scrub 修复。

### 第六段：弹性与运维（P28–P33）——GPFS 运维灵魂

**P28 · 在线扩容 + restripe**（3 周）
在线加 chunkserver/加盘；增量 rebalance（新老文件按新宽度条带化，类 mmrestripefs）；rebalance 限速不伤业务。
Gate：扩一倍节点后聚合带宽近似线性增长；扩容全程业务在线。

**P29 · 在线缩容 + 排水退役**（2–3 周）
节点/盘排水（drain）：数据迁出 → 标记只读 → 摘除；等价 mmdeldisk。
Gate：排水过程零数据不可用窗口。

**P30 · declustered 副本重建**（2–3 周）
盘/节点故障后副本欠额检测 → 全池并行重建（declustered）→ 自动恢复额定副本数；重建限速。
Gate：杀一块盘后副本自动恢复；重建期间业务降级 <30%。

**P31 · porfs-fsck**（3 周）
在线巡检（只读一致性检查）+ 离线修复（目录树/inode/extent 映射/配额账本）；等价 mmfsck。
Gate：注入 20 类损坏（孤儿 inode、断链目录、错配额等）全部检出并可修复。

**P32 · 滚动升级**（2 周）
协议版本协商（min-common-version）；N-1→N 在线升级演练；盘格式版本门（改格式先升 format.md）。
Gate：混合版本集群跑满负载零错误；逐台升级业务无感知。

**P33 · 备份与恢复**（2–3 周）
快照一致性点导出 + 增量清单（类 mmbackup 思路，对接 restic/rsync/tar）；全量+增量恢复演练。
Gate：备份→灾难恢复 RTO/RPO 达标并写进运维手册。

### 第七段：v1.0 封版（P34）

**P34 · v1.0 封版（Gate 阶段）**（2 周）
运维手册（安装/升级/回滚/巡检/故障处置）；长稳 30 天；安全基线审计。
Gate：按手册由非作者完成一次部署 + 一次故障处置。

### 第八段：企业与生态（P35–P40）——对齐 GPFS 高级基本盘，全部承诺交付

**P35 · EC 8+2 冷池 + DoM 小文件**（4–6 周）
容量池纠删码（`reed-solomon-simd`，8+2 起步，接口预留 LDC 宽条带）；≤1MB 小文件随元数据存放免数据路径（DoM）。
Gate：EC 池读带宽 ≥ 单盘 6×；杀任意 2 盘零丢失。

**P36 · 加密**（3 周）
at-rest：per-fileset AES-256-XTS（RustCrypto），密钥外置（KMS 对接）；传输：RPC 全链路 TLS。
Gate：加密池性能降级 <15%；拔盘数据不可读。

**P37 · 异步压缩/去重**（3–4 周）
容量池后台 zstd 压缩 + 指纹去重（永在写延迟路径外）；per-pool 开关。
Gate：可压缩数据集容量节省 ≥1.8×；写路径零回归。

**P38 · WORM/不可变文件**（2 周）
immutability + retention period（类 mmchattr --immutable）；合规删除审计日志。
Gate：retention 内任何路径（含 root）无法改/删；过期可删。

**P39 · 多集群 + AFM**（5–7 周）
远程挂载另一集群命名空间（GPFS multi-cluster 对齐）+ AFM 式站点读写缓存（异步预热、断网可写回）。
Gate：跨集群读缓存命中后走本地带宽；断网写恢复后一致性校验零错误；语义文档化。

**P40 · 生态与硬件终章**（可并行拆分）
NFSv4/S3 网关（`nfsserve`/`s3s` 库打底，CES 对齐；SMB 不做——无成熟 Rust 库且非目标市场）+
RDMA/零拷贝数据面（Zig 第二落点）。
MDS 分片是全路线图唯一"预案"项：>50 节点才启动，不属于 GPFS 对齐的基本盘。

## 3. 产能与纪律

- P1–P34 合计约 14–20 个月（每周 10–15h）；全职约 6–8 个月。P35–P40 全部承诺交付，排序按真实需求。
- 每个 Gate 不过就不进下一阶段；P6/P13/P18/P34 是"要不要继续"的决策点。
- 人手（AI agent 配额）增加时，第五/六段内部可适度并行（P21–P33 多为正交子系统）。
- 改盘格式前先改 `docs/format.md` 并升 version——格式是契约，代码是仆从。

## 4. 当前行动

1. ✅ 仓库清理、ROADMAP v6、P1（format v0 + extent store + bench + fio 基线）
2. 下一步：**P2 · WAL + 组提交 + 崩溃恢复**
