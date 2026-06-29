use crate::NewsFeedSource;

pub const FEEDS: &[NewsFeedSource] = &[
    NewsFeedSource {
        name: "Google News",
        url: "https://news.google.com/rss/search?q=artificial+intelligence+OR+machine+learning+OR+generative+AI+OR+LLM&hl=en-US&gl=US&ceid=US:en",
    },
    NewsFeedSource {
        name: "TechCrunch",
        url: "https://techcrunch.com/category/artificial-intelligence/feed/",
    },
    NewsFeedSource {
        name: "VentureBeat",
        url: "https://venturebeat.com/category/ai/feed/",
    },
    NewsFeedSource {
        name: "MIT Technology Review",
        url: "https://www.technologyreview.com/topic/artificial-intelligence/feed/",
    },
    NewsFeedSource {
        name: "Unite.AI",
        url: "https://www.unite.ai/feed/",
    },
    NewsFeedSource {
        name: "AI News",
        url: "https://www.artificialintelligence-news.com/feed/",
    },
    NewsFeedSource {
        name: "MarkTechPost",
        url: "https://www.marktechpost.com/feed/",
    },
    // New specialist AI news sources
    NewsFeedSource {
        name: "The AI Report",
        url: "https://theaireport.com/feed/",
    },
    NewsFeedSource {
        name: "AI Trends",
        url: "https://aitrends.com/feed/",
    },
    NewsFeedSource {
        name: "Synced Review",
        url: "https://syncedreview.com/feed/",
    },
    NewsFeedSource {
        name: "TechCrunch Layoffs",
        url: "https://techcrunch.com/tag/layoffs/feed/",
    },
    NewsFeedSource {
        name: "Wired",
        url: "https://www.wired.com/feed/tag/ai/latest/rss",
    },
    NewsFeedSource {
        name: "OpenAI Newsroom",
        url: "https://openai.com/news/rss.xml",
    },
    NewsFeedSource {
        name: "Google Research AI Blog",
        url: "https://blog.google/technology/ai/rss/",
    },
    NewsFeedSource {
        name: "Hugging Face Blog",
        url: "https://huggingface.co/blog/feed.xml",
    },
    NewsFeedSource {
        name: "Ahead of AI",
        url: "https://magazine.sebastianraschka.com/feed",
    },
    // South Asia AI/Layoffs and local news sources
    NewsFeedSource {
        name: "Google News India (AI/Layoffs)",
        url: "https://news.google.com/rss/search?q=(artificial+intelligence+OR+AI)+(layoff+OR+layoffs+OR+job+cuts+OR+job+losses)+India&hl=en-IN&gl=IN&ceid=IN:en",
    },
    NewsFeedSource {
        name: "Google News Pakistan (AI/Layoffs)",
        url: "https://news.google.com/rss/search?q=(artificial+intelligence+OR+AI)+(layoff+OR+layoffs+OR+job+cuts+OR+job+losses)+Pakistan&hl=en-PK&gl=PK&ceid=PK:en",
    },
    NewsFeedSource {
        name: "Google News Sri Lanka (AI/Layoffs)",
        url: "https://news.google.com/rss/search?q=(artificial+intelligence+OR+AI)+(layoff+OR+layoffs+OR+job+cuts+OR+job+losses)+%22Sri+Lanka%22&hl=en-US&gl=US&ceid=US:en",
    },
    NewsFeedSource {
        name: "Inc42",
        url: "https://inc42.com/feed/",
    },
    NewsFeedSource {
        name: "ProPakistani",
        url: "https://propakistani.pk/category/tech/feed",
    },
    NewsFeedSource {
        name: "Daily FT",
        url: "https://www.ft.lk/rss/IT-Telecom-Tech",
    },
];
