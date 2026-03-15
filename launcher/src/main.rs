#[macro_use]
extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket::response::content::RawHtml;

#[get("/")]
fn main_page() -> RawHtml<&'static str> {
    RawHtml(include_str!("../../apps/main-page/dist/index.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![main_page]) // mount the html
        .mount("/", FileServer::from(relative!("../apps/main-page/dist")))
}
