use sycamore::prelude::*;

const BUTTON_CLASS: &str =
    "w-full transition-colors bg-green-500 text-white font-semibold py-2 px-4
                    hover:bg-green-600
                    dark:bg-green-700 dark:hover:bg-green-800
                    border border-transparent
                    bg-opacity-80
                    dark:bg-opacity-70";

#[component]
pub fn Adder<G: Html>() -> View<G> {
    
    view! {
        div(class="w-full") {
            button(
                class=BUTTON_CLASS
            ) {
                span(class="mr-2") { "+" }
                span { "Add Page" }
            }
        }
    }
}
