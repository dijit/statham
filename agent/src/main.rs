#![windows_subsystem = "windows"]
use crate::password_generator::{generate_password, Case, List};
use log::warn;
use std::io::Error;

#[cfg(target_os = "linux")]
#[path = "helpers_linux.rs"]
mod helpers;
#[cfg(target_os = "macos")]
#[path = "helpers_macos.rs"]
mod helpers;

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
#[path = "helpers_windows.rs"]
mod helpers;
mod password_generator;

#[cfg(target_os = "windows")]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MessageBoxW, MB_OK};
    let title: Vec<u16> = OsStr::new("Statham Password Demo")
        .encode_wide()
        .chain(once(0))
        .collect();
    let content: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe { MessageBoxW(null_mut(), content.as_ptr(), title.as_ptr(), MB_OK) };
    if ret == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(ret)
    }
}
#[cfg(not(target_os = "windows"))]
fn print_message(msg: &str) -> Result<i32, Error> {
    println!("{}", msg);
    Ok(0)
}

fn main() {
    /*
    let out = format!(
        "local user count: {}",
        helpers::how_many_local_users().expect("Failure to call")
    );
    print_message(&out).expect("Failed to write");
     */

    let username = "sth_Admin".to_string();
    let password = generate_password(List::Short2, Case::Mixed, 4, "-".to_string());
    println!("Password generated is: {}", &password);
    /*
    match helpers::check_user_exists(&username) {
        true => println!("Discovered that the user exists"),
        false => println!("Did not find that user"),
    };
    match helpers::check_user_exists(&username) {
        false => {
            let msg = format!("User {} not found, adding now", &username);
            print_message(&msg).expect(&msg);
            helpers::add_user(&username, &password).unwrap();
            helpers::add_to_admin_group(&username).unwrap();
        },
        true => {
            let msg = format!("User {} exists, deleting now", &username);
            print_message(&msg).expect(&msg);
            helpers::del_user(&username).expect("Unable to delete user");
        }
    }
     */
}
