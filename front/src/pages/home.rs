use crate::components::header::Header;
use crate::components::spark::Spark;
use sycamore::prelude::*;

#[component]
pub fn Home<G: Html>() -> View<G> {
    view! {
        Header()
        Spark()
    }
}
