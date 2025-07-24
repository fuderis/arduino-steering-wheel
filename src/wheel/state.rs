use crate::prelude::*;

/// The steering wheel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub wheel: u16,
    pub gas: u16,
    pub brake: u16,
    pub clutch: u16,
    pub up: bool,
    pub down: bool,
    pub handbrake: bool,
}

impl ::std::default::Default for State {
    fn default() -> Self {
        Self {
            wheel: 510,
            gas: 0,
            brake: 0,
            clutch: 0,
            up: false,
            down: false,
            handbrake: false,
        }
    }
}
