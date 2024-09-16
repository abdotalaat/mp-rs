fn main() {
    // Ignore errors (the directory may already exist)
    let _ = std::fs::create_dir("src/proto");

    tonic_build::configure()
        .out_dir("src/proto")
        .compile(
            &["proto/coordinator.proto", "proto/worker.proto"],
            &["proto/"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
}
