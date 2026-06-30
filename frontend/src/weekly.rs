use leptos::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use crate::Language;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeeklyRoundup {
    pub id: String,
    pub title: String,
    pub title_bn: String,
    pub summary: String,
    pub summary_bn: String,
    pub content: String,
    pub content_bn: String,
}

async fn fetch_weekly_roundups() -> Result<Vec<WeeklyRoundup>, String> {
    let res = Request::get("/weekly_roundups.json").send().await.map_err(|e| e.to_string())?;
    let data = res.json::<Vec<WeeklyRoundup>>().await.map_err(|e| e.to_string())?;
    Ok(data)
}

#[component]
pub fn WeeklyRoundupListPage() -> impl IntoView {
    let lang = expect_context::<ReadSignal<Language>>();
    let roundups_resource = create_resource(
        || (),
        |_| async move { fetch_weekly_roundups().await }
    );

    view! {
        <div class="container about-container">
            <header class="about-header">
                <h1 class="about-title">{move || if lang.get() == Language::Bn { "সাপ্তাহিক এআই রাউন্ডআপ" } else { "Weekly AI Roundup" }}</h1>
                <p class="about-subtitle">{move || if lang.get() == Language::Bn { "প্রতি সপ্তাহের গুরুত্বপূর্ণ এআই খবর" } else { "Key AI news from every week" }}</p>
            </header>
            
            <div class="weekly-list">
                <Suspense fallback=move || view! { <div class="loading-indicator"><div class="loading-spinner"></div></div> }>
                    {move || roundups_resource.get().map(|res| {
                        match res {
                            Ok(roundups) => {
                                roundups.into_iter().map(|item| {
                                    view! {
                                        <a href=format!("/weekly-roundup/{}", item.id) class="weekly-card" style="text-decoration: none; color: inherit; display: block; margin-bottom: 20px; padding: 20px; background: var(--bg-color); border-radius: 12px; box-shadow: 0 4px 6px var(--shadow-color); border: 1px solid var(--border-color);">
                                            <h2 class="card-title" style="margin-top: 0;">
                                                {let title = item.title.clone(); let title_bn = item.title_bn.clone(); move || if lang.get() == Language::Bn { title_bn.clone() } else { title.clone() }}
                                            </h2>
                                            <p class="card-summary" style="margin-bottom: 0;">
                                                {let summary = item.summary.clone(); let summary_bn = item.summary_bn.clone(); move || if lang.get() == Language::Bn { summary_bn.clone() } else { summary.clone() }}
                                            </p>
                                        </a>
                                    }
                                }).collect_view()
                            },
                            Err(e) => view! { <div class="error-msg">"Error loading roundups: " {e}</div> }.into_view()
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}

use leptos_router::use_params_map;

#[component]
pub fn WeeklyRoundupDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let lang = expect_context::<ReadSignal<Language>>();

    let roundups_resource = create_resource(
        || (),
        |_| async move { fetch_weekly_roundups().await }
    );

    view! {
        <div class="container about-container">
            <Suspense fallback=move || view! { <div class="loading-indicator"><div class="loading-spinner"></div></div> }>
                {move || {
                    let current_id = id();
                    roundups_resource.get().map(|res| {
                        match res {
                            Ok(roundups) => {
                                if let Some(item) = roundups.into_iter().find(|r| r.id == current_id) {
                                    view! {
                                        <div>
                                            <a href="/weekly-roundup" class="about-back-btn" style="display: inline-block; margin-bottom: 20px;">
                                                {move || if lang.get() == Language::Bn { "← ফিরে যান" } else { "← Back to List" }}
                                            </a>
                                            <header class="about-header" style="text-align: left; padding: 0; margin-bottom: 30px; background: none; box-shadow: none;">
                                                <h1 class="about-title" style="font-size: 2rem; font-family: 'Playfair Display', serif;">
                                                    {let title = item.title.clone(); let title_bn = item.title_bn.clone(); move || if lang.get() == Language::Bn { title_bn.clone() } else { title.clone() }}
                                                </h1>
                                            </header>
                                            
                                            <div class="weekly-content" style="line-height: 1.8; color: var(--text-color);">
                                                {let content = item.content.clone(); let content_bn = item.content_bn.clone(); move || {
                                                    let text = if lang.get() == Language::Bn { content_bn.clone() } else { content.clone() };
                                                    let paragraphs: Vec<String> = text.split("\n\n").map(|s| s.to_string()).collect();
                                                    paragraphs.into_iter().map(|para| {
                                                        if para.starts_with("### ") {
                                                            let title = para.trim_start_matches("### ").to_string();
                                                            view! { <h3 style="margin-top: 1.5em; color: var(--primary-color); font-family: 'Playfair Display', serif;">{title}</h3> }.into_view()
                                                        } else {
                                                            view! { <p style="margin-bottom: 1em; color: var(--text-secondary);">{para}</p> }.into_view()
                                                        }
                                                    }).collect_view()
                                                }}
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div>"Roundup not found"</div> }.into_view()
                                }
                            },
                            Err(e) => view! { <div class="error-msg">"Error loading roundup: " {e}</div> }.into_view()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
