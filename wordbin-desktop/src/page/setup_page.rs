use crate::app::invoke;
use crate::i18n::{use_i18n, Locale};
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_i18n::t;
use leptos_meta::Stylesheet;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use serde::Serialize;
use shared_types::settings::{Settings, Theme};

#[derive(Serialize)]
struct SetSettingsArgs {
    settings: Settings,
}
#[derive(Serialize)]
struct SetDoneArgs {
    done: bool,
}

#[component]
pub fn SetupPage() -> impl IntoView {
    let i18n = use_i18n();
    let navigate = use_navigate();

    let (step, set_step) = signal(0u8);
    let (theme, set_theme) = signal(Theme::System);
    let (server_url, set_server_url) = signal(String::new());

    Effect::new(move |_| {
        let value = match theme.get() {
            Theme::Light => Some("light"),
            Theme::Dark => Some("dark"),
            Theme::System => None,
        };

        if let Some(document) = document().document_element() {
            match value {
                Some(v) => {
                    let _ = document.set_attribute("data-theme", v);
                }
                None => {
                    let _ = document.remove_attribute("data-theme");
                }
            }
        }
    });

    let on_finish = move || {
        let settings = Settings {
            theme: theme.get_untracked(),
            server_url: server_url.get_untracked(),
            language: i18n.get_locale().into(),
        };

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&SetSettingsArgs { settings }).unwrap();
            invoke("set_settings", args).await;
            let args = serde_wasm_bindgen::to_value(&SetDoneArgs { done: true }).unwrap();
            invoke("set_setup_done", args).await;
        });

        navigate("/", NavigateOptions::default());
    };

    view! {
        <Stylesheet href="/setup.css" />
        <div class="setup-page">
        <div class="card">
            <div class="logo-row">
                <div class="logo-icon">
                    <i class="ti ti-books" aria-hidden="true"></i>
                </div>
                <span class="logo-text">"wordbin"</span>
            </div>

            <div class="step-dots">
                <div
                    class="dot"
                    class:active=move || step.get() == 0
                    class:done=move || { step.get() > 0 }
                ></div>
                <div class="dot" class:active=move || step.get() == 1></div>
            </div>

            <div class="step" class:active=move || step.get() == 0>
                <div class="step-title">{t!(i18n, setup.welcome_title)}</div>
                <div class="step-sub">
                    {t!(i18n, setup.welcome_sub)}
                </div>

                <div class="field-label">{t!(i18n, setup.interface_language)}</div>
                <div class="lang-grid">
                    <LangOption
                        value=Locale::en
                        flag="🇬🇧"
                        label="English"
                    />
                    <LangOption
                        value=Locale::ru
                        flag="🇷🇺"
                        label="Русский"
                    />
                    <LangOption
                        value=Locale::de
                        flag="🇩🇪"
                        label="Deutsch"
                    />
                    <LangOption
                        value=Locale::fr
                        flag="🇫🇷"
                        label="Français"
                    />
                    <LangOption
                        value=Locale::ja
                        flag="🇯🇵"
                        label="日本語"
                    />
                    <LangOption
                        value=Locale::ko
                        flag="🇰🇷"
                        label="한국어"
                    />
                    <LangOption
                        value=Locale::zh
                        flag="🇨🇳"
                        label="中文"
                    />
                    <LangOption
                        value=Locale::es
                        flag="🇪🇸"
                        label="Español"
                    />
                </div>

                <div class="field-label">{t!(i18n, setup.theme)}</div>
                <div class="theme-row">
                    <ThemeOption
                        value=Theme::Light
                        icon="ti-sun"
                        theme=theme
                        set_theme=set_theme
                    />
                    <ThemeOption
                        value=Theme::Dark
                        icon="ti-moon"
                        theme=theme
                        set_theme=set_theme
                    />
                    <ThemeOption
                        value=Theme::System
                        icon="ti-device-desktop"
                        theme=theme
                        set_theme=set_theme
                    />
                </div>

                <div class="footer">
                    <span></span>
                    <button class="next-btn" on:click=move |_| set_step.set(1)>
                        {t!(i18n, setup.continue_btn)}
                    </button>
                </div>
            </div>

            <div class="step" class:active=move || step.get() == 1>
                <div class="step-title">{t!(i18n, setup.connect_title)}</div>
                <div class="step-sub">
                    {t!(i18n, setup.connect_sub)}
                </div>

                <div class="field-label">{t!(i18n, setup.server_address)}</div>
                <div class="input-wrap" style="margin-bottom: 14px;">
                    <i class="ti ti-server" aria-hidden="true"></i>
                    <input
                        type="text"
                        placeholder="http://localhost:3000"
                        prop:value=server_url
                        on:input=move |e| set_server_url.set(event_target_value(&e))
                    />
                </div>

                <div class="field-label">{t!(i18n, setup.access_token)}</div>
                <div class="input-wrap" style="margin-bottom: 16px;">
                    <i class="ti ti-lock" aria-hidden="true"></i>
                    <input type="password" placeholder="••••••••••••••••" />
                </div>

                <button class="connect-btn">
                    <i
                        class="ti ti-plug"
                        style="font-size:14px; vertical-align:-2px; margin-right:6px;"
                        aria-hidden="true"
                    ></i>
                    {t!(i18n, setup.test_connection)}
                </button>

                <div class="footer">
                    <button class="back-btn" on:click=move |_| set_step.set(0)>
                        {t!(i18n, setup.back_btn)}
                    </button>
                    <button class="next-btn" on:click=move |_| on_finish()>
                        {t!(i18n, setup.finish_setup)}
                    </button>
                </div>
            </div>
        </div>
        </div>
    }
}

#[component]
fn LangOption(value: Locale, flag: &'static str, label: &'static str) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <button
            class="option-btn"
            class:selected=move || i18n.get_locale() == value
            on:click=move |_| i18n.set_locale(value)
        >
            <span>{flag}</span>
            {label}
        </button>
    }
}

#[component]
fn ThemeOption(
    value: Theme,
    icon: &'static str,
    theme: ReadSignal<Theme>,
    set_theme: WriteSignal<Theme>,
) -> impl IntoView {
    let i18n = use_i18n();

    let icon_class = format!("ti {icon}");

    view! {
        <button
            class="theme-opt"
            class:selected=move || theme.get() == value
            on:click=move |_| set_theme.set(value)
        >
            <i class=icon_class aria-hidden="true"></i>
            {move || match value {
                Theme::Light  => t!(i18n, setup.theme_light).into_any(),
                Theme::Dark   => t!(i18n, setup.theme_dark).into_any(),
                Theme::System => t!(i18n, setup.theme_system).into_any(),
            }}
        </button>
    }
}
