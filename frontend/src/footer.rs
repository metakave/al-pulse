use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="app-footer">
            <div class="footer-content">
                <p class="footer-text">
                    "AI Pulse - Curated Intelligence // Idea & Execution by Sadiq M Alam, Enterprise AI Consultant // For inquiry get in touch at "
                    <a href="mailto:hello@sadiqalam.com" class="footer-link">"hello@sadiqalam.com"</a>
                    " // "
                    <a href="/about" target="_blank" class="footer-link">"About"</a>
                </p>
            </div>
        </footer>
    }
}
