#[macro_use]
extern crate rocket;
use rocket::fs::{FileServer, relative};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("../apps/main-page/dist")).rank(1)) //main-page on /
        .mount("/app1", FileServer::from(relative!("../apps/app1/dist")).rank(2)) // app1 on /app1
}