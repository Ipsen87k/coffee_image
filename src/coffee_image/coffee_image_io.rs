use std::{path::PathBuf, env, fs::File};
use std::io::prelude::{BufRead,Write};
use super::{error::Error, convert::image_wrap::ImageConverter, rng::generate_strings, };

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


pub struct TextFile{
    file:File,
}

impl TextFile {
    pub fn open_text() -> Self {
        let temp_text_file_name = format!("{}.txt",generate_strings());
    
        let path = get_result_folder().map(|mut save_path|{
            save_path.push(temp_text_file_name);
            save_path
        });

        let text_file = TextFile{file:match File::create(path.unwrap()) {
            Ok(file)=> file,
            Err(error) => panic!("{}",error),
        }};
    
        text_file
    }
    
    pub fn write_char(&mut self,output_char:&str){
        self.file.write(output_char.as_bytes());
    }
    
    pub fn close_file(self){
        todo!()
    }
}

impl Drop for TextFile{
    fn drop(&mut self) {
      println!("Text File は閉じられました")  
    }
}