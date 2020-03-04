#![feature(rustc_private)]
extern crate winapi;
extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_language(winapi::um::winnt::MAKELANGID(
            winapi::um::winnt::LANG_ENGLISH,
            winapi::um::winnt::SUBLANG_ENGLISH_US
        ));
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
}