use log::warn;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
#[allow(unused_imports)]
use std::os::windows::prelude::*;
use std::ptr;
use winapi::shared::minwindef::BYTE;
use winapi::um::errhandlingapi::GetLastError;

use std::mem::transmute_copy;
use widestring::WideCString;
use winapi::{
    shared::{
        lmcons::MAX_PREFERRED_LENGTH,
        minwindef::{
            //BOOL,
            //BYTE,
            DWORD,
            //FILETIME,
            LPBYTE,
            LPDWORD,
            LPVOID,
            //PBYTE,
            //ULONG,
        },
        ntdef::LPCWSTR,
        winerror::ERROR_MORE_DATA,
    },
    um::lmaccess::{
        NetLocalGroupAddMembers, NetUserAdd, NetUserDel, NetUserEnum, LOCALGROUP_MEMBERS_INFO_3,
        USER_INFO_0, USER_INFO_1,
    },
    um::lmapibuf::NetApiBufferFree,
};

macro_rules! wprintln {
    ($fmt:expr) => (print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\r\n"), $($arg)*));
}

fn to_wchar<'a>(string: &str) -> Vec<u16> {
    OsStr::new(string)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}

fn to_dword<'a>(word: u32) -> u32 {
    word
}

const ADMIN_GROUP: &str = "Administrators";

pub fn add_user<'a>(user: &'a String, password: &'a String) -> Result<bool, ()> {
    let mut _username: Vec<u16> = to_wchar(&user);
    let mut _password: Vec<u16> = to_wchar(&password);
    let mut _comment: Vec<u16> = to_wchar("Added By Statham");
    let _ptr_hostname: *mut u16 = ptr::null_mut();
    let _ptr_error: *mut u32 = ptr::null_mut();
    // Info: https://docs.microsoft.com/en-us/windows/win32/api/lmaccess/ns-lmaccess-user_info_1
    let mut user_info = USER_INFO_1 {
        usri1_name: _username.as_mut_ptr(),
        usri1_password: _password.as_mut_ptr(),
        usri1_password_age: to_dword(0),
        usri1_priv: to_dword(1),
        usri1_home_dir: to_wchar(&format!("C:\\users\\{}", &user)).as_mut_ptr(),
        usri1_comment: _comment.as_mut_ptr(),
        usri1_flags: to_dword(0),
        usri1_script_path: to_wchar(&"").as_mut_ptr(),
    };
    let user_info_ptr = &mut user_info as *mut _ as _;
    let res = unsafe { NetUserAdd(_ptr_hostname, 1, user_info_ptr, _ptr_error) };
    match res {
        5 => {
            wprintln!("Return from lmaccess::NetUserAdd = 5; Access Denied when creating user");
        }
        87 => {
            wprintln!("Return from lmaccess::NetUserAdd = 87; Unable to add a user to administrators directly");
        }
        2202 => {
            wprintln!("Return from lmaccess::NetUserAdd = 2202; Invalid username or group");
        }
        2224 => {
            wprintln!("Return from lmaccess::NetUserAdd = 2224; User already exists");
        }
        2245 => {
            wprintln!("Return from lmaccess::NetUserAdd = 2245; Password complexity requirements not held");
        }
        _ => {
            wprintln!("Result from NetUserAdd: {:#}", res);
        }
    };
    if !_ptr_error.is_null() {
        wprintln!("FAILED")
    }
    wprintln!("Created user {}", &user);
    Ok(true)
}

pub fn add_to_admin_group<'a>(user: &'a String) -> Result<bool, bool> {
    // [LPCWSTR] If this parameter is NULL, the local computer is used.
    let _servername: LPCWSTR = ptr::null_mut(); //in
                                                // [DWORD] Information level of the group (and which user struct to use)
    let mut _username = to_wchar(user);
    let _groupname: LPCWSTR = to_wchar(ADMIN_GROUP).as_mut_ptr();
    // https://docs.microsoft.com/en-us/windows/win32/api/lmaccess/ns-lmaccess-localgroup_members_info_3
    let mut local_user_buf = LOCALGROUP_MEMBERS_INFO_3 {
        // Docs say this should be `&lt;DomainName&gt;\&lt;AccountName&gt;` but it looks wrong to me
        lgrmi3_domainandname: _username.as_mut_ptr(),
    };
    let user_info_ptr = &mut local_user_buf as *mut _ as _;
    let res = unsafe {
        NetLocalGroupAddMembers(
            _servername,
            to_wchar(&ADMIN_GROUP).as_mut_ptr(),
            3,
            user_info_ptr,
            1,
        )
    };
    match res {
        2220 => {
            wprintln!(
                "NetLocalGroupAddMembers: Could not find group {}",
                ADMIN_GROUP
            );
            Err(false)
        }
        0 => Ok(true),
        _ => {
            wprintln!("Result from NetLocalGroupAddMembers: {:?}", res);
            Err(false)
        }
    }
}

fn get_local_users() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let server: LPCWSTR = ptr::null();
    let level: DWORD = 1; // level 1 means USER_INFO_1, level 0 means USER_INFO_0
    let filter: DWORD = 0;
    let mut entries_read: DWORD = 0;
    let mut total_entries: DWORD = 0;
    let mut resume_handle: DWORD = 0;
    loop {
        // We must create a new USER_INFO_1 structure each time we call this function
        let mut users_buffer_ptr: LPBYTE = ptr::null_mut();
        let api_ret = unsafe {
            // https://learn.microsoft.com/en-us/windows/win32/api/lmaccess/nf-lmaccess-netuserenum
            // crate: https://docs.rs/winapi/0.3.9/winapi/um/lmaccess/fn.NetUserEnum.html
            NetUserEnum(
                server,
                level,
                filter,
                &mut users_buffer_ptr,
                MAX_PREFERRED_LENGTH,
                &mut entries_read,
                &mut total_entries,
                &mut resume_handle,
            )
        };

        scopeguard::defer! {
            unsafe { NetApiBufferFree(users_buffer_ptr as LPVOID); } // no memory clean up? for shame :|
        }

        if result.is_empty() {
            let err = unsafe { GetLastError() };
            println!("Query failed with error: {}", err);
        }

        // ADD BETTER ERROR HANDLING
        if api_ret != 0 && api_ret != ERROR_MORE_DATA {
            break; // API Failed
        }

        let user_infos_buffer: &[u8] = unsafe {
            std::slice::from_raw_parts(
                users_buffer_ptr,
                ::core::mem::size_of::<USER_INFO_1>() * (entries_read as usize),
            )
        };
        let (_, user_info_structs_body, _) = unsafe { user_infos_buffer.align_to::<USER_INFO_1>() };
        let loop_result = user_info_structs_body
            .iter()
            .map(|p_user| {
                let user_name_wc: WideCString =
                    unsafe { WideCString::from_ptr_str(p_user.usri1_name) };
                user_name_wc.to_string_lossy()
            })
            .collect::<Vec<_>>();

        result.extend(loop_result);

        if api_ret != ERROR_MORE_DATA {
            break;
        }
    }
    result
}

#[allow(unused_variables)]
pub fn check_user_exists(user: &String) -> Result<bool, ()> {
    let users = get_local_users();
    if users.contains(&user) {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Debug)]
pub struct USER_NAME {
    usri0_name: LPCWSTR,
}

#[allow(dead_code)] // used for tests
pub fn how_many_local_users() -> Result<usize, ()> {
    let server_name: LPCWSTR = ptr::null();
    let level: DWORD = 0;
    let filter: DWORD = 0x0002;
    let mut entries_read: DWORD = 0;
    let mut total_entries: DWORD = 0;
    let resume_handle: LPDWORD = ptr::null_mut();
    //let users: Vec<USER_NAME> = Vec::with_capacity(usize::MAX);
    unsafe {
        let mut users_buffer_ptr: LPBYTE = ptr::null_mut();
        let net_user_enum_ret = NetUserEnum(
            server_name,
            level,
            filter,
            &mut users_buffer_ptr,
            MAX_PREFERRED_LENGTH,
            &mut entries_read,
            &mut total_entries,
            resume_handle,
        );
        warn!("function_return: {:?}", net_user_enum_ret);
        let mut users_vec: Vec<USER_NAME> = Vec::with_capacity(total_entries as usize);
        let _unknown = users_vec.as_mut_ptr() as *mut Vec<USER_NAME>;
        //let raw_ptr  =  users_buffer_ptr.offset_from(0xf0 as *const BYTE) as *mut Vec<USER_NAME>;
        //let rust_reference = unsafe { raw_ptr.as_mut() };
        //if rust_reference.is_none() { println!("you fcked up") };
        //let rust_reference = rust_reference.unwrap();
        warn!("users_buffer_ptr: {:?}", users_buffer_ptr);
    };
    Ok(total_entries as usize)
}

pub fn del_user(user: &String) -> Result<bool, ()> {
    let _ptr_hostname: *mut u16 = ptr::null_mut();
    let mut _username: Vec<u16> = to_wchar(&user);
    unsafe {
        let res = NetUserDel(_ptr_hostname, _username.as_mut_ptr());
        match res {
            _ => {
                wprintln!("Result from NetUserDel: {:#}", res);
            }
        };
        //std::mem::forget(res);
    }
    Ok(true)
}

#[test]
fn test_check_user_exists() {
    // Guest should always exist
    let user = "Guest".to_string();
    assert_eq!(check_user_exists(&user).unwrap(), true);
    let bad_user = "bad_jmh".to_string();
    assert_eq!(check_user_exists(&bad_user).unwrap(), false);
}

/*
#[test]
fn test_user_in_admin_group_yes() {
    let user = "jmh".to_string();
    let password = "JanHarasym123!!@@".to_string();
    assert_eq!(add_user(&user, &password).unwrap(), true);
    assert_eq!(add_user_to_admins(&user), true);
    assert_eq!(check_user_exists(&user), true);
    assert_eq!(check_user_is_admin(&user), true);
    del_user(&user);
}
*/

#[test]
fn test_many_users() {
    let number_of_users = how_many_local_users().unwrap();
    for i in 0..99 {
        let user = format!("sth_jmh_{}", i).to_string();
        let password = "JanHarasym123!!@@".to_string();
        assert_eq!(add_user(&user, &password).unwrap(), true);
        assert_eq!(number_of_users + 1 == how_many_local_users().unwrap(), true);
        assert_eq!(del_user(&user).unwrap(), true);
    }
    let new_number_of_users = how_many_local_users().unwrap();
    assert_eq!(new_number_of_users == number_of_users, true);
}
