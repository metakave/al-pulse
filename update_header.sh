#!/bin/bash
cat << 'INNER_EOF' > /tmp/header_patch.txt
<div class="nav-and-status">
    <div class="desktop-controls">
        <ul class="desktop-nav-links">
            <li><a href="/weekly-roundup">"Weekly AI Roundup"</a></li>
            <li><a href="/about">"About AI PulseQ"</a></li>
            <li><a href="/changelog">"Version & Changelog"</a></li>
            <li><a href="/sources">"News Sources"</a></li>
        </ul>
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
        <button class="theme-toggle" on:click=on_theme_toggle.clone()>
            {move || if theme.get() == Theme::Light { "🌙" } else { "☀️" }}
        </button>
    </div>
INNER_EOF
