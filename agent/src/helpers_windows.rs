use log::warn;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
#[allow(unused_imports)]
use std::os::windows::prelude::*;
use std::ptr;
use winapi::um::errhandlingapi::GetLastError;

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

#[derive(Debug)]
struct UserInfo0Native {
    #[allow(dead_code)]
    usri1_name: String,
}
#[derive(Debug)]
struct UserInfo1Native {
    // https://learn.microsoft.com/en-us/windows/win32/api/lmaccess/ns-lmaccess-user_info_1
    usri1_name: String,
    #[allow(dead_code)]
    usri1_password_age: u32,
    #[allow(dead_code)]
    usri1_priv: u32,
    #[allow(dead_code)]
    usri1_home_dir: String,
    #[allow(dead_code)]
    usri1_comment: String,
    #[allow(dead_code)]
    usri1_flags: u32,
    #[allow(dead_code)]
    usri1_script_path: String,
}

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
        0 => {}
        5 => {
            wprintln!("[ERR] lmaccess::NetUserAdd CODE 5; Access Denied when creating user");
        }
        87 => {
            wprintln!("[ERR] lmaccess::NetUserAdd CODE 87; Unable to add a user to administrators directly");
        }
        2202 => {
            wprintln!("[ERR] lmaccess::NetUserAdd CODE 2202; Invalid username or group");
        }
        2224 => {
            wprintln!("[ERR] lmaccess::NetUserAdd CODE 2224; User already exists");
        }
        2245 => {
            wprintln!("[ERR] lmaccess::NetUserAdd CODE 2245; Password complexity requirements not held");
        }
        _ => {
            wprintln!("[ERR] lmaccess::NetUserAdd CODE {:#}: Unknown to this program", res);
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
                "[ERR] lmaccess::NetLocalGroupAddMembers CODE 2220: Could not find group {}",
                ADMIN_GROUP
            );
            Err(false)
        }
        0 => Ok(true),
        _ => {
            wprintln!("[ERR] lmaccess::NetLocalGroupAddMembers CODE {:?}: Unknown to this program", res);
            Err(false)
        }
    }
}

fn get_local_users() -> Vec<UserInfo1Native> {
    let mut result: Vec<UserInfo1Native>;

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

        // Cleanup memory after we exit this scope
        scopeguard::defer! {
            unsafe { NetApiBufferFree(users_buffer_ptr as LPVOID); }
        }

        let user_infos_buffer: &[u8] = unsafe {
            std::slice::from_raw_parts(
                users_buffer_ptr,
                ::core::mem::size_of::<USER_INFO_1>() * (entries_read as usize),
            )
        };
        let (_, user_info_structs_body, _) = unsafe { user_infos_buffer.align_to::<USER_INFO_1>() };
        result = user_info_structs_body
            .iter()
            .map(|info| {
                let user_name_wc: WideCString =
                    unsafe { WideCString::from_ptr_str(info.usri1_name) };
                let user_homedir_wc: WideCString =
                    unsafe { WideCString::from_ptr_str(info.usri1_home_dir) };
                let user_comment_wc: WideCString =
                    unsafe { WideCString::from_ptr_str(info.usri1_comment) };
                let user_script_wc: WideCString =
                    unsafe { WideCString::from_ptr_str(info.usri1_script_path) };

                UserInfo1Native {
                    usri1_name: user_name_wc.to_string_lossy(),
                    usri1_password_age: info.usri1_password_age,
                    usri1_priv: info.usri1_priv,
                    usri1_home_dir: user_homedir_wc.to_string_lossy(),
                    usri1_comment: user_comment_wc.to_string_lossy(),
                    usri1_flags: info.usri1_flags,
                    usri1_script_path: user_script_wc.to_string_lossy(),
                }
            })
            .collect::<Vec<_>>();

        //result.extend(loop_result);

        if result.is_empty() {
            let err = unsafe { GetLastError() };
            println!("Query failed with error: {}", err);
        }

        if api_ret != ERROR_MORE_DATA {
            break;
        }
    }
    result
}

pub fn check_user_exists(user: &String) -> bool {
    get_local_users().iter().any(|u| &u.usri1_name == user)
}

#[allow(dead_code)] // used for tests
pub fn how_many_local_users() -> Result<usize, ()> {
    let server_name: LPCWSTR = ptr::null();
    let level: DWORD = 0;
    let filter: DWORD = 0x0002;
    let mut entries_read: DWORD = 0;
    let mut total_entries: DWORD = 0;
    let resume_handle: LPDWORD = ptr::null_mut();
    let mut users_buffer_ptr: LPBYTE = ptr::null_mut();
    unsafe {
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
    };

    scopeguard::defer! {
            unsafe { NetApiBufferFree(users_buffer_ptr as LPVOID); } // no memory clean up? for shame :|
    };

    let user_infos_buffer: &[u8] = unsafe {
        std::slice::from_raw_parts(
            users_buffer_ptr,
            ::core::mem::size_of::<USER_INFO_0>() * (entries_read as usize),
        )
    };
    let (_, user_info_structs_body, _) = unsafe { user_infos_buffer.align_to::<USER_INFO_0>() };
    let result = user_info_structs_body
        .iter()
        .map(|p_user| {
            let user_name_wc: WideCString = unsafe { WideCString::from_ptr_str(p_user.usri0_name) };
            user_name_wc.to_string_lossy()
        })
        .collect::<Vec<_>>();
    warn!("users_buffer_ptr: {:?}", result);

    Ok(total_entries as usize)
}

pub fn del_user(user: &String) -> Result<bool, ()> {
    let _ptr_hostname: *mut u16 = ptr::null_mut();
    let mut _username: Vec<u16> = to_wchar(&user);
    unsafe {
        let res = NetUserDel(_ptr_hostname, _username.as_mut_ptr());
        match res {
            0 => {}
            2221 => wprintln!("[ERR] 2221 lmaccess::NetUserDel: User {} could not be found", user),
            _ => {
                wprintln!("[ERR] {:#} from lmaccess::NetUserDel: unknown to this program", res);
            }
        };
        //std::mem::forget(res);
    }
    wprintln!("Deleted user {}", &user);
    Ok(true)
}

#[test]
fn test_check_user_exists() {
    // Guest should always exist
    let user = "Guest".to_string();
    assert_eq!(check_user_exists(&user), true);
    let bad_user = "bad_jmh".to_string();
    assert_eq!(check_user_exists(&bad_user), false);
}

#[test]
fn test_check_local_user_count() {
    assert_eq!(how_many_local_users().unwrap(), 4);
}

#[test]
fn test_user_in_admin_group_yes() {
    let user = "jmh".to_string();
    let password = "JanHarasym123!!@@".to_string();
    assert!(add_user(&user, &password).unwrap());
    //assert_eq!(add_user_to_admins(&user), true);
    assert!(
        check_user_exists(&user)
    );
    //assert_eq!(check_user_is_admin(&user), true);
    match del_user(&user){
        Ok(true) => wprintln!("Successfully deleted {:?}", &user),
        _ => wprintln!("Did not successfully delete: {:?}", &user)
    };
}

#[test]
fn test_many_users() {
    let number_of_users = how_many_local_users().unwrap();
    for i in 0..100 {
        let user = format!("sth_jmh_{}", i).to_string();
        let password = "JanHarasym123!!@@".to_string();
        let before = how_many_local_users().unwrap();
        assert!(add_user(&user, &password).unwrap());
        assert_eq!(before + 1, how_many_local_users().unwrap());
    };
    wprintln!("New number of users is {}", how_many_local_users().unwrap());
    assert_eq!(number_of_users+100, how_many_local_users().unwrap());
    for i in 0..100 {
        let user = format!("sth_jmh_{}", i).to_string();
        assert!(del_user(&user).unwrap());
    };
    let new_number_of_users = how_many_local_users().unwrap();
    assert_eq!(new_number_of_users == number_of_users, true);
}
