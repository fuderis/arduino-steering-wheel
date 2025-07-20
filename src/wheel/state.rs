use crate::prelude::*;

/// The steering wheel state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    pub wheel: u16,
    pub gas: u16,
    pub brake: u16,
    pub clutch: u16,
    pub up: bool,
    pub down: bool,
    pub handbrake: bool,
}
