//! Mount-time configuration for [`crate::PorfsFs`]: attribute-cache TTLs.
//!
//! The kernel caches attributes and lookup results for these durations.
//! Shorter TTLs trade metadata freshness for performance. Regular-file opens
//! always revalidate at the MDS and use direct I/O, so cached data never
//! survives across an open; namespace and attribute observations may still
//! remain stale until their configured TTL expires.

use std::time::Duration;

/// Attribute-cache timeouts reported in lookup/getattr/setattr replies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MountConfig {
    /// TTL for `getattr`/`setattr` attribute replies (default 1s).
    pub attr_ttl: Duration,
    /// TTL for positive `lookup`/`create`/`mkdir`/`link` entry replies
    /// (default 1s).
    pub entry_ttl: Duration,
    /// TTL intended for negative (`ENOENT`) lookups (default 0 = no caching).
    /// Reserved: FUSE error replies carry no timeout field, so v0 cannot
    /// convey it to the kernel; the field documents the intended policy for
    /// when the transport grows the knob.
    pub negative_ttl: Duration,
    /// Mount with `default_permissions`: the kernel enforces mode/uid/gid
    /// on every access (default true). Disable for benches and for the
    /// in-process mount tests, where the mount owner intentionally differs
    /// from the inode owners.
    pub default_permissions: bool,
}

impl Default for MountConfig {
    fn default() -> Self {
        Self {
            attr_ttl: Duration::from_secs(1),
            entry_ttl: Duration::from_secs(1),
            negative_ttl: Duration::ZERO,
            default_permissions: true,
        }
    }
}
