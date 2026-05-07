use icondata::{LuCaseSensitive, LuCirclePlus, LuGlobe, LuList, LuSettings};
use js_sys::futures::JsFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_i18n::{t, t_string};
use leptos_icons::Icon;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::i18n::use_i18n;

#[component]
pub fn PopupPage(on_settings: impl Fn() + 'static) -> impl IntoView {
    let (source, set_source) = signal(String::new());

    let i18n = use_i18n();

    spawn_local(async move {
        if let Some(url) = get_current_url().await {
            let hostname = extract_hostname(&url);
            set_source.set(hostname);
        }
    });

    view! {
      <div class="header">
          <div class="dot-wrap" id="dot">
              <div class="dot-inner"></div>
          </div>
          <span class="logo">wordbin</span>
      </div>

      <div class="body">

          <div class="field">
              <label>{t!(i18n, word_label)}</label>
              <div class="input-wrap">
                  <Icon icon=LuCaseSensitive />
                  <input id="word-input" type="text" placeholder={move || t_string!(i18n, word_placeholder)} autocomplete="off" spellcheck="false" />
              </div>
          </div>

          <div class="field source-wrap">
              <label>{t!(i18n, source_label)}" "<span class="hint" id="auto-hint">{t!(i18n, source_hint)}</span></label>
              <div class="input-wrap">
                  <Icon icon=LuGlobe />
                  <input prop:value=source id="source-input" type="text" placeholder={move || t_string!(i18n, source_placeholder)} autocomplete="off" spellcheck="false" />
              </div>
          </div>

          <div class="field">
              <label>{t!(i18n, notes_label)}" "<span class="hint">{t!(i18n, notes_hint)}</span></label>
              <div class="notes-wrap">
                  <textarea id="notes-input" rows="2" placeholder={move || t_string!(i18n, notes_placeholder)}></textarea>
              </div>
          </div>

          <button class="save-btn" id="save-btn">
              <Icon icon=LuCirclePlus />
              {t!(i18n, save_word)}
          </button>

      </div>

      <div class="footer">
          <span class="count" id="count">{t!(i18n, words_saved)}</span>
          <div class="footer-icons">
              <button title={move || t_string!(i18n, all_words)}>
                  <Icon icon=LuList />
              </button>
              <button on:click=move |_| on_settings() title={move || t_string!(i18n, settings)} id="settings-btn">
                  <Icon icon=LuSettings />
              </button>
          </div>
      </div>

      <div class="toast" id="toast"></div>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = query)]
    fn tabs_query(query_info: &JsValue) -> js_sys::Promise;
}

async fn get_current_url() -> Option<String> {
    let query = js_sys::Object::new();
    js_sys::Reflect::set(&query, &"active".into(), &JsValue::TRUE).ok()?;
    js_sys::Reflect::set(&query, &"currentWindow".into(), &JsValue::TRUE).ok()?;

    let result = JsFuture::from(tabs_query(&query)).await.ok()?;

    let tabs = js_sys::Array::from(&result);
    let tab = tabs.get(0);

    let url = js_sys::Reflect::get(&tab, &"url".into()).ok()?;
    url.as_string()
}

fn extract_hostname(url: &str) -> String {
    url::Url::parse(url)
        .map(|u| u.host_str().unwrap_or("").replace("www.", ""))
        .unwrap_or_default()
}
