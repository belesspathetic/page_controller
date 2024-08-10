#![allow(unused_imports)]
use sycamore::{prelude::*, suspense::Suspense};
use shared::models::Page;


use crate::components::win::Win;
use super::win::ContentComponent;


#[derive(Props)]
pub struct InnerComponent<G: Html> {
    pub children: Children<G>,
    pub key: String,
}

#[component]
pub fn Inner<G: Html>(props: InnerComponent<G>) -> View<G> {
    let key = props.key;
    view! {
        Me(key=key)
    }
}

#[component]
pub async fn AsyncMe<G: Html>(props: InnerComponent<G>) -> View<G> {
    
    let page = Page::new(props.key).await.unwrap();

    let name = page.name;
    let followers = page.followers_count;
    let page_id = page.id;

    let href = format!("https://www.facebook.com/profile.php?id={}", page_id);

    view! {
        div(class="text-zinc-700 dark:text-zinc-200") {
            a(class="text-xl font-semibold ", target="_blank", href=href) {
                (name)
            }
            p() {
                "ID: " (page_id)
            }
            p() {
                "Followers: " (followers)
            }
            a(href="#", class="text-blue-500 hover:underline dark:text-emerald-500") {
                "View more details"
            }
        }
        
    }
}

#[component]
pub fn Me<G: Html>(props: InnerComponent<G>) -> View<G> {
    view! {
        Suspense(fallback=view! {
            Spinner()
        }) {
            AsyncMe(key=props.key)
        }
    }
}

#[component]
pub fn Spinner<G: Html>() -> View<G> {
    view! {
        div(class="w-8 h-8 border-4 border-blue-500 rounded-full animate-spin dark:border-emerald-500") {
            div(class="absolute border-t-4 border-blue-500 border-solid rounded-full w-full h-full dark:border-emerald-500") {}
        }
    }
}