use std::ffi::OsStr;
use std::ptr;
use std::os::windows::ffi::OsStrExt;
#[allow(unused_imports)]
use std::os::windows::prelude::*;

use crate::winapi::um::lmaccess;
#[allow(unused_imports)]
use crate::winapi::um::winnt::{BOOLEAN, LONG, LPCWSTR, LPWSTR, PSID, PVOID, PZPWSTR, SID_NAME_USE};
#[allow(unused_imports)]
use crate::winapi::shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, LPBYTE, LPDWORD, LPVOID, PBYTE, ULONG};
use crate::winapi::shared::lmcons::MAX_PREFERRED_LENGTH;
//use crate::winapi::shared::winerror;

fn to_wchar<'a>(string: &str) -> Vec<u16> {
    OsStr::new(string).encode_wide().chain(Some(0).into_iter()).collect()
}

fn to_dword<'a>(word: u32) -> u32 {
    word
}

const ADMIN_GROUP: &str = "Administrators";

pub fn add_user<'a>(user: &'a String, password: &'a String) -> Result<bool, ()> {
    println!("Hello from windows {}", user.to_string());
    let mut _username: Vec<u16> = to_wchar(&user);
    let mut _password: Vec<u16> = to_wchar(&password);
    let mut _comment: Vec<u16> = to_wchar("Added By Statham");
    let _ptr_hostname: *mut u16 = ptr::null_mut();
    let _ptr_error: *mut u32 = ptr::null_mut();
    // Info: https://docs.microsoft.com/en-us/windows/win32/api/lmaccess/ns-lmaccess-user_info_1
    let mut user_info = lmaccess::USER_INFO_1 {
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
    let res = unsafe {
        lmaccess::NetUserAdd(
            _ptr_hostname,
            1,
            user_info_ptr,
            _ptr_error,
        )
    };
    match res {
        5 => {
            println!("Return from lmaccess::NetUserAdd = 5; Access Denied when creating user");
        },
        87 => {
            println!("Return from lmaccess::NetUserAdd = 87; Unable to add a user to administrators directly");
        },
        2202 => {
            println!("Return from lmaccess::NetUserAdd = 2202; Invalid username or group");
        },
        2224 => {
            println!("Return from lmaccess::NetUserAdd = 2224; User already exists");
        },
        2245 => {
            println!("Return from lmaccess::NetUserAdd = 2245; Password complexity requirements not held");
        },
        _ => {
            println!("Result from NetUserAdd: {:#}", res);
        },
    };
    if !_ptr_error.is_null() {
       println!("FAILED")
    }
    println!("Created user {}", &user);
    Ok(true)
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn add_to_admin_group<'a>(user: &'a String) -> Result<bool, ()> {
    println!("Add user {} to {} called", &user, &ADMIN_GROUP);
    // [LPCWSTR] If this parameter is NULL, the local computer is used.
    let _servername: LPCWSTR = ptr::null_mut(); //in
    // [DWORD] Information level of the group (and which user struct to use)
    let mut _username = to_wchar(user);
    let _groupname: LPCWSTR = to_wchar(ADMIN_GROUP).as_mut_ptr();
    // https://docs.microsoft.com/en-us/windows/win32/api/lmaccess/ns-lmaccess-localgroup_members_info_3
    let mut local_user_buf = lmaccess::LOCALGROUP_MEMBERS_INFO_3 {
        // Docs say this should be `&lt;DomainName&gt;\&lt;AccountName&gt;` but it looks wrong to me
        lgrmi3_domainandname: _username.as_mut_ptr()
    };
    let user_info_ptr = &mut local_user_buf as *mut _ as _;
    let res = unsafe {
        // FIXME: I think this is not actually working
        lmaccess::NetLocalGroupAddMembers(
            _servername,
            _groupname,
            3,
             user_info_ptr,
            1,
        )
    };
    match res {
        2220 => {
            unsafe { println!("Could not find group {}", *_groupname) }
        }
        _ => {
            println!("Return {:?}", res)
        }
    }
    Ok(true)

}

#[allow(dead_code)]
fn get_local_users() -> Vec<String> {
    use winapi::{
        um::lmaccess::{
            NetUserEnum,
            USER_INFO_0
        },
        shared::{
            ntdef::{
                LPCWSTR
            },
            winerror::{
                ERROR_MORE_DATA
            }
        },

    };
    use std::mem::transmute_copy;
    use widestring::WideCString;

    let mut result:Vec<String> = Vec::new();

    let server: LPCWSTR = ptr::null();
    let level: DWORD = 1;
    let filter: DWORD = 0;
    let mut entries_read: DWORD = 0;
    let mut total_entries: DWORD = 0;
    let resume_handle: LPDWORD = ptr::null_mut();

    loop {
        let mut users_buffer_ptr: LPBYTE = ptr::null_mut();
        let api_ret = unsafe {
            NetUserEnum(server,
                        level,
                        filter,
                        &mut users_buffer_ptr,
                        MAX_PREFERRED_LENGTH,
                        &mut entries_read,
                        &mut total_entries,
                        resume_handle)
        };

        // ADD BETTER ERROR HANDLING
        if api_ret != 0 && api_ret != ERROR_MORE_DATA {
            break // API Failed
        }

        let mut tmpbuffer = users_buffer_ptr;  // Copy pointer to temporary buffer
        for _i in 0..entries_read as usize { // Iterate over read entries
            let slice:&[u8] = unsafe {
                std::slice::from_raw_parts(tmpbuffer, 1 as usize)
            };
            let p_user:USER_INFO_0 = unsafe {
                transmute_copy(&slice[0])
            };
            let user_name_wc:WideCString = unsafe {
                WideCString::from_ptr_str(p_user.usri0_name)
            };
            let user_name:String = user_name_wc.to_string_lossy();
            result.push(user_name);
            tmpbuffer = unsafe {
                tmpbuffer.add(56)
            };
        }
        if api_ret != ERROR_MORE_DATA {
            break
        }
    }
    result
}

#[allow(unused_variables)]
pub fn check_user_exists(user: String) -> Result<bool, ()> {
    println!("Check user called with user \"{}\"", &user);
    let users = get_local_users();
    if users.contains(&user) {
        println!("User found");
        Ok(true)
    } else {
        Ok(false)
    }
}

#[allow(dead_code)]
fn how_many_local_users() -> Result<usize, ()> {
    let users = get_local_users();
    Ok(users.len())
}

#[allow(dead_code)]
fn del_user(user: &String) -> Result<bool, ()> {
    println!("Goodbye from windows {}", user.to_string());
    let _ptr_hostname: *mut u16 = ptr::null_mut();
    let mut _username: Vec<u16> = to_wchar(&user);
    unsafe {
        let res = lmaccess::NetUserDel(
            _ptr_hostname,
            _username.as_mut_ptr()
        );
        match res {
            _ => {
                println!("Result from NetUserDel: {:#}", res);
            },
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
    assert_eq!(add_user_to_admins(&user, &ADMIN_GROUP), true);
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
        assert_eq!(number_of_users+1 == how_many_local_users().unwrap(), true);
        assert_eq!(del_user(&user).unwrap(), true);
    };
    let new_number_of_users = how_many_local_users().unwrap();
    assert_eq!(new_number_of_users == number_of_users, true);
}