use sycamore::prelude::*;

#[component]
pub fn KeyForm<G: Html>() -> View<G> {
    let value = create_signal(String::new());

    let on_submit = move |event: web_sys::SubmitEvent| {
        event.prevent_default();
        web_sys::console::log_1(&value.get_clone_untracked().into()); // Log value to console
    };

    view! {
        form(on:submit=on_submit, class="flex flex-col gap-4 max-w-sm mx-auto") {
            input(
                bind:value=value,
                class="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 dark:focus:ring-emerald-600",
                placeholder="Enter Page Access Token"

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
