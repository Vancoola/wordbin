#![allow(
    clippy::match_same_arms,
    clippy::ignored_unit_patterns,
    clippy::used_underscore_binding,
    clippy::default_trait_access
)]

mod api;
mod chrome_perms;
mod chrome_tabs;
mod page;
mod settings;

use crate::i18n::{I18nContextProvider, use_i18n};
use crate::page::popup_page::PopupPage;
use crate::page::settings_page::SettingsPage;
use crate::page::words_page::WordsPage;
use crate::settings::Settings;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;
use wordbin_types::word::WordCount;

include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

#[derive(Clone, PartialEq)]
enum Page {
    Popup,
    Settings,
    Words,
}

fn initial_page() -> Page {
    match chrome_tabs::location_hash().as_str() {
        "#settings" => Page::Settings,
        "#words" => Page::Words,
        _ => Page::Popup,
    }
}

#[component]
fn App() -> impl IntoView {
    let settings = settings::load();
    provide_context(RwSignal::new(settings));

    provide_context(RwSignal::new(WordCount { count: 0 }));

    let (page, set_page) = signal(initial_page());

    view! {
        <I18nContextProvider>
            <LocaleSync />
            <WordCountSync />
            {move || match page.get() {
                Page::Popup => view! {
                    <PopupPage set_page />
                }.into_any(),
                Page::Settings => view! {
                    <SettingsPage />
                }.into_any(),
                Page::Words => view! {
                    <WordsPage set_page />
                }.into_any(),
            }}
        </I18nContextProvider>
    }
}

#[component]
fn LocaleSync() -> impl IntoView {
    let i18n = use_i18n();
    let settings = expect_context::<RwSignal<Settings>>();

    Effect::new(move |_| {
        i18n.set_locale(settings.get().language);
    });
}

#[component]
fn WordCountSync() -> impl IntoView {
    let word_count = expect_context::<RwSignal<WordCount>>();

    spawn_local(async move {
        if let Ok(wc) = api::word_count().await {
            word_count.set(WordCount { count: wc });
        }
    });
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    chrome_tabs::mark_window_mode();
    mount_to_body(App);
}
