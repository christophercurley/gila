fn main() {
    #[cfg(target_os = "windows")]
    {
        // Re-run the build script if the icon changes
        println!("cargo:rerun-if-changed=assets/gila.ico");

        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/gila.ico");
        // (optional) version info (shows in file properties > Details)
        res.set("FileDescription", "Gila");
        res.set("ProductName", "Gila");
        res.set("LegalCopyright", "© 2025 Christopher Curley");
        res.set("OriginalFilename", "Gila.exe");
        res.compile().expect("Failed to compile Windows resources");
    }
}