#![allow(dead_code)]
use crate::components::{dm::DarkModeToggle, patchnote::PatchNote, win::Win};
use sycamore::{prelude::*, web::portal::Portal};

use super::win::ContentComponent;

const DIV: &str = "sticky top-0 z-40 w-full backdrop-blur flex-none transition-colors duration-500 lg:z-50 lg:border-b lg:border-zinc-900/10 dark:border-zinc-50/[0.06] bg-[#fafafa]/95 supports-backdrop-blur:bg-[#fafafa]/60 dark:bg-transparent";
static GITHUB: &str = "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z";

#[derive(Props)]
pub struct PortalProps<G: Html> {
    children: Children<G>,
}

#[component]
pub fn HeaderContent<G: Html>(props: ContentComponent<G>) -> View<G> {
    let on_click = props.on_close;
    view! {
        div(class=DIV) {
            div(class="max-w-8xl mx-auto") {
                div(class="py-4 border-b border-zinc-900/10 lg:px-8 lg:border-0 dark:border-zinc-300/10 px-4") {
                    div(class="relative flex items-center") {
                        a(class="mr-3 flex-none w-[2.0625rem] overflow-hidden md:w-auto", href="/") {
                            "page_controller"
                        }
                        div(class="relative hidden lg:flex items-center ml-auto") {
                            nav(class="text-sm leading-6 font-semibold text-zinc-700 dark:text-zinc-200") {
                                ul(class="flex space-x-8") {
                                    li(class="hover:text-blue-500 dark:hover:text-emerald-500") {
                                        a(role="button", on:click=move |_| {
                                            on_click.set(true)
                                        }) {
                                            "What's new?"
                                        }
                                    }
                                }
                            }
                            div(class="flex items-center border-l border-orange-500 ml-6 pl-6 dark:border-orange-800") {
                                DarkModeToggle()
                                a(class="ml-6 block text-zinc-400 hover:text-zinc-500 dark:hover:text-zinc-300", target="_blank", rel="noopener noreferrer", href="https://github.com/belesspathetic") {
                                    svg(class="dark:hover:fill-[#c9510c] transition colors", viewBox="0 0 16 16", aria-hidden="true", height="30px", width="30px",) {
                                        path(d=GITHUB)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Header<G: Html>(props: ContentComponent<G>) -> View<G> {
    let on_click = props.on_close;
    view! {

            // Always render the header content
            HeaderContent(on_close=on_click.clone())

            // Conditionally render the portal with the window if `on_click` is true
            (if on_click.get() {
                view! {
                    Portal(selector="body") {
                        Win(on_close=on_click) {
                            PatchNote()
                        }
                    }
                }
            } else {
                view! {}
            })

    }
}
