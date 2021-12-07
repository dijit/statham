use crate::winapi::um::lmaccess;

const ADMIN_GROUP: String = "Administrators".to_string();

pub fn add_user() -> Result<bool, Err> {
    unimplemented!()
}

pub fn add_to_admin_group(&user: String) -> Result<bool, Err> {
    unimplemented!()
}

pub fn check_user(&user: String) -> Result<bool, Err> {
    /*
        Check if a user exists and is in the Admin group
    */
    unimplemented!()
}