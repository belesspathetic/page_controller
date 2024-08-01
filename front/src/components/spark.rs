use sycamore::prelude::*;

#[component]
pub fn Spark<G: Html>() -> View<G> {
    view! {
        div(class="absolute z-20 top-0 inset-x-0 flex justify-center overflow-hidden pointer-events-none") {
            div(class="w-[108rem] flex-none flex justify-end") {
                picture() {
                    source(srcset="./static/light.avif")
                    img(class="w-[71.75rem] flex-none max-w-none dark:hidden", src="./static/light.png", decoding="async")
                }
                picture() {
                    source(srcset="./static/dark.avif")
                    img(class="w-[90rem] flex-none max-w-none hidden dark:block", src="./static/dark.png", decoding="async")
                }
            }
        }
    }
}
