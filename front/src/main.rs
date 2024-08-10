use front::pages::home::Home;
use sycamore::prelude::*;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use web_sys::window;

use front::components::adder::Adder;
use front::components::dm::DarkMode;

use shared::api::health_api::fb_health_check_api;

const BODY_CLASS: &str =
    "antialiased transition-colors text-zinc-500 dark:text-zinc-400 bg-[#fafafa] dark:bg-zinc-800";

fn main() {
    // logging
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(App);
}

#[component]
fn App<G: Html>() -> View<G> {
    let on_close = create_signal(false);
    // set body css class
    let window = window().unwrap_throw();

    let document = window.document().unwrap_throw();
    let body = document.body().unwrap_throw();
    body.set_class_name(BODY_CLASS);

    // local storage
    let local_storage = window.local_storage().unwrap_throw();
    
    let dark_mode = if let Some(local_storage) = &local_storage {
        let dark_mode_key = local_storage.get_item("dark_mode").unwrap_throw();
        dark_mode_key.as_deref() == Some("true") || (dark_mode_key.is_none())
    } else {
        false
    };
    let dark_mode = DarkMode(create_signal(dark_mode));
    provide_context(dark_mode);

    create_effect(move || {
        let DarkMode(dark_mode) = use_context::<DarkMode>();
        let html = document.document_element().expect("no main tag");

        if dark_mode.get() {
            html.class_list()
                .add_1("dark")
                .expect("Failed to add dark class");
        } else {
            html.class_list()
                .remove_1("dark")
                .expect("Failed to remove dark class");
        }

        if let Some(local_storage) = &local_storage {
            local_storage
                .set_item("dark_mode", &dark_mode.get().to_string())
                .unwrap();
        }
    });


    view! {
        Home()
        Adder(on_close=on_close)
    }
}

#[component]
async fn Health<G: Html>() -> View<G> {
    match fb_health_check_api().await {
        Ok(_) => view! {p() {"200"}},
        Err(_) => view! {p() {"400"}},
    }
}


