# PolarisFS 磁盘格式 v0（FORMAT_VERSION = 1）

> 契约文件：改格式先改本文档并升 version。所有多字节整数均为 little-endian。
> 设备 = 单个常规文件或块设备。一切 I/O 以 4KiB（BLOCK）对齐。

## 1. 设备布局

```
[0          .. 4KiB)   superblock 副本 A
[4KiB       .. 8KiB)   superblock 副本 B
[8KiB       .. 1MiB)   保留（全零）
[1MiB(DATA_START) ..)  extent log 区：extent 记录顺序追加
```

## 2. Superblock（恰好 4KiB）

| 偏移 | 大小 | 字段 | 说明 |
|---|---|---|---|
| 0    | 8  | magic        | `b"PORFS_SB"` |
| 8    | 4  | format_version | u32，= 1 |
| 12   | 4  | flags        | u32；bit0 = clean_unmount（正常卸载时置位，挂载时清除） |
| 16   | 8  | device_size  | u64，设备总字节数 |
| 24   | 8  | data_start   | u64，= 1MiB |
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

## 4. 打开（mount）扫描与自愈

从 DATA_START 顺序扫描：读 header → 校验 magic/header_crc → 校验 disk_len 对齐 → 跳 data_len
（可选深校验 data_crc）→ 建立内存索引 extent_id → {offset, disk_len, inode, logical_offset, flags}。
**遇到第一条坏记录即停**：将 tail 截断到该记录起点（log-structured 打捞语义：
坏点之后的内容视为未确认写，一律丢弃）。tombstone 记录使索引中对应 extent_id 失效。

## 5. 写路径不变量

- 追加写永远发生在 tail，且一次 sync 批次内 header+data 同一 4KiB 对齐写 I/O。
- 确认顺序：extent 数据 fdatasync 成功 → 才算"已确认"→ 才更新 superblock tail。
  倒挂（superblock 指向未落盘 extent）不允许出现；扫描自愈保证最坏情况是丢未确认写。
- O_DIRECT 优先；不支持的文件系统（tmpfs 等）回退 buffered I/O 并在 open 日志中声明。

## 6. 常量

```
BLOCK_SIZE       = 4096
DATA_START       = 1 MiB
EXTENT_DATA_MAX  = 4 MiB
SB_MAGIC         = b"PORFS_SB"
EXT_MAGIC        = 0x4558_5431
FORMAT_VERSION   = 1
```

v1 规划（不进 v0）：WAL 区段、检查点区段、free-space 区间表（P2 引入）。
