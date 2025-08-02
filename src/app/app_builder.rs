use crate::prelude::*;
use super::{ TrayBuilder, LOGGER, Config };
use tauri::WindowEvent;

pub type EventHandler = Arc<dyn Fn() + Send + Sync + 'static>;

/// The application builder
pub struct AppBuilder {
    pub(super) name: String,
    pub(super) version: String,
    
    pub(super) config_path: Option<PathBuf>,
    pub(super) logs_dir: Option<PathBuf>,
    pub(super) logs_max_count: Option<usize>,

    pub(super) system_tray: Option<TrayBuilder>,
    pub(super) invoke_handlers: Option<Box<dyn Fn(tauri::ipc::Invoke) -> bool + Send + Sync + 'static>>,

    pub(super) plugin_prevent: bool,
    pub(super) plugin_autostart: bool,
    pub(super) plugin_autostart_args: Vec<String>,

    pub(super) on_start: Option<EventHandler>,
    pub(super) on_hide: Option<EventHandler>,
    pub(super) on_show: Option<EventHandler>,
    pub(super) on_close: Option<EventHandler>,

    pub(super) hide_on_start: bool,
    pub(super) hide_to_tray: bool,
    pub(super) hide_to_tray_always: bool,
}

impl ::std::default::Default for AppBuilder {
    fn default() -> Self {
        Self {
            name: str!("Test App"),
            version: str!("0.1.0"),
            
            config_path: None,
            logs_dir: None,
            logs_max_count: None,

            system_tray: None,
            invoke_handlers: None,

            plugin_prevent: true,
            plugin_autostart: false,
            plugin_autostart_args: vec![],

            on_start: None,
            on_hide: None,
            on_show: None,
            on_close: None,

            hide_on_start: false,
            hide_to_tray: false,
            hide_to_tray_always: false,
        }
    }
}

impl AppBuilder {
    /// Creates a new application builder
    pub fn new<S: Into<String>>(name: S, version: S) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            ..Self::default()
        }
    }

    /// Sets the application name
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the application version
    pub fn version<S: Into<String>>(mut self, version: S) -> Self {
        self.version = version.into();
        self
    }
    
    /// Sets a config file path
    pub fn config<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.config_path.replace(path.as_ref().to_path_buf());
        self
    }

    /// Sets a logs dir path & max files count
    pub fn logger<P: AsRef<Path>>(mut self, dir: P, max_count: usize) -> Self {
        self.logs_dir.replace(dir.as_ref().to_path_buf());
        self.logs_max_count.replace(max_count);
        self
    }
    
    /// Sets a system tray icon path
    pub fn system_tray(mut self, tray: TrayBuilder) -> Self {
        self.system_tray.replace(tray);
        self
    }

    /// Sets an invoke handlers
    pub fn invokes<F>(mut self, invoke_handlers: F) -> Self
    where
        F: Fn(tauri::ipc::Invoke) -> bool + Send + Sync + 'static
    {
        self.invoke_handlers.replace(Box::new(invoke_handlers));
        self
    }

    /// Disables the plugin 'prevent'
    pub fn disable_prevent(mut self) -> Self {
        self.plugin_prevent = false;
        self
    }

    /// Enables the plugin 'autostart'
    pub fn autostart(mut self, args: &[&str]) -> Self {
        self.plugin_autostart = true;
        self.plugin_autostart_args = args.into_iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        self
    }

    /// Sets the event 'on_start' handler
    pub fn on_start(mut self, handler: EventHandler) -> Self {
        self.on_start.replace(handler);
        self
    }

    /// Sets the event 'on_hide' handler
    pub fn on_hide(mut self, handler: EventHandler) -> Self {
        self.on_hide.replace(handler);
        self
    }

    /// Sets the event 'on_show' handler
    pub fn on_show(mut self, handler: EventHandler) -> Self {
        self.on_show.replace(handler);
        self
    }

    /// Sets the event 'on_close' handler
    pub fn on_close(mut self, handler: EventHandler) -> Self {
        self.on_close.replace(handler);
        self
    }

    /// Sets the flag hide window on start
    pub fn hide_on_start(mut self) -> Self {
        self.hide_on_start = true;
        self
    }
    
    /// Sets the flag hide window to tray
    pub fn hide_to_tray(mut self) -> Self {
        self.hide_to_tray = true;
        self
    }

    /// Sets the flag hide window to tray (on close also)
    pub fn hide_to_tray_always(mut self) -> Self {
        self.hide_to_tray = true;
        self.hide_to_tray_always = true;
        self
    }

    /// Builds application
    pub fn build(self) -> Result<tauri::App> {
        // init app logger:
        if self.logs_dir.is_none() {
            LOGGER.init()?;
        } else {
            LOGGER.init_with(&self.logs_dir.unwrap(), self.logs_max_count.unwrap())?;
        }

        // init app config:
        if let Some(path) = self.config_path {
            *App::get_config() = Config::new(&path)?;
        }

        // create default tauri builder:
        let mut tauri_builder = tauri::Builder::default();

        // set invoke handlers:
        if let Some(handlers) = self.invoke_handlers {
            tauri_builder = tauri_builder.invoke_handler(handlers);
        }

        // set plugin 'prevent default'
        if self.plugin_prevent {
            tauri_builder = tauri_builder.plugin(
                tauri_plugin_prevent_default::Builder::new()
                    .with_flags(tauri_plugin_prevent_default::Flags::empty())
                    .build(),
            );
        }

        // set plugin 'autostart'
        if self.plugin_autostart {
            tauri_builder = tauri_builder.plugin(
                tauri_plugin_autostart::Builder::new()
                    .args(self.plugin_autostart_args)
                    .app_name(self.name.clone())
                    .build()
            );
        }

        // run tauri builder:
        tauri_builder
            .setup(move |app| {
                let app_handle = app.app_handle().clone();
                App::set_app_handle(app_handle.clone());

                // set system tray icon:
                if let Some(tray) = self.system_tray {
                    App::set_tray(tray.build().unwrap()).unwrap();
                }

                // start event 'on start' handler:
                if let Some(handler) = self.on_start {
                    (*handler)();
                }

                // set window events:
                if let Some(window) = app_handle.get_webview_window("main") {
                    window.on_window_event(move |event| {
                        let window = app_handle.clone().get_webview_window("main").unwrap();

                        match event {
                            // set event 'on close' handler:
                            WindowEvent::CloseRequested { api, .. } => {
                                api.prevent_close();

                                if self.hide_to_tray_always {
                                    if let Some(handler) = self.on_hide.as_ref() {
                                        (*handler)();
                                    }
                                    
                                    App::hide_window().ok();
                                } else {
                                    if let Some(handler) = self.on_close.as_ref() {
                                        (*handler)();
                                    }

                                    App::save_config().unwrap();
                                    App::save_logs().unwrap();
                                    App::remove_tray().ok();
                                    App::exit(0).ok();
                                }
                            }

                            // set event resize handlers:
                            WindowEvent::Resized(_) => {
                                // on hide:
                                if window.is_minimized().unwrap_or(false) {
                                    if let Some(handler) = self.on_hide.as_ref() {
                                        (*handler)();
                                    }

                                    if self.hide_to_tray || self.hide_to_tray_always {
                                        App::hide_window().ok();
                                    }
                                }
                                // on show:
                                else {
                                    if let Some(handler) = self.on_show.as_ref() {
                                        (*handler)();
                                    }
                                }
                            }

                            _ => {}
                        }
                    });
                }

                // hide window on start:
                if self.hide_on_start {
                    App::hide_window().ok();
                }

                Ok(())
            })
            .build(tauri::generate_context!())
            .map_err(Into::into)
    }
    
    /// Builds and runs the application
    pub fn run(self) -> Result<()> {
        let app = self.build()?;
        App::run(app)
    }
}
