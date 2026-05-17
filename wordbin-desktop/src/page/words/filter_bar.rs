use leptos::prelude::*;

#[component]
pub fn FilterBar() -> impl IntoView {
    view! {
        <div class="filter-bar">
            <div class="chip active">"All"</div>
            <div class="chip">"New"</div>
            <div class="chip">"Review"</div>
            <div class="chip">"Learned"</div>
            <span class="filter-count">"247 words"</span>
        </div>
    }
}