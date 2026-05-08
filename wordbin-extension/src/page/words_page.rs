use crate::Page;
use crate::api::fetch_words;
use crate::i18n::use_i18n;
use icondata::LuArrowLeft;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_i18n::t;
use leptos_icons::Icon;
use wordbin_types::{WordCount, WordResponse};

#[derive(Clone, PartialEq)]
enum WordFilter {
    All,
    New,
    Learning,
    Known,
}

#[component]
pub fn WordsPage(set_page: WriteSignal<Page>) -> impl IntoView {
    let i18n = use_i18n();
    let word_count = expect_context::<RwSignal<WordCount>>();
    let (filter, set_filter) = signal(WordFilter::All);

    const LIMIT: i64 = 20;

    let (words, set_words) = signal(Vec::<WordResponse>::new());
    let (offset, set_offset) = signal(0i64);
    let (has_more, set_has_more) = signal(true);
    let (loading, set_loading) = signal(false);

    let load_more = move || {
        if loading.get_untracked() || !has_more.get_untracked() { return; }
        set_loading.set(true);

        let current_offset = offset.get_untracked();
        let status = match filter.get_untracked() {
            WordFilter::All      => None,
            WordFilter::New      => Some("new".to_string()),
            WordFilter::Learning => Some("learning".to_string()),
            WordFilter::Known    => Some("known".to_string()),
        };

        spawn_local(async move {
            match fetch_words(LIMIT, current_offset, status).await {
                Ok(new_words) => {
                    leptos::logging::log!("fetched: {}", new_words.len());
                    if (new_words.len() as i64) < LIMIT {
                        set_has_more.set(false);
                    }
                    set_offset.update(|o| *o += LIMIT);
                    set_words.update(|w| w.extend(new_words));
                }
                Err(e) => leptos::logging::log!("error: {:?}", e),
            }
            set_loading.set(false);
        });
    };

    Effect::new(move |_| {
        let _ = filter.get();
        set_words.set(Vec::new());
        set_offset.set(0);
        set_has_more.set(true);
        load_more();
    });

    load_more();

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
            <button
                class="filter-btn"
                class:active=move || filter.get() == WordFilter::All
                on:click=move |_| set_filter.set(WordFilter::All)
            >all</button>
            <button
                class="filter-btn"
                class:active=move || filter.get() == WordFilter::New
                on:click=move |_| set_filter.set(WordFilter::New)
            >new</button>
            <button
                class="filter-btn"
                class:active=move || filter.get() == WordFilter::Learning
                on:click=move |_| set_filter.set(WordFilter::Learning)
            >learning</button>
            <button
                class="filter-btn"
                class:active=move || filter.get() == WordFilter::Known
                on:click=move |_| set_filter.set(WordFilter::Known)
            >known</button>
        </div>

        <div class="word-list" on:scroll=move |e| {
            let el = event_target::<web_sys::HtmlElement>(&e);
            let near_bottom = el.scroll_top() + el.client_height() >= el.scroll_height() - 40;
            if near_bottom { load_more(); }
        }>
            <For
                each=move || words.get()
                key=|w| w.id
                let:word
            >
                <WordRow word=word />
            </For>

            {move || has_more.get().then(|| view! {
                <div class="sentinel loading-hint">"loading..."</div>
            })}
        </div>

        <div class="footer">
            <span class="count">{move || word_count.get().count}" "{t!(i18n, words_saved)}</span>
        </div>
    }
}

#[component]
fn WordRow(word: WordResponse) -> impl IntoView {
    let status = word.status.clone();
    view! {
        <div class="word-row">
            <div class="word-info">
                <span class="word-name">{word.word}</span>
                <span class="word-meta">{word.source}</span>
            </div>
            <span class=format!("word-status status-{}", status)>
                {word.status}
            </span>
        </div>
    }
}
