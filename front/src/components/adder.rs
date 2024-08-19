use sycamore::{prelude::*, web::portal::Portal};

use crate::{components::{keyform::KeyForm, win::Win}, pages::home::HomeProps};

const BUTTON_CLASS: &str = "w-full transition-colors bg-blue-500 text-white font-semibold py-2 px-4
    hover:bg-blue-600
    dark:bg-emerald-800 dark:hover:bg-emerald-900";



#[component]
pub fn Adder<G: Html>(props: HomeProps<G>) -> View<G> {
    let on_click = props.on_click;
    let update_keys = props.update_keys;
    view! {
        div(class="w-full") {
            button(
                class=BUTTON_CLASS, on:click= move |_| {
                    on_click.set(true);
                }
            ) {
                span(class="mr-2") { "+" }
                span { "Add Page" }
            }

            (if on_click.get() {
                let update_keys = update_keys.clone();
                view! {
                    Portal(selector="body") {
                        Win(on_click=on_click) {
                            KeyForm(on_click=on_click, update_keys=update_keys.unwrap())
                        }
                    }
                }
            } else {
                view! {}
            })
        }
    }
}
