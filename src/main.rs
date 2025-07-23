#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::{ prelude::*, Wheel };

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::builder("Steering Wheel", "0.3.0")
        .logger(path!("$/Steering Wheel/logs"), 20)
        .config(path!("$/Steering Wheel/config.json"))
        .system_tray(Tray::builder()
            .menu(Tray::menu_builder()
                .item("show_hide", "Show/hide window", true)
                .item("exit", "Quit app", true)
            )
            .on_menu(|id| {
                match id {
                    "show_hide" => App::show_hide_window().unwrap(),
                    "exit" => App::exit(0).unwrap(),
                    _ => warn!("Unreached menu id '{id}'!"),
                }
            })
        )
        .invokes(tauri::generate_handler![
        ])
        .on_close(Arc::new(|| {
            App::save_config().unwrap();
            App::save_logs().unwrap();
            App::remove_tray().unwrap();
            App::exit(0).unwrap();
        }))
        .autostart(None)
        // .hide_on_start()
        .hide_to_tray(true)
        .build()?;

    Wheel::new()?.spawn_listenner().await?;
    
    App::run(app)
}
