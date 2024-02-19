fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/*.proto")?;
    tonic_build::configure()
        .out_dir("src")
        .compile(
            &["proto/application.proto", "proto/project.proto"],
            &["proto"],
        )?;
    Ok(())
}
