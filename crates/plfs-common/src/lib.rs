//! plfs-common: shared types and the data-plane wire contract (design doc §5,
//! wire format v1). The `ChunkStore` gRPC service is generated from
//! `proto/plfs/data/v1/chunkstore.proto` — do not edit the proto without
//! bumping the wire-format version.

pub mod data {
    pub mod v1 {
        tonic::include_proto!("plfs.data.v1");
    }
}

#[cfg(test)]
mod tests {
    use super::data::v1::PutRequest;

    #[test]
    fn put_request_fields_roundtrip() {
        let req = PutRequest {
            chunk_id: vec![0xAB; 16],
            version: 42,
            crc32c: 0xDEAD_BEEF,
            payload: b"hello".to_vec(),
            seal: true,
        };
        assert_eq!(req.chunk_id, vec![0xAB; 16]);
        assert_eq!(req.version, 42);
        assert_eq!(req.crc32c, 0xDEAD_BEEF);
        assert_eq!(req.payload, b"hello");
        assert!(req.seal);
    }
}
