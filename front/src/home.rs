use sycamore::prelude::*;
use crate::components::header::Header;
use crate::components::spark::Spark;

#[component]
pub fn Home<G: Html>() -> View<G> {

    view! {
        Spark()
        Header()
    }
}