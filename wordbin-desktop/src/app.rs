use crate::i18n::I18nContextProvider;
use crate::page::setup_page::SetupPage;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_navigate;
use leptos_router::{path, NavigateOptions};
use wasm_bindgen::prelude::*;
use crate::page::home_page::HomePage;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {

    provide_meta_context();

    let (setup_done, set_setup_done) = signal::<Option<bool>>(None);

    spawn_local(async move {
        let result = invoke("get_setup_done", JsValue::NULL).await;
        let done: bool = serde_wasm_bindgen::from_value(result).unwrap_or(false);
        set_setup_done.set(Some(done));
    });

    view! {
        <Router>
            <RedirectGuard setup_done=setup_done />
            <I18nContextProvider>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/setup") view=SetupPage />
                </Routes>
            </I18nContextProvider>
        </Router>
    }
}

#[component]
fn RedirectGuard(setup_done: ReadSignal<Option<bool>>) -> impl IntoView {
    let navigate = use_navigate();

    Effect::new(move |_| {
        if let Some(done) = setup_done.get() {
            if !done {
                navigate("/setup", NavigateOptions::default());
            }
        }
    });
}
