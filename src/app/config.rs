use crate::{ prelude::*, };
use std::fs;

pub static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(Config::default())));

/// Application config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)] path: PathBuf,
    pub com_port: u16,
    pub baud_rate: u32,

    pub wheel_dead_zone: u16,
    pub wheel_degs_limit: u16,
    pub wheel_degs_max_possible: u16,
    pub wheel_smooth_rate: f32,

    pub feedback_dead_zone: u16,
    pub feedback_min_speed: u16,
    pub feedback_max_speed: u16,

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

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            path: path!("/config.json"),
            com_port: 6,
            baud_rate: 115200,

            wheel_dead_zone: 1100,
            wheel_degs_limit: 540,
            wheel_degs_max_possible: 1800,
            wheel_smooth_rate: 0.85,

            feedback_dead_zone: 2500,
            feedback_min_speed: 435,
            feedback_max_speed: 480,

            gas_dead_zone: 0,
            gas_value_limit: 170,
            gas_smooth_rate: 0.85,

            brake_dead_zone: 0,
            brake_value_limit: 150,
            brake_smooth_rate: 0.85,
            
            clutch_dead_zone: 0,
            clutch_value_limit: 60,
            clutch_smooth_rate: 0.85,
        }
    }
}

impl Config {
    /// Reads/writes config file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        
        // reading config file:
        let config = if path.exists() {
            Config::read(path)?
        }
        // or writing default config file:
        else {
            let mut cfg = Config::default();
            cfg.save_to(path)?;

            cfg
        };

        Ok(config)
    }
    
    /// Reads config from file
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        // read file:
        let json_str = fs::read_to_string(&path)?;

        let mut cfg: Config = serde_json::from_str(&json_str)?;
        cfg.path = path;

        // parse json:
        Ok(cfg)
    }
    
    /// Updates config file
    pub fn save(&mut self) -> Result<()> {
        self.save_to(self.path.clone())
    }

    /// Saves config to custom path
    pub fn save_to<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.path = path.as_ref().to_path_buf();

        // to json string:
        let json_str = serde_json::to_string_pretty(self)?;

        // create dir:
        if let Some(dir) = self.path.parent() {
            fs::create_dir_all(dir)?;
        }
        
        // write file:
        fs::write(&self.path, json_str)?;
        
        Ok(())
    }
}
