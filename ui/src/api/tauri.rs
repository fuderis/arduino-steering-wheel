use crate::prelude::*;

#[wasm_bindgen(module = "/js/tauri.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn js_invoke_handler(name: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn js_listen_event(name: &str, handler: &Function) -> Result<JsValue, JsValue>;
}

/// Invoke application handler
pub async fn invoke_handler<T: serde::de::DeserializeOwned>(name: &str, args: Value) -> Result<T, JsValue> {
    match js_invoke_handler(name, to_value(&args).unwrap()).await {
        Ok(js_val) => {
            from_value(js_val).map_err(|e| {
                web_sys::console::error_1(&JsValue::from_str(&format!("(de)serialize error: {}", e)));
                JsValue::from_str(&format!("(de)serialize error: {}", e))
            })
        }
        Err(err) => {
            web_sys::console::error_1(&err);
            Err(err)
        }
    }
}

/// Listen to application events
pub async fn listen_event<T: serde::de::DeserializeOwned>(name: &str, handler: &Function) -> Result<T, JsValue> {
    match js_listen_event(name, handler).await {
        Ok(js_val) => {
            from_value(js_val).map_err(|e| {
                web_sys::console::error_1(&JsValue::from_str(&format!("(de)serialize error: {}", e)));
                JsValue::from_str(&format!("(de)serialize error: {}", e))
            })
        }
        Err(err) => {
            web_sys::console::error_1(&err);
            Err(err)
        }
    }
}
