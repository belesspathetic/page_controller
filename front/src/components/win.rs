use sycamore::prelude::*;

use crate::pages::home::HomeProps;

#[component]
pub fn Win<G: Html>(props: HomeProps<G>) -> View<G> {
    let children = props.children.call();

    let on_close = props.on_click;


    view! {
        (if on_close.get() {
            let children = children.clone();
            view! {
                div(class="fixed inset-0 bg-zinc-800 bg-opacity-50 flex items-center justify-center z-50 filter: blur(4px)") {
                    // Modal window
                    div(class="bg-[#fafafa] p-6 rounded-lg shadow-lg relative max-w-lg w-full max-h-[90vh] overflow-auto") {
                        // Close button
                        button(class="absolute top-2 right-2 text-zinc-600 hover:text-zinc-900", on:click=move |_| {
                            on_close.set(false);
                        }) {
                            "✖"
                        }
                        // Content
                        (children)
                    }
                }
            }

        } else {
            view! {}
        })

    }
}
