mod page;

use crate::page::popup_page::PopupPage;
use crate::page::settings_page::SettingsPage;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::i18n::I18nContextProvider;

include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));


#[derive(Clone, PartialEq)]
enum Page {
    Popup,
    Settings,
}

#[component]
fn App() -> impl IntoView {
    let (page, set_page) = signal(Page::Popup);

    view! {
        <I18nContextProvider>
            {move || match page.get() {
                Page::Popup => view! {
                    <PopupPage on_settings=move || set_page.set(Page::Settings) />
                }.into_any(),
                Page::Settings => view! {
                    <SettingsPage on_back=move || set_page.set(Page::Popup) />
                }.into_any(),
            }}
        </I18nContextProvider>
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    mount_to_body(App);
}
