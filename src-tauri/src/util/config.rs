use std::{
    env::consts::OS,
    fs::{self, create_dir, File},
    io::Write,
    path::Path,
};

use homedir::my_home;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub game_dir: String,
    pub show_help: bool,
}

pub fn get_conf_path() -> String {
    let os = OS;
    let dir = my_home().unwrap().unwrap();
    let dir_str = dir.to_str().unwrap().to_string();
    let main_folder_path = if os == "windows" {
        format!("{}/AppData/Roaming/Catullus", dir_str)
    } else {
        format!("{}/.config/Catullus", dir_str)
    };
    return main_folder_path;
}

pub fn setup_folders() {
    let pat = get_conf_path();
    let main_folder = Path::new(&pat);
    if !main_folder.exists() {
        create_dir(main_folder).unwrap();
    }
    let cache_dir = format!("{}/cache", pat);
    let cache_folder = Path::new(&cache_dir);
    if !cache_folder.exists() {
        create_dir(cache_folder).unwrap();
    }
    let templates_dir = format!("{}/templates", pat);
    let templates_folder = Path::new(&templates_dir);
    if !templates_folder.exists() {
        create_dir(templates_folder).unwrap();
    }
    let ffmpeg_dir = format!("{}/ffmpeg", pat);
    let ffmpeg_folder = Path::new(&ffmpeg_dir);
    if !ffmpeg_folder.exists() {
        create_dir(ffmpeg_folder).unwrap();
    }
}

pub fn load_config() -> Option<Config> {
    let pat = get_conf_path();
    let config = File::open(format!("{}/config.json", pat));
    if config.is_err() {
        return None;
    }
    let file = config.unwrap();
    let real_config: Option<Config> = serde_json::from_reader(file).ok();
    if real_config.is_none() {
        return None;
    }
    let real_config = real_config.unwrap();
    if fs::read_dir(&real_config.game_dir).is_err() {
        return None;
    }
    return Some(real_config);
}

pub fn save_config(config: Config) {
    let pat = get_conf_path();
    let config_str = serde_json::to_string_pretty(&config).unwrap();
    let mut file = File::create(format!("{}/config.json", pat)).unwrap();
    file.write(config_str.as_bytes()).unwrap();
}

#[tauri::command]
pub async fn set_game_dir(app: AppHandle) {
    app.dialog().file().pick_folder(move |folder| {
        if folder.is_some() {
            app.emit("selectedGameDir", folder.unwrap()).unwrap();
        }
    });
}

#[tauri::command]
pub async fn disable_help(_app: AppHandle) {
    let mut conf = load_config().unwrap();
    conf.show_help = false;
    save_config(conf);
}

#[tauri::command]
pub async fn save_game_dir(dir: String) {
    let config = Config {
        game_dir: dir.clone(),
        show_help: true,
    };
    save_config(config);
}

#[tauri::command]
pub async fn done_setup(app: AppHandle) {
    app.restart();
}
