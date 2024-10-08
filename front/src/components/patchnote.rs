use shared::api::patchnote_api::patchnote_api;
use sycamore::prelude::*;

#[component]
pub async fn PatchNote<G: Html>() -> View<G> {


    
    let resp = patchnote_api().await;
    let default = "Back is down".to_string();
    let html = match resp {
        Ok(r) => {
            let html = r.text().await.unwrap_or(default);
            html
        },
        Err(_) => {
            default
        },
    };


    view! {
        div(dangerously_set_inner_html=html.clone())
    }
}


