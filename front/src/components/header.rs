use sycamore::prelude::*;

const DIV: &str = "sticky top-0 z-40 w-full backdrop-blur flex-none transition-colors duration-500 lg:z-50 lg:border-b lg:border-slate-900/10 dark:border-slate-50/[0.06] bg-white/95 supports-backdrop-blur:bg-white/60 dark:bg-transparent";


#[component]
pub fn Header<G: Html>() -> View<G> {
    view! {
        div(class=DIV) {
            div(class="max-w-8xl mx-auto") {
                div(class="py-4 border-b border-slate-900/10 lg:px-8 lg:border-0 dark:border-slate-300/10 px-4") {
                    div(class="relative flex items-center") {
                        a(class="mr-3 flex-none w-[2.0625rem] overflow-hidden md:w-auto", href="/") {
                            "page_controller"
                        }
                    }
                }
            }
        }
    }
}