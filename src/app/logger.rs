use crate::prelude::*;
use std::fs;
use chrono::Local;

pub static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::default());

/// Application logger
#[derive(Debug)]
pub struct Logger {
    pub logs: Arc<Mutex<Vec<String>>>,
    pub logs_dir: Mutex<Option<PathBuf>>,
    pub files_limit: Mutex<Option<usize>>,
}

impl ::std::default::Default for Logger {
    fn default() -> Self {
        Self {
            logs: Arc::new(Mutex::new(vec![])),
            logs_dir: Mutex::new(None),
            files_limit: Mutex::new(None)
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let dt = Local::now().format("%Y-%m-%dT%H:%M:%S%.6f");
            let log = fmt!("[{dt}] [{}] {}", record.level(), record.args());

            // printing to terminal:
            println!("{log}");

            self.logs.lock().unwrap().push(log);
        }
    }

    fn flush(&self) {}
}

impl Logger {   
    /// Creates a new logger
    pub fn new<P: AsRef<Path>>(logs_dir: Option<P>, files_limit: Option<usize>) -> Self {
        Self {
            logs: Arc::new(Mutex::new(vec![])),
            logs_dir: Mutex::new(logs_dir.map(|dir| dir.as_ref().to_path_buf())),
            files_limit: Mutex::new(files_limit),
        }
    }

    /// Initializes a logger
    pub fn init(&'static self) -> Result<()> {
        log::set_logger(self).map_err(Error::from)?;
        log::set_max_level(log::LevelFilter::Info);

        Ok(())
    }

    /// Initializes a logger with custom parameters
    pub fn init_with<P: AsRef<Path>>(&'static self, logs_dir: P, files_limit: usize) -> Result<()> {
        *self.logs_dir.lock().unwrap() = Some(logs_dir.as_ref().to_path_buf());
        *self.files_limit.lock().unwrap() = Some(files_limit);
        
        self.init()
    }

    /// Collects logs and clears them
    pub fn take(&self) -> Vec<String> {
        let mut logs_lock = self.logs.lock().unwrap();

        std::mem::take(&mut *logs_lock)
    }

    /// Saves logs to file
    pub fn save(&self) -> Result<()> {
        let dir = if let Some(dir) = self.logs_dir.lock().unwrap().as_ref() { dir.clone() }else{ return Ok(()) };
        let path = dir.join( Local::now().format("%Y-%m-%d_%H-%M-%S.log").to_string() );

        // create file dir:
        let dir = path.parent().unwrap();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        // removing old log files:
        self.cleanup()?;
        
        // writing logs to file:
        let logs_str = self.logs.lock().unwrap().join("\n");
        fs::write(&path, logs_str)?;
        
        Ok(())
    }

    /// Removes an extra old log files
    fn cleanup(&self) -> Result<()> {
        let logs_dir_lock = self.logs_dir.lock().unwrap();
        let files_limit_lock = self.files_limit.lock().unwrap();
        if logs_dir_lock.is_none() || files_limit_lock.is_none() { return Ok(()) }

        let dir = logs_dir_lock.clone().unwrap();
        let limit = files_limit_lock.clone().unwrap();
        
        let mut log_files: Vec<PathBuf> = fs::read_dir(&dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "log") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        // sort files by time:
        log_files.sort_by_key(|path| fs::metadata(path).and_then(|m| m.created()).ok());

        // remove extra files:
        if log_files.len() > limit {
            for old_file in &log_files[0..log_files.len() - limit] {
                let _ = fs::remove_file(old_file);
            }
        }

        Ok(())
    }
}
