use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

// Export feed definitions for reuse
mod feeds;
use feeds::FEEDS;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

mod db;

struct AppState {
    pool: sqlx::PgPool,
    last_refreshed_at: Mutex<i64>,
}

#[tokio::main]
async fn main() {
    // 1. Initialize PostgreSQL Database
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/ai_news".to_string()
    });
    println!("Initializing PostgreSQL database...");
    let pool = db::init_db(&database_url).await.expect("Failed to initialize database");

    // Pre-populate database on start if it's empty so the user doesn't see a blank page
    match db::get_news_items(&pool, None, None, false, false, 0).await {
        Ok(items) => {
            if items.is_empty() {
                println!("Database is empty. Running initial news synchronization...");
                if let Err(e) = fetch_and_save_feeds(&pool).await {
                    eprintln!("Failed initial feed sync: {:?}", e);
                }
            } else {
                println!("Database has {} articles already stored.", items.len());
            }
        }
        Err(e) => {
            eprintln!("Error checking database status: {:?}", e);
        }
    }

    // 2. Set up shared AppState
    let state = Arc::new(AppState {
        pool: pool.clone(),
        last_refreshed_at: Mutex::new(chrono::Utc::now().timestamp()),
    });

    // 3. Spawn periodic 30-minute background synchronization worker
    let pool_clone = pool.clone();
    let state_clone = state.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(30 * 60)).await;
            println!("Periodic 30-minute news synchronization started...");
            let sync_start = chrono::Utc::now().timestamp();
            match fetch_and_save_feeds(&pool_clone).await {
                Ok(_) => {
                    let mut last = state_clone.last_refreshed_at.lock().await;
                    *last = sync_start;
                    println!("Periodic sync completed successfully.");
                }
                Err(e) => {
                    eprintln!("Periodic sync failed: {:?}", e);
                }
            }
        }
    });

    // 4. Configure router & CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Fallback static file serving (for SPA routing, falling back to frontend/dist/index.html)
    let serve_dir = ServeDir::new("frontend/dist")
        .not_found_service(ServeFile::new("frontend/dist/index.html"));

    let app = Router::new()
        .route("/api/news", get(get_news))
        .route("/api/news/:id/favorite", post(toggle_favorite_route))
        .route("/api/news/refresh", post(manual_refresh_route))
        .route("/api/status", get(get_status))
        .route("/api/news/stats", get(get_news_stats_route))
        .route("/api/feeds", get(get_feeds))
        .layer(cors)
        .fallback_service(serve_dir)
        .with_state(state);

        // 5. Start Server
    // Vercel provides the port via the PORT environment variable
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3005);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// REST Route: Get list of news items
#[derive(Deserialize)]
struct NewsParams {
    q: Option<String>,
    category: Option<String>,
    favorites: Option<bool>,
    archive: Option<bool>,
}

async fn get_news(
    State(state): State<Arc<AppState>>,
    Query(params): Query<NewsParams>,
) -> impl IntoResponse {
    let favorites_only = params.favorites.unwrap_or(false);
    let show_archived = params.archive.unwrap_or(false);
    let five_days_ago = chrono::Utc::now().timestamp() - 5 * 24 * 60 * 60;

    match db::get_news_items(&state.pool, params.q, params.category, favorites_only, show_archived, five_days_ago).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => {
            eprintln!("Database error in get_news: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

// REST Route: Toggle favorite status for an item
#[derive(Deserialize)]
struct FavoritePayload {
    is_favorite: bool,
}

async fn toggle_favorite_route(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<FavoritePayload>,
) -> impl IntoResponse {
    match db::toggle_favorite(&state.pool, &id, payload.is_favorite).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            eprintln!("Database error in toggle_favorite_route: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

// REST Route: Manually trigger feed sync
async fn manual_refresh_route(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    println!("Manual synchronization requested...");
    let sync_start = chrono::Utc::now().timestamp();
    match fetch_and_save_feeds(&state.pool).await {
        Ok(_) => {
            let mut last = state.last_refreshed_at.lock().await;
            *last = sync_start;
            println!("Manual sync completed successfully.");
            StatusCode::OK.into_response()
        }
        Err(e) => {
            eprintln!("Manual sync failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Refresh failed: {:?}", e),
            )
                .into_response()
        }
    }
}

// REST Route: Get API Status (last sync timestamp)
async fn get_status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let last = *state.last_refreshed_at.lock().await;
    Json(serde_json::json!({ "last_refreshed_at": last }))
}

// REST Route: Get news statistics
async fn get_news_stats_route(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let last = *state.last_refreshed_at.lock().await;
    match db::get_news_stats(&state.pool, last).await {
        Ok((total, since_last)) => {
            Json(serde_json::json!({
                "total_count": total,
                "since_last_sync": since_last,
            })).into_response()
        }
        Err(e) => {
            eprintln!("Database error in get_news_stats_route: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

// New handler: return list of RSS feed sources
async fn get_feeds() -> impl IntoResponse {
    // Map FEEDS to simple name/url objects
    let list: Vec<_> = FEEDS.iter().map(|f| serde_json::json!({"name": f.name, "url": f.url})).collect();
    Json(list)
}
// Translation Helper: Calls keyless, public Google Translate API
async fn translate_text(text: &str, target_lang: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    
    let client = reqwest::Client::new();
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl={}&dt=t&q={}",
        target_lang,
        urlencoding::encode(trimmed)
    );

    match client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(res) => {
            if let Ok(json) = res.json::<serde_json::Value>().await {
                let mut translated = String::new();
                if let Some(sentences) = json.get(0).and_then(|v| v.as_array()) {
                    for sentence in sentences {
                        if let Some(s_array) = sentence.as_array() {
                            if let Some(t_text) = s_array.get(0).and_then(|v| v.as_str()) {
                                translated.push_str(t_text);
                            }
                        }
                    }
                }
                if !translated.is_empty() {
                    return translated.trim().to_string();
                }
            }
        }
        Err(e) => {
            eprintln!("Translation error for text (returning original): {:?}", e);
        }
    }
    
    // Fallback: return original text if translation fails
    text.to_string()
}

#[derive(Clone)]
pub struct NewsFeedSource {
    pub name: &'static str,
    pub url: &'static str,
}

// Helper: Scans Google News RSS search and other AI news feeds, inserts results into PostgreSQL
async fn fetch_and_save_feeds(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Use the shared constant FEEDS defined in feeds.rs
    let feeds = FEEDS.to_vec();

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(12))
        .build()?;

    let mut total_added = 0;

    for feed in feeds {
        println!("Scanning aggregated source: {} ({})", feed.name, feed.url);
        match client.get(feed.url).send().await {
            Ok(response) => {
                if !response.status().is_success() {
                    eprintln!("HTTP error fetching {}: {}", feed.name, response.status());
                    continue;
                }
                match response.bytes().await {
                    Ok(bytes) => {
                        match feed_rs::parser::parse(&bytes[..]) {
                            Ok(parsed_feed) => {
                                let mut feed_added = 0;
                                // Take top 15 entries per feed to avoid rate limits
                                let entries_to_process = parsed_feed.entries.into_iter().take(15);
                                
                                for entry in entries_to_process {
                                    let item_url = entry.links.first().map(|l| l.href.clone()).unwrap_or_default();
                                    if item_url.is_empty() {
                                        continue;
                                    }

                                    // Deduplicate: skip if URL already exists in SQLite
                                    if db::article_exists(pool, &item_url).await.unwrap_or(false) {
                                        continue;
                                    }

                                    // Clean title and source
                                    let raw_title = entry.title.map(|t| t.content).unwrap_or_else(|| "Untitled Headline".to_string());
                                    let (title_en, actual_source) = if feed.name == "Google News" {
                                        let parts: Vec<&str> = raw_title.rsplitn(2, " - ").collect();
                                        if parts.len() == 2 {
                                            (parts[1].trim().to_string(), parts[0].trim().to_string())
                                        } else {
                                            (raw_title, feed.name.to_string())
                                        }
                                    } else {
                                        (raw_title, feed.name.to_string())
                                    };
                                    
                                    // Combine summary and content
                                    let raw_summary = entry.summary.map(|s| s.content)
                                        .or_else(|| entry.content.and_then(|c| c.body))
                                        .unwrap_or_default();

                                    let summary_en = strip_html(&raw_summary);

                                    // Translate title and summary into Bengali
                                    println!("Translating English headline: \"{}\"", title_en);
                                    let title_bn = translate_text(&title_en, "bn").await;
                                    
                                    let summary_bn = if summary_en.is_empty() {
                                        None
                                    } else {
                                        let translated = translate_text(&summary_en, "bn").await;
                                        if translated == summary_en {
                                            None
                                        } else {
                                            Some(translated)
                                        }
                                    };

                                    let category = categorize_article(&title_en, &summary_en);

                                    // Get timestamp, fall back to current time
                                    let published_at = entry.published.or(entry.updated)
                                        .map(|dt| dt.timestamp())
                                        .unwrap_or_else(|| chrono::Utc::now().timestamp());

                                    // Generate deterministic UUID v5 from article URL
                                    let id = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, item_url.as_bytes()).to_string();

                                    let item = db::NewsItem {
                                        id,
                                        title_en,
                                        title_bn,
                                        url: item_url,
                                        source: actual_source,
                                        summary_en: if summary_en.is_empty() { None } else { Some(summary_en) },
                                        summary_bn,
                                        category: category.to_string(),
                                        published_at,
                                        created_at: chrono::Utc::now().timestamp(),
                                        is_favorite: false,
                                    };

                                    if let Err(e) = db::insert_news_item(pool, &item).await {
                                        eprintln!("Error saving item: {:?}", e);
                                    } else {
                                        feed_added += 1;
                                        total_added += 1;
                                    }
                                }
                                println!("Saved/updated {} items from {}.", feed_added, feed.name);
                            }
                            Err(e) => {
                                eprintln!("XML parsing error for {}: {:?}", feed.name, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading bytes from {}: {:?}", feed.name, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Network request failed for {}: {:?}", feed.name, e);
            }
        }
    }

    println!("Completed multi-source feed synchronization. Total new items: {}", total_added);
    Ok(())
}

// Helper: Categorizes article based on title & description keywords
fn categorize_article(title: &str, summary: &str) -> &'static str {
    let text = format!("{} {}", title, summary).to_lowercase();

    // Check Job Impact keywords first to prioritize it
    if text.contains("layoff")
        || text.contains("laid off")
        || text.contains("job loss")
        || text.contains("job cut")
        || text.contains("cut jobs")
        || text.contains("lose jobs")
        || text.contains("replace workers")
        || text.contains("worker replacement")
        || text.contains("reducing headcount")
        || text.contains("unemployment")
        || text.contains("downsizing")
        || text.contains("job displacement")
        || text.contains("hiring freeze")
        || text.contains("new jobs")
        || text.contains("create jobs")
        || text.contains("job creation")
    {
        return "Job Impact";
    }

    if text.contains("llm")
        || text.contains("gpt")
        || text.contains("chatgpt")
        || text.contains("claude")
        || text.contains("gemini")
        || text.contains("llama")
        || text.contains("generative ai")
        || text.contains("text-to-")
        || text.contains("diffusion")
        || text.contains("midjourney")
        || text.contains("openai")
        || text.contains("prompt engineering")
    {
        return "LLMs & Generative AI";
    }

    if text.contains("robot")
        || text.contains("robotics")
        || text.contains("drone")
        || text.contains("autonomous")
        || text.contains("self-driving")
        || text.contains("driverless")
        || text.contains("humanoid")
        || text.contains("tesla bot")
        || text.contains("figure ai")
    {
        return "Robotics & Autonomous";
    }

    if text.contains("ethics")
        || text.contains("regulation")
        || text.contains("law")
        || text.contains("policy")
        || text.contains("copyright")
        || text.contains("lawsuit")
        || text.contains("bias")
        || text.contains("safety")
        || text.contains("governance")
        || text.contains("eu ai act")
        || text.contains("censor")
    {
        return "AI Ethics & Policy";
    }

    if text.contains("research")
        || text.contains("paper")
        || text.contains("scientific")
        || text.contains("university")
        || text.contains("breakthrough")
        || text.contains("algorithm")
        || text.contains("dataset")
        || text.contains("academic")
        || text.contains("benchmark")
    {
        return "Research & Science";
    }

    // Default category
    "Industry & Tech Giants"
}

// Helper: Strips simple HTML tags and normalizes entities
fn strip_html(html: &str) -> String {
    let mut in_tag = false;
    let mut result = String::new();
    
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }

    // Normalize entity characters and clean extra spacing
    result
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("\n", " ")
        .replace("\r", "")
        .replace("  ", " ")
        .trim()
        .to_string()
}
