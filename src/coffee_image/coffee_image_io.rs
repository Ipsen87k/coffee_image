use std::{path::PathBuf, env};

use super::{error::Error, convert::image_wrap::ImageConverter, };

const RESULT_FOLDER_NAME:&str = ".resultImages";

pub async fn image_open() -> Result<PathBuf,Error>{
    let handle = rfd::AsyncFileDialog::new()
        .set_title("画像パスを教えてください")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    Ok(handle.path().to_owned())
}
pub async fn save(path:Option<PathBuf>,image:ImageConverter,) -> Result<PathBuf,Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("保存場所")
            .save_file()
            .await
            .ok_or(Error::DialogClosed)
            .map(|handle| handle.path().to_owned())?
    };

    image.save_converted_image(&path);
    Ok(path)
}

pub fn get_result_folder() -> Result<PathBuf,Error> {
    let mut result_folder_path = env::current_dir().map_err(|error| error.kind()).map_err(Error::IOFailed)?;
    result_folder_path.push(RESULT_FOLDER_NAME);

    Ok(result_folder_path)
}