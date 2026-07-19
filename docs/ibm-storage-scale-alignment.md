# IBM Storage Scale / GPFS alignment contract

## Purpose and boundary

PolarisFS is a clean-room implementation targetting IBM Storage Scale (GPFS)
compatibility. It must not copy IBM source code, binaries, or documentation
text. It must, however, reproduce the documented administrative model,
observable filesystem semantics, failure and recovery behavior, and command
line interface as closely as the platform permits. “Similar effect” is not an
acceptance criterion.

This document is the compatibility contract. `ROADMAP.md` schedules the work;
the IBM command reference remains the authoritative complete command list and
syntax source. Every implemented compatibility command must record the exact
IBM Storage Scale release and reference page it was compared against.

## IBM reference baseline

The baseline is the IBM Storage Scale documentation current at the start of
each implementation phase. Release-pinned behavior must be selected before
writing a command or on-disk compatibility feature; do not silently mix
releases.

- [Documentation home](https://www.ibm.com/docs/en/storage-scale)
- [Quick reference](https://www.ibm.com/docs/en/storage-scale/latest?topic=overview-quick-reference)
- [Command reference](https://www.ibm.com/docs/en/storage-scale/latest?topic=reference-command)
- [Architecture overview](https://www.ibm.com/docs/en/storage-scale/latest?topic=architecture-storage-scale-overview)
- [Cluster nodes and roles](https://www.ibm.com/docs/en/storage-scale/latest?topic=clusters-cluster-nodes-node-roles)
- [Network Shared Disks (NSDs)](https://www.ibm.com/docs/en/storage-scale/latest?topic=concepts-network-shared-disks-nsds)
- [Storage pools](https://www.ibm.com/docs/en/storage-scale/latest?topic=pools-storage-pool-concepts)
- [Metadata and data pools](https://www.ibm.com/docs/en/storage-scale/latest?topic=management-metadata-data-pools)
- [Filesets](https://www.ibm.com/docs/en/storage-scale/latest?topic=systems-managing-filesets)
- [Cluster Export Services (CES)](https://www.ibm.com/docs/en/storage-scale/latest?topic=clusters-cluster-export-services)

## Architecture alignment

IBM Storage Scale is a role-based clustered filesystem centered on cluster
membership, shared-storage abstractions (NSDs), storage pools, metadata and
data placement, filesets, and optional protocol services through CES.
PolarisFS must align each of these concepts before claiming parity:

| IBM concept | Required PolarisFS alignment |
|---|---|
| Cluster, node roles, daemon lifecycle, quorum, and membership | The same administrative objects, lifecycle states, failure boundaries, and fencing/recovery guarantees. |
| NSD and disk management | An explicit disk/NSD compatibility layer, including identity, failure groups, add/change/remove, and operational state. A chunkserver is not an undocumented substitute. |
| Filesystem, storage pool, and metadata/data placement | Matching configuration objects and placement/recovery semantics, not merely equivalent throughput. |
| Filesets, junctions, snapshots, quotas, and policy scopes | The same scope boundaries and interactions, including snapshot/clone and policy effects. |
| Token/lock and cache-coherence model | GPFS-grade token/lease arbitration, revocation, epoch fencing, and failure recovery. Close-to-open is only an interim state. |
| CES and protocol services | Explicit service lifecycle, export configuration, authentication, and health behavior. |

An implementation choice may differ internally only when it preserves the
documented object model and all relevant externally observable behavior. Any
unavoidable deviation requires an entry in the compatibility matrix, a user
visible diagnostic, tests, and an approved roadmap item to remove it. No
feature may be described as “GPFS parity” while such a deviation remains.

## CLI compatibility rules

1. The target administrative ABI is IBM's task-specific `mm*` command family,
   not a single PolarisFS-only shell. New administrative capabilities therefore
   receive their corresponding `mm*` command name, operands, option spelling,
   defaults, exit status class, and machine-readable output where IBM specifies
   them.
2. Existing `porfs` and `porfsadm` commands are bootstrap/developer interfaces,
   not evidence of CLI parity. They remain documented as transitional until
   their compatible `mm*` replacement exists.
3. Commands must validate prerequisites, privilege, node selection, and
   destructive-operation confirmation at the same boundary as the IBM command.
   A renamed command with weaker validation is not compatible.
4. Help text, diagnostics, and output must be original; compatibility concerns
   syntax and semantics, not copying IBM prose. Tests compare structured
   behavior, accepted/rejected invocations, exit classes, and stable field
   names—not copyrighted text.
5. Each command has a version-pinned specification record: IBM command page,
   supported operands/options, output contract, unsupported cases, tests, and
   phase gate. Unsupported options fail explicitly; they must never be ignored.
6. Packaging must prevent an accidental collision with a locally installed IBM
   Storage Scale command set. The production compatibility package owns `mm*`;
   developer builds use an explicit compatibility command directory.

## Command-family inventory and implementation map

This is the planning inventory, grouped from IBM's quick and command
references. It intentionally does not replace the authoritative reference:
before a family starts, its phase must import the full release-pinned command
and option inventory into the compatibility matrix.

| Family | IBM command surface to align | PolarisFS phase |
|---|---|---|
| Cluster lifecycle | `mmcrcluster`, `mmchcluster`, `mmlscluster`, `mmaddnode`, `mmchnode`, `mmrmnode`, `mmstartup`, `mmshutdown`, `mmgetstate`, `mmnetverify`, `mmruncommand` | P14–P16, P32 |
| Filesystem lifecycle | `mmcrfs`, `mmchfs`, `mmlsfs`, `mmdelfs`, `mmmount`, `mmumount` | P13.5, P16 |
| NSD, disk, and pool administration | `mmcrnsd`, `mmchnsd`, `mmlsnsd`, `mmdelnsd`, `mmadddisk`, `mmchdisk`, `mmdeldisk`, `mmlsdisk`, `mmrestripefs` | P25, P28–P30 |
| Filesets and namespace | `mmcrfileset`, `mmchfileset`, `mmlinkfileset`, `mmunlinkfileset`, `mmlsfileset` | P22 |
| Snapshots and clones | `mmcrsnapshot`, `mmlssnapshot`, `mmdelsnapshot` | P21–P22 |
| Quotas and attributes | `mmsetquota`, `mmlsquota`, `mmrepquota`, `mmchattr`, `mmlsattr` | P23, P24, P38 |
| Policy and data lifecycle | `mmapplypolicy`, `mmlspolicy`, policy-related `mmch*` controls | P25–P27 |
| Health, performance, and support | `mmhealth`, `mmpmon`, `mmperfmon`, `mmfsadm`, `mmfsck` | P12, P31 |
| Backup and recovery | `mmbackup` and IBM-documented restore/recovery commands | P33 |
| Security and key management | `mmauth`, `mmkeyserv`, encryption-related `mm*` controls | P36 |
| AFM and multi-cluster | `mmafmctl` and AFM-related `mmchfileset` operations | P39 |
| CES/protocol services | `mmces` and the IBM-documented protocol service commands | P40 |

## Current status

Current commands are `porfs mkfs`, `info`, `bench`, `mds-check`, `mount`, and
`chunkserver`, plus `porfsadm status`. They are PolarisFS-native bootstrap
interfaces and do **not** yet have IBM Storage Scale command-line parity.
They must not be presented as substitutes for the command families above.

P13.5 establishes the compatibility harness and maps the current bootstrap
workflow to the filesystem-lifecycle family first. Later phases add their
corresponding families only after their underlying IBM-equivalent object model
and recovery behavior exist.

## Per-command acceptance record

Each command implementation must add a row to the compatibility matrix with:

| Field | Requirement |
|---|---|
| Reference | IBM Storage Scale release, command-reference URL, and access date |
| Invocation | Supported operands/options, defaults, mutual exclusions, and privilege/node rules |
| Results | Exit-status classes, stable output fields, diagnostics, and idempotency |
| Semantics | Cluster object mutations, failure/retry behavior, and recovery effects |
| Evidence | Positive, negative, multi-node, upgrade, and fault-injection tests |
| Gap | Explicit deviation, why it exists, user impact, and the removal phase |

The phase gate is complete only when all rows in that phase's imported command
inventory are implemented or explicitly rejected as out of product scope by a
recorded product decision. There is no silent “effectively equivalent” pass.
