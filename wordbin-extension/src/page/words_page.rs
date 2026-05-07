use crate::Page;
use icondata::LuArrowLeft;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn WordsPage(set_page: WriteSignal<Page>) -> impl IntoView {
    view! {
        <div class="header">
            <div class="header-left">
                <button class="back-btn" title="Back" on:click= move |_| set_page.set(Page::Popup)>
                    <Icon icon=LuArrowLeft width="14px" height="14px" />
                </button>
                <span class="page-title">words</span>
            </div>
            <span class="wordbin-label">wordbin</span>
        </div>

        <div class="search-bar">
            <div class="input-wrap">
                <input type="text" placeholder="search..." autocomplete="off" spellcheck="false" />
            </div>
        </div>

        <div class="filter-bar">
            <button class="filter-btn active">all</button>
            <button class="filter-btn">new</button>
            <button class="filter-btn">learning</button>
            <button class="filter-btn">known</button>
        </div>

        <div class="word-list">

            <div class="word-row">
                <div class="word-info">
                    <span class="word-name">ephemeral</span>
                    <span class="word-meta">"github.com · 2 days ago"</span>
                </div>
                <span class="word-status status-new">new</span>
            </div>

            <div class="word-row">
                <div class="word-info">
                    <span class="word-name">tenacious</span>
                    <span class="word-meta">"netflix.com · 5 days ago"</span>
                </div>
                <span class="word-status status-learning">learning</span>
            </div>

            <div class="word-row">
                <div class="word-info">
                    <span class="word-name">resilience</span>
                    <span class="word-meta">"book · 1 week ago"</span>
                </div>
                <span class="word-status status-new">new</span>
            </div>

            <div class="word-row">
                <div class="word-info">
                    <span class="word-name">ubiquitous</span>
                    <span class="word-meta">"manual · 2 weeks ago"</span>
                </div>
                <span class="word-status status-known">known</span>
            </div>

            <div class="word-row">
                <div class="word-info">
                    <span class="word-name">pragmatic</span>
                    <span class="word-meta">"news.ycombinator.com · 3 weeks ago"</span>
                </div>
                <span class="word-status status-learning">learning</span>
            </div>

            <div class="word-row">
                <div class="word-info">
                    <span class="word-name">ambiguous</span>
                    <span class="word-meta">"github.com · 1 month ago"</span>
                </div>
                <span class="word-status status-known">known</span>
            </div>

            <div class="word-row">
                <div class="word-info">
                    <span class="word-name">serendipity</span>
                    <span class="word-meta">"browser · 1 month ago"</span>
                </div>
                <span class="word-status status-new">new</span>
            </div>

            <div class="sentinel"></div>

        </div>

        <div class="footer">
            <span class="count">7 words</span>
        </div>
    }
}
