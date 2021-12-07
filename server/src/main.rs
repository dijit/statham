#[macro_use]
extern crate rocket;
use rocket_sync_db_pools::{postgres, database};

#[database("my_db")]
pub struct MyPgDatabase(postgres::Client);

mod db;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(MyPgDatabase::fairing())
        .mount("/",
                          routes![
                              hello,
                              db::get_by_mail_and_ip,
                              db::get_by_username_and_hostname,
                          ]
        )
        .launch().await;
}