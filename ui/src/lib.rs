pub mod prelude;    use prelude::*;

pub mod counter;    pub use counter::Counter;

/// Import tauri API:
#[wasm_bindgen(module = "/js/tauri.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn invoke_handler(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn listen_event(name: &str, handler: &Function) -> Result<JsValue, JsValue>;
}
