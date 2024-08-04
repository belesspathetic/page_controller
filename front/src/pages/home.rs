use crate::components::header::Header;
use crate::components::spark::Spark;
use sycamore::prelude::*;

#[component]
pub fn Home<G: Html>() -> View<G> {
    let on_click = create_signal(false);
    view! {
        Header(on_close=on_click)
        Spark()
    }
}
