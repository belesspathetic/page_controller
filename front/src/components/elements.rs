use sycamore::prelude::*;
use crate::components::inner::Inner;
use crate::pages::home::HomeProps;

#[derive(Clone)]
pub struct CurrentKey(pub String);

#[component]
pub fn Elements<G: Html>(props: HomeProps<G>) -> View<G> {
    let keys = props.keys;
    let on_click = props.on_click;
    
    view! {
        ul {
            Keyed(
                iterable=*keys,
                view= move |x| view! {
                    li(class="bg-gray p-10 shadow-md w-full flex items-center justify-between") { 
                        Inner(key=x, on_click=on_click)
                     }
                },
                key=|x| x.clone(),
            )
        }
    }
}