fn main() {
    tonic_build::configure()
        .compile(
            &[
                "proto/geyser.proto",
                "proto/solana_storage.proto",
            ],
            &["proto"],
        )
        .expect("failed to compile protos");
}
