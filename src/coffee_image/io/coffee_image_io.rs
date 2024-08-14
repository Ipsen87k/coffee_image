use std::fs::{self, read_dir, remove_file};

use std::io::prelude::Write;

use std::{env, fs::File, path::PathBuf};

use crate::coffee_image::convert::image_wrap::ImageConverter;
use crate::coffee_image::error::Error;
use crate::coffee_image::save_format::{SaveFormat, self};

const RESULT_FOLDER_NAME: &str = ".resultImages";
const FILTER_NAME:&str ="image";

pub async fn image_open() -> Result<PathBuf, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("画像パスを教えてください")
        .add_filter(FILTER_NAME, &save_format::SAVEFORMATS)
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    Ok(handle.path().to_owned())
}
pub async fn save(
    path: Option<PathBuf>,
    temp_image_path: ImageConverter,
    save_format: SaveFormat,
) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("保存場所")
            .add_filter(FILTER_NAME, &[save_format.to_string()])
            .save_file()
            .await
            .ok_or(Error::DialogClosed)
            .map(|handle| handle.path().to_owned())?
    };

    temp_image_path.save_converted_image(&path, save_format);
    Ok(path)
}

pub fn get_result_folder() -> Result<PathBuf, Error> {
    let mut result_folder_path = env::current_dir()
        .map_err(|error| error.kind())
        .map_err(Error::IOFailed)?;
    result_folder_path.push(RESULT_FOLDER_NAME);

    Ok(result_folder_path)
}

pub fn remove_all_temp_file() {
    let temp_dir = get_result_folder().unwrap();
    let read_dir = read_dir(temp_dir);
    let dir = read_dir.unwrap();

    for dir_entry in dir.into_iter() {
        let file_path = dir_entry.unwrap().path();
        let _ = remove_file(file_path).map_err(|error| {
        });
    }
}

pub fn mkdir_result_temp_folder() -> Result<(), Error> {
    let result_temp_dir = get_result_folder()?;

    if let Ok(metadata) = fs::metadata(&result_temp_dir) {
        if metadata.is_dir() {
            //fallthrough
        } else {
            let _ = fs::create_dir(result_temp_dir)
                .map_err(|error| error.kind())
                .map_err(Error::IOFailed)?;
        }
    } else {
        let _ = fs::create_dir(result_temp_dir)
            .map_err(|error| error.kind())
            .map_err(Error::IOFailed)?;
    }
    Ok(())
}
