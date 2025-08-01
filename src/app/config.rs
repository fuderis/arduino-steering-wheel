use crate::{ prelude::*, };
use super::config_parts::*;
use std::fs;

pub static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(Config::default())));

/// Application config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)] path: PathBuf,
    pub comport: ConfigComport,
    pub wheel: ConfigWheel,
    pub feedback: ConfigFeedback,
    pub pedals: ConfigPedals,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            path: path!("/config.json"),
            comport: ConfigComport::default(),
            wheel: ConfigWheel::default(),
            feedback: ConfigFeedback::default(),
            pedals: ConfigPedals::default(),
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
