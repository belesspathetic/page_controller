use gloo_timers::callback::Interval;
use shared::api::status_api::status_api;
use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;
use super::inner::InnerComponent;

#[component]
pub async fn ManualStatus<G: Html>(props: InnerComponent<G>) -> View<G> {
    let key = props.key.clone();
    let status = create_signal(String::new());
    let holder = create_signal(true);

    let statuses = create_signal(vec![
        "Waiting",
        "Downloading",
        "Montage",
        "Uploading",
        "Success",
    ]);

    // Обновляем статус через интервал
    spawn_local(async move {
        let interval = Interval::new(7000, move || {
            holder.set(true);
        });

        interval.forget();
    });

    // Эффект обновления статуса при изменении holder
    create_effect(move || {
        if holder.get() {
            let key = key.clone();
            spawn_local(async move {
                match status_api(key).await {
                    Ok(response) => status.set(response),
                    Err(err) => eprintln!("Error fetching status: {:?}", err),
                }
                holder.set(false); // Сбросить holder после обновления
            });
        }
    });


    view! {
        div(class="flex flex-col space-y-0.5") {
            Keyed(
                iterable=*statuses,
                view=move |status_text| view! {
                    div(class="flex items-center space-x-2") {
                        // Цветная точка
                        div(class= format!("w-2 h-2 transition-colors duration-500 rounded-full {}", if status.get_clone() == status_text { "bg-yellow-500" } else { "bg-gray-500" })) {}

                        // Текст статуса
                        p(class="text-sm") { (status_text) }
                    }
                },
                key=|status_text| status_text.to_string(),
            )
        }
    }
}

#[component]
pub fn ManualLabel<G: Html>() -> View<G> {
    view! {
        div(class="flex items-center") {
            div(class="flex items-center space-x-2") {
                div(class="relative flex items-center justify-center w-px h-24 bg-gray-400") {
                    span(class="absolute -left-12 -rotate-90 text-lg font-semibold select-none") { "MANUAL" }
                }
            }
        }
    }
}