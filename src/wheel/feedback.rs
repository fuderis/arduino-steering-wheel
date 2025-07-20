use crate::prelude::*;

/// The feedback motor direction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Left,
    Right,
    None
}

/// The feedback motor properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub motor: Direction,
    pub power: u16,
}
