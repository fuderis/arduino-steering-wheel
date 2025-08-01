use crate::prelude::*;

/// Com port settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigComport {
    pub com_port: u16,
    pub baud_rate: u32,
}

impl ::std::default::Default for ConfigComport {
    fn default() -> Self {
        Self {
            com_port: 6,
            baud_rate: 115200,
        }
    }
}

/// Wheel settings 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigWheel {
    pub wheel_bias: i16,
    pub wheel_dead_zone: u16,
    pub wheel_degs_limit: u16,
    pub wheel_degs_max_possible: u16,
    pub wheel_smooth_rate: f32,
    pub wheel_reverse_direction: bool,
}

impl ::std::default::Default for ConfigWheel {
    fn default() -> Self {
        Self {
            wheel_bias: 0,
            wheel_dead_zone: 6,
            wheel_degs_limit: 540,
            wheel_degs_max_possible: 1980,
            wheel_smooth_rate: 0.6,
            wheel_reverse_direction: false,
        }
    }
}

/// Feedback settings 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFeedback {
    pub feedback_dead_zone: u16,
    pub feedback_min_power: u16,
    pub feedback_max_power: u16,
    pub feedback_exponent: f32,
}

impl ::std::default::Default for ConfigFeedback {
    fn default() -> Self {
        Self {
            feedback_dead_zone: 10,
            feedback_min_power: 470,
            feedback_max_power: 480,
            feedback_exponent: 1.8,
        }
    }
}

/// Pedals settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigPedals {
    pub gas_dead_zone: u16,
    pub gas_value_limit: u16,
    pub gas_smooth_rate: f32,

    pub brake_dead_zone: u16,
    pub brake_value_limit: u16,
    pub brake_smooth_rate: f32,

    pub clutch_dead_zone: u16,
    pub clutch_value_limit: u16,
    pub clutch_smooth_rate: f32,
}

impl ::std::default::Default for ConfigPedals {
    fn default() -> Self {
        Self {
            gas_dead_zone: 2,
            gas_value_limit: 170,
            gas_smooth_rate: 0.3,

            brake_dead_zone: 2,
            brake_value_limit: 150,
            brake_smooth_rate: 0.3,
            
            clutch_dead_zone: 2,
            clutch_value_limit: 60,
            clutch_smooth_rate: 0.3,
        }
    }
}
