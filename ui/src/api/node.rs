use crate::prelude::*;

#[wasm_bindgen(module = "/js/node.js")]
extern "C" {
    fn js_new_node(selector: &str) -> JsValue;

    // Attributes:
    fn js_has_attr(node: &JsValue, name: &str) -> bool;
    fn js_get_attr(node: &JsValue, name: &str) -> String;
    fn js_set_attr(node: &JsValue, name: &str, value: &str);
    fn js_remove_attr(node: &JsValue, name: &str);

    // ID:
    fn js_get_id(node: &JsValue) -> String;
    fn js_set_id(node: &JsValue, id: &str);
    fn js_remove_id(node: &JsValue);

    // Classes:
    fn js_get_class_list(node: &JsValue) -> JsValue;
    fn js_has_class(node: &JsValue, name: &str) -> bool;
    fn js_set_class(node: &JsValue, name: &str);
    fn js_remove_class(node: &JsValue, name: &str);

    // Text & Html:
    fn js_get_text(node: &JsValue) -> String;
    fn js_set_text(node: &JsValue, value: &str);
    fn js_get_html(node: &JsValue) -> String;
    fn js_set_html(node: &JsValue, value: &str);
    fn js_get_outer_html(node: &JsValue) -> String;
    fn js_set_outer_html(node: &JsValue, value: &str);

    // Insertions:
    fn js_insert_to(node: &JsValue, index: i32, child: &JsValue);
    fn js_insert_front(node: &JsValue, child: &JsValue);
    fn js_insert_back(node: &JsValue, child: &JsValue);

    // Events:
    // fn js_set_event(node: &JsValue, name: &str, handler: &JsValue);
}

/// The DOM element controller
#[derive(Debug, Clone)]
pub struct Node {
    node: JsValue,
}

impl Node {
    /// Creates a new instance of Node
    pub fn new(selector: &str) -> Arc<Self> {
        Arc::new(Self {
            node: js_new_node(&selector),
        })
    }

    // Attributes:
    pub fn has_attr(self: Arc<Self>, name: &str) -> bool {
        js_has_attr(&self.node, name)
    }
    pub fn get_attr(self: Arc<Self>, name: &str) -> String {
        js_get_attr(&self.node, name)
    }
    pub fn set_attr(self: Arc<Self>, name: &str, value: &str) -> Arc<Self> {
        js_set_attr(&self.node, name, value);
        self
    }
    pub fn remove_attr(self: Arc<Self>, name: &str) -> Arc<Self> {
        js_remove_attr(&self.node, name);
        self
    }

    // ID:
    pub fn get_id(self: Arc<Self>) -> String {
        js_get_id(&self.node)
    }
    pub fn set_id(self: Arc<Self>, id: &str) -> Arc<Self> {
        js_set_id(&self.node, id);
        self
    }
    pub fn remove_id(self: Arc<Self>) -> Arc<Self> {
        js_remove_id(&self.node);
        self
    }

    // Classes:
    pub fn get_class_list(self: Arc<Self>) -> Vec<String> {
        from_value(js_get_class_list(&self.node)).unwrap()
    }
    pub fn has_class(self: Arc<Self>, name: &str) -> bool {
        js_has_class(&self.node, name)
    }
    pub fn set_class(self: Arc<Self>, name: &str) -> Arc<Self> {
        js_set_class(&self.node, name);
        self
    }
    pub fn remove_class(self: Arc<Self>, name: &str) -> Arc<Self> {
        js_remove_class(&self.node, name);
        self
    }

    // Text & Html:
    pub fn get_text(self: Arc<Self>) -> String {
        js_get_text(&self.node)
    }
    pub fn set_text(self: Arc<Self>, value: &str) -> Arc<Self> {
        js_set_text(&self.node, value);
        self
    }
    pub fn get_html(self: Arc<Self>) -> String {
        js_get_html(&self.node)
    }
    pub fn set_html(self: Arc<Self>, value: &str) -> Arc<Self> {
        js_set_html(&self.node, value);
        self
    }
    pub fn get_outer_html(self: Arc<Self>) -> String {
        js_get_outer_html(&self.node)
    }
    pub fn set_outer_html(self: Arc<Self>, value: &str) -> Arc<Self> {
        js_set_outer_html(&self.node, value);
        self
    }

    // Insertions:
    pub fn insert_to(self: Arc<Self>, index: i32, child: &JsValue) -> Arc<Self> {
        js_insert_to(&self.node, index, child);
        self
    }
    pub fn insert_front(self: Arc<Self>, child: &JsValue) -> Arc<Self> {
        js_insert_front(&self.node, child);
        self
    }
    pub fn insert_back(self: Arc<Self>, child: &JsValue) -> Arc<Self> {
        js_insert_back(&self.node, child);
        self
    }

    // Events:
    // pub fn set_event(self: Arc<Self>, name: &str, handler: &JsValue);
}
