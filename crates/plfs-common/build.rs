fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(
            &[
                "proto/plfs/data/v1/chunkstore.proto",
                "proto/plfs/meta/v1/metaraft.proto",
                "proto/plfs/registry/v1/registry.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
