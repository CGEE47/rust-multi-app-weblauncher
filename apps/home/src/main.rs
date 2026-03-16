use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <style>
            "body {
                background-color: grey;
                color: white;
                padding: 20px;
                font-family: 'Courier New', monospace;
            }"
            </style>
            <h1 style="text-align:center;"> Welcome to the Home page </h1>
            <p style="text-align:center;"> Click the buttons below!</p>
            <div style="display:flex; justify-content:center; gap:20px;">
                <a href="../app1" style="
                display:inline-block;
                width:200px;
                height:50px;
                line-height:50px;
                text-align:center;
                background:#444;
                color:white;
                border-radius:6px;
                text-decoration:none;"
                > App1 
                </a>

                <a href="../app1" style="
                display:inline-block;
                width:200px;
                height:50px;
                line-height:50px;
                text-align:center;
                background:#444;
                color:white;
                border-radius:6px;
                text-decoration:none;"
                > App1 
                </a>
            </div>
        }
    })
}
