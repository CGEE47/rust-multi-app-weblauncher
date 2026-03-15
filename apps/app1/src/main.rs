// trunk build --public-url /app1/

use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, App1!"</p> })
}