fn main() {
    #[cfg(target_os = "windows")]
    {
        #[cfg(target_env = "msvc")]
        {
            // todo(windows): This is to avoid stack overflow. Remove it when solved.
            println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
        }

        let manifest = std::path::Path::new("resources/windows/manifest.xml");
        let icon = std::path::Path::new("resources/windows/app-icon.ico");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rerun-if-changed={}", icon.display());
    
        embed_manifest::embed_manifest(embed_manifest::new_manifest(manifest.to_str().unwrap()))
            .unwrap();

        let mut res = winresource::WindowsResource::new();
        res.set_icon(icon.to_str().unwrap());
        if let Err(e) = res.compile() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}