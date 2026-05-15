use js_sys::{Object, Promise, Reflect};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["chrome", "windows"], js_name = create)]
    fn js_windows_create(create_props: &JsValue) -> Promise;

    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = getURL)]
    fn js_get_url(path: &str) -> String;
}

/// Open an extension page in a standalone popup window (no tab UI).
/// Use this for any flow that must survive popup focus loss — e.g.
/// `chrome.permissions.request`, which would kill the action popup but
/// not a `windows.create({ type: 'popup' })` window.
pub fn open_window(path: &str, width: u32, height: u32) {
    let absolute = js_get_url(path);
    let props = Object::new();
    let _ = Reflect::set(
        &props,
        &JsValue::from_str("url"),
        &JsValue::from_str(&absolute),
    );
    let _ = Reflect::set(
        &props,
        &JsValue::from_str("type"),
        &JsValue::from_str("popup"),
    );
    let _ = Reflect::set(
        &props,
        &JsValue::from_str("width"),
        &JsValue::from_f64(f64::from(width)),
    );
    let _ = Reflect::set(
        &props,
        &JsValue::from_str("height"),
        &JsValue::from_f64(f64::from(height)),
    );
    let _ = Reflect::set(&props, &JsValue::from_str("focused"), &JsValue::TRUE);
    let _ = js_windows_create(&props);
}

/// Close the current window — works for both action popup and self-opened window.
pub fn close_window() {
    if let Some(w) = web_sys::window() {
        let _ = w.close();
    }
}

/// Current `window.location.hash`, or empty string.
pub fn location_hash() -> String {
    web_sys::window()
        .map(|w| w.location().hash().unwrap_or_default())
        .unwrap_or_default()
}

/// Add a CSS class on `<body>` if we're running outside the action popup
/// (i.e. opened via `open_window`, detectable by a non-empty location hash).
pub fn mark_window_mode() {
    if location_hash().is_empty() {
        return;
    }
    if let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    {
        let _ = body.class_list().add_1("mode-window");
    }
}
