mod api;
mod page;
mod settings;

use crate::i18n::{I18nContextProvider, use_i18n};
use crate::page::popup_page::PopupPage;
use crate::page::settings_page::SettingsPage;
use crate::page::words_page::WordsPage;
use crate::settings::Settings;
use leptos::prelude::*;
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
    let settings = settings::load();
    provide_context(RwSignal::new(settings));

    let (page, set_page) = signal(Page::Popup);

    view! {
        <I18nContextProvider>
            <LocaleSync />
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

#[component]
fn LocaleSync() -> impl IntoView {
    let i18n = use_i18n();
    let settings = expect_context::<RwSignal<Settings>>();

    Effect::new(move |_| {
        i18n.set_locale(settings.get().language);
    });

    view! {}
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
