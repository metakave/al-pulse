use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteConnectOptions, FromRow, SqlitePool};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewsItem {
    pub id: String,
    pub title_en: String,
    pub title_bn: String,
    pub url: String,
    pub source: String,
    pub summary_en: Option<String>,
    pub summary_bn: Option<String>,
    pub category: String,
    pub published_at: i64, // Unix timestamp in seconds
    pub created_at: i64,   // Unix timestamp in seconds
    pub is_favorite: bool,
}

pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let connection_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(connection_options).await?;

    // Create news_items table with bilingual columns
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS news_items (
            id TEXT PRIMARY KEY,
            title_en TEXT NOT NULL,
            title_bn TEXT NOT NULL,
            url TEXT UNIQUE NOT NULL,
            source TEXT NOT NULL,
            summary_en TEXT,
            summary_bn TEXT,
            category TEXT NOT NULL,
            published_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            is_favorite INTEGER NOT NULL DEFAULT 0
        );"
    )
    .execute(&pool)
    .await?;

    // Index on url for fast lookups/conflict resolution
    sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS idx_news_items_url ON news_items(url);")
        .execute(&pool)
        .await?;

    // Index on published_at for sorting performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_news_items_published ON news_items(published_at DESC);")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn insert_news_item(pool: &SqlitePool, item: &NewsItem) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO news_items (
            id, title_en, title_bn, url, source, summary_en, summary_bn, 
            category, published_at, created_at, is_favorite
         )
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
         ON CONFLICT(url) DO UPDATE SET
             title_en = excluded.title_en,
             title_bn = excluded.title_bn,
             summary_en = excluded.summary_en,
             summary_bn = excluded.summary_bn,
             category = excluded.category
         -- keeps original is_favorite state and id"
    )
    .bind(&item.id)
    .bind(&item.title_en)
    .bind(&item.title_bn)
    .bind(&item.url)
    .bind(&item.source)
    .bind(&item.summary_en)
    .bind(&item.summary_bn)
    .bind(&item.category)
    .bind(item.published_at)
    .bind(item.created_at)
    .bind(item.is_favorite)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_news_items(
    pool: &SqlitePool,
    q: Option<String>,
    category: Option<String>,
    favorites_only: bool,
    show_archived: bool,
    five_days_ago: i64,
) -> Result<Vec<NewsItem>, sqlx::Error> {
    // Construct search term with SQL wildcards if present
    let search_param = q.map(|s| format!("%{}%", s));
    
    // SQLite boolean handles 1 for true, 0 for false
    let fav_int = if favorites_only { 1 } else { 0 };
    let archive_int = if show_archived { 1 } else { 0 };

    // Matches queries on both English and Bengali content columns
    let items = sqlx::query_as::<_, NewsItem>(
        "SELECT 
            id, title_en, title_bn, url, source, summary_en, summary_bn, 
            category, published_at, created_at, is_favorite
         FROM news_items
         WHERE (?1 IS NULL 
                OR title_en LIKE ?1 
                OR title_bn LIKE ?1 
                OR summary_en LIKE ?1 
                OR summary_bn LIKE ?1 
                OR source LIKE ?1)
           AND (?2 IS NULL OR category = ?2)
           AND (?3 = 0 OR is_favorite = 1)
           AND (?3 = 1 OR (?4 = 0 AND published_at >= ?5) OR (?4 = 1 AND published_at < ?5))
         ORDER BY published_at DESC
         LIMIT 100"
    )
    .bind(search_param)
    .bind(category)
    .bind(fav_int)
    .bind(archive_int)
    .bind(five_days_ago)
    .fetch_all(pool)
    .await?;

    Ok(items)
}

pub async fn toggle_favorite(pool: &SqlitePool, id: &str, is_fav: bool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE news_items SET is_favorite = ?1 WHERE id = ?2")
        .bind(is_fav)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn article_exists(pool: &SqlitePool, url: &str) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT 1 FROM news_items WHERE url = ?1")
        .bind(url)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

pub async fn get_news_stats(pool: &SqlitePool, last_sync: i64) -> Result<(i32, i32), sqlx::Error> {
    let total_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM news_items")
        .fetch_one(pool)
        .await?;

    let since_last_sync: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM news_items WHERE created_at >= ?1")
        .bind(last_sync)
        .fetch_one(pool)
        .await?;

    Ok((total_count, since_last_sync))
}
