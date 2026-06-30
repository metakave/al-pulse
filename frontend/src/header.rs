use leptos::*;
use crate::{Language, Theme, localize};
use gloo_net::http::Request;

#[component]
pub fn GlobalHeader() -> impl IntoView {
    let lang = expect_context::<ReadSignal<Language>>();
    let set_lang = expect_context::<WriteSignal<Language>>();
    let theme = expect_context::<ReadSignal<Theme>>();
    let set_theme = expect_context::<WriteSignal<Theme>>();
    let trigger_sync_action = expect_context::<Action<(), Result<i64, String>>>();
    let last_sync_timestamp = expect_context::<ReadSignal<Option<i64>>>();
    let seconds_to_sync = expect_context::<ReadSignal<i64>>();
    let set_seconds_to_sync = expect_context::<WriteSignal<i64>>();
    let set_last_sync_timestamp = expect_context::<WriteSignal<Option<i64>>>();
    
    let (total_curated, set_total_curated) = create_signal(0_i64);

    // Initial hook to load synchronization status and stats
    create_effect(move |_| {
        spawn_local(async move {
            let res = Request::get("/api/news/stats").send().await;
            if let Ok(r) = res {
                if let Ok(data) = r.json::<serde_json::Value>().await {
                    if let Some(ts) = data.get("last_sync").and_then(|v| v.as_i64()) {
                        set_last_sync_timestamp.set(Some(ts));
                    }
                    if let Some(total) = data.get("total_count").and_then(|v| v.as_i64()) {
                        set_total_curated.set(total);
                    }
                }
            }
        });
    });

    // Update countdown timer every second
    create_effect(move |_| {
        if let Ok(handle) = set_interval_with_handle(move || {
            if let Some(last) = last_sync_timestamp.get() {
                let now = (js_sys::Date::now() / 1000.0) as i64;
                let elapsed = now - last;
                let diff = 2700 - elapsed;
                
                if diff <= 0 {
                    if !trigger_sync_action.pending().get() {
                        trigger_sync_action.dispatch(());
                    }
                    set_seconds_to_sync.set(2700);
                } else {
                    set_seconds_to_sync.set(diff);
                }
            }
        }, std::time::Duration::from_secs(1)) {
            on_cleanup(move || {
                handle.clear();
            });
        }
    });

    let is_menu_open = create_rw_signal(false);

    let on_theme_toggle = move |_| {
        let next_theme = if theme.get() == Theme::Dark { Theme::Light } else { Theme::Dark };
        set_theme.set(next_theme);
        
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let theme_str = if next_theme == Theme::Light { "light" } else { "dark" };
                    let _ = html.set_attribute("data-theme", theme_str);
                }
            }
        }
    };

    view! {
        <header class="app-header">
            /* Logo Section */
            <div class="brand-section">
                <div class="brand-logo-container">
                    <svg class="brand-logo-svg" viewBox="0 0 24 24" fill="currentColor" style="width: 28px; height: 28px; color: #6366f1;">
                        <path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z"/>
                        <path d="m5 3 1 2.5L8.5 6 6 7 5 9.5 4 7 1.5 6 4 5 5 3Z"/>
                        <path d="m19 17 1 2.5 2.5.5-2.5 1-1 2.5-1-2.5-2.5-1 2.5-1 1-2.5Z"/>
                    </svg>
                    <div class="pulse-dot"></div>
                </div>
                <div class=move || format!("brand-text lang-{:?}", lang.get())>
                    <a href="/" style="text-decoration: none; color: inherit;">
                        <h1>{move || localize(lang.get(), "title")}</h1>
                    </a>
                    <p>{move || localize(lang.get(), "tagline")}</p>
                </div>
            </div>

            /* Nav and Status */
            <div class="nav-and-status">
                <div class="desktop-controls">
                    <div class="lang-switcher">
                        <button 
                            class=move || if lang.get() == Language::En { "lang-btn active" } else { "lang-btn" }
                            on:click=move |_| set_lang.set(Language::En)
                        >
                            "EN"
                        </button>
                        <button 
                            class=move || if lang.get() == Language::Bn { "lang-btn active" } else { "lang-btn" }
                            on:click=move |_| set_lang.set(Language::Bn)
                        >
                            "বাংলা"
                        </button>
                    </div>
                    <button class="theme-toggle-btn" on:click=on_theme_toggle.clone()>
                        {move || if theme.get() == Theme::Light { "🌙" } else { "☀️" }}
                    </button>
                </div>

                <div class="status-widget">
                    <div class=move || if trigger_sync_action.pending().get() { "status-indicator syncing" } else { "status-indicator" }></div>
                    <span class="sync-time-text next-sync-label">
                        {move || {
                            let total_secs = seconds_to_sync.get();
                            if trigger_sync_action.pending().get() {
                                localize(lang.get(), "syncing").to_string()
                            } else if last_sync_timestamp.get().is_none() {
                                localize(lang.get(), "auto_sync").to_string() + "..."
                            } else {
                                let m = total_secs / 60;
                                let s = total_secs % 60;
                                format!("{}{:02}:{:02}", localize(lang.get(), "auto_sync"), m, s)
                            }
                        }}
                    </span>
                    <button 
                        class="refresh-btn sync-btn"
                        disabled=move || trigger_sync_action.pending().get()
                        on:click=move |_| trigger_sync_action.dispatch(())
                    >
                        {move || if trigger_sync_action.pending().get() {
                            view! { <span class="sync-spinner"></span> }.into_view()
                        } else {
                            view! {
                                <svg class="sync-icon-svg" viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                    <polyline points="23 4 23 10 17 10"></polyline>
                                    <polyline points="1 20 1 14 7 14"></polyline>
                                    <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
                                </svg>
                                <span class="sync-text">{localize(lang.get(), "sync_now")}</span>
                            }.into_view()
                        }}
                    </button>
                </div>

                /* Hamburger Button */
                <button class="hamburger-btn" on:click=move |_| is_menu_open.update(|b| *b = !*b)>
                    <svg viewBox="0 0 24 24" width="24" height="24" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="3" y1="12" x2="21" y2="12"></line>
                        <line x1="3" y1="6" x2="21" y2="6"></line>
                        <line x1="3" y1="18" x2="21" y2="18"></line>
                    </svg>
                </button>
            </div>
        </header>

        /* Sidebar Menu */
        <div class=move || if is_menu_open.get() { "sidebar-overlay open" } else { "sidebar-overlay" } on:click=move |_| is_menu_open.set(false)></div>
        <div class=move || if is_menu_open.get() { "sidebar-menu open" } else { "sidebar-menu" }>
            <div class="sidebar-header">
                <button class="close-menu-btn" on:click=move |_| is_menu_open.set(false)>
                    <svg viewBox="0 0 24 24" width="24" height="24" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="18" y1="6" x2="6" y2="18"></line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </button>
            </div>
            
            <div class="sidebar-mobile-controls">
                <div class="lang-switcher">
                    <button 
                        class=move || if lang.get() == Language::En { "lang-btn active" } else { "lang-btn" }
                        on:click=move |_| { set_lang.set(Language::En); is_menu_open.set(false); }
                    >
                        "EN"
                    </button>
                    <button 
                        class=move || if lang.get() == Language::Bn { "lang-btn active" } else { "lang-btn" }
                        on:click=move |_| { set_lang.set(Language::Bn); is_menu_open.set(false); }
                    >
                        "বাংলা"
                    </button>
                </div>
                <button class="theme-toggle-btn" on:click=move |e| { on_theme_toggle(e); is_menu_open.set(false); }>
                    {move || if theme.get() == Theme::Light { "🌙" } else { "☀️" }}
                </button>
            </div>
            
            <ul class="sidebar-links">
                <li><a href="/" on:click=move |_| is_menu_open.set(false)>"Latest News"</a></li>
                <li><a href="/favorites" on:click=move |_| is_menu_open.set(false)>"My Favorites"</a></li>
                <li><a href="/archive" on:click=move |_| is_menu_open.set(false)>"Archive"</a></li>
                <li><a href="/weekly-roundup" on:click=move |_| is_menu_open.set(false)>"Weekly AI Roundup"</a></li>
                <li><a href="/about" on:click=move |_| is_menu_open.set(false)>"About AI PulseQ"</a></li>
                <li><a href="/sources" on:click=move |_| is_menu_open.set(false)>"News Sources"</a></li>
                <li><a href="/changelog" on:click=move |_| is_menu_open.set(false)>"Version & Changelog"</a></li>
            </ul>

            <div style="margin-top: 20px; font-size: calc(1.1rem - 3pt); color: var(--text-muted); text-align: left; padding: 0 1rem; margin-left: 0; line-height: 1.5;">
                {move || {
                    let total = total_curated.get();
                    let date_str = if let Some(ts) = last_sync_timestamp.get() {
                        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ts as f64 * 1000.0));
                        let locale = if lang.get() == Language::Bn { "bn-BD" } else { "en-US" };
                        let options = js_sys::Object::new();
                        js_sys::Reflect::set(&options, &wasm_bindgen::JsValue::from_str("dateStyle"), &wasm_bindgen::JsValue::from_str("medium")).unwrap();
                        js_sys::Reflect::set(&options, &wasm_bindgen::JsValue::from_str("timeStyle"), &wasm_bindgen::JsValue::from_str("short")).unwrap();
                        date.to_locale_string(locale, &options).as_string().unwrap_or_default()
                    } else {
                        "".to_string()
                    };
                    
                    if lang.get() == Language::Bn {
                        format!("মোট {}টি এআই সংবাদ সংকলিত। শেষ সিঙ্ক: {}", crate::translate_digits(total), date_str)
                    } else {
                        format!("{} AI News articles curated. Last Synced {}", total, date_str)
                    }
                }}
            </div>
        </div>
    }
}
