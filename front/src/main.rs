use shared::fb_health_check;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use front::home::Home;
use web_sys::window;

const BODY_CLASS: &str = "antialiased text-slate-500 dark:text-slate-400 bg-white dark:bg-slate-900";

#[derive(Clone)]
struct DarkMode(Signal<bool>);

fn main() {
    // logging
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(App);
}

#[component]
fn App<G: Html>() -> View<G> {
    // set body css class
    let window = window().expect("window not set");
    
    let document = window.document().expect("document not set");
    let body = document.body().expect("body not set");
    body.set_class_name(BODY_CLASS);

    // local storage
    let local_storage = window.local_storage().expect("local storage not set").unwrap();
    


    // let dark_mode = match local_storage.get_item("dark_mode") {

    // }

    
    // if dark_mode.is_empty() {
    //     local_storage.set_item("dark_mode", "false").unwrap();
    // } else if dark_mode == "false".to_string() {
    //     DarkMode(create_signal(false));
    // } else {
    //     DarkMode(create_signal(true));
    // }

    
    view! {
        Home()
        div {
            p { "Facebook status" }
            Suspense(fallback=view! { "Loading..." }) {
                Health() {}
            }
        }
    }
}

#[component]
async fn Health<G: Html>() -> View<G> {
    match fb_health_check().await {
        Ok(_) => view! {p() {"200"}},
        Err(_) => view! {p() {"400"}},
    }
}