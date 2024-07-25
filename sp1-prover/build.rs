fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/src/proto/aggregator/v1/aggregator.proto")?;
    tonic_build::compile_protos("../proto/src/proto/executor/v1/executor.proto")?;
    tonic_build::compile_protos("../proto/src/proto/hashdb/v1/hashdb.proto")?;
    Ok(())
}
