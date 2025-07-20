use crate::prelude::*;
use super::{ LOGGER, CONFIG, SYSTEM_TRAY, AppBuilder, Tray, Config };
pub use tauri::{ Runtime, Emitter, Manager, ipc::InvokeHandler };

pub static APP: Lazy<App> = Lazy::new(|| App::default());
pub static APP_HANDLE: Lazy<Arc<Mutex<Option<tauri::AppHandle>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// The application
pub struct App {
    name: Arc<Mutex<String>>,
    version: Arc<Mutex<String>>,
}

impl ::std::default::Default for App {
    fn default() -> Self {
        Self {
            name: Arc::new(Mutex::new(str!("Test App"))),
            version: Arc::new(Mutex::new(str!("0.1.0"))),
        }
    }
}

impl App {
    /// Creates an application builder
    pub fn builder<S: Into<String>>(name: S, version: S) -> AppBuilder {
        AppBuilder::default()
            .name(name)
            .version(version)
    }
    
    /// Sets a tauri app handle
    pub(super) fn set_app_handle(app_handle: tauri::AppHandle) {
        APP_HANDLE.lock().unwrap().replace(app_handle);
    }

    /// Gets a tauri app handle
    pub fn get_app_handle() -> Result<tauri::AppHandle> {
        if let Some(handle) = APP_HANDLE.lock().unwrap().as_ref() {
            Ok(handle.clone())
        } else {
            Err(Error::AppHandleUninitialized.into())
        }
    }

    /// Sets the application name
    pub fn set_name<S: Into<String>>(name: S) {
        *APP.name.lock().unwrap() = name.into();
    }

    /// Gets the application name
    pub fn get_name() -> String {
        APP.name.lock().unwrap().clone()
    }

    /// Sets the application version
    pub fn set_version<S: Into<String>>(version: S) {
        *APP.version.lock().unwrap() = version.into();
    }

    /// Gets the application version
    pub fn get_version() -> String {
        APP.version.lock().unwrap().clone()
    }

    /// Sets a system tray
    pub fn set_tray(system_tray: Tray) -> Result<()> {
        Self::remove_tray()?;
        SYSTEM_TRAY.lock().unwrap().replace(system_tray);

        Ok(())
    }

    /// Sends event to frontend
    pub fn emit_event(event: &str, payload: impl serde::Serialize + Clone) {
        let guard = APP_HANDLE.lock().unwrap();

        if let Some(app) = guard.as_ref() {
            app.emit(event, payload).ok();
        }
    }

    /// Removes a system tray
    pub fn remove_tray() -> Result<()> {
        if let Some(tray) = SYSTEM_TRAY.lock().unwrap().take() {
            tray.remove()?;
        }

        Ok(())
    }

    /// Shows/Hides the application window
    pub fn show_hide_window() -> Result<()> {
        let window = Self::get_app_handle()?.get_webview_window("main").ok_or_else(|| Error::NoWindowWithName(str!("main")))?;

        if window.is_visible().unwrap_or(false) {
            Self::hide_window()
        } else {
            Self::show_window()
        }
    }

    /// Shows the application window
    pub fn show_window() -> Result<()> {
        let window = Self::get_app_handle()?.get_webview_window("main").ok_or_else(|| Error::NoWindowWithName(str!("main")))?;
        window.show().ok();
        window.unminimize().ok();
        window.set_focus().ok();

        Ok(())
    }
    
    /// Hides the application window
    pub fn hide_window() -> Result<()> {
        let window = Self::get_app_handle()?.get_webview_window("main").ok_or_else(|| Error::NoWindowWithName(str!("main")))?;
        window.hide()?;

        Ok(())
    }

    /// Quits from the application
    pub fn exit(code: i32) -> Result<()> {
        Self::remove_tray()?;
        Self::get_app_handle()?.exit(code);

        Ok(())
    }

    /// Gets the application config
    pub fn get_config() -> MutexGuard<'static, Config> {
        CONFIG.lock().unwrap()
    }
    
    /// Saves the application config to file
    pub fn save_config() -> Result<()> {
        CONFIG.lock().unwrap().save()
    }

    /// Saves the application logs to file
    pub fn save_logs() -> Result<()> {
        LOGGER.save()
    }

    /// Runs the application
    pub fn run(app: tauri::App) -> Result<()> {
        app.run(|_app_handle, _event| {});

        Ok(())
    }
}
