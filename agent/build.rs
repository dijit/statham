#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("assets/windows_manifest.xml");
        res.set_icon("assets/icon_head.ico");
        res.set_icon_with_id("assets/icon.ico", "2");
        res.set_language(0x0809); // en_GB
        res.compile().unwrap();
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
