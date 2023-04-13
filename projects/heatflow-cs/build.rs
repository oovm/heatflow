// copy binding/RsMethods.cs to target/debug/build/heatflow-cs-*/out/RsMethods.cs
fn main() -> std::io::Result<()> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
    let source = here.join("binding/RsMethods.cs").canonicalize()?;
    println!("cargo:rerun-if-changed={}", source.display());
    let target = here.join("../../../target/release/RsMethods.cs").canonicalize()?;
    std::fs::copy(source, target)?;
    Ok(())
}