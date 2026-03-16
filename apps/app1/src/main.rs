// trunk build --public-url /app1/

use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { 
        <style>
            "body{background-color:grey;}"
            "body{color:white;}"
            "body{padding:20px}"
        </style>
        
        <button><a href="../">back</a></button>
        <p>"Welcome to App1"</p> })
}