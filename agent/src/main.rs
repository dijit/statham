#[cfg(target_os = "linux")]
#[path = "helpers_linux.rs"]
mod helpers;

#[cfg(target_os = "windows")]
#[path = "helpers_windows.rs"]
mod helpers;

fn main() {
    helpers::add_user().unwrap();
    println!("Hello, world!");
}
