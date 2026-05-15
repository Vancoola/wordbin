use crate::api::health_check;
use crate::chrome_perms::{has_origin, remove_origin, request_origin};
use crate::i18n::{I18nLocaleTrait, Locale, t, t_string, use_i18n};
use crate::settings;
use crate::settings::Settings;
use gloo_timers::future::TimeoutFuture;
use icondata::{LuArrowLeft, LuGlobe, LuSave};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_icons::Icon;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SaveState {
    Idle,
    Saved,
    Denied,
    Error,
}

fn normalize_url(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("http://{trimmed}")
    }
}

fn schedule_toast_clear(state: WriteSignal<SaveState>) {
    spawn_local(async move {
        TimeoutFuture::new(2000).await;
        state.set(SaveState::Idle);
    });
}

#[allow(clippy::too_many_lines)]
#[component]
pub fn SettingsPage() -> impl IntoView {
    let i18n = use_i18n();

    let (online, set_online) = signal(false);
    let (checking, set_checking) = signal(true);

    let settings_ctx = expect_context::<RwSignal<Settings>>();
    let initial = settings_ctx.get_untracked();

    let (language, set_language) = signal(initial.language);
    let (server_url, set_server_url) = signal(initial.server_url);
    let (auto_detect, set_auto_detect) = signal(initial.auto_detect_source);
    let (close_after_save, set_close_after_save) = signal(initial.close_after_save);
    let (api_token, set_api_token) = signal(initial.api_token);
    let (save_state, set_save_state) = signal(SaveState::Idle);

    Effect::new(move |_| {
        spawn_local(async move {
            let result = health_check().await;
            set_online.set(result);
            set_checking.set(false);
        });
    });

    let on_save = move |_ev: leptos::ev::MouseEvent| {
        let old_url = settings_ctx.get_untracked().server_url;
        let raw_url = server_url.get();
        let url = normalize_url(&raw_url);

        if url != raw_url {
            set_server_url.set(url.clone());
        }

        spawn_local(async move {
            let already = has_origin(&url).await.unwrap_or(false);

            let granted = if already {
                true
            } else {
                match request_origin(&url).await {
                    Ok(g) => g,
                    Err(e) => {
                        web_sys::console::error_1(&e);
                        set_save_state.set(SaveState::Error);
                        schedule_toast_clear(set_save_state);
                        return;
                    }
                }
            };

            if !granted {
                set_save_state.set(SaveState::Denied);
                schedule_toast_clear(set_save_state);
                return;
            }

            if !old_url.is_empty() && old_url != url {
                let _ = remove_origin(&old_url).await;
            }

            let updated = Settings {
                language: language.get(),
                server_url: url,
                auto_detect_source: auto_detect.get(),
                close_after_save: close_after_save.get(),
                api_token: api_token.get(),
            };
            settings::save(&updated);
            settings_ctx.set(updated);
            set_save_state.set(SaveState::Saved);
            schedule_toast_clear(set_save_state);

            set_checking.set(true);
            set_online.set(health_check().await);
            set_checking.set(false);
        });
    };

    view! {
        <div class="header">
            <div class="header-left">
                <button class="back-btn" title={move || t_string!(i18n, back)} on:click=move |_| crate::chrome_tabs::close_window()>
                    <Icon icon=LuArrowLeft width="14px" height="14px" />
                </button>
                <span class="page-title">{t!(i18n, settings_lower)}</span>
            </div>
            <span class="wordbin-label">wordbin</span>
        </div>

        <div class="body">

            <div class="section">
                <div class="section-label">{t!(i18n, interface_section)}</div>
                <div class="setting-row">
                    <div class="setting-info">
                        <span class="setting-name">{t!(i18n, language_label)}</span>
                        <span class="setting-hint">{t!(i18n, language_hint)}</span>
                    </div>
                    <select
                        prop:value=move || i18n.get_locale().as_str()
                        on:change=move |e| {
                            let val = event_target_value(&e);
                            let locale = Locale::from_str(&val).unwrap_or_default();
                            i18n.set_locale(locale);
                            set_language.set(locale);
                        }
                    >
                        {Locale::get_all()
                            .iter()
                            .map(|locale| {
                                let name = match locale {
                                    Locale::en => "english",
                                    Locale::ru => "русский",
                                    Locale::de => "deutsch",
                                    Locale::fr => "french",
                                };
                                view! {
                                    <option value=locale.as_str()>{name}</option>
                                }
                            })
                            .collect_view()
                        }
                    </select>
                </div>
            </div>

            <div class="section">
                <div class="section-label">{t!(i18n, server_section)}</div>
                <div class="setting-row full-row">
                    <span class="setting-name">{t!(i18n, server_url_label)}</span>
                    <div class="input-wrap">
                        <Icon icon=LuGlobe width="12px" height="12px" />
                        <input type="text" id="server-url"
                            prop:value=server_url
                            on:input=move |e| set_server_url.set(event_target_value(&e))
                            placeholder="http://localhost:3000" />
                    </div>
                </div>
                <div class="setting-row" style="margin-top:8px;">
                    <span class="setting-name">{t!(i18n, connection_label)}</span>
                    <div class="status">
                        <div class="status-dot" class:error=move || !checking.get() && !online.get()></div>
                        <span class="status-text" class:error=move || !checking.get() && !online.get()>
                            {move || match (checking.get(), online.get()) {
                                (true, _)      => t_string!(i18n, connection_checking),
                                (false, true)  => t_string!(i18n, connection_online),
                                (false, false) => t_string!(i18n, connection_error),
                            }}
                        </span>
                    </div>
                </div>
            </div>

            <div class="section">
                <div class="section-label">{t!(i18n, auth_section)}</div>
                <div class="setting-row full-row">
                    <span class="setting-name">{t!(i18n, api_token_label)}</span>
                    <div class="input-wrap">
                        <input type="password" id="api-token"
                            prop:value=api_token
                            on:input=move |e| set_api_token.set(event_target_value(&e))
                            placeholder=move || t_string!(i18n, api_token_placeholder) />
                    </div>
                    <span class="setting-hint">{t!(i18n, api_token_hint)}</span>
                </div>
            </div>

            <div class="section">
                <div class="section-label">{t!(i18n, defaults_section)}</div>
                <div class="setting-row">
                    <div class="setting-info">
                        <span class="setting-name">{t!(i18n, auto_detect_label)}</span>
                        <span class="setting-hint">{t!(i18n, auto_detect_hint)}</span>
                    </div>
                    <div
                        class="toggle"
                        class:on=auto_detect
                        on:click=move |_| set_auto_detect.update(|v| *v = !*v)>
                        <div class="toggle-thumb"></div>
                    </div>
                </div>
                <div class="setting-row">
                    <div class="setting-info">
                        <span class="setting-name">{t!(i18n, close_after_save_label)}</span>
                        <span class="setting-hint">{t!(i18n, close_after_save_hint)}</span>
                    </div>
                    <div
                        class="toggle"
                        class:on=close_after_save
                        on:click=move |_| set_close_after_save.update(|v| *v = !*v)>
                        <div class="toggle-thumb"></div>
                    </div>
                </div>
            </div>

            <button on:click=on_save class="save-btn">
                <Icon icon=LuSave width="12px" height="12px" />
                {t!(i18n, save_settings)}
            </button>

            <div
                class="toast"
                class:success=move || save_state.get() == SaveState::Saved
                class:error=move || matches!(save_state.get(), SaveState::Denied | SaveState::Error)
                class:show=move || save_state.get() != SaveState::Idle
            >
                {move || match save_state.get() {
                    SaveState::Saved => t_string!(i18n, settings_saved),
                    SaveState::Denied => t_string!(i18n, settings_denied),
                    SaveState::Error => t_string!(i18n, settings_error),
                    SaveState::Idle => "",
                }}
            </div>

        </div>

        <div class="version">{t!(i18n, version)}</div>
    }
}
