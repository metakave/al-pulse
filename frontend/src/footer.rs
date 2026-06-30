use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="app-footer">
            <div class="footer-content">
                <p class="footer-text">
                    "এআই পালসকিউ - বুদ্ধিমত্তার সংকলন // ভাবনা ও নির্বাহী প্রধান "
                    <a href="https://www.sadiqalam.com/ai-powered-business-transformation" target="_blank" rel="noopener noreferrer" class="footer-link">"সাদিক মোহাম্মদ আলম, এন্টারপ্রাইজ এআই পরামর্শক"</a>
                    " // যেকোন প্রশ্নের জন্য যোগাযোগ করুন "
                    <a href="mailto:hello@sadiqalam.com" class="footer-link">"hello@sadiqalam.com"</a>
                </p>
            </div>
        </footer>
    }
}
