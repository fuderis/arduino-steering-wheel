use crate::{ prelude::*, };
use super::{ MenuBuilder, TrayBuilder };

pub static SYSTEM_TRAY: Lazy<Arc<Mutex<Option<Tray>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// The system system tray
pub struct Tray {
    app_handle: Option<tauri::AppHandle>,
    tray: tauri::tray::TrayIcon,
}

impl Tray {
    /// Creates a system tray builder
    pub fn builder() -> TrayBuilder {
        TrayBuilder::default()
    }

    /// Creates a system tray menu builder
    pub fn menu_builder() -> MenuBuilder {
        MenuBuilder::default()
    }

    /// Creates a new system tray from builder
    pub(super) fn new(tray: tauri::tray::TrayIcon) -> Self {
        Self {
            app_handle: None,
            tray,
        }
    }

    /// Changes the system tray icon
    pub fn set_icon<P: AsRef<Path>>(&mut self, icon_path: P) -> Result<()> {
        let icon = tauri::image::Image::from_path(icon_path.as_ref())?;
        self.tray.set_icon(Some(icon))?;

        Ok(())
    }

    /// Removes the system tray
    pub fn remove(self) -> Result<()> {
        if let Some(app_handle) = self.app_handle {
            app_handle.remove_tray_by_id(self.tray.id());

            Ok(())
        } else {
            Err(Error::AppHandleUninitialized.into())
        }
    }
}
