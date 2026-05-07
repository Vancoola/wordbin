use crate::Page;
use crate::i18n::use_i18n;
use icondata::{LuCaseSensitive, LuCirclePlus, LuGlobe, LuList, LuSettings};
use js_sys::futures::JsFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_i18n::{t, t_string};
use leptos_icons::Icon;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use wordbin_types::CreateWord;
use crate::api::add_word;

#[component]
pub fn PopupPage(set_page: WriteSignal<Page>) -> impl IntoView {
    let i18n = use_i18n();

    let (word, set_word)     = signal(String::new());
    let (source, set_source) = signal(String::new());
    let (notes, set_notes)   = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error)   = signal(false);

    spawn_local(async move {
        if let Some(url) = get_current_url().await {
            let hostname = extract_hostname(&url);
            set_source.set(hostname);
        }
    });

    let on_save = move || {
        let word_val = word.get();
        if word_val.is_empty() { return; }

        set_loading.set(true);
        set_error.set(false);

        let payload = CreateWord {
            word: word_val,
            source: source.get(),
            notes: notes.get(),
        };

        spawn_local(async move {
            match add_word(payload).await {
                Ok(_) => {
                    set_word.set(String::new());
                    set_notes.set(String::new());
                }
                Err(_) => set_error.set(true),
            }
            set_loading.set(false);
        });
    };

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
                  <input prop:value=word on:input=move |e| set_word.set(event_target_value(&e)) id="word-input" type="text" placeholder={move || t_string!(i18n, word_placeholder)} autocomplete="off" spellcheck="false" />
              </div>
          </div>

          <div class="field source-wrap">
              <label>{t!(i18n, source_label)}" "<span class="hint" id="auto-hint">{t!(i18n, source_hint)}</span></label>
              <div class="input-wrap">
                  <Icon icon=LuGlobe />
                  <input prop:value=source on:input=move |e| set_source.set(event_target_value(&e)) id="source-input" type="text" placeholder={move || t_string!(i18n, source_placeholder)} autocomplete="off" spellcheck="false" />
              </div>
          </div>

          <div class="field">
              <label>{t!(i18n, notes_label)}" "<span class="hint">{t!(i18n, notes_hint)}</span></label>
              <div class="notes-wrap">
                  <textarea prop:value=notes on:input=move |e| set_notes.set(event_target_value(&e)) id="notes-input" rows="2" placeholder={move || t_string!(i18n, notes_placeholder)}></textarea>
              </div>
          </div>

          <button on:click=move |_| on_save() class="save-btn" id="save-btn">
              <Icon icon=LuCirclePlus />
              {t!(i18n, save_word)}
          </button>

      </div>

      <div class="footer">
          <span class="count" id="count">{t!(i18n, words_saved)}</span>
          <div class="footer-icons">
              <button on:click=move |_| set_page.set(Page::Words) title={move || t_string!(i18n, all_words)}>
                  <Icon icon=LuList />
              </button>
              <button on:click=move |_| set_page.set(Page::Settings) title={move || t_string!(i18n, settings)} id="settings-btn">
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
