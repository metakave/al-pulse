use leptos::*;
mod footer;
use crate::footer::Footer;
use leptos_router::*;
use leptos_meta::*;
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
            "title" => "AI PulseQ",
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
            "loading" => "Connecting to AI PulseQ database...",
            "read" => "Read",
            "job_impact_badge" => "Job",
            _ => "",
        },
        Language::Bn => match key {
            "title" => "এআই পালসকিউ",
            "tagline" => "বুদ্ধিমত্তার সংকলন",
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
            "loading" => "এআই পালসকিউ ডাটাবেসের সাথে সংযুক্ত হচ্ছে...",
            "read" => "পড়ুন",
            "job_impact_badge" => "কর্মসংস্থান প্রভাব",
            _ => "",
        }
    }
}

// Category Localization
fn translate_category(lang: Language, cat: &str) -> String {
    if lang == Language::En {
        if cat == "Job Impact" {
            return "AI Job Impact".to_string();
        }
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
        "Google News India (AI/Layoffs)" => "গুগল নিউজ ইন্ডিয়া (এআই/লেঅফ)".to_string(),
        "Google News Pakistan (AI/Layoffs)" => "গুগল নিউজ পাকিস্তান (এআই/লেঅফ)".to_string(),
        "Google News Sri Lanka (AI/Layoffs)" => "গুগল নিউজ শ্রীলঙ্কা (এআই/লেঅফ)".to_string(),
        "Inc42" => "আইএনসি৪২".to_string(),
        "ProPakistani" => "প্রোপাকিস্তানি".to_string(),
        "Daily FT" => "ডেইলি এফটি".to_string(),
        "OpenAI Newsroom" => "ওপেনএআই নিউজরুম".to_string(),
        "Google Research AI Blog" => "গুগল রিসার্চ এআই ব্লগ".to_string(),
        "Hugging Face Blog" => "হাগিং ফেস ব্লগ".to_string(),
        "Ahead of AI" => "অ্যাহেড অব এআই".to_string(),
        _ => src.to_string(),
    }
}

// Bengali Digit Converter Helper
pub fn translate_digits(num: i64) -> String {
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
    text.contains("layoff") || text.contains("lay off") || text.contains("laid off") ||
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
                <h1 class="about-title">"About AI PulseQ"</h1>
                <p class="about-subtitle">"Bilingual Curation & Tech Insights"</p>
            </header>
            <div class="about-body">
                <p class="about-paragraph">
                    "AI PulseQ is a curated intelligence platform dedicated to aggregating, analyzing, and synthesizing artificial intelligence news from around the globe. In an era where technological advancements shift daily, keeping pace with generative models, LLMs, robotic automations, policies, and industry transformations can be overwhelming. AI PulseQ aims to simplify that landscape by providing clean, condensed, and accessible bilingual summaries."
                </p>
                <p class="about-paragraph">
                    "This platform operates with a built-in automated polling system that crawls authoritative global tech channels every 45 minutes, translates key updates into Bengali using real-time endpoints, and auto-categorizes articles. Special attention is given to tracking automation's impact on employment, highlighting job layoffs and displacement warnings."
                </p>
                <p class="about-paragraph">
                    "The ideation, design, and technical execution of this curation project was done by "
                    <strong><a href="https://sadiqalam.com" target="_blank" rel="noopener noreferrer" style="color: inherit; text-decoration: underline;">"Sadiq M Alam"</a></strong>
                    ", Enterprise AI Consultant, with the goal of keeping people updated about AI News from all over the world."
                </p>
                <a href="/" class="about-back-btn">
                    "← Back to Feed"
                </a>
            </div>
        </div>
    }
}

#[component]
fn ChangelogPage() -> impl IntoView {
    view! {
        <div class="container about-container">
            <header class="about-header">
                <h1 class="about-title">"Change Log"</h1>
                <p class="about-subtitle">"Release notes and updates"</p>
            </header>
            <div class="changelog-body">
                <div class="changelog-timeline">
                    <div class="changelog-item">
                        <div class="changelog-dot"></div>
                        <div class="changelog-content">
                            <div class="changelog-version">"AI PulseQ version 1.0.1"</div>
                            <div class="changelog-date">"June 2026"</div>
                            <ul class="changelog-list">
                                <li>"Renamed application title to AI PulseQ."</li>
                                <li>"Introduced a new sleek slide-in hamburger menu for desktop."</li>
                                <li>"Optimized and cleaned up the mobile header layout."</li>
                                <li>"Added this Change Log page to track future updates."</li>
                                <li>"Adding Weekly Roundup."</li>
                            </ul>
                        </div>
                    </div>
                    <div class="changelog-item">
                        <div class="changelog-dot"></div>
                        <div class="changelog-content">
                            <div class="changelog-version">"AI Pulse version 1.0.0"</div>
                            <div class="changelog-date">"Initial Release"</div>
                            <ul class="changelog-list">
                                <li>"Launched core functionality with automated news polling."</li>
                                <li>"Bilingual curation with real-time translation."</li>
                                <li>"Auto-categorization of news articles."</li>
                            </ul>
                        </div>
                    </div>
                </div>
                <a href="/" class="about-back-btn">
                    "← Back to Feed"
                </a>
            </div>
        </div>
    }
}

#[component]
fn NewsSourcesPage() -> impl IntoView {
    view! {
        <div class="container about-container">
            <header class="about-header">
                <h1 class="about-title">"News Sources"</h1>
                <p class="about-subtitle">"List of curated RSS feeds powering AI PulseQ (Excluding Google News aggregations)"</p>
            </header>
            <div class="sources-body">
                <ul class="sources-list" style="list-style: none; padding: 0; display: flex; flex-direction: column; gap: 1rem;">
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"TechCrunch"</strong>": "<a href="https://techcrunch.com/category/artificial-intelligence/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://techcrunch.com/category/artificial-intelligence/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"VentureBeat"</strong>": "<a href="https://venturebeat.com/category/ai/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://venturebeat.com/category/ai/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"MIT Technology Review"</strong>": "<a href="https://www.technologyreview.com/topic/artificial-intelligence/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.technologyreview.com/topic/artificial-intelligence/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Unite.AI"</strong>": "<a href="https://www.unite.ai/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.unite.ai/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"AI News"</strong>": "<a href="https://www.artificialintelligence-news.com/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.artificialintelligence-news.com/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"MarkTechPost"</strong>": "<a href="https://www.marktechpost.com/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.marktechpost.com/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"The AI Report"</strong>": "<a href="https://theaireport.com/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://theaireport.com/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"AI Trends"</strong>": "<a href="https://aitrends.com/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://aitrends.com/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Synced Review"</strong>": "<a href="https://syncedreview.com/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://syncedreview.com/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"TechCrunch Layoffs"</strong>": "<a href="https://techcrunch.com/tag/layoffs/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://techcrunch.com/tag/layoffs/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Wired"</strong>": "<a href="https://www.wired.com/feed/tag/ai/latest/rss" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.wired.com/feed/tag/ai/latest/rss"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"OpenAI Newsroom"</strong>": "<a href="https://openai.com/news/rss.xml" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://openai.com/news/rss.xml"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Google Research AI Blog"</strong>": "<a href="https://blog.google/technology/ai/rss/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://blog.google/technology/ai/rss/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Hugging Face Blog"</strong>": "<a href="https://huggingface.co/blog/feed.xml" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://huggingface.co/blog/feed.xml"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Ahead of AI"</strong>": "<a href="https://magazine.sebastianraschka.com/feed" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://magazine.sebastianraschka.com/feed"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Inc42"</strong>": "<a href="https://inc42.com/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://inc42.com/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"ProPakistani"</strong>": "<a href="https://propakistani.pk/category/tech/feed" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://propakistani.pk/category/tech/feed"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Daily FT"</strong>": "<a href="https://www.ft.lk/rss/IT-Telecom-Tech" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.ft.lk/rss/IT-Telecom-Tech"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Crunchbase News"</strong>": "<a href="https://news.crunchbase.com/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://news.crunchbase.com/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Anthropic News"</strong>": "<a href="https://www.anthropic.com/feed.xml" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.anthropic.com/feed.xml"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"InformationWeek"</strong>": "<a href="https://www.informationweek.com/rss.xml" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.informationweek.com/rss.xml"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Oracle News"</strong>": "<a href="https://www.oracle.com/news/rss/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.oracle.com/news/rss/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Amazon News"</strong>": "<a href="https://www.aboutamazon.com/news/rss.xml" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.aboutamazon.com/news/rss.xml"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Cisco Newsroom"</strong>": "<a href="https://newsroom.cisco.com/c/services/i/servlets/newsroom/rssfeed.json" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://newsroom.cisco.com/c/services/i/servlets/newsroom/rssfeed.json"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"PayPal News"</strong>": "<a href="https://newsroom.paypal-corp.com/news?template=rss" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://newsroom.paypal-corp.com/news?template=rss"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Cloudflare Blog"</strong>": "<a href="https://blog.cloudflare.com/rss" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://blog.cloudflare.com/rss"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Chegg Investor Relations"</strong>": "<a href="https://investor.chegg.com/rss" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://investor.chegg.com/rss"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"The American Bazaar"</strong>": "<a href="https://americanbazaaronline.com/feed/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://americanbazaaronline.com/feed/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Ars Technica"</strong>": "<a href="http://feeds.arstechnica.com/arstechnica/index" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"http://feeds.arstechnica.com/arstechnica/index"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"ZDNet"</strong>": "<a href="https://www.zdnet.com/news/rss.xml" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.zdnet.com/news/rss.xml"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"CNET"</strong>": "<a href="https://www.cnet.com/rss/news/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.cnet.com/rss/news/"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"PCMag"</strong>": "<a href="https://www.pcmag.com/feeds/rss/latest" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.pcmag.com/feeds/rss/latest"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Engadget"</strong>": "<a href="https://www.engadget.com/rss.xml" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.engadget.com/rss.xml"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"The Verge"</strong>": "<a href="https://www.theverge.com/rss/index.xml" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.theverge.com/rss/index.xml"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Gizmodo"</strong>": "<a href="https://gizmodo.com/rss" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://gizmodo.com/rss"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Times of India Technology"</strong>": "<a href="https://timesofindia.indiatimes.com/rssfeeds/66949542.cms" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://timesofindia.indiatimes.com/rssfeeds/66949542.cms"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"Bloomberg Professional Insights"</strong>": "<a href="https://www.bloomberg.com/professional/insights/category/artificial-intelligence" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.bloomberg.com/professional/insights/category/artificial-intelligence"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"BBC News AI"</strong>": "<a href="https://www.bbc.com/news/topics/ce1qrvleleqt" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.bbc.com/news/topics/ce1qrvleleqt"</a></li>
                    <li style="padding: 1rem; background: var(--panel-bg); border: 1px solid var(--panel-border); border-radius: var(--border-radius-md);"><strong>"The Information"</strong>": "<a href="https://www.theinformation.com/" target="_blank" style="color: var(--accent-cyan); word-break: break-all;">"https://www.theinformation.com/"</a></li>
                </ul>
                <a href="/" class="about-back-btn" style="margin-top: 2rem;">
                    "← Back to Feed"
                </a>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

mod header;
mod weekly;

use crate::header::GlobalHeader;
use crate::weekly::{WeeklyRoundupListPage, WeeklyRoundupDetailPage};

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    
    // Lift state globally
    let (lang, set_lang) = create_signal(Language::En);
    let (theme, set_theme) = create_signal(Theme::Light);
    let (last_sync_timestamp, set_last_sync_timestamp) = create_signal(None::<i64>);
    let (seconds_to_sync, set_seconds_to_sync) = create_signal(2700i64); 
    let (active_tab, set_active_tab) = create_signal(Tab::Latest);
    let (refresh_trigger, set_refresh_trigger) = create_signal(0i32);
    
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

    provide_context(lang);
    provide_context(set_lang);
    provide_context(theme);
    provide_context(set_theme);
    provide_context(last_sync_timestamp);
    provide_context(set_last_sync_timestamp);
    provide_context(seconds_to_sync);
    provide_context(set_seconds_to_sync);
    provide_context(active_tab);
    provide_context(set_active_tab);
    provide_context(refresh_trigger);
    provide_context(set_refresh_trigger);
    provide_context(trigger_sync_action);

    view! {
        <Router>
            <div class=move || format!("app-container theme-{:?}", theme.get())>
                <GlobalHeader />
                <Routes>
                    <Route path="" view=|| view! { <Home /> } />
                    <Route path="/weekly-roundup" view=|| view! { <WeeklyRoundupListPage /> } />
                    <Route path="/weekly-roundup/:id" view=|| view! { <WeeklyRoundupDetailPage /> } />
                    <Route path="/about" view=|| view! { <AboutPage /> } />
                    <Route path="/changelog" view=|| view! { <ChangelogPage /> } />
                    <Route path="/sources" view=|| view! { <NewsSourcesPage /> } />
                    <Route path="/:category" view=|| view! { <Home /> } />
                    <Route path="*any" view=|| view! { "Page not found." } />
                </Routes>
                <Footer />
            </div>
        </Router>
    }
}

fn category_to_slug(cat: &str) -> String {
    match cat {
        "LLMs & Generative AI" => "llms-generative-ai".to_string(),
        "Robotics & Autonomous" => "robotics-autonomous".to_string(),
        "Industry & Tech Giants" => "industry-tech-giants".to_string(),
        "Research & Science" => "research-science".to_string(),
        "AI Ethics & Policy" => "ai-ethics-policy".to_string(),
        "Job Impact" => "ai-job-impact".to_string(),
        _ => "".to_string(),
    }
}

fn slug_to_category(slug: &str) -> String {
    match slug {
        "llms-generative-ai" => "LLMs & Generative AI".to_string(),
        "robotics-autonomous" => "Robotics & Autonomous".to_string(),
        "industry-tech-giants" => "Industry & Tech Giants".to_string(),
        "research-science" => "Research & Science".to_string(),
        "ai-ethics-policy" => "AI Ethics & Policy".to_string(),
        "ai-job-impact" => "Job Impact".to_string(),
        _ => "All".to_string(),
    }
}

#[component]
fn Home() -> impl IntoView {
    let lang = expect_context::<ReadSignal<Language>>();
    let theme = expect_context::<ReadSignal<Theme>>();
    let refresh_trigger = expect_context::<ReadSignal<i32>>();
    let set_refresh_trigger = expect_context::<WriteSignal<i32>>();
    let last_sync_timestamp = expect_context::<ReadSignal<Option<i64>>>();
    let set_last_sync_timestamp = expect_context::<WriteSignal<Option<i64>>>();
    let seconds_to_sync = expect_context::<ReadSignal<i64>>();
    let trigger_sync_action = expect_context::<Action<(), Result<i64, String>>>();
    
    let (search_query, set_search_query) = create_signal(String::new());
    
    let params = use_params_map();
    let active_category = create_memo(move |_| {
        if let Some(slug) = params.get().get("category") {
            slug_to_category(slug)
        } else {
            "All".to_string()
        }
    });
    let active_tab = expect_context::<ReadSignal<Tab>>();
    let set_active_tab = expect_context::<WriteSignal<Tab>>();
    let (current_page, set_current_page) = create_signal(1usize);
    let (is_menu_open, set_is_menu_open) = create_signal(false);

    create_effect(move |_| {
        // Track the inputs so we reset the page whenever they change
        let _ = active_category.get();
        let _ = active_tab.get();
        let _ = search_query.get();
        
        // Reset current page to 1
        set_current_page.set(1);
    });

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



    // 6. Cycling mechanism: rotates card position every 60 seconds
    let (cycle_offset, set_cycle_offset) = create_signal(0usize);
    let (is_fading, set_is_fading) = create_signal(false);
    
    create_effect(move |_| {
        if let Ok(handle) = set_interval_with_handle(move || {
            // Step 1: Start fade out
            set_is_fading.set(true);
            
            // Step 2: Cycle position and fade in after a brief delay
            set_timeout(move || {
                set_cycle_offset.update(|offset| *offset += 1);
                set_is_fading.set(false);
            }, std::time::Duration::from_millis(350));
        }, std::time::Duration::from_secs(60)) {
            on_cleanup(move || {
                handle.clear();
            });
        }
    });

    // Hook to reload feed and display toast when sync finishes
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
            // Do NOT clear action value here, otherwise other effects might miss it.
        }
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
        <Title text=move || format!("{} News - AI PulseQ", active_category.get()) />
        <Meta name="description" content=move || format!("AI PulseQ: Latest curated news and insights about {} in the AI world.", active_category.get()) />
        <div class="container">

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
                            let current_id = id.clone();
                            let slug = category_to_slug(&current_id);
                            let href = if slug.is_empty() { "/".to_string() } else { format!("/{}", slug) };
                            let is_active = move || active_category.get() == current_id;
                            view! {
                                <a 
                                    href=href
                                    class=move || if is_active() { "category-pill active" } else { "category-pill" }
                                >
                                    {display_name}
                                </a>
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
                                     // 1. Pagination slice
                                     let page_size = 48usize;
                                     let total_items = items.len();
                                     let total_pages = (total_items + page_size - 1) / page_size;
                                     
                                     view! {
                                         <div class=move || if is_fading.get() { "news-grid grid-fade-out" } else { "news-grid" }>
                                             {move || {
                                                 let current_p = current_page.get().min(total_pages).max(1);
                                                 let start_idx = (current_p - 1) * page_size;
                                                 let mut page_items = items.clone().into_iter().skip(start_idx).take(page_size).collect::<Vec<_>>();
                                                 
                                                 let page_len = page_items.len();
                                                 if page_len > 0 {
                                                     let offset = cycle_offset.get() % page_len;
                                                     page_items.rotate_left(offset);
                                                 }
                                                 
                                                 page_items.into_iter().enumerate().map(|(idx, item)| {
                                                     let current_lang = lang.get();
                                                     let item_id = item.id.clone();
                                                     let is_fav = item.is_favorite;
                                                     
                                                     let job_loss = is_job_displacement(&item.title_en, &item.summary_en.clone().unwrap_or_default());
                                                     
                                                     let title_display = if current_lang == Language::Bn { item.title_bn.clone() } else { item.title_en.clone() };
                                                     let summary_display = if current_lang == Language::Bn { 
                                                         item.summary_bn.clone().unwrap_or_default() 
                                                     } else { 
                                                         item.summary_en.clone().unwrap_or_default() 
                                                     };
                                                     
                                                     let category_display = if current_lang == Language::En {
                                                         match item.category.as_str() {
                                                             "LLMs & Generative AI" => "Gen AI",
                                                             "Robotics & Autonomous" => "Robotics",
                                                             "Industry & Tech Giants" => "Tech Giants",
                                                             "Research & Science" => "Research",
                                                             "AI Ethics & Policy" => "Ethics",
                                                             "Job Impact" => "Job",
                                                             _ => item.category.as_str(),
                                                         }.to_string()
                                                     } else {
                                                         translate_category(current_lang, &item.category)
                                                     };
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
                                                         
                                                         let id_for_update = item_id.clone();
                                                         news_resource.update(move |data| {
                                                             if let Some(Ok(ref mut list)) = data {
                                                                 if let Some(target) = list.iter_mut().find(|i| i.id == id_for_update) {
                                                                     target.is_favorite = next_fav;
                                                                 }
                                                             }
                                                         });

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
                                                                     <span class="arrow">" ->"</span>
                                                                 </a>
                                                             </div>
                                                         </article>
                                                     }
                                                 }).collect::<Vec<_>>()
                                             }}
                                         </div>

                                         // Pagination Component
                                         {move || {
                                             if total_pages > 1 {
                                                 view! {
                                                     <div class="pagination-container">
                                                         <button 
                                                             class="pagination-btn"
                                                             disabled=move || current_page.get().min(total_pages).max(1).le(&1)
                                                             on:click=move |_| {
                                                                 let p = current_page.get().min(total_pages).max(1);
                                                                 if p > 1 {
                                                                     set_current_page.set(p - 1);
                                                                     if let Some(window) = web_sys::window() {
                                                                         window.scroll_to_with_x_and_y(0.0, 400.0);
                                                                     }
                                                                 }
                                                             }
                                                         >
                                                             {move || if lang.get() == Language::Bn { "পূর্ববর্তী" } else { "Previous" }}
                                                         </button>
                                                         
                                                         <span class="pagination-info">
                                                             {move || {
                                                                 let current_p = current_page.get().min(total_pages).max(1);
                                                                 let page_str = if lang.get() == Language::Bn { translate_digits(current_p as i64) } else { current_p.to_string() };
                                                                 let total_str = if lang.get() == Language::Bn { translate_digits(total_pages as i64) } else { total_pages.to_string() };
                                                                 if lang.get() == Language::Bn {
                                                                     format!("পৃষ্ঠা {} এর {}", page_str, total_str)
                                                                 } else {
                                                                     format!("Page {} of {}", page_str, total_str)
                                                                 }
                                                             }}
                                                         </span>
                                                         
                                                         <button 
                                                             class="pagination-btn"
                                                             disabled=move || current_page.get().min(total_pages).max(1).ge(&total_pages)
                                                             on:click=move |_| {
                                                                 let p = current_page.get().min(total_pages).max(1);
                                                                 if p < total_pages {
                                                                     set_current_page.set(p + 1);
                                                                     if let Some(window) = web_sys::window() {
                                                                         window.scroll_to_with_x_and_y(0.0, 400.0);
                                                                     }
                                                                 }
                                                             }
                                                         >
                                                             {move || if lang.get() == Language::Bn { "পরবর্তী" } else { "Next" }}
                                                         </button>
                                                     </div>
                                                 }.into_view()
                                             } else {
                                                 view! { <div /> }.into_view()
                                             }
                                         }}
                                     }.into_view()
                                }
                            }
                            Err(_) => {
                                view! {
                                    <div class="empty-state" style="border-color: var(--favorite-color);">
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
            /* Persistent Footer Removed - Now in App */


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
