use super::{MyPgDatabase};
//use std::env;

#[get("/by_machine_name/<hostname>/<user>")]
pub async fn get_by_username_and_hostname(pg: MyPgDatabase, user: String, hostname: String) -> Result<String, String> {
    assert_ne!(user, "".to_string(), "Username can't be empty");
    assert_ne!(hostname, "".to_string(), "Hostname can't be empty");
    let query = "SELECT password FROM statham_passwords \
                        JOIN statham_users ON statham_users.id = user_id \
                        JOIN statham_servers on statham_servers.id = machine_id \
                        WHERE username = $1::text and hostname = $2::text";
    let mut ret: String = "".to_string();
    for row in pg.run(
            move |c| c.query(query, &[&user, &hostname]))
            .await.unwrap() {
        ret = row.get(0);
    }
    if ret.is_empty() {
        Err(ret)
    }
    else {
        Ok(ret)
    }
}

#[get("/by_machine_ip/<ip>/<user>")]
pub async fn get_by_username_and_ip(pg: MyPgDatabase, user: String, ip: String) -> Result<String, String> {
    assert_ne!(user, "".to_string(), "Username can't be empty");
    assert_ne!(ip, "".to_string(), "ip can't be empty");
    let query = "SELECT password FROM statham_passwords \
                        JOIN statham_users ON statham_users.id = user_id \
                        JOIN statham_servers on statham_servers.id = machine_id \
                        WHERE username = $1::text and v4_ip_address = $2::inet";
    let mut ret: String = "".to_string();
    for row in pg.run(
        move |c| c.query(query, &[&user, &ip]))
        .await.unwrap() {
        ret = row.get(0);
    }
    if ret.is_empty() {
        Err(ret)
    } else {
        Ok(ret)
    }
}

#[get("/by_mail_ip/<ip>/<email>")]
pub async fn get_by_mail_and_ip(pg: MyPgDatabase, email: String, ip: String) -> Result<String, String> {
    assert_ne!(email, "".to_string(), "Email can't be empty");
    assert_ne!(ip, "".to_string(), "ip can't be empty");
    let query = "SELECT password FROM statham_passwords \
                        JOIN statham_users ON statham_users.id = user_id \
                        JOIN statham_servers on statham_servers.id = machine_id \
                        WHERE email = $1::text and v4_ip_address = $2::inet";
    let mut ret: String = "".to_string();
    for row in pg.run(
        move |c| c.query(query, &[&email, &ip]))
        .await.unwrap() {
        ret = row.get(0);
    }
    if ret.is_empty() {
        Err(ret)
    } else {
        Ok(ret)
    }
}



