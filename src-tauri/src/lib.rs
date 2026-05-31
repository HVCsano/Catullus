use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Listener, Manager, PhysicalSize, Size,
};

use crate::util::ffmpeg::{download_ffmpeg, setup_ffmpeg};

mod util;

#[tauri::command]
async fn update_done(app: AppHandle) {
    app.emit("setloadertext", "Konfiguráció betöltése").unwrap();
    let main = tauri::WebviewWindowBuilder::from_config(
        &app,
        &app.config().app.windows.get(1).unwrap().clone(),
    )
    .unwrap();
    let config = util::config::load_config();
    if config.is_none() {
        let loader = app.get_webview_window("loader").unwrap();
        app.emit("setloadertext", "Konfiguráció nem létezik")
            .unwrap();
        let main = main.build().unwrap();
        let main_clone = main.clone();
        main.once("panel", move |_| {
            main_clone.emit("changepanel", "setup").unwrap();
        });
        loader.close().unwrap();
        main.show().unwrap();
        main.set_focus().unwrap();
        return;
    }
    app.emit("setloadertext", "ffmpeg letöltése").unwrap();
    let ffmpeg_dl = download_ffmpeg().await;
    if ffmpeg_dl.is_err() {
        app.emit("setloadertext", ffmpeg_dl.unwrap_err()).unwrap();
        return;
    }
    if ffmpeg_dl.unwrap() {
        app.emit("setloadertext", "ffmpeg előkészítése").unwrap();
        let fset = setup_ffmpeg().await;

        if fset.is_err() {
            app.emit(
                "setloadertext",
                &format!("ffmpeg hiba: {}", fset.unwrap_err().to_string()),
            )
            .unwrap();
            return;
        }
    }
    app.emit("setloadertext", "Felület előkészítése").unwrap();
    let main = main.build().unwrap();
    let main_clone = main.clone();
    main.once("panel", move |_| {
        main_clone.emit("changepanel", "main").unwrap();
    });
    let loader = app.get_webview_window("loader").unwrap();
    loader.close().unwrap();
    main.show().unwrap();
    main.set_focus().unwrap();
    main.set_size(Size::Physical(PhysicalSize {
        height: 720,
        width: 1280,
    }))
    .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            util::config::setup_folders();
            let quit_i = MenuItem::with_id(app, "quit", "Kilépés", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            TrayIconBuilder::new()
                .menu(&menu)
                .title("Catullus")
                .tooltip("Catullus")
                .icon(app.default_window_icon().unwrap().clone())
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            update_done,
            stop_app,
            util::config::set_game_dir,
            util::config::save_game_dir,
            util::config::done_setup,
            util::files::load_files_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn stop_app(app: AppHandle) {
    app.exit(0);
}
