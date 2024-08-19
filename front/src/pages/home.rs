use std::rc::Rc;

use crate::components::adder::Adder;
use crate::components::header::Header;
use crate::components::spark::Spark;
use crate::components::elements::Elements;

use sycamore::prelude::*;

#[derive(Props)]
pub struct HomeProps<G: Html> {
    #[allow(dead_code)]
    pub children: Children<G>,
    #[prop(default)]
    pub on_click: Signal<bool>,
    #[prop(default)]
    pub keys: Signal<Vec<String>>,
    #[prop(default)]
    pub update_keys: Option<Rc<dyn Fn(String)>>,
}

#[component]
pub fn Home<G: Html>(props: HomeProps<G>) -> View<G> {
    let keys = props.keys;

    let update_keys = Rc::new(move |new_key: String| {
        let mut new_keys = keys.get_clone_untracked();
        new_keys.push(new_key);
        keys.set(new_keys);
    });

    view! {
        Header()
        Spark()
        Elements(keys=keys)
        Adder(update_keys=update_keys)
    }
}
