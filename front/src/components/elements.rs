use sycamore::prelude::*;
use crate::components::inner::Inner;
use crate::get_keys;

#[component]
pub fn Elements<G: Html>() -> View<G> {
    let keys = get_keys();

    let count = create_signal(keys);
    view! {
        ul {
            Keyed(
                iterable=*count,
                view=|x| view! {
                    li(class="bg-gray p-10 shadow-md w-full") { 
                        Inner(key=x)
                     }
                },
                key=|x| x.clone(),
            )
        }
    }
}