use crate::Page;
use crate::api::add_word;
use crate::i18n::use_i18n;
use crate::settings::Settings;
use icondata::{LuCaseSensitive, LuCirclePlus, LuGlobe, LuList, LuSettings};
use js_sys::futures::JsFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_i18n::{t, t_string};
use leptos_icons::Icon;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use wordbin_types::word::{CreateWord, WordCount};

#[derive(Clone, PartialEq)]
enum ToastState {
    Hidden,
    EmptyWord,
    EmptySource,
    ServerError,
}

#[allow(clippy::too_many_lines)]
#[component]
pub fn PopupPage(set_page: WriteSignal<Page>) -> impl IntoView {
    let i18n = use_i18n();
    let word_count = expect_context::<RwSignal<WordCount>>();
    let settings_ctx = expect_context::<RwSignal<Settings>>();

    let (word, set_word) = signal(String::new());
    let (source, set_source) = signal(String::new());
    let (notes, set_notes) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (toast, set_toast) = signal(ToastState::Hidden);

    if settings_ctx.get_untracked().auto_detect_source {
        spawn_local(async move {
            if let Some(url) = get_current_url().await {
                let hostname = extract_hostname(&url);
                set_source.set(hostname);
            }
        });
    }

    let on_save = move || {
        let word_val = word.get();
        if word_val.is_empty() {
            set_toast.set(ToastState::EmptyWord);
            return;
        }

        if source.get().is_empty() {
            set_toast.set(ToastState::EmptySource);
            return;
        }

        set_loading.set(true);
        set_toast.set(ToastState::Hidden);

        let payload = CreateWord {
            value: word_val,
            source: source.get(),
            notes: notes.get(),
        };

        spawn_local(async move {
            match add_word(payload).await {
                Ok(_) => {
                    set_word.set(String::new());
                    set_notes.set(String::new());
                    if settings_ctx.get_untracked().close_after_save {
                        crate::chrome_tabs::close_window();
                    }
                }
                Err(_) => set_toast.set(ToastState::ServerError),
            }
            set_loading.set(false);
            word_count.update(|wc| wc.count += 1);
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
                  <input prop:value=word on:input=move |e| set_word.set(event_target_value(&e)) id="word-input" type="text" placeholder={move || t_string!(i18n, word_placeholder)} autocomplete="off" spellcheck="false" required />
              </div>
          </div>

          <div class="field source-wrap">
              <label>{t!(i18n, source_label)}" "
                {move || settings_ctx.get().auto_detect_source.then(|| view! {
                    <span class="hint" id="auto-hint">{t!(i18n, source_hint)}</span>
                })}
            </label>
              <div class="input-wrap">
                  <Icon icon=LuGlobe />
                  <input prop:value=source on:input=move |e| set_source.set(event_target_value(&e)) id="source-input" type="text" placeholder={move || t_string!(i18n, source_placeholder)} autocomplete="off" spellcheck="false" required />
              </div>
          </div>

          <div class="field">
              <label>{t!(i18n, notes_label)}" "<span class="hint">{t!(i18n, notes_hint)}</span></label>
              <div class="notes-wrap">
                  <textarea prop:value=notes on:input=move |e| set_notes.set(event_target_value(&e)) id="notes-input" rows="2" placeholder={move || t_string!(i18n, notes_placeholder)}></textarea>
              </div>
          </div>

          <button on:click=move |_| on_save() class:disabled=loading prop:disabled=loading class="save-btn" id="save-btn">
              <Icon icon=LuCirclePlus />
              {t!(i18n, save_word)}
          </button>

        <div
            class="toast"
            class:error=move || toast.get() != ToastState::Hidden
            class:show=move || toast.get() != ToastState::Hidden
        >
            {move || match toast.get() {
                ToastState::EmptyWord   => t_string!(i18n, error_empty_word),
                ToastState::EmptySource => t_string!(i18n, error_empty_source),
                ToastState::ServerError => t_string!(i18n, error_server),
                ToastState::Hidden      => "",
            }}
        </div>

      </div>

      <div class="footer">
          <span class="count" id="count">{move || word_count.get().count}" "{t!(i18n, words_saved)}</span>
          <div class="footer-icons">
              <button on:click=move |_| set_page.set(Page::Words) title={move || t_string!(i18n, all_words)}>
                  <Icon icon=LuList />
              </button>
              <button on:click=move |_| crate::chrome_tabs::open_window("index.html#settings", 420, 640) title={move || t_string!(i18n, settings)} id="settings-btn">
                  <Icon icon=LuSettings />
              </button>
          </div>
      </div>
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
