use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

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

pub async fn init_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(database_url).await?;

    // Create news_items table with bilingual columns (PostgreSQL syntax)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS news_items (
            id VARCHAR(255) PRIMARY KEY,
            title_en TEXT NOT NULL,
            title_bn TEXT NOT NULL,
            url TEXT UNIQUE NOT NULL,
            source VARCHAR(255) NOT NULL,
            summary_en TEXT,
            summary_bn TEXT,
            category VARCHAR(255) NOT NULL,
            published_at BIGINT NOT NULL,
            created_at BIGINT NOT NULL,
            is_favorite BOOLEAN NOT NULL DEFAULT FALSE
        );"
    )
    .execute(&pool)
    .await?;

    // Indexes for query speed in PostgreSQL
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_news_items_published ON news_items(published_at DESC);")
        .execute(&pool)
        .await?;

    // Create visitor_counter table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS visitor_counter (
            id INT PRIMARY KEY,
            count BIGINT NOT NULL DEFAULT 100
        );"
    )
    .execute(&pool)
    .await?;

    // Initialize with 99 so the first visit increments it to 100
    sqlx::query(
        "INSERT INTO visitor_counter (id, count) VALUES (1, 99) ON CONFLICT (id) DO NOTHING;"
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn insert_news_item(pool: &PgPool, item: &NewsItem) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO news_items (
            id, title_en, title_bn, url, source, summary_en, summary_bn, 
            category, published_at, created_at, is_favorite
         )
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
         ON CONFLICT(url) DO UPDATE SET
             title_en = EXCLUDED.title_en,
             title_bn = EXCLUDED.title_bn,
             summary_en = EXCLUDED.summary_en,
             summary_bn = EXCLUDED.summary_bn,
             category = EXCLUDED.category,
             source = EXCLUDED.source"
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
    pool: &PgPool,
    q: Option<String>,
    category: Option<String>,
    favorites_only: bool,
    show_archived: bool,
    five_days_ago: i64,
) -> Result<Vec<NewsItem>, sqlx::Error> {
    // Construct search term with SQL wildcards if present
    let search_param = q.map(|s| format!("%{}%", s));

    // Matches queries on both English and Bengali content columns in PostgreSQL
    let items = sqlx::query_as::<_, NewsItem>(
        "SELECT 
            id, title_en, title_bn, url, source, summary_en, summary_bn, 
            category, published_at, created_at, is_favorite
         FROM news_items
         WHERE ($1::text IS NULL 
                OR title_en ILIKE $1 
                OR title_bn ILIKE $1 
                OR summary_en ILIKE $1 
                OR summary_bn ILIKE $1 
                OR source ILIKE $1)
           AND ($2::text IS NULL OR category = $2)
           AND ($3 = FALSE OR is_favorite = TRUE)
           AND ($3 = TRUE OR ($4 = FALSE AND published_at >= $5) OR ($4 = TRUE AND published_at < $5))
         ORDER BY published_at DESC
         LIMIT 100"
    )
    .bind(search_param)
    .bind(category)
    .bind(favorites_only)
    .bind(show_archived)
    .bind(five_days_ago)
    .fetch_all(pool)
    .await?;

    Ok(items)
}

pub async fn toggle_favorite(pool: &PgPool, id: &str, is_fav: bool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE news_items SET is_favorite = $1 WHERE id = $2")
        .bind(is_fav)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn article_exists(pool: &PgPool, url: &str) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT 1 FROM news_items WHERE url = $1")
        .bind(url)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

pub async fn get_news_stats(pool: &PgPool, last_sync: i64) -> Result<(i64, i64), sqlx::Error> {
    let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM news_items")
        .fetch_one(pool)
        .await?;

    let since_last_sync: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM news_items WHERE created_at >= $1")
        .bind(last_sync)
        .fetch_one(pool)
        .await?;

    Ok((total_count, since_last_sync))
}

pub async fn increment_visitor_count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let count: i64 = sqlx::query_scalar(
        "UPDATE visitor_counter SET count = count + 1 WHERE id = 1 RETURNING count"
    )
    .fetch_one(pool)
    .await?;
    Ok(count)
}
