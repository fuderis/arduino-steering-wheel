use crate::prelude::*;
use super::{ Tray, MenuBuilder };
use tauri::tray::{ TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState };

pub type TrayClickHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type TrayMenuHandler = Arc<dyn Fn(&str) + Send + Sync + 'static>;

/// The system tray builder
pub struct TrayBuilder {
    icon_path: Option<PathBuf>,
    title: Option<String>,
    menu: Option<MenuBuilder>,
    on_click: Option<TrayClickHandler>,
    on_menu: Option<TrayMenuHandler>,
}

impl Default for TrayBuilder {
    fn default() -> Self {
        Self {
            icon_path: None,
            title: None,
            menu: None,
            on_click: None,
            on_menu: None,
        }
    }
}

impl TrayBuilder {
    /// Sets a system tray icon from path
    pub fn icon<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.icon_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Sets a system tray menu
    pub fn menu(mut self, menu: MenuBuilder) -> Self {
        self.menu = Some(menu);
        self
    }

    /// Sets a system tray tooltip message
    pub fn title<S: Into<String>>(mut self, message: S) -> Self {
        self.title = Some(message.into());
        self
    }

    /// Sets the event 'on click' handler
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static
    {
        self.on_click = Some(Arc::new(handler));
        self
    }

    /// Sets the event 'on menu' handler
    pub fn on_menu<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static
    {
        self.on_menu = Some(Arc::new(handler));
        self
    }

    /// Builds the system tray
    pub(super) fn build(self) -> Result<Tray> {
        let app_handle = App::get_app_handle()?;
        
        let mut tauri_builder = TrayIconBuilder::new()
            .show_menu_on_left_click(false);

        // set icon:
        tauri_builder = tauri_builder.icon(tauri::image::Image::from_path(
            if let Some(icon_path) = self.icon_path {
                icon_path
            } else {
                path!("/icon.ico")
            }
        )?);

        // set title:
        tauri_builder = tauri_builder.tooltip(
            if let Some(title) = self.title {
                title
            } else {
                App::get_name().into()
            }
        );

        // set menu:
        if let Some(menu) = self.menu {
            tauri_builder = tauri_builder.menu(&menu.build(&app_handle));
        }

        // clone event handlers:
        let click_handler = self.on_click.clone();
        let menu_handler = self.on_menu.clone();

        tauri_builder = tauri_builder
            .on_tray_icon_event(move |_tray, event| {
                // set event 'on click':
                if let TrayIconEvent::Click { button, button_state, .. } = event {
                    if button == MouseButton::Left && button_state == MouseButtonState::Up {
                        if let Some(ref handler) = click_handler {
                            handler();
                        }
                        
                        App::show_hide_window().ok();
                    }
                }
            })
            // set event 'on menu':
            .on_menu_event(move |_app, event| {
                if let Some(ref handler) = menu_handler {
                    handler(event.id.as_ref());
                }
            });

        Ok(Tray::new(tauri_builder.build(&app_handle)?))
    }
}
