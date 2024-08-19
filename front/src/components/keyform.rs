use sycamore::prelude::*;
use web_sys::{window, Storage};

use crate::pages::home::HomeProps;


#[component]
pub fn KeyForm<G: Html>(props: HomeProps<G>) -> View<G> {
    let value = create_signal(String::new());
    let on_click = props.on_click;
    let update_keys = props.update_keys;
    let on_submit = move |event: web_sys::SubmitEvent| {
        
        event.prevent_default();
        // Log value to console
        // web_sys::console::log_1(&value.get_clone_untracked().into()); 
        let window = window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();

        let mut current_vec = read_keys_json(&storage);

        current_vec.push(value.to_string());

        let new_vec = serde_json::to_string(&current_vec).unwrap();

        storage.set_item("keys", new_vec.as_str()).unwrap();

        if let Some(ref update_fn) = update_keys {
            update_fn(value.get_clone_untracked());
        }

        value.set(String::new());

        

        on_click.set(false);

    };

    view! {
        form(on:submit=on_submit, class="flex flex-col gap-4 max-w-sm mx-auto") {
            input(
                bind:value=value,
                class="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 dark:focus:ring-emerald-600",
                placeholder="Enter Page Access Token",
                required=true,

            ) {}
            button(
                type="submit",
                class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-emerald-600 dark:hover:bg-emerald-700 dark:focus:ring-emerald-500"
            ) {
                "Add"
            }
        }
    }
}

fn read_keys_json(local_storage: &Storage) -> Vec<String> {
    let mut keys = Vec::new();
    if let Ok(Some(json_str)) = local_storage.get_item("keys") {
        if let Ok(parsed_keys) = serde_json::from_str::<Vec<String>>(&json_str) {
            keys = parsed_keys;
        }
    }
    keys
}
