use std::path::PathBuf;

use clipboard_rs::common::RustImage;
use clipboard_rs::RustImageData;
use tauri::{Error, Result};
use temp_dir::TempDir;
use uuid::Uuid;

pub fn build_temp_file_path(file_name: &str) -> PathBuf {
    let temp_dir = TempDir::new().unwrap();
    temp_dir.path().join(file_name)
}

pub fn build_temp_file_path_random() -> PathBuf {
    let temp_dir = TempDir::new().unwrap();
    temp_dir.path().join(Uuid::new_v4().to_string())
}

pub fn save_temp_file(file_name: &str, data: &[u8]) -> PathBuf {
    let file_path = build_temp_file_path(file_name);
    std::fs::write(&file_path, data).unwrap();
    file_path
}

pub fn clipboard_image_saver(data: RustImageData) -> Result<(String, String)> {
    let temp_path = build_temp_file_path_random();
    let temp_thumbnail_path = build_temp_file_path_random();

    if data.is_empty() {
        return Err(Error::from(anyhow::anyhow!(
            "Failed to get clipboard image"
        )));
    }

    match data.thumbnail(400, 400) {
        Ok(thumbnail) => match thumbnail.save_to_path(temp_thumbnail_path.to_str().unwrap()) {
            Ok(_) => {}
            Err(e) => return Err(Error::Anyhow(anyhow::anyhow!(e.to_string()))),
        },
        Err(e) => return Err(Error::Anyhow(anyhow::anyhow!(e.to_string()))),
    }

    // 尝试将原始图像转换为 PNG 并保存
    match data.to_png() {
        Ok(image) => match image.save_to_path(temp_path.to_str().unwrap()) {
            Ok(_) => {}
            Err(e) => return Err(Error::Anyhow(anyhow::anyhow!(e.to_string()))),
        },
        Err(e) => return Err(Error::Anyhow(anyhow::anyhow!(e.to_string()))),
    }

    Ok((
        temp_path.to_string_lossy().to_string(),
        temp_thumbnail_path.to_string_lossy().to_string(),
    ))
}
