//! FUSE client + cache + WAL — implemented from S6.

#[cfg(test)]
mod tests {
    #[test]
    fn shell_crate_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "plfs-client");
    }
}
