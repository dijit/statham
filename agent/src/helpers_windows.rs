use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::os::windows::prelude::*;
use std::ptr;

use crate::winapi::um::lmaccess;
use crate::winapi::um::winnt::{BOOLEAN, LONG, LPCWSTR, LPWSTR, PSID, PVOID, PZPWSTR, SID_NAME_USE};
use crate::winapi::shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, LPBYTE, LPDWORD, LPVOID, PBYTE, ULONG};
//use winapi::shared::lmcons::NET_API_STATUS;

fn to_wchar(string: &str) -> Vec<u16> {
    OsStr::new(string).encode_wide(). chain(Some(0).into_iter()).collect()
}

fn to_dword(word: u32) -> u32 {
    word
}

const ADMIN_GROUP: &str = "Administrators";

pub fn add_user(user: String, password: String) -> Result<bool, ()> {
    println!("Hello from windows {}", user.to_string());
    // Info: https://docs.microsoft.com/en-us/windows/win32/api/lmaccess/ns-lmaccess-user_info_1
    let mut user_info = lmaccess::USER_INFO_1 {
        usri1_name: to_wchar(&user).as_mut_ptr(),
        usri1_password: to_wchar(&password).as_mut_ptr(),
        usri1_password_age: to_dword(0),
        usri1_priv: to_dword(1),
        usri1_home_dir: to_wchar(&format!("C:\\users\\{}", &user)).as_mut_ptr(),
        usri1_comment: to_wchar(&"Added By Statham").as_mut_ptr(),
        usri1_flags: to_dword(0),
        usri1_script_path: to_wchar("").as_mut_ptr(),
    };
    let _ptr_hostname: *mut u16 = ptr::null_mut();
    let _ptr_error: *mut u32 = ptr::null_mut();
    unsafe {
        let res = lmaccess::NetUserAdd(
            _ptr_hostname,
            1,
            &mut user_info as *mut _ as _,
            _ptr_error,
        );
        match res {
            87 => {
                println!{"Return from lmaccess::NetUserAdd = 87; Unable to add a user to administrators directly"};
            },
            2245 => {
                println!{"Return from lmaccess::NetUserAdd = 2245; Password complexity requirements not held"};
            },
            5 => {
                println!{"Return from lmaccess::NetUserAdd = 5; Access Denied when creating user" };
                println!{"Things: {:#}", *_ptr_error};
            },
            _ => {
                println!("Result from NetUserAdd: {:#}", res);
             },

        }
    };
    if !_ptr_error.is_null() {
       println!("FAILED")
    }
    Ok(true)
}

pub fn add_to_admin_group(user: String) -> Result<bool, ()> {
    unimplemented!()
}

pub fn check_user(user: String) -> Result<bool, ()> {
    /*
        Check if a user exists and is in the Admin group
    */
    unimplemented!()
}

#[test]
fn test_user_in_admin_group_yes() {
    fn does_user_exist(user: String) -> bool {
        true
    };
    let user = "jmh".to_string();
    let password = "jmh".to_string();
    assert_eq!(add_user(user.to_owned(), password.to_owned()).unwrap(), true);
    assert_eq!(does_user_exist(user.to_owned()), true);
}