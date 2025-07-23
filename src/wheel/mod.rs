pub mod state;      pub use state::State;
pub mod feedback;   pub use feedback::{ Feedback, Direction };
pub mod wheel;      pub use wheel::Wheel;

use crate::prelude::*;

pub static CONFIG_UPDATED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub static WINDOW_VISIBLE: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
