use leptos::prelude::*;

#[component]
pub fn Topbar() -> impl IntoView {
    view! {
        <div class="topbar">
            <div class="search-wrap">
                <i class="ti ti-search"></i>
                <input type="text" placeholder="Search words…" aria-label="Search words" />
            </div>
            <button class="add-btn">
                <i class="ti ti-plus"></i>"Add word"
            </button>
            <button class="theme-btn" title="Toggle theme" aria-label="Toggle theme">
                <i class="ti ti-moon"></i>
            </button>
        </div>
    }
}
