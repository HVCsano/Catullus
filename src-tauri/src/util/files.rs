use std::{collections::HashMap, fs::File, io};

use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::util::config::{get_conf_path, load_config};

#[derive(Debug, Serialize, Deserialize)]
pub struct FilesFile {
    pub files: HashMap<String, [String; 2]>,
}

#[tauri::command]
pub async fn load_files_file() -> Option<FilesFile> {
    let pat = get_conf_path();
    let conf = load_config().unwrap();

    let config = File::open(format!("{}/defaultfiles.json", pat));
    if config.is_err() {
        return None;
    }
    let file = config.unwrap();
    let real_config: Option<FilesFile> = serde_json::from_reader(file).ok();
    if real_config.is_none() {
        return None;
    }
    let real_config = real_config.unwrap();
    for (dir, [val, _]) in real_config.files.iter() {
        let file = File::open(format!(
            "{}/mods/deathmatch/resources/{}",
            conf.game_dir, dir
        ));
        if file.is_err() {
            return None;
        }
        let mut hasher = sha2::Sha256::new();
        io::copy(&mut file.unwrap(), &mut hasher).unwrap();
        let hash = hasher.finalize();
        if val != &format!("{:x}", hash) {
            return None;
        }
    }
    return Some(real_config);
}
