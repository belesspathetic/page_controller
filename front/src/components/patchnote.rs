use shared::api::patchnote_api::patchnote_api;
use sycamore::prelude::*;

#[component]
pub async fn PatchNote<G: Html>() -> View<G> {
    let html_content = create_signal(String::new());

    let resp = patchnote_api().await.unwrap();
    let html = resp.text().await.unwrap();

    html_content.set(html);
    view! {
        div(dangerously_set_inner_html=html_content.get_clone_untracked())
    }
}
