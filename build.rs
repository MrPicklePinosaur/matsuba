
// rpc codegen for client and server
fn main() {
    tonic_build::compile_protos("proto/matsuba.proto")
        .unwrap_or_else(|e| panic!("Failed to compile {:?}", e));
}
