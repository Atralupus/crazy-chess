fn main() {
    tonic_build::compile_protos("../shared/proto/chess.proto")
        .expect("Failed to compile gRPC definitions");
}
