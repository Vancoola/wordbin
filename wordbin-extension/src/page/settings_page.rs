use crate::i18n::*;
use crate::settings::Settings;
use crate::{Page, settings};
use icondata::{LuArrowLeft, LuGlobe, LuSave};
use leptos::prelude::*;
use leptos_icons::Icon;
use std::str::FromStr;

#[component]
pub fn SettingsPage(set_page: WriteSignal<Page>) -> impl IntoView {
    let i18n = use_i18n();

    let settings_ctx = expect_context::<RwSignal<Settings>>();
    let (language, set_language) = signal(settings_ctx.get().language);
    let (server_url, set_server_url) = signal(settings_ctx.get().server_url);
    let (auto_detect, set_auto_detect) = signal(settings_ctx.get().auto_detect_source);
    let (close_after_save, set_close_after_save) = signal(settings_ctx.get().close_after_save);

    let on_save = move || {
        let updated = Settings {
            language: language.get(),
            server_url: server_url.get(),
            auto_detect_source: auto_detect.get(),
            close_after_save: close_after_save.get(),
        };
        settings::save(&updated);
        settings_ctx.set(updated);
    };

    view! {
        <div class="header">
            <div class="header-left">
                <button class="back-btn" title={move || t_string!(i18n, back)} on:click=move |_| set_page.set(Page::Popup)>
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
                    <div class="status" id="status">
                        <div class="status-dot" id="status-dot"></div>
                        <span class="status-text" id="status-text">{t!(i18n, connection_checking)}</span>
                    </div>
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

            <button on:click=move |_| on_save() class="save-btn">
                <Icon icon=LuSave width="12px" height="12px" />
                {t!(i18n, save_settings)}
            </button>

        </div>

        <div class="toast" id="toast">{t!(i18n, settings_saved)}</div>

        <div class="version">{t!(i18n, version)}</div>
    }
}
