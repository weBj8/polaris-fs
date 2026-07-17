# PolarisFS 磁盘格式 v2（FORMAT_VERSION = 2）

> 契约文件：改格式先改本文档并升 version。所有多字节整数均为 little-endian。
> 设备 = 单个常规文件或块设备。一切 I/O 以 4KiB（BLOCK）对齐。
> v2 变更（P2 引入）：DATA_START 1MiB→64MiB；原保留区改为检查点区（checkpoint slots）；
> 明确组提交（commit）与持久度地平线语义。v1(v0) 设备不迁移、直接拒绝（v0 未出开发机）。

## 1. 设备布局

```
[0          .. 4KiB)   superblock 副本 A
[4KiB       .. 8KiB)   superblock 副本 B
[8KiB       .. 32MiB)  checkpoint slot A（容量 32MiB-8KiB）
[32MiB      .. 64MiB)  checkpoint slot B（容量 32MiB）
[64MiB(DATA_START) ..) extent log 区：extent 记录顺序追加
```

## 2. Superblock（恰好 4KiB）

| 偏移 | 大小 | 字段 | 说明 |
|---|---|---|---|
| 0    | 8  | magic        | `b"PORFS_SB"` |
| 8    | 4  | format_version | u32，= 2 |
| 12   | 4  | flags        | u32；bit0 = clean_unmount（正常卸载时置位，挂载时清除） |
| 16   | 8  | device_size  | u64，设备总字节数 |
| 24   | 8  | data_start   | u64，= 64MiB |
| 32   | 8  | tail         | u64，下一次追加的字节偏移 |
| 40   | 8  | extent_count | u64，存活 extent 数（不含 tombstone） |
| 48   | 8  | sync_seq     | u64，每次 sync +1；两副本中取 sync_seq 大者 |
| 56   | 16 | uuid         | 文件系统 UUID |
| 72   | 8  | created_at   | unix 秒 |
| 80   | 8  | last_sync_at | unix 秒 |
| 88   | 4004 | reserved   | 全零 |
| 4092 | 4  | sb_crc32c    | u32，对 [0..4092) 的 CRC32C |

打开规则：两副本各自校验 magic + version + CRC；取有效且 sync_seq 大者；
两者皆有效但 tail 不同属正常（后写副本可能未落盘）；皆无效 → 拒绝挂载。
写规则：sync 时**先写副本 B 再写副本 A**（A 为权威），sync_seq 递增。

## 3. Extent 记录

header 64 字节 + data（data_len 字节）+ 零填充至 4KiB 边界。
单条 data_len ≤ EXTENT_DATA_MAX = 4MiB。整段（header+data+pad）称 disk_len，必为 4KiB 倍数。

header 布局（64B）：

| 偏移 | 大小 | 字段 | 说明 |
|---|---|---|---|
| 0  | 4 | magic          | u32 = 0x4558_5431（"EXT1" LE） |
| 4  | 2 | header_len     | u16 = 64 |
| 6  | 2 | flags          | u16；bit0 = tombstone（逻辑删除） |
| 8  | 8 | extent_id      | u64，单调递增，全设备唯一 |
| 16 | 8 | inode          | u64，所属 inode（P1 阶段仅透传记录） |
| 24 | 8 | logical_offset | u64，文件内逻辑字节偏移 |
| 32 | 4 | data_len       | u32，有效数据字节数（不含填充） |
| 36 | 4 | data_crc32c    | u32，对 data[0..data_len) |
| 40 | 4 | disk_len       | u32，header+data+pad 总字节数（4KiB 倍数） |
| 44 | 16 | reserved       | 全零 |
| 60 | 4 | header_crc32c  | u32，对 header[0..60) |

## 4. 检查点区（checkpoint slots）

检查点 = 内存索引（extent_id → 位置）在某 log 位置的一致性快照，使挂载免于全扫。
两个 slot 乒乓写入；slot 内 = 64B header + entry 数组（每条 40B，4KiB 内不跨块即可，
payload 总长按 4KiB 向上取整补零）。

checkpoint header（64B）：

| 偏移 | 大小 | 字段 | 说明 |
|---|---|---|---|
| 0  | 4 | magic           | u32 = 0x3150_4B43（"CKP1" LE） |
| 4  | 2 | header_len      | u16 = 64 |
| 6  | 2 | flags           | u16 = 0 |
| 8  | 8 | slot_gen        | u64，跨两个 slot 单调递增；有效 slot 中取大者 |
| 16 | 8 | covered_tail    | u64，本检查点索引覆盖到的 log 偏移（扫描从该处继续） |
| 24 | 8 | next_extent_id  | u64 |
| 32 | 8 | extent_count    | u64（存活数） |
| 40 | 8 | live_bytes      | u64 |
| 48 | 4 | entry_count     | u32 |
| 52 | 4 | payload_crc32c  | u32，对 payload（entry_count×40B，不含补零） |
| 56 | 4 | reserved        | 全零 |
| 60 | 4 | header_crc32c   | u32，对 header[0..60) |

entry（40B）：`extent_id u64, offset u64, disk_len u32, data_len u32, inode u64, logical_offset u64`。
只存**存活**条目（tombstone 在检查点时刻直接丢弃）。

写协议：`checkpoint()` 时 slot_gen = 最近有效 gen + 1，写入**较旧**的 slot（乒乓），
整 slot 一次 4KiB 对齐写 + fdatasync。读协议：两 slot 各自校验
（magic/header_crc/payload_crc/entry_count×40 ≤ slot 容量），取 slot_gen 大者。
**容量回退**：存活条目数 × 40B + 64B > slot 容量时不写检查点（挂载退化为全扫，永远正确）。

## 5. 打开（mount）流程 v2

1. 读 superblock（取 sync_seq 大者）→ tail、data_start。
2. 读检查点：有效 → 载入索引，从 covered_tail 继续扫描 log 至 tail
   （应用 append/tombstone）；无效或不存在 → 从 DATA_START 全扫。
3. 扫描中**遇到第一条坏记录即停**：tail 截断到该记录起点（log-structured 打捞语义：
   坏点之后的内容视为未确认写，一律丢弃）。

## 6. 组提交（commit）与持久度地平线

log-structured 设计中 **extent log 即 WAL**：append 立即落盘（4KiB 对齐整块写），
但只有在 `sync()`（= fdatasync 数据区 + 双 superblock 更新）之后才算"已确认"。

- **持久度地平线** `confirmed_id`：最近一次成功 sync 时已写入的最大 extent_id；
  id ≤ confirmed_id 的 extent 在崩溃后必须可读（已确认写零丢失）；
  id > confirmed_id 的可能存在也可能被打捞截断（调用方不得依赖）。
- **组提交策略**（实现侧，非格式）：`set_commit_policy(max_pending)` —— 自上次 sync
  累积 max_pending 次 append 后自动 sync（单次 fdatasync 摊销一批写 = 组提交）。
- **自动检查点**：每 CKPT_INTERVAL（=8）次 sync 自动写一次检查点（摊销索引序列化成本）。

## 7. 写路径不变量

- 追加写永远发生在 tail，且一次 sync 批次内 header+data 同一 4KiB 对齐写 I/O。
- 确认顺序：extent 数据 fdatasync 成功 → 才算"已确认"→ 才更新 superblock tail。
  倒挂（superblock 指向未落盘 extent）不允许出现；扫描自愈保证最坏情况是丢未确认写。
- O_DIRECT 优先；不支持的文件系统（tmpfs 等）回退 buffered I/O 并在 open 日志中声明。

## 8. 常量

```
BLOCK_SIZE       = 4096
DATA_START       = 64 MiB
CKPT_SLOT_A_OFF  = 8 KiB
CKPT_SLOT_B_OFF  = 32 MiB
CKPT_SLOT_CAP    = 32 MiB - 8 KiB   (slot A/B 同容量定义，B 实际多 8KiB 但按 A 算)
CKPT_INTERVAL    = 8                (每 8 次 sync 自动检查点)
EXTENT_DATA_MAX  = 4 MiB
SB_MAGIC         = b"PORFS_SB"
EXT_MAGIC        = 0x4558_5431
CKPT_MAGIC       = 0x3150_4B43
FORMAT_VERSION   = 2
```

v3 规划（不进 v2）：free-space 区间表（GC 用，P27 引入）。
