fn main() {
    // if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
    //     let mut res = winresource::WindowsResource::new();
    //     res.set_manifest_file("resources/manifest.xml");
    //     res.set_icon("resources/me3.ico");
    //     res.compile().unwrap();
    // }

    println!("cargo::rerun-if-changed=ui/create-profile.slint");
    slint_build::compile("ui/create-profile.slint").unwrap();
}
