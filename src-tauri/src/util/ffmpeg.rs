use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

use sevenz_rust2::decompress_file;

use crate::util::config::get_conf_path;

pub async fn download_ffmpeg() -> Result<bool, String> {
    let url = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-git-essentials.7z";
    let client = reqwest::Client::builder().build().unwrap();

    let res = client.get(url).send().await;
    if res.is_err() {
        return Err(String::from("ffmpeg letöltése sikertelen, part 1"));
    }
    let res = res.unwrap();

    if !res.status().is_success() {
        return Err(String::from("ffmpeg letöltése sikertelen, part 2"));
    }

    let pat = get_conf_path();

    let urlpat = format!("{}/ffmpeg/dlurl.ffmpeg", pat);

    if Path::new(&urlpat).exists() {
        let urlfile = fs::read_to_string(&urlpat).unwrap();
        let exefolder = format!("{}/ffmpeg/ffmpeg.exe", pat);
        if &urlfile == res.url().as_str() && Path::new(&exefolder).exists() {
            return Ok(false);
        }
    }

    let _ = fs::create_dir(format!("{}/cache/ffmpeg", pat));
    let mut linkfile = File::create(&urlpat).unwrap();
    linkfile.write(res.url().as_str().as_bytes()).unwrap();

    let filename = "ffmpeg-git-essentials.7z";
    let ffmpeg_path = format!("{}/cache/ffmpeg/{}", pat, filename);

    let bytes = res.bytes().await.map_err(|e| e.to_string())?;
    let mut file = File::create(&ffmpeg_path).map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;

    Ok(true)
}

pub async fn setup_ffmpeg() -> Result<(), Box<dyn Error>> {
    let pat = get_conf_path();

    let file_pat = format!("{}/cache/ffmpeg/ffmpeg-git-essentials.7z", pat);

    decompress_file(file_pat, format!("{}/cache/ffmpeg/", pat))?;

    let dir = fs::read_dir(format!("{}/cache/ffmpeg", pat))?;

    for ent in dir {
        let ent = ent?;
        let meta = ent.metadata()?;

        if !meta.is_dir() {
            continue;
        }

        let exe_folder = format!(
            "{}/cache/ffmpeg/{}/bin/ffmpeg.exe",
            pat,
            ent.file_name().into_string().unwrap()
        );

        if Path::new(&exe_folder).exists() {
            fs::copy(exe_folder, format!("{}/ffmpeg/ffmpeg.exe", pat))?;
            break;
        }
    }

    let _ = fs::remove_dir_all(format!("{}/cache/ffmpeg", pat));

    Ok(())
}
