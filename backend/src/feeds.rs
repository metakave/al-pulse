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
    NewsFeedSource {
        name: "Crunchbase News",
        url: "https://news.crunchbase.com/feed/",
    },
    NewsFeedSource {
        name: "Anthropic News",
        url: "https://www.anthropic.com/feed.xml",
    },
    NewsFeedSource {
        name: "InformationWeek",
        url: "https://www.informationweek.com/rss.xml",
    },
    NewsFeedSource {
        name: "Oracle News",
        url: "https://www.oracle.com/news/rss/",
    },
    NewsFeedSource {
        name: "Amazon News",
        url: "https://www.aboutamazon.com/news/rss.xml",
    },
    NewsFeedSource {
        name: "Cisco Newsroom",
        url: "https://newsroom.cisco.com/c/services/i/servlets/newsroom/rssfeed.json",
    },
    NewsFeedSource {
        name: "PayPal News",
        url: "https://newsroom.paypal-corp.com/news?template=rss",
    },
    NewsFeedSource {
        name: "Cloudflare Blog",
        url: "https://blog.cloudflare.com/rss",
    },
    NewsFeedSource {
        name: "Chegg Investor Relations",
        url: "https://investor.chegg.com/rss",
    },
    NewsFeedSource {
        name: "Google News (AI Layoffs & Job Cuts)",
        url: "https://news.google.com/rss/search?q=(%22Tech+Lay+Off%22+OR+%22Lay+Off%22+OR+%22Job+Cut%22)+AND+%22AI%22&hl=en-US&gl=US&ceid=US:en",
    },
    NewsFeedSource {
        name: "The American Bazaar",
        url: "https://americanbazaaronline.com/feed/",
    },
    // Mainstream & Consumer Tech
    NewsFeedSource {
        name: "Ars Technica",
        url: "http://feeds.arstechnica.com/arstechnica/index",
    },
    NewsFeedSource {
        name: "ZDNet",
        url: "https://www.zdnet.com/news/rss.xml",
    },
    NewsFeedSource {
        name: "CNET",
        url: "https://www.cnet.com/rss/news/",
    },
    NewsFeedSource {
        name: "PCMag",
        url: "https://www.pcmag.com/feeds/rss/latest",
    },
    NewsFeedSource {
        name: "Engadget",
        url: "https://www.engadget.com/rss.xml",
    },
    NewsFeedSource {
        name: "The Verge",
        url: "https://www.theverge.com/rss/index.xml",
    },
    NewsFeedSource {
        name: "Gizmodo",
        url: "https://gizmodo.com/rss",
    },
    NewsFeedSource {
        name: "Google News (Tech Job Layoffs)",
        url: "https://news.google.com/rss/search?q=%22Tech+Job+Lay+off%22+OR+%22Lay+Off+AI%22+OR+%22Job+Cut+AI%22&hl=en-US&gl=US&ceid=US:en",
    },
    NewsFeedSource {
        name: "Google News (Robo Taxi)",
        url: "https://news.google.com/rss/search?q=%22robo+taxi%22&hl=en-US&gl=US&ceid=US:en",
    },
    NewsFeedSource {
        name: "Google News (Autonomous Car)",
        url: "https://news.google.com/rss/search?q=%22autonomous+car%22&hl=en-US&gl=US&ceid=US:en",
    },
];
