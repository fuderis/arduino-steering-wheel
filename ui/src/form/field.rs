// use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Int(i32),
    Str(String),
}

#[derive(Clone, PartialEq)]
pub enum FieldKind{
    Number {
        min: i32,
        max: i32,
        step: i32,
    },

    Select {
        items: Vec<(String, String)>,
    },

    Range {
        min: i32,
        max: i32,
        step: i32,
    },
}

#[derive(Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub label: String,
    pub kind: FieldKind,
    pub value: FieldValue,
}
