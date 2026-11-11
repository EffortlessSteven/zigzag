fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=packaging/windows/AppIcon.ico");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // On Windows every ZigZag .exe gets its icon and version metadata from a
    // resource compiled in by winresource. Elsewhere this file is a no-op.
    #[cfg(windows)]
    {
        let icon = std::path::Path::new("packaging/windows/AppIcon.ico");
        if !icon.exists() {
            println!(
                "cargo:warning=packaging/windows/AppIcon.ico is missing; \
                 run packaging/windows/make-icon.sh to regenerate it."
            );
            return;
        }
        let mut res = winresource::WindowsResource::new();
        res.set_icon(icon.to_str().unwrap());
        res.set("ProductName", "ZigZag");
        res.set("FileDescription", "ZigZag embroidery studio");
        res.set("CompanyName", "ZigZag");
        res.set("LegalCopyright", "Apache-2.0 licensed. See NOTICE.");
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));
        if let Err(e) = res.compile() {
            println!("cargo:warning=winresource compile failed: {e}");
        }
    }
}
