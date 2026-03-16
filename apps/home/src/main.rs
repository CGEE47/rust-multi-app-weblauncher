use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <style> background-color: grey; </style>
            
           <h1> Welcome to the Home page </h1>

           <p> If you want to navigate to App1, click below</p>

           <a href="../app1">  - App1</a>
        }
    })

}
