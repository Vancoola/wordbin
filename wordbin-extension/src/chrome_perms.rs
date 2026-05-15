use js_sys::{Array, Object, Promise, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["chrome", "permissions"], js_name = request)]
    fn js_request(perms: &JsValue) -> Promise;

    #[wasm_bindgen(js_namespace = ["chrome", "permissions"], js_name = contains)]
    fn js_contains(perms: &JsValue) -> Promise;

    #[wasm_bindgen(js_namespace = ["chrome", "permissions"], js_name = remove)]
    fn js_remove(perms: &JsValue) -> Promise;
}

pub fn perms_object(origin_pattern: &str) -> Result<JsValue, JsValue> {
    let origins = Array::new();
    origins.push(&JsValue::from_str(origin_pattern));
    let obj = Object::new();
    Reflect::set(&obj, &JsValue::from_str("origins"), &origins)?;
    Ok(obj.into())
}

pub fn to_match_pattern(url: &str) -> Result<String, &'static str> {
    let parsed = url::Url::parse(url).map_err(|_| "invalid url")?;
    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return Err("only http/https supported");
    }
    let host = parsed.host_str().ok_or("missing host")?;
    let port = parsed.port().map(|p| format!(":{p}")).unwrap_or_default();
    Ok(format!("{scheme}://{host}{port}/*"))
}

pub async fn has_origin(server_url: &str) -> Result<bool, JsValue> {
    let pattern = to_match_pattern(server_url).map_err(JsValue::from_str)?;
    let res = JsFuture::from(js_contains(&perms_object(&pattern)?)).await?;
    Ok(res.as_bool().unwrap_or(false))
}

pub async fn request_origin(server_url: &str) -> Result<bool, JsValue> {
    let pattern = to_match_pattern(server_url).map_err(JsValue::from_str)?;
    let res = JsFuture::from(js_request(&perms_object(&pattern)?)).await?;
    Ok(res.as_bool().unwrap_or(false))
}

pub async fn remove_origin(server_url: &str) -> Result<bool, JsValue> {
    let pattern = to_match_pattern(server_url).map_err(JsValue::from_str)?;
    let res = JsFuture::from(js_remove(&perms_object(&pattern)?)).await?;
    Ok(res.as_bool().unwrap_or(false))
}
