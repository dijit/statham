const ADMIN_GROUP: &str = "wheel";

pub fn add_user(user: &String, password: &str) -> Result<bool, ()> {
    unimplemented!()
}

pub fn add_to_admin_group(user: &str) -> Result<bool, ()> {
    unimplemented!()
}

pub fn check_user_exists(user: &String) -> bool {
    /*
        Check if a user exists and is in the Admin group
    */
    if list_users().unwrap().contains(user) {
        return true;
    }
    false
}
pub fn how_many_local_users() -> Result<i32, ()> {
    Ok(list_users()?.len() as i32)
}
pub fn del_user(user: &String) -> Result<bool, ()> {
    unimplemented!()
}

pub fn list_users() -> Result<Vec<String>, ()> {
    // Only list admin users, this is the only way I could do it.
    // It's *almost* worse than the Windows way.
    use std::process::Command;
    let user_grp = "Staff";

    let output = Command::new("dscl")
        .arg(".")
        .arg("-list")
        .arg("/Users")
        .output()
        .expect("Failed to execute dscl command");

    if !output.status.success() {
        eprintln!("Error: Failed to list users");
        return Err(());
    }

    let users = String::from_utf8_lossy(&output.stdout);

    let mut user_vec: Vec<String> = Vec::new();
    for uname in users.lines() {
        let check_output = Command::new("dsmemberutil")
            .arg("checkmembership")
            .arg("-U")
            .arg(uname)
            .arg("-G")
            .arg(user_grp)
            .output()
            .expect("Failed to execute dsmemberutil command");

        let check_result = String::from_utf8_lossy(&check_output.stdout);

        if check_result.contains("is a member") {
            user_vec.push(uname.to_string());
        }
    }

    Ok(user_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_users() {
        assert_eq!(
            list_users().unwrap(),
            vec![std::env::var("USER").unwrap(), "root".to_string()]
        )
    }
    #[test]
    fn test_how_many_local_users() {
        assert_eq!(how_many_local_users().unwrap(), 2)
    }
    #[test]
    fn test_check_user_exists_which_does_exist() {
        assert_eq!(check_user_exists(&"root".to_string()), true)
    }
    #[test]
    fn test_check_user_exists_which_does_not_exist() {
        assert_eq!(check_user_exists(&"averynaughtyperson".to_string()), false)
    }
}
