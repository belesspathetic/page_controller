#![allow(unused_imports)]
use sycamore::futures::spawn_local;
use sycamore::{prelude::*, suspense::Suspense, web::portal::Portal};
use shared::models::{ManualData, Page};

use shared::api::manual_upload_api::manual_upload_api;
use web_sys::{wasm_bindgen::JsValue, Event};

use crate::components::manualstatus::{ManualLabel, ManualStatus};
use crate::components::{elements::CurrentKey, win::Win};

#[derive(Props)]
pub struct InnerComponent<G: Html> {
    pub children: Children<G>,
    #[prop(default)]
    pub key: String,
    #[prop(default)]
    pub on_click: Signal<bool>,
}

#[component]
pub fn Inner<G: Html>(props: InnerComponent<G>) -> View<G> {
    let on_click = props.on_click;
    let key = props.key.clone();
    view! {
        Me(key=props.key, on_click=on_click)
        Upload(key=key.clone(), on_click=props.on_click)
    }
}

#[component]
pub fn Me<G: Html>(props: InnerComponent<G>) -> View<G> {
    view! {
        Suspense(fallback=view! {
            Spinner()
        }) {
            AsyncMe(key=props.key, on_click=props.on_click)
        }
    }
}

#[component]
pub async fn AsyncMe<G: Html>(props: InnerComponent<G>) -> View<G> {

    
    let page = Page::new(props.key.clone()).await;

    


    let page: Page = match page {
        Ok(p) => p,
        Err(_) => Page::default(),
    };

    

    let name = page.name.clone();
    let followers = page.followers_count.clone();
    let page_id = page.id.clone();

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
pub fn Upload<G: Html>(props: InnerComponent<G>) -> View<G> {
    let on_click = props.on_click;
    let key = props.key.clone();

    view! {
        div(class="flex items-center space-x-4") {
            ManualLabel()
            ManualStatus(key=key)
            UploadButton(key=props.key.clone(), on_click=on_click) 
        }
    }
}


#[component]
pub fn UploadButton<G: Html>(props: InnerComponent<G>) -> View<G> {
    let on_click = props.on_click;
    let key = props.key.clone();

    view! {
        button(
            class="bg-blue-500 transition-colors text-white font-semibold py-2 px-4 rounded hover:bg-blue-600 dark:bg-emerald-700 dark:hover:bg-emerald-800",
            on:click=move |_| {
                on_click.set(true);
                web_sys::console::log_1(&format!("Button clicked with key: {}", key.clone()).into());

                web_sys::window().unwrap()
                    .local_storage().unwrap()
                    .unwrap()
                    .set_item("current_key", &key)
                    .unwrap();
                
            }
        ) {
            "Upload"
        }
        (if on_click.get() {
            view!{
                Portal(selector="body") {
                    Win(on_click=on_click) {
                        ManualUploadForm(on_click=on_click)
                    }
                }
            }
        } else {
            view! {}
        })
    }
}








#[component]
pub fn ManualUploadForm<G: Html>(props: InnerComponent<G>) -> View<G> {
    static INPUT_CLASS: &str = "p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 dark:focus:ring-emerald-600";
    let on_click = props.on_click;
    let url = create_signal(String::new());
    let title = create_signal(String::new());
    let tags = create_signal(String::new());
    let video_or_reels = create_signal("video".to_string());
    let montage_or_no_montage = create_signal("montage".to_string()); 


    let on_submit = move |event: web_sys::SubmitEvent| {
        let key = web_sys::window().unwrap()
        .local_storage().unwrap()
        .unwrap()
        .get_item("current_key")
        .unwrap_or_default();
        event.prevent_default();

        let url = url.get_clone_untracked();

        let title = title.get_clone_untracked();
        let tags = tags.get_clone_untracked();
        let video_or_reels = video_or_reels.get_clone_untracked();
        let montage_or_no_montage = montage_or_no_montage.get_clone_untracked();

        let data = ManualData::new().url(&url).add_key(key.unwrap().as_str()).add_tags(&tags).add_title(&title).video_or_reels(&video_or_reels).montage_or_no_montage(&montage_or_no_montage);

        let json_data = serde_json::json!(data);
        let json_string = json_data.clone().to_string();
        web_sys::console::log_1(&JsValue::from_str(&json_string));

        spawn_local(async move {
            on_click.set(false);
            match manual_upload_api(data).await {
                Ok(_) => web_sys::console::log_1(&JsValue::from_str("Upload successful")),
                Err(e) => web_sys::console::log_1(&JsValue::from_str(&format!("Error: {}", e))),
            }
        });
    };

    view! {
        form(data-key=props.key, on:submit= on_submit, class="flex flex-col gap-4 max-w-sm mx-auto") {
            label {
                "Url: "
                input(
                    bind:value=url,
                    class=INPUT_CLASS,
                    placeholder="Enter video url",
                ) {}
            }
            label {
                "Title: "
                input(
                    bind:value=title,
                    class=INPUT_CLASS,
                    placeholder="Enter title",
                ) {}
            }
            label {
                "Tags: "
                input(
                    bind:value=tags,
                    class=INPUT_CLASS,
                    placeholder="Enter tags",
                ) {}
            }
            label {
                "Video/Reels: "
                select(
                    bind:value=video_or_reels,
                    class=INPUT_CLASS
                ) {
                    option(value="video") { "Video" }
                    option(value="reels") { "Reels" }
                }
            }
            label {
                "Montage/No Montage: "
                select(
                    bind:value=montage_or_no_montage,
                    class=INPUT_CLASS
                ) {
                    option(value="montage") { "Montage" }
                    option(value="no_montage") { "No Montage" }
                }
            }
            button(
                type="submit",
                class="transition-colors px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-emerald-600 dark:hover:bg-emerald-700 dark:focus:ring-emerald-500"
            ) {
                "Upload"
            }
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