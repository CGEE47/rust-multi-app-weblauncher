use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
        <style>
            "body{background-color:grey;}"
            "body{color:white;}"
            "body{padding:20px}"
        </style>

        <h1> Welcome to the Home page </h1>

        <p> If you want to navigate to App1, click below</p>

        <button><a href="../app1">App1</a></button>
        }
    })
}
