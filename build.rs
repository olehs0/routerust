fn main() {
    tonic_build::compile_protos("proto/route.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    tonic_build::compile_protos("proto/car.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
