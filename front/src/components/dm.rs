use sycamore::prelude::*;
use web_sys::window;

#[derive(Clone)]
pub struct DarkMode(pub Signal<bool>);

#[component]
pub fn DarkModeToggle<G: Html>() -> View<G> {
    static MOON: &str = "M480-120q-150 0-255-105T120-480q0-150 105-255t255-105q14 0 27.5 1t26.5 3q-41 29-65.5 75.5T444-660q0 90 63 153t153 63q55 0 101-24.5t75-65.5q2 13 3 26.5t1 27.5q0 150-105 255T480-120Zm0-80q88 0 158-48.5T740-375q-20 5-40 8t-40 3q-123 0-209.5-86.5T364-660q0-20 3-40t8-40q-78 32-126.5 102T200-480q0 116 82 198t198 82Zm-10-270Z";
    static SUN: &str = "M480-360q50 0 85-35t35-85q0-50-35-85t-85-35q-50 0-85 35t-35 85q0 50 35 85t85 35Zm0 80q-83 0-141.5-58.5T280-480q0-83 58.5-141.5T480-680q83 0 141.5 58.5T680-480q0 83-58.5 141.5T480-280ZM200-440H40v-80h160v80Zm720 0H760v-80h160v80ZM440-760v-160h80v160h-80Zm0 720v-160h80v160h-80ZM256-650l-101-97 57-59 96 100-52 56Zm492 496-97-101 53-55 101 97-57 59Zm-98-550 97-101 59 57-100 96-56-52ZM154-212l101-97 55 53-97 101-59-57Zm326-268Z";
    
    let DarkMode(dark_mode) = use_context::<DarkMode>();

    let on_click = move |_| {
        toggle(dark_mode);
    };

    view! {
        button(on:click=on_click) {
            svg(class="fill-zinc-400 transition-colors hover:fill-yellow-400 hidden dark:block", xmlns="http://www.w3.org/2000/svg", height="34px", width="34px", viewBox="0 -960 960 960") {
                path(d=MOON)
            }
            svg(class="fill-zinc-400 transition-colors hover:fill-blue-400 dark:hidden", xmlns="http://www.w3.org/2000/svg", height="34px", width="34px", viewBox="0 -960 960 960") {
                path(d=SUN)
            }
            
        }

    }
}

fn toggle(dark_mode: Signal<bool>) {
    let window = window().expect("window not set");

    let document = window.document().expect("document not set");

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

    dark_mode.set(!dark_mode.get());
}
