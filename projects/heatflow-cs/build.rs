// copy binding/RsMethods.cs to target/debug/build/heatflow-cs-*/out/RsMethods.cs
fn main() -> std::io::Result<()> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
    let source = here.join("binding/RsMethods.cs").canonicalize()?;
    println!("cargo:rerun-if-changed={}", source.display());
    let target = if cfg!(debug_assertions) {
        here.join("../../target/debug/RsMethods.cs")
    } else {
        here.join("../../target/release/RsMethods.cs")
    };
    std::fs::copy(source, target)?;
    Ok(())
}