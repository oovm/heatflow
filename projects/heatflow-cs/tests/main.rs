use std::path::Path;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
pub fn build_cs() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;
    println!("Working directory: {}", here.display());
    csbindgen::Builder::default()
        .input_extern_file(here.join("src/lib.rs").canonicalize()?.to_string_lossy())        // required
        .csharp_dll_name("rs_heatflow")         // required
        .csharp_class_name("RsMethods")     // optional, default: NativeMethods
        .csharp_namespace("RsBind")          // optional, default: CsBindgen
        .csharp_class_accessibility("internal") // optional, default: internal
        .csharp_entry_point_prefix("")          // optional, default: ""
        .csharp_method_prefix("")               // optional, default: ""
        .csharp_use_function_pointer(true)      // optional, default: true
        // .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .generate_csharp_file(here.join("binding/RsMethods.cs"))     // required
        .unwrap();
    Ok(())
}