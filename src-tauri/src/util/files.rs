use std::{
    collections::HashMap,
    fs::{create_dir, remove_file, File},
    io::{self, Write},
    path::Path,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use sha2::Digest;
use tauri::{AppHandle, Emitter, Listener};

use rayon::prelude::*;

use crate::util::config::{get_conf_path, load_config};

#[derive(Debug, Serialize, Deserialize)]
pub struct FilesFile {
    pub count: i64,
    pub files: HashMap<String, (String, Option<String>)>,
}

#[tauri::command]
pub async fn load_files_file() -> Option<FilesFile> {
    let pat = get_conf_path();
    let conf = load_config().unwrap();

    let config = File::open(format!("{}/templates/default_v4/files.json", pat));
    if config.is_err() {
        return None;
    }
    let file = config.unwrap();
    let real_config: Option<FilesFile> = serde_json::from_reader(file).ok();
    if real_config.is_none() {
        return None;
    }
    let real_config = real_config.unwrap();
    for (dir, (val, _)) in real_config.files.iter() {
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

#[tauri::command]
pub async fn generate_files_file(app: AppHandle) {
    let conf = load_config().unwrap();

    app.emit("changepanel", "simpleprogress").unwrap();

    let appc = app.clone();

    app.once("progress", move |_| {
        appc.emit("setprogress-title", "Fájlstruktúra generálás")
            .unwrap();
        appc.emit("setprogress-desc", "Előkészítés...").unwrap();
        appc.emit("setprogress-state", 0).unwrap();
    });

    let pat = get_conf_path();

    let temp_dir = format!("{}/templates/default_v4", pat);
    let temp_dir = Path::new(&temp_dir);
    if !temp_dir.exists() {
        create_dir(temp_dir).unwrap();
    }

    let listfile = format!("{}/templates/default_v4/files.json", pat);
    let _ = remove_file(&listfile);

    let game_dir = format!("{}/mods/deathmatch/resources", conf.game_dir);
    let game_dir = Path::new(&game_dir);

    if !game_dir.exists() {
        app.emit("changepanel", "main/gamedirerr").unwrap();
        return;
    }
    let folders = game_dir
        .read_dir()
        .unwrap()
        .filter(|p| {
            p.as_ref()
                .map(|dir| {
                    dir.file_name()
                        .to_str()
                        .map_or(false, |s| s.starts_with("v4_"))
                })
                .unwrap_or(false)
        })
        .map(|f| f.unwrap())
        .collect::<Vec<_>>();

    let foldercount = folders.len();

    let mut files = FilesFile {
        count: 0,
        files: HashMap::new(),
    };

    let accepted_files = vec!["dds", "wav", "mp3", "png"];

    let mut all_files = Vec::new();

    for (i, fold) in folders.iter().enumerate() {
        app.emit("setprogress-desc", "Generálás...").unwrap();
        app.emit("setprogress-undesc", "Ez sokáig is eltarthat")
            .unwrap();
        app.emit("setprogress-state", (i + 1) * 100 / foldercount)
            .unwrap();

        let mut folders = vec![Path::new(&fold.path()).read_dir().unwrap()];

        while let Some(folder) = folders.pop() {
            for entry in folder {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_dir() {
                    folders.push(Path::new(&entry.path()).read_dir().unwrap());
                } else if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                    if accepted_files.contains(&ext) {
                        all_files.push(entry.path());
                    }
                }
            }
        }
    }

    let file_count = Mutex::new(0);
    let results: HashMap<String, (String, Option<String>)> = all_files
        .par_iter()
        .filter_map(|path| {
            let mut file = File::open(path).ok()?;
            let mut hasher = sha2::Sha256::new();
            io::copy(&mut file, &mut hasher).ok()?;
            let hash = format!("{:x}", hasher.finalize());

            let relative = path.strip_prefix(game_dir).ok()?.to_str()?.to_string();

            let mut count = file_count.lock().unwrap();
            *count += 1;

            Some((relative, (hash, None)))
        })
        .collect();
    files.count = results.len() as i64;
    files.files = results;
    let data_pretty = serde_json::to_string_pretty(&files).unwrap();
    let mut file = File::create(listfile).unwrap();
    file.write(data_pretty.as_bytes()).unwrap();
}
