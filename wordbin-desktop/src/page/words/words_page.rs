use leptos::prelude::*;
use crate::page::words::filter_bar::FilterBar;
use crate::page::words::word_list::WordList;
use crate::page::words::detail_panel::DetailPanel;
use crate::page::words::topbar::Topbar;

#[component]
pub fn WordsPage() -> impl IntoView {
    view! {
        <main class="main">
            <Topbar />
            <FilterBar />
            <WordList />
        </main>
        <DetailPanel />
    }
}