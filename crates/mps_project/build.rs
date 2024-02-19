fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &["proto/application.proto", "proto/project.proto"],
        &["proto"],
    )?;
    Ok(())
}
