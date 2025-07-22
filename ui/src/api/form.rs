use crate::prelude::*;

#[wasm_bindgen(module = "/js/form.js")]
extern "C" {
    #[wasm_bindgen(js_name = new_form)]
    fn js_new_form(selector: &str) -> JsValue;

    #[wasm_bindgen(js_name = new_forms)]
    fn js_new_forms(selector: &str) -> JsValue;
}

/// The DOM form controller
pub struct Form;

/// The DOM forms delegator
pub struct Forms;
