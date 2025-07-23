use crate::prelude::*;
use super::{ CONFIG_UPDATED, WINDOW_VISIBLE, APP_CLOSED, State, Feedback, Direction };

use std::io::{ BufReader, BufRead };
use serialport::SerialPort;
use vigem_client::{ Client, TargetId, Xbox360Wired, XGamepad, XButtons, XRequestNotification };

/// The steering wheel controller
pub struct Wheel {
    com_port: Arc<TokioMutex<Box<dyn SerialPort + Send>>>,
    gamepad: Arc<TokioMutex<Xbox360Wired<Arc<Client>>>>,
}

impl Wheel {
    /// Creates a new steering wheel controller
    pub fn new() -> Result<Arc<Self>> {
        // connecting to serial port:
        let Config { com_port, baud_rate, .. } = App::get_config().clone();
        let com_port = serialport::new(&fmt!("COM{}", com_port), baud_rate)
            .timeout(Duration::from_millis(5))
            .open()
            .map_err(|e| Error::FailedToGetCOMPort(e))?;
        
        // creating a new gamepad device:
        let client = Arc::new(Client::connect().map_err(|e| Error::NoVigemBusFound(e))?);
        let gamepad = Xbox360Wired::<Arc<Client>>::new(client, TargetId::XBOX360_WIRED);
        
        Ok(Arc::new(Self {
            com_port: Arc::new(TokioMutex::new(com_port)),
            gamepad: Arc::new(TokioMutex::new(gamepad)),
        }))
    }
    
    /// Spawns the serial port listenner
    pub async fn spawn_listenner(self: Arc<Self>) -> Result<()> {
        // starting the gamepad Xbox360 emulation:
        {
            let mut gamepad = self.gamepad.lock().await;
            gamepad.plugin().map_err(|e| Error::FailedToTurnOnGamepad(e))?;
            gamepad.wait_ready().map_err(|e| Error::FailedToTurnOnGamepad(e))?;
        }

        // spawning serial port listenner:
        let s1 = self.clone();
        tokio::spawn(async move {
            if let Err(e) = s1.listenner().await {
                err!("serial port listenner panicked with: {e}");
            }
        });

        Ok(())
    }

    /// The serial port listenner
    pub async fn listenner(self: Arc<Self>) -> Result<()> {
        // get the serial port controller:
        let mut com_port = self.com_port.lock().await;
        let mut line = String::new();

        // get the gamepad emulator:
        let mut gamepad = self.gamepad.lock().await;

        // get the gamepad vibration notifier:
        let mut x_notify = pin!({
            gamepad.request_notification()
                .map_err(|e| Error::FailedToTurnOnGamepad(e))?
        });

        // get steering wheel config:
        let mut config = App::get_config().clone();
        // calculating wheel degs limit:
        let mut wheel_limit = (config.wheel_degs_limit as f32 * 1020.0 / (config.wheel_degs_max_possible * 2) as f32).round() as u16;
        let mut wheel_limit_to_side = (wheel_limit as f32).round() as u16;
        // init empty wheel state:
        let mut prev_state = State::default();

        info!("Reading wheel state & emulating gamepad..");

        loop {
            line.clear();

            if CONFIG_UPDATED.swap(false, Ordering::SeqCst) {
                config = App::get_config().clone();
                wheel_limit = (config.wheel_degs_limit as f32 * 1020.0 / (config.wheel_degs_max_possible * 2) as f32).round() as u16;
                wheel_limit_to_side = (wheel_limit as f32).round() as u16;
            }
            
            let mut reader = BufReader::new(&mut **com_port);
            match reader.read_line(&mut line) {
                Ok(0) => continue,
                Ok(_) => {
                    let json = line.trim();
                    if json.is_empty() { continue }

                    // parsing wheel state & running handler:
                    match serde_json::from_str::<State>(json) {
                        Ok(state) => {
                            prev_state = self.handle_state(
                                &mut com_port,
                                &mut gamepad,
                                &mut x_notify,
                                state,
                                &mut prev_state,
                                &config,
                                wheel_limit,
                                wheel_limit_to_side,
                            ).await?;

                            std::thread::sleep(std::time::Duration::from_millis(10));
                        }

                        Err(_e) => {
                            // dbg!(_e);  // DEBUG: listenner error
                            continue
                        }
                    };
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(e) => return Err(e.into())
            }
        }
    }

    /// Handle steering wheel state
    async fn handle_state(&self,
        com_port: &mut Box<dyn SerialPort + Send>,
        gamepad: &mut Xbox360Wired<Arc<Client>>,
        x_notify: &mut Pin<&mut XRequestNotification>,
        mut state: State,
        prev_state: &mut State,
        config: &Config,
        wheel_limit: u16,
        wheel_limit_to_side: u16,
    ) -> Result<State> {
        // filtration potentiometer values:
        state.wheel = Self::filter_value(
            state.wheel,
            prev_state.wheel,
            config.wheel_dead_zone,
            true,
            wheel_limit,
            config.wheel_smooth_rate,
            config.wheel_bias,
        );
        let wheel_centered_value = state.wheel as i16 - 510;

        // dbg!(&wheel_centered_value);  // DEBUG: wheel value

        state.gas = Self::filter_value(
            state.gas,
            prev_state.gas,
            config.gas_dead_zone,
            false,
            config.gas_value_limit,
            config.gas_smooth_rate,
            0
        );

        state.brake = Self::filter_value(
            state.brake,
            prev_state.brake,
            config.brake_dead_zone,
            false,
            config.brake_value_limit,
            config.brake_smooth_rate,
            0
        );

        state.clutch = Self::filter_value(
            state.clutch,
            prev_state.clutch,
            config.clutch_dead_zone,
            false,
            config.clutch_value_limit,
            config.clutch_smooth_rate,
            0
        );

        // activating pressed buttons:
        let mut buttons = XButtons { raw: 0 };
        if state.up         { buttons.raw |= XButtons::A; }
        if state.down       { buttons.raw |= XButtons::X; }
        if state.handbrake  { buttons.raw |= XButtons::B; }

        let gamepad_state = XGamepad {
            buttons,
            left_trigger: Self::to_trigger_value(state.brake, config.brake_value_limit),
            right_trigger: Self::to_trigger_value(state.gas, config.gas_value_limit),
            thumb_lx: Self::to_axis_value(wheel_centered_value, wheel_limit),
            thumb_ly: 0,
            thumb_rx: 0,
            thumb_ry: Self::to_absolute_axis_value(state.clutch, config.clutch_value_limit),
        };

        // get feedback direction:
        let feedback_direct =
            if wheel_centered_value == 0 {
                Direction::Center
            } else if (wheel_centered_value > 0) ^ config.wheel_reverse_direction {
                Direction::Left
            } else {
                Direction::Right
            };

        // get vibration value:
        let vibration_value = match x_notify.as_mut().poll(true) {
            Ok(Some(notif)) => notif.large_motor.into(),
            _ => 0,
        };
        
        // calculating feedback power:
        let feedback_power = if wheel_centered_value.abs() > config.feedback_dead_zone as i16 {
            config.feedback_min_power

            + Self::calculate_wheel_feedback(
                wheel_centered_value,
                wheel_limit_to_side,
                config.feedback_dead_zone,
                config.feedback_min_power,
                config.feedback_max_power,
                config.feedback_exponent,
            )

            + Self::calculate_vibration_feedback(
                vibration_value,
                config.feedback_max_power
            )
        } else {
            0
        }.clamp(0, config.feedback_max_power);

        let feedback = Feedback {
            motor: feedback_direct,
            power: feedback_power,
        };

        // sending feedback to the motor:
        self.send_feedback(com_port, &feedback).await?;
        // updating gamepad state:
        self.update_gamepad(gamepad, &gamepad_state).await?;

        if WINDOW_VISIBLE.load(Ordering::Relaxed) {
            App::emit_event("update-state", json!(
                {
                    "wheel": wheel_centered_value,
                    "wheel_min": config.wheel_dead_zone,
                    "wheel_max": wheel_limit_to_side,

                    "feeback": feedback.power,
                    "feeback_min": config.feedback_min_power,
                    "feeback_max": config.feedback_max_power,

                    "gas": state.gas,
                    "gas_min": config.gas_dead_zone,
                    "gas_max": config.gas_value_limit,

                    "brake": state.brake,
                    "brake_min": config.brake_dead_zone,
                    "brake_max": config.brake_value_limit,

                    "clutch": state.clutch,
                    "clutch_min": config.clutch_dead_zone,
                    "clutch_max": config.clutch_value_limit,
                }
            ));
        }

        Ok(state)
    }
    
    /// Send feedback response
    async fn send_feedback(&self, com_port: &mut Box<dyn SerialPort + Send>, feedback: &Feedback) -> Result<()> {
        let feedback_json = serde_json::to_string(feedback).unwrap();

        com_port.write_all(feedback_json.as_bytes())?;
        com_port.write_all(b"\n")?;
        
        Ok(())
    }

    /// Update gamepad state
    async fn update_gamepad(&self, gamepad: &mut Xbox360Wired<Arc<Client>>, state: &XGamepad) -> Result<()> {        
        gamepad.update(state)
            .map_err(|e| Error::FailedToUpdateController(e))?;

        Ok(())
    }

    /// Filters potentiometer value
    fn filter_value(value: u16, prev_value: u16, dead_zone: u16, dead_zone_from_center: bool, max_value: u16, smooth_rate: f32, bias: i16) -> u16 {        
        let smoothed = (prev_value as f32 * smooth_rate + value as f32 * (1.0 - smooth_rate)).round() as u16;

        let mut value = if dead_zone_from_center {
            let smoothed = smoothed as i16 - 510;
            let max_value = max_value as i16;
            
            if smoothed.abs() > dead_zone as i16 {
                (smoothed.clamp(-max_value, max_value) + 510) as u16
            } else {
                510
            }
        }
        else if smoothed > dead_zone {
            smoothed.clamp(0, max_value)
        }
        else {
            0
        };

        if bias != 0 {
            value = (value as i32 + bias as i32).clamp(0, max_value as i32) as u16;
        }

        value
    }

    /// Increase the feedback power by a wheel angle value
    fn calculate_wheel_feedback(wheel_centered_value: i16, wheel_max_value_to_side: u16, feedback_dead_zone: u16, feedback_min_power: u16, feedback_max_power: u16, feedback_exponent: f32) -> u16 {
        let wheel_value = wheel_centered_value.abs() as u16;
        
        if wheel_value > feedback_dead_zone {
            let wheel_value = wheel_value - feedback_dead_zone;
            let wheel_max_value = wheel_max_value_to_side - feedback_dead_zone;
            let feedback_max_power = feedback_max_power - feedback_min_power;

            let proportion = (wheel_value as f32 / wheel_max_value as f32).powf(feedback_exponent);

            (feedback_max_power as f32 * proportion.clamp(0.0, 1.0)).round() as u16
        } else {
            0
        }
    }

    /// Increase the feedback power by a vibration value
    fn calculate_vibration_feedback(vibration: u32, max_feedback: u16) -> u16 {
        if vibration > 0 {
            let scaled = (vibration as f32) / 65535.0;
            (max_feedback as f32 * scaled).round() as u16
        } else {
            0
        }
    }

    /// Convert the axis value [min,max] to the gamepad range [-32768..32767]
    fn to_axis_value(value: i16, max: u16) -> i16 {
        let max = max as i16;
        let value = value.clamp(-max, max) as f32;
        ((value as f32) * 32767.0 / (max as f32)).round() as i16
    }

    /// Convert the axis value [0,max] to the gamepad range [0..32767]
    fn to_absolute_axis_value(value: u16, max: u16) -> i16 {
        let value = value.clamp(0, max) as f32;
        ((value as f32) * 32767.0 / (max as f32)).round() as i16
    }

    /// Convert the trigger value [min,max] to the gamepad range [0..255]
    fn to_trigger_value(value: u16, max: u16) -> u8 {
        let value = value.clamp(0, max) as f32;
        ((value as f32) * 255.0 / (max as f32)).round() as u8
    }
}
