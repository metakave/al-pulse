use leptos::*;
use crate::{Language, localize};
use gloo_net::http::Request;

#[component]
pub fn FavoritesPage() -> impl IntoView {
    let (favorites, set_favorites) = create_signal::<Vec<serde_json::Value>>(Vec::new());
    let lang = expect_context::<ReadSignal<Language>>();

    // Load favorite news on mount
    create_effect(move |_| {
        let lang = lang.get();
        spawn_local(async move {
            if let Ok(resp) = Request::get("/api/news?favorites=true").send().await {
                if let Ok(data) = resp.json::<Vec<serde_json::Value>>().await {
                    set_favorites.set(data);
                }
            }
        });
    });

    view! {
        <div class="favorites-page">
            <h2>{move || localize(lang.get(), "my_favorites")}</h2>
            <For
                each=move || favorites.get().into_iter().enumerate()
                key=|(i, _)| *i
                let:item
            >
                {move |(idx, news_item): (usize, serde_json::Value)| {
                    view! {
                        <div class="news-card">{format!("{:?}", news_item)}</div>
                    }
                }}
            </For>
        </div>
    }
}
