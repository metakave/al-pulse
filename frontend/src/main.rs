use leptos::*;
mod footer;
use crate::footer::Footer;
use leptos_router::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct NewsItem {
    id: String,
    title_en: String,
    title_bn: String,
    url: String,
    source: String,
    summary_en: Option<String>,
    summary_bn: Option<String>,
    category: String,
    published_at: i64,
    created_at: i64,
    is_favorite: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct NewsStats {
    total_count: i32,
    since_last_sync: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct Toast {
    id: String,
    message: String,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Language {
    En,
    Bn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Theme {
    Light,
    Dark,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tab {
    Latest,
    Archive,
    Favorites,
}

// UI Localization Dictionary
fn localize(lang: Language, key: &str) -> &'static str {
    match lang {
        Language::En => match key {
            "title" => "AI Pulse",
            "tagline" => "Curated Intelligence",
            "latest_news" => "Latest News",
            "archive" => "Archive",
            "my_favorites" => "My Favorites",
            "auto_sync" => "Auto-Sync: ",
            "syncing" => "Syncing...",
            "sync_now" => "Sync Now",
            "search_placeholder" => "Search headlines, summaries or sources...",
            "no_articles" => "No articles found",
            "no_articles_desc" => "Try refining your search query, selecting another category, or syncing fresh headlines.",
            "no_favorites_desc" => "Click the heart icon on any article to add it to your favorites.",
            "sync_failed" => "Sync Connection Failed",
            "loading" => "Connecting to AI Pulse database...",
            "read" => "Read",
            "job_impact_badge" => "⚠️ Job Impact",
            _ => "",
        },
        Language::Bn => match key {
            "title" => "এআই পালস",
            "tagline" => "নির্বাচিত সংবাদ সংকলন",
            "latest_news" => "সর্বশেষ সংবাদ",
            "archive" => "আর্কাইভ",
            "my_favorites" => "আমার প্রিয়",
            "auto_sync" => "স্বয়ংক্রিয় সিঙ্ক: ",
            "syncing" => "সিঙ্ক হচ্ছে...",
            "sync_now" => "সিঙ্ক করুন",
            "search_placeholder" => "শিরোনাম, বিবরণ বা সোর্স খুঁজুন...",
            "no_articles" => "কোনো নিবন্ধ পাওয়া যায়নি",
            "no_articles_desc" => "অনুগ্রহ করে আপনার অনুসন্ধান পরিবর্তন করুন, অন্য ক্যাটাগরি বেছে নিন অথবা নতুন খবর সিঙ্ক করুন।",
            "no_favorites_desc" => "আপনার প্রিয় তালিকায় যুক্ত করতে যেকোনো খবরের হার্ট আইকনে ক্লিক করুন।",
            "sync_failed" => "সিঙ্ক সংযোগ ব্যর্থ হয়েছে",
            "loading" => "এআই পালস ডাটাবেসের সাথে সংযুক্ত হচ্ছে...",
            "read" => "পড়ুন",
            "job_impact_badge" => "⚠️ কর্মসংস্থান প্রভাব",
            _ => "",
        }
    }
}

// Category Localization
fn translate_category(lang: Language, cat: &str) -> String {
    if lang == Language::En {
        return cat.to_string();
    }
    match cat {
        "LLMs & Generative AI" => "এলএলএম এবং জেনারেটিভ এআই".to_string(),
        "Robotics & Autonomous" => "রোবোটিক্স এবং স্বায়ত্তশাসিত".to_string(),
        "Industry & Tech Giants" => "শিল্প ও প্রযুক্তি জায়ান্ট".to_string(),
        "Research & Science" => "গবেষণা ও বিজ্ঞান".to_string(),
        "AI Ethics & Policy" => "এআই নীতি ও নৈতিকতা".to_string(),
        "Job Impact" => "কর্মসংস্থান প্রভাব".to_string(),
        _ => cat.to_string(),
    }
}

// Source Publication Localization
fn translate_source(lang: Language, src: &str) -> String {
    if lang == Language::En {
        return src.to_string();
    }
    match src {
        "TechCrunch" => "টেকক্রাঞ্চ".to_string(),
        "VentureBeat" => "ভেঞ্চারবিট".to_string(),
        "Wired" => "ওয়্যার্ড".to_string(),
        "The New York Times" => "দ্য নিউ ইয়র্ক টাইমস".to_string(),
        "MIT Technology Review" => "এমআইটি টেকনোলজি রিভিউ".to_string(),
        "Forbes" => "ফোর্বস".to_string(),
        "Bloomberg" => "ব্লুমবার্গ".to_string(),
        "Reuters" => "রয়টার্স".to_string(),
        "Unite.AI" => "ইউনাইট.এআই".to_string(),
        "AI News" => "এআই নিউজ".to_string(),
        "MarkTechPost" => "মার্কটেকপোস্ট".to_string(),
        _ => src.to_string(),
    }
}

// Bengali Digit Converter Helper
fn translate_digits(num: i64) -> String {
    let s = num.to_string();
    let mut bn = String::new();
    for c in s.chars() {
        match c {
            '0' => bn.push('০'),
            '1' => bn.push('১'),
            '2' => bn.push('২'),
            '3' => bn.push('৩'),
            '4' => bn.push('৪'),
            '5' => bn.push('৫'),
            '6' => bn.push('৬'),
            '7' => bn.push('৭'),
            '8' => bn.push('৮'),
            '9' => bn.push('৯'),
            _ => bn.push(c),
        }
    }
    bn
}

// Localized Relative Date Formatter
fn format_relative_time(lang: Language, timestamp: i64) -> String {
    let now = (js_sys::Date::now() / 1000.0) as i64;
    let diff = now - timestamp;
    
    if diff < 0 {
        return if lang == Language::En { "Just now".to_string() } else { "এইমাত্র".to_string() };
    }
    
    let mins = diff / 60;
    if mins < 1 {
        return if lang == Language::En { "Just now".to_string() } else { "এইমাত্র".to_string() };
    }
    
    let hours = mins / 60;
    if hours < 1 {
        return if lang == Language::En {
            format!("{}m ago", mins)
        } else {
            format!("{} মিনিট আগে", translate_digits(mins))
        };
    }
    
    let days = hours / 24;
    if days < 1 {
        return if lang == Language::En {
            format!("{}h ago", hours)
        } else {
            format!("{} ঘণ্টা আগে", translate_digits(hours))
        };
    }
    
    if days == 1 {
        return if lang == Language::En { "Yesterday".to_string() } else { "গতকাল".to_string() };
    }
    
    if lang == Language::En {
        format!("{} days ago", days)
    } else {
        format!("{} দিন আগে", translate_digits(days))
    }
}

// Job Impact / Layoff Keyword Scanner (scans English fields)
fn is_job_displacement(title: &str, summary: &str) -> bool {
    let text = format!("{} {}", title, summary).to_lowercase();
    text.contains("layoff") || text.contains("laid off") ||
    text.contains("job loss") || text.contains("job cut") ||
    text.contains("cut jobs") || text.contains("lose jobs") ||
    text.contains("replace workers") || text.contains("worker replacement") ||
    text.contains("reducing headcount") || text.contains("unemployment") ||
    text.contains("downsizing") || text.contains("job displacement")
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="container about-container">
            <header class="about-header">
                <h1 class="about-title">"About AI Pulse"</h1>
                <p class="about-subtitle">"Bilingual Curation & Tech Insights"</p>
            </header>
            <div class="about-body">
                <p class="about-paragraph">
                    "AI Pulse is a curated intelligence platform dedicated to aggregating, analyzing, and synthesizing artificial intelligence news from around the globe. In an era where technological advancements shift daily, keeping pace with generative models, LLMs, robotic automations, policies, and industry transformations can be overwhelming. AI Pulse aims to simplify that landscape by providing clean, condensed, and accessible bilingual summaries."
                </p>
                <p class="about-paragraph">
                    "This platform operates with a built-in automated polling system that crawls authoritative global tech channels every 30 minutes, translates key updates into Bengali using real-time endpoints, and auto-categorizes articles. Special attention is given to tracking automation's impact on employment, highlighting job layoffs and displacement warnings."
                </p>
                <p class="about-paragraph">
                    "The ideation, design, and technical execution of this curation project was done by "
                    <strong>"Sadiq M Alam"</strong>
                    ", Enterprise AI Consultant, with the goal of keeping people updated about AI News from all over the world."
                </p>
                <a href="/" class="about-back-btn">
                    "← Back to Feed"
                </a>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {
        <Router>
            <Routes>
                <Route path="" view=|| view! { <App /> } />
                <Route path="/about" view=|| view! { <AboutPage /> } />
                <Route path="*any" view=|| view! { "Page not found." } />
            </Routes>
        </Router>
    });
}

#[component]
fn App() -> impl IntoView {
    // 1. Reactive Signals
    let (lang, set_lang) = create_signal(Language::En);
    let (theme, set_theme) = create_signal(Theme::Dark);
    let (search_query, set_search_query) = create_signal(String::new());
    let (active_category, set_active_category) = create_signal("All".to_string());
    let (active_tab, set_active_tab) = create_signal(Tab::Latest);

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
    let (refresh_trigger, set_refresh_trigger) = create_signal(0);
    
    // 2. Status Signals
    let (last_sync_timestamp, set_last_sync_timestamp) = create_signal(None::<i64>);
    let (seconds_to_sync, set_seconds_to_sync) = create_signal(1800); 
    let (toasts, set_toasts) = create_signal(Vec::<Toast>::new());

    // Helper: Toast Manager
    let show_toast = move |msg: &str| {
        let timestamp = js_sys::Date::now();
        let toast_id = format!("toast-{}", timestamp);
        let msg_str = msg.to_string();
        
        set_toasts.update(|t| {
            t.push(Toast {
                id: toast_id.clone(),
                message: msg_str,
            });
        });
        
        let id_to_remove = toast_id;
        set_timeout(move || {
            set_toasts.update(|t| {
                t.retain(|item| item.id != id_to_remove);
            });
        }, std::time::Duration::from_secs(3));
    };

    // 3. Resource to fetch news items
    let news_resource = create_resource(
        move || (
            search_query.get(),
            active_category.get(),
            active_tab.get(),
            refresh_trigger.get()
        ),
        |(q, cat, tab, _)| async move {
            let fav = tab == Tab::Favorites;
            let archive = tab == Tab::Archive;
            fetch_news(q, cat, fav, archive).await
        }
    );

    // Resource to fetch news stats
    let stats_resource = create_resource(
        move || (
            refresh_trigger.get(),
            news_resource.get(),
        ),
        |(trigger, _)| async move {
            fetch_stats(trigger).await
        }
    );

    // 4. Action for manual synchronizing
    let trigger_sync_action = create_action(move |_: &()| {
        async move {
            let res = Request::post("/api/news/refresh")
                .send()
                .await
                .map_err(|e| e.to_string())?;
            
            if res.ok() {
                let ts = fetch_api_status().await.unwrap_or_else(|_| (js_sys::Date::now() / 1000.0) as i64);
                Ok(ts)
            } else {
                Err(format!("Server error: {}", res.status()))
            }
        }
    });

    // 6. Shuffling mechanism: triggers random card ordering every 45 seconds
    let (shuffle_seed, set_shuffle_seed) = create_signal(0usize);
    let (is_fading, set_is_fading) = create_signal(false);
    
    create_effect(move |_| {
        let _ = set_interval_with_handle(move || {
            // Step 1: Start fade out
            set_is_fading.set(true);
            
            // Step 2: Swap order and fade in after a brief delay
            set_timeout(move || {
                set_shuffle_seed.update(|seed| *seed += 1);
                set_is_fading.set(false);
            }, std::time::Duration::from_millis(300));
        }, std::time::Duration::from_secs(45));
    });

    // 7. Initial hook to load synchronization status
    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(ts) = fetch_api_status().await {
                set_last_sync_timestamp.set(Some(ts));
            }
        });
    });

    // 6. Hook to reload feed and display toast when sync finishes
    create_effect(move |_| {
        if let Some(result) = trigger_sync_action.value().get() {
            match result {
                Ok(ts) => {
                    set_last_sync_timestamp.set(Some(ts));
                    set_refresh_trigger.update(|n| *n += 1);
                    
                    let success_msg = localize(lang.get(), "toast_sync_success");
                    show_toast(success_msg);
                }
                Err(e) => {
                    show_toast(&format!("Sync failed: {}", e));
                }
            }
            trigger_sync_action.value().set(None);
        }
    });

    // 7. Auto countdown timer (every 1 second)
    create_effect(move |_| {
        let _ = set_interval_with_handle(move || {
            if let Some(last) = last_sync_timestamp.get() {
                let now = (js_sys::Date::now() / 1000.0) as i64;
                let diff = (last + 30 * 60) - now;
                
                if diff <= 0 {
                    if !trigger_sync_action.pending().get() {
                        trigger_sync_action.dispatch(());
                    }
                } else {
                    set_seconds_to_sync.set(diff);
                }
            }
        }, std::time::Duration::from_secs(1));
    });

    // Formatter: Sync Countdown
    let format_countdown = move || {
        let current_lang = lang.get();
        let total_secs = seconds_to_sync.get();
        
        if trigger_sync_action.pending().get() {
            localize(current_lang, "syncing").to_string()
        } else if last_sync_timestamp.get().is_none() {
            if current_lang == Language::En { "--m --s".to_string() } else { "--মিঃ --সেঃ".to_string() }
        } else {
            let mins = total_secs / 60;
            let secs = total_secs % 60;
            if current_lang == Language::En {
                format!("{:02}m {:02}s", mins, secs)
            } else {
                format!("{}মিঃ {}সেঃ", translate_digits(mins), translate_digits(secs))
            }
        }
    };

    // Category configurations
    let categories = move || {
        let current_lang = lang.get();
        vec![
            ("All".to_string(), translate_category(current_lang, "All")),
            ("LLMs & Generative AI".to_string(), translate_category(current_lang, "LLMs & Generative AI")),
            ("Robotics & Autonomous".to_string(), translate_category(current_lang, "Robotics & Autonomous")),
            ("Industry & Tech Giants".to_string(), translate_category(current_lang, "Industry & Tech Giants")),
            ("Research & Science".to_string(), translate_category(current_lang, "Research & Science")),
            ("AI Ethics & Policy".to_string(), translate_category(current_lang, "AI Ethics & Policy")),
            ("Job Impact".to_string(), translate_category(current_lang, "Job Impact")),
        ]
    };

    view! {
        <div class="container">
            <header class="app-header">
                <div class="brand-section">
                    <div class="brand-logo-container">
                        <span class="brand-logo-icon">"⚡"</span>
                        <div class="pulse-dot"></div>
                    </div>
                    <div class="brand-text">
                        <h1>{move || localize(lang.get(), "title")}</h1>
                        <p>{move || localize(lang.get(), "tagline")}</p>
                    </div>
                </div>

                <div class="nav-and-status">
                    /* Language Switcher Toggle */
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

                    /* Theme Switcher Toggle */
                    <button 
                        class="theme-toggle-btn"
                        on:click=on_theme_toggle
                        title=move || if lang.get() == Language::En { "Toggle theme" } else { "থিম পরিবর্তন করুন" }
                    >
                        {move || if theme.get() == Theme::Light {
                            view! {
                                <svg viewBox="0 0 24 24">
                                    <path d="M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9 9-4.03 9-9c0-.46-.04-.92-.1-1.36-.98 1.37-2.58 2.26-4.4 2.26-2.98 0-5.4-2.42-5.4-5.4 0-1.81.89-3.42 2.26-4.4-.44-.06-.9-.1-1.36-.1z"/>
                                </svg>
                            }.into_view()
                        } else {
                            view! {
                                <svg viewBox="0 0 24 24">
                                    <path d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58a.996.996 0 000 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41L7.4 4.58a.996.996 0 00-1.41 0zM15.54 18.01a.996.996 0 000 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41l-1.06-1.06a.996.996 0 00-1.41 0zM7.05 18.01l-1.06 1.06a.996.996 0 101.41 1.41l1.06-1.06a.996.996 0 10-1.41-1.41zm11.35-13.43a.996.996 0 00-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06c.38-.38.38-1.02 0-1.41z"/>
                                </svg>
                            }.into_view()
                        }}
                    </button>

                    /* Nav Tabs (Feed vs Archive vs Favorites) */
                    <div class="nav-tabs">
                        <button 
                            class=move || if active_tab.get() == Tab::Latest { "tab-btn active" } else { "tab-btn" }
                            on:click=move |_| set_active_tab.set(Tab::Latest)
                        >
                            {move || localize(lang.get(), "latest_news")}
                        </button>
                        <button 
                            class=move || if active_tab.get() == Tab::Archive { "tab-btn active" } else { "tab-btn" }
                            on:click=move |_| set_active_tab.set(Tab::Archive)
                        >
                            {move || localize(lang.get(), "archive")}
                        </button>
                        <button 
                            class=move || if active_tab.get() == Tab::Favorites { "tab-btn active" } else { "tab-btn" }
                            on:click=move |_| set_active_tab.set(Tab::Favorites)
                        >
                            {move || localize(lang.get(), "my_favorites")}
                        </button>
                    </div>

                    /* Sync Status Panel */
                    <div class="status-widget">
                        <div class=move || if trigger_sync_action.pending().get() { "status-indicator syncing" } else { "status-indicator" }></div>
                        <button 
                            class="refresh-btn" 
                            disabled=move || trigger_sync_action.pending().get()
                            on:click=move |_| trigger_sync_action.dispatch(())
                        >
                            {move || if trigger_sync_action.pending().get() {
                                view! { <span class="spin-icon">"⟳"</span> }.into_view()
                            } else {
                                view! { <span>{move || localize(lang.get(), "sync_now")}</span> }.into_view()
                            }}
                        </button>
                    </div>
                </div>
            </header>

            /* Statistics Banner */
            {move || stats_resource.get().map(|res| {
                match res {
                    Ok(stats) => {
                        let total = stats.total_count;
                        let since_last = stats.since_last_sync;
                        
                        let text = if lang.get() == Language::En {
                            format!("{} AI News articles curated, {} new since last sync. Next sync in ", total, since_last)
                        } else {
                            format!("মোট {}টি এআই সংবাদ সংকলিত, শেষ সিঙ্কের পর {}টি নতুন খবর। পরবর্তী সিঙ্ক হতে বাকি ", translate_digits(total as i64), translate_digits(since_last as i64))
                        };

                        view! {
                            <div class="stats-banner">
                                <span class="stats-text">
                                    {text}
                                    <span class="stats-countdown">{format_countdown}</span>
                                </span>
                            </div>
                        }.into_view()
                    }
                    Err(_) => view! {}.into_view()
                }
            })}

            /* Dashboard Search & Categories Controls */
            <section class="dashboard-controls">
                <div class="search-wrapper">
                    <input 
                        type="text" 
                        class="search-input" 
                        placeholder=move || localize(lang.get(), "search_placeholder")
                        prop:value=search_query
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                    />
                    /* Magnifying Glass Icon */
                    <svg class="search-icon" viewBox="0 0 24 24">
                        <path d="M9.5 3A6.5 6.5 0 0 1 16 9.5c0 1.61-.59 3.09-1.56 4.23l.27.27h.79l5 5-1.5 1.5-5-5v-.79l-.27-.27A6.516 6.516 0 0 1 9.5 16 6.5 6.5 0 0 1 3 9.5 6.5 6.5 0 0 1 9.5 3m0 2C7 5 5 7 5 9.5S7 14 9.5 14 14 12 14 9.5 12 5 9.5 5z"/>
                    </svg>
                </div>

                <div class="categories-wrapper">
                    <div class="categories-list">
                        {move || categories().into_iter().map(|(id, display_name)| {
                            let current_id = id;
                            let id_for_active = current_id.clone();
                            let id_for_click = current_id.clone();
                            let is_active = move || active_category.get() == id_for_active;
                            view! {
                                <button 
                                    class=move || if is_active() { "category-pill active" } else { "category-pill" }
                                    on:click=move |_| set_active_category.set(id_for_click.clone())
                                >
                                    {display_name}
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </section>

            /* Feed Content Grid */
            <main>
                {move || {
                    news_resource.get().map(|res| {
                        match res {
                            Ok(items) => {
                                if items.is_empty() {
                                    view! {
                                        <div class="empty-state">
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                                <circle cx="12" cy="12" r="10" />
                                                <path d="M8 12h8" />
                                            </svg>
                                            <h3>{move || localize(lang.get(), "no_articles")}</h3>
                                            <p>
                                                {move || {
                                                    if active_tab.get() == Tab::Favorites {
                                                        localize(lang.get(), "no_favorites_desc")
                                                    } else {
                                                        localize(lang.get(), "no_articles_desc")
                                                    }
                                                }}
                                            </p>
                                        </div>
                                    }.into_view()
                                } else {
                                     // Shuffle items using a simple seed-based hashing selector
                                     let mut shuffled_items = items;
                                     let seed = shuffle_seed.get();
                                     if seed > 0 {
                                         // Deterministically pseudo-randomize item layout using their ID hashes
                                         shuffled_items.sort_by(|a, b| {
                                             let hash_a = a.id.chars().fold(seed, |acc, c| acc.wrapping_add(c as usize).wrapping_mul(31));
                                             let hash_b = b.id.chars().fold(seed, |acc, c| acc.wrapping_add(c as usize).wrapping_mul(31));
                                             hash_a.cmp(&hash_b)
                                         });
                                     }

                                     view! {
                                         <div class=move || if is_fading.get() { "news-grid grid-fade-out" } else { "news-grid" }>
                                             {shuffled_items.into_iter().enumerate().map(|(idx, item)| {
                                                 let current_lang = lang.get();
                                                 let item_id = item.id.clone();
                                                 let is_fav = item.is_favorite;
                                                 
                                                 // Scan job impact status from English texts
                                                 let job_loss = is_job_displacement(&item.title_en, &item.summary_en.clone().unwrap_or_default());
                                                 
                                                 // Localize text fields based on Language Switcher
                                                 let title_display = if current_lang == Language::Bn { item.title_bn.clone() } else { item.title_en.clone() };
                                                 let summary_display = if current_lang == Language::Bn { 
                                                     item.summary_bn.clone().unwrap_or_default() 
                                                 } else { 
                                                     item.summary_en.clone().unwrap_or_default() 
                                                 };
                                                 
                                                 let category_display = translate_category(current_lang, &item.category);
                                                 let source_display = translate_source(current_lang, &item.source);
                                                 let date_display = format_relative_time(current_lang, item.published_at);
                                                 
                                                 let cat_class = match item.category.as_str() {
                                                     "LLMs & Generative AI" => "category-badge llm",
                                                     "Robotics & Autonomous" => "category-badge robotics",
                                                     "Industry & Tech Giants" => "category-badge giants",
                                                     "Research & Science" => "category-badge research",
                                                     "AI Ethics & Policy" => "category-badge ethics",
                                                     "Job Impact" => "category-badge job-impact",
                                                     _ => "category-badge",
                                                 };
                                                 
                                                 let card_classes = if job_loss { 
                                                     if is_fading.get() { "news-card job-impact fading" } else { "news-card job-impact" } 
                                                 } else { 
                                                     if is_fading.get() { "news-card fading" } else { "news-card" }
                                                 };
                                                 let url_display = item.url.clone();
                                                 
                                                 let on_fav_toggle = move |e: web_sys::MouseEvent| {
                                                     e.stop_propagation();
                                                     let next_fav = !is_fav;
                                                     
                                                     // Optimistic UI updates
                                                     let id_for_update = item_id.clone();
                                                     news_resource.update(move |data| {
                                                         if let Some(Ok(ref mut list)) = data {
                                                             if let Some(target) = list.iter_mut().find(|i| i.id == id_for_update) {
                                                                 target.is_favorite = next_fav;
                                                             }
                                                         }
                                                     });

                                                     // Send backend updates
                                                     let id_param = item_id.clone();
                                                     spawn_local(async move {
                                                         let payload = serde_json::json!({ "is_favorite": next_fav });
                                                         let url = format!("/api/news/{}/favorite", id_param);
                                                         match Request::post(&url)
                                                             .header("Content-Type", "application/json")
                                                             .json(&payload)
                                                         {
                                                             Ok(builder) => {
                                                                 if let Ok(response) = builder.send().await {
                                                                     if response.ok() {
                                                                         let toast_key = if next_fav { "toast_fav_added" } else { "toast_fav_removed" };
                                                                         show_toast(localize(current_lang, toast_key));
                                                                     } else {
                                                                         show_toast("Failed to update favorites.");
                                                                     }
                                                                 } else {
                                                                     show_toast("Connection error.");
                                                                 }
                                                             }
                                                             Err(_) => show_toast("Error building request."),
                                                         }
                                                     });
                                                 };

                                                 view! {
                                                     <article class=card_classes style=format!("order: {}", idx)>
                                                         <div class="card-meta">
                                                             <span class="source-badge">{source_display}</span>
                                                             
                                                             // Highlight with Job Impact Badge if flagged
                                                             {if job_loss {
                                                                 view! {
                                                                     <span class="job-impact-badge">
                                                                         {move || localize(lang.get(), "job_impact_badge")}
                                                                     </span>
                                                                 }.into_view()
                                                             } else {
                                                                 view! {
                                                                     <span class=cat_class>{category_display.clone()}</span>
                                                                 }.into_view()
                                                             }}
                                                             
                                                             <button 
                                                                 class=move || if is_fav { "favorite-btn is-fav" } else { "favorite-btn" }
                                                                 on:click=on_fav_toggle
                                                                 title=if is_fav { "Remove from favorites" } else { "Save to favorites" }
                                                             >
                                                                 <svg viewBox="0 0 24 24">
                                                                     <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
                                                                 </svg>
                                                             </button>
                                                         </div>
                                                         
                                                         <div class="card-content">
                                                             <a class="card-title" href=url_display.clone() target="_blank" rel="noopener noreferrer">
                                                                 {title_display}
                                                            </a>
                                                            <p class="card-summary">{summary_display}</p>
                                                        </div>
                                                        
                                                        <div class="card-footer">
                                                            <span>{date_display}</span>
                                                            <a class="read-link" href=url_display target="_blank" rel="noopener noreferrer">
                                                                {move || localize(lang.get(), "read")}
                                                                <span class="arrow">" ↗"</span>
                                                            </a>
                                                        </div>
                                                    </article>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_view()
                                }
                            }
                            Err(_) => {
                                view! {
                                    <div class="empty-state" style="border-color: var(--favorite-color);">
                                        <Footer />
                                    </div>
                                }.into_view()
                            }
                        }
                    }).unwrap_or_else(|| {
                        view! {
                            <div class="loading-indicator">
                                <div class="loading-spinner"></div>
                                <p>{move || localize(lang.get(), "loading")}</p>
                            </div>
                        }.into_view()
                    })
                }}
            </main>

            /* Persistent Footer */
                        <Footer />


            /* Floating Toasts */
            <div class="toast-container">
                {move || toasts.get().into_iter().map(|toast| {
                    view! {
                        <div class="toast">
                            <span class="toast-icon">"⚡"</span>
                            <span class="toast-message">{toast.message}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

// Returns the API base URL — empty string in local dev (relative), full URL in production
fn api_base() -> String {
    // Set VITE_API_URL (or API_URL) at build time for production, e.g. https://your-app.railway.app
    option_env!("API_URL").unwrap_or("").to_string()
}

// Global API Helper: Fetch news from Backend
async fn fetch_news(q: String, category: String, favorites: bool, archive: bool) -> Result<Vec<NewsItem>, String> {
    let mut url = format!("{}/api/news?favorites={}&archive={}", api_base(), favorites, archive);
    if !q.is_empty() {
        let encoded_q = js_sys::encode_uri_component(&q);
        let q_str: String = encoded_q.into();
        url.push_str(&format!("&q={}", q_str));
    }
    if !category.is_empty() && category != "All" {
        let encoded_cat = js_sys::encode_uri_component(&category);
        let cat_str: String = encoded_cat.into();
        url.push_str(&format!("&category={}", cat_str));
    }

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP request failed with status: {}", response.status()));
    }

    response
        .json::<Vec<NewsItem>>()
        .await
        .map_err(|e| e.to_string())
}

// Global API Helper: Fetch API Status
async fn fetch_api_status() -> Result<i64, String> {
    let url = format!("{}/api/status", api_base());
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP status request failed: {}", response.status()));
    }

    #[derive(Deserialize)]
    struct Status {
        last_refreshed_at: i64,
    }

    let status = response
        .json::<Status>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(status.last_refreshed_at)
}

// Global API Helper: Fetch news stats from Backend
async fn fetch_stats(refresh_trigger: i32) -> Result<NewsStats, String> {
    let url = format!("{}/api/news/stats?t={}", api_base(), refresh_trigger);
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP stats request failed: {}", response.status()));
    }

    response
        .json::<NewsStats>()
        .await
        .map_err(|e| e.to_string())
}
