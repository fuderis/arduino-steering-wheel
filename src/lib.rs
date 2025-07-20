pub mod error;    pub use error::{ StdResult, Result, Error };
pub mod prelude;  pub use prelude::*;
pub mod app;      pub use app::*;

pub mod wheel;    pub use wheel::{ Wheel, State };

/// Generates an unique ID
pub fn uniq_id() -> String {
    use std::time::{ SystemTime, UNIX_EPOCH };
    
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let random: u16 = rand::random();
    format!("{}{:04x}", millis, random)
}
