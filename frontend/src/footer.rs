use leptos::*;
use crate::Language;
use leptos::expect_context;

#[component]
pub fn Footer() -> impl IntoView {
    let lang = expect_context::<ReadSignal<Language>>();
    view! {
        <footer class="app-footer">
            <div class="footer-content">
                <p class="footer-text">
                    {move || if lang.get() == Language::En {
                        view! {
                            "AI PulseQ - Curated Intelligence // Idea & Execution by "
                            <a href="https://www.sadiqalam.com/ai-powered-business-transformation" target="_blank" rel="noopener noreferrer" class="footer-link">"Sadiq M Alam, Enterprise AI Consultant"</a>
                            " // For inquiry get in touch at "
                            <a href="mailto:hello@sadiqalam.com" class="footer-link">"hello@sadiqalam.com"</a>
                        }
                    } else {
                        view! {
                            "এআই পালসকিউ - বুদ্ধিমত্তার সংকলন // ভাবনা ও নির্বাহী প্রধান "
                            <a href="https://www.sadiqalam.com/ai-powered-business-transformation" target="_blank" rel="noopener noreferrer" class="footer-link">"সাদিক মোহাম্মদ আলম, এন্টারপ্রাইজ এআই পরামর্শক"</a>
                            " // যেকোন প্রশ্নের জন্য যোগাযোগ করুন "
                            <a href="mailto:hello@sadiqalam.com" class="footer-link">"hello@sadiqalam.com"</a>
                        }
                    }}
                </p>
            </div>
        </footer>
    }
}
