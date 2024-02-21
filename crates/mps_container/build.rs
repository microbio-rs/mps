fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["proto/docker.proto"], &["proto"])?;
    Ok(())
}
