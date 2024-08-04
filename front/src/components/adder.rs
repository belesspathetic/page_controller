use sycamore::{prelude::*, web::portal::Portal};

use crate::components::{keyform::KeyForm, win::Win};

use super::win::ContentComponent;

const BUTTON_CLASS: &str = "w-full bg-blue-500 text-white font-semibold py-2 px-4
    hover:bg-blue-600
    dark:bg-emerald-800 dark:hover:bg-emerald-900";

#[component]
pub fn Adder<G: Html>(props: ContentComponent<G>) -> View<G> {
    let on_click = props.on_close;

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
                view! {
                    Portal(selector="body") {
                        Win(on_close=on_click) {
                            KeyForm()
                        }
                    }
                }
            } else {
                view! {}
            })
        }
    }
}
