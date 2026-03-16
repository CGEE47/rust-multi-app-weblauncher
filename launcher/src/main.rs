#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;

const APPS: &[(&str, &str)] = &[
    ("home", "/"),     // home app launching on /
    ("app1", "/app1"), // app1 app launchin on /app1
];

#[launch]
fn rocket() -> _ {
    let mut rocket = rocket::build();
    let mut rank = 1;
    //loops through the APPS const and launches all apps
    for &(app, path) in APPS {
        let dist = format!("apps/{app}/dist");
        rocket = rocket.mount(path, FileServer::from(dist.as_str()).rank(rank));
        rank += 1;
    }
    rocket
}
