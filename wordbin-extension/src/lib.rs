mod api;
mod page;

use crate::i18n::I18nContextProvider;
use crate::page::popup_page::PopupPage;
use crate::page::settings_page::SettingsPage;
use crate::page::words_page::WordsPage;
use leptos::prelude::*;
use reqwest::Client;
use wasm_bindgen::prelude::*;

include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

#[derive(Clone, PartialEq)]
enum Page {
    Popup,
    Settings,
    Words,
}

#[component]
fn App() -> impl IntoView {
    let (page, set_page) = signal(Page::Popup);

    view! {
        <I18nContextProvider>
            {move || match page.get() {
                Page::Popup => view! {
                    <PopupPage set_page />
                }.into_any(),
                Page::Settings => view! {
                    <SettingsPage set_page />
                }.into_any(),
                Page::Words => view! {
                    <WordsPage set_page />
                }.into_any(),
            }}
        </I18nContextProvider>
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
