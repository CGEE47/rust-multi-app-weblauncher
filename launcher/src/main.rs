#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;

const APPS: &[(&str, &str)] = &[
    ("home", "/"), // home app launching on /
    ("app1", "/app1"),  // app1 app launchin on /app1
];

#[launch]
fn rocket() -> _ {
    let mut rocket = rocket::build();
    let mut rank = 1;

    for &(app, mount_path) in APPS {
        let dist = format!("apps/{}/dist", app);

        rocket = rocket.mount(
            mount_path,
            FileServer::from(std::path::Path::new(&dist)).rank(rank),
        );

        rank += 1;
    }
    rocket
}

// v=app1; cd apps/$v && trunk build --public-url /$v && cd ../..
