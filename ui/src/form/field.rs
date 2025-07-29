use crate::prelude::*;

/// Form field value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
}

/// Form field kind
#[derive(Clone, PartialEq)]
pub enum FieldKind {
    Text,
    
    Number {
        min: i32,
        max: i32,
        step: i32,
    },
    NumberFloat {
        min: f32,
        max: f32,
        step: f32,
    },

    Range {
        min: i32,
        max: i32,
        step: i32,
    },
    RangeFloat {
        min: f32,
        max: f32,
        step: f32,
    },

    Select {
        items: Vec<(String, String)>,
    },

    Check,
    Switch,
}

/// The form field
#[derive(Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub label: String,
    pub kind: FieldKind,
    pub value: FieldValue,
}
