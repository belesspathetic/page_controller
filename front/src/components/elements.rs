use sycamore::prelude::*;

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
                    li { (x) }
                },
                key=|x| x.clone(),
            )
        }
    }
}