use macron::{ Display, From, Error };

/// Std Result alias
pub type StdResult<T, E> = std::result::Result<T, E>;
/// Result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

/// Application error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    String(String),

    #[from]
    Logger(log::SetLoggerError),

    #[display = "Tauri AppHandle is uninitialized!"]
    AppHandleUninitialized,

    #[display = "No window with name '{0}' was found"]
    NoWindowWithName(String),

    #[display = "ViGEmBus driver not found — download and install!"]
    NoVigemBusFound(vigem_client::Error),

    #[display = "Failed to turn on the virtual gamepad"]
    FailedToTurnOnGamepad(vigem_client::Error),

    #[display = "Failed to update a controller: {0}"]
    FailedToUpdateController(vigem_client::Error),

    #[display = "Failed to get COM port: {0}"]
    FailedToGetCOMPort(serialport::Error),
}
