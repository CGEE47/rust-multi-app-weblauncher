// trunk build --public-url /app1/

use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { 
        <a href="../">back</a>
        <p>"Welcome to App1"</p> })
}