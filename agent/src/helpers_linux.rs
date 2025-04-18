use std::io;

const ADMIN_GROUP: &str = "wheel";

pub fn add_user(user: &String, password: &String) -> Result<bool, ()> {
    unimplemented!()
}

pub fn add_to_admin_group(user: &String) -> Result<bool, ()> {
    unimplemented!()
}

pub fn check_user_exists(user: &String) -> bool {
    /*
        Check if a user exists and is in the Admin group
    */
    unimplemented!()
}
pub fn how_many_local_users() -> Result<i32, std::io::Error> {
    Ok(list_users()?.len() as i32)
}
pub fn del_user(user: &String) -> Result<bool, ()> {
    unimplemented!()
}

pub fn list_users() -> Result<Vec<String>, io::Error> {
    let mut users = vec!["root".to_string()];
    let reader = std::fs::read_to_string("/etc/group")?;

    for line in reader.lines() {
        if line.starts_with(ADMIN_GROUP) {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 4 {
                let users_str = parts[3].trim();
                if !users_str.is_empty() {
                    let i_users = users_str.split(',').map(|s| s.to_string()).collect();
                    users.push(i_users)
                }
            }

        }
    }
    users.sort();
    Ok(users)
}
