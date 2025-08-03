#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::{ prelude::*, CONFIG_UPDATED, WINDOW_VISIBLE, APP_CLOSED, Wheel, };

/// Returns app config
#[tauri::command]
async fn get_config() -> StdResult<Config, String> {
    Ok(Config::read(path!("$/Steering Wheel/config.json")).unwrap_or_default())
}

/// Updates config by parts
#[tauri::command]
async fn update_config_part(name: String, json: String) -> StdResult<(), String> {
    {
        let mut config = App::get_config();
    
        match name.as_ref() {
            "comport-settings" => match serde_json::from_str(&json) {
                Ok(part) => { config.comport = part; Ok(()) },
                Err(e) => Err(str!(e)),
            }
            "wheel-settings" => match serde_json::from_str(&json) {
                Ok(part) => { config.wheel = part; Ok(()) },
                Err(e) => Err(str!(e)),
            }
            "feedback-settings" => match serde_json::from_str(&json) {
                Ok(part) => { config.feedback = part; Ok(()) },
                Err(e) => Err(str!(e)),
            }
            "pedals-settings" => match serde_json::from_str(&json) {
                Ok(part) => { config.pedals = part; Ok(()) },
                Err(e) => Err(str!(e)),
            }
            _ => Err(str!("Unexpected config part name '{name}'")),
        }
        .map_err(|e| {
            let e = str!("Failed to parse a config part '{name}': {e}");
    
            err!("{e}");
            e
        })?;
    }

    // dbg!(&name, &json);  // DEBUG: Config part data

    App::save_config().ok();
    CONFIG_UPDATED.swap(true, Ordering::SeqCst);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::builder("Steering Wheel", "0.3.0")
        .logger(path!("$/Steering Wheel/logs"), 20)
        .config(path!("$/Steering Wheel/config.json"))

        .invokes(tauri::generate_handler![
            get_config,
            update_config_part,
        ])
        
        .system_tray(Tray::builder()
            .menu(Tray::menu_builder()
                .item("show_hide", "Show/hide window", true)
                .item("exit", "Quit app", true)
            )
            .on_menu(|id| {
                match id {
                    "show_hide" => {
                        App::show_hide_window().ok();
                    }
                    "exit" => {
                        APP_CLOSED.swap(true, Ordering::SeqCst);
                        App::exit(0).ok();
                    }
                    _ => warn!("Unreached menu id '{id}'!"),
                }
            })
        )

        .on_hide(Arc::new(|| {
            WINDOW_VISIBLE.swap(false, Ordering::SeqCst);
        }))
        .on_show(Arc::new(|| {
            WINDOW_VISIBLE.swap(true, Ordering::SeqCst);
        }))

        .autostart(&[])
        .hide_on_start()
        .hide_to_tray_always()
        .build()?;

    Wheel::spawn_listenner().await?;
    
    App::run(app)
}
