use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .all_rustc()
        .all_sysinfo()
        .emit()?;

    tonic_build::configure().compile(
        &["proto/application.proto", "proto/project.proto"],
        &["proto"],
    )?;

    Ok(())
}
