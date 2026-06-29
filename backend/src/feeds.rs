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
];
