use std::io::Error;

#[cfg(target_os = "linux")]
#[path = "helpers_linux.rs"]
mod helpers;

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
#[path = "helpers_windows.rs"]
mod helpers;

#[cfg(target_os = "windows")]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}

fn main() {
    let username = "sth_Admin".to_string();
    let password = "!Password1234*****".to_string();
    //let username = "jmh".to_string();
    match helpers::check_user_exists(&username).unwrap() {
        false => {
            helpers::add_user(&username, &password).unwrap();
            helpers::add_to_admin_group(&username).unwrap();
        },
        true => {
            let msg = format!("User {} already exists", &username);
            print_message(&msg).expect(&msg);
        }
    }
}
