// trunk build --public-url /app1/

use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
        <style>
            "body{background-color:grey;}"
            "body{color:white;}"
            "body{padding:20px}"
        </style>

        <a href="../" style="
            display:flex;
            justify-content:center;
            align-items:center;
            width:120px;
            height:40px;
            background:#444;
            color:white;
            border-radius:6px;
            text-decoration:none;"
            > back
        </a>
        <h1> Welcome to App1 </h1> }
    })
}
