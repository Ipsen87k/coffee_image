use std::path::PathBuf;

use super::error::Error;



pub async fn image_open() -> Result<PathBuf,Error>{
    let handle = rfd::AsyncFileDialog::new()
        .set_title("画像パスを教えてください")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    Ok(handle.path().to_owned())
}