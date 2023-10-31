use super::{convert::image_wrap::ImageConverter, error::Error, rng::generate_strings};
use std::fs::{read_dir, remove_file};
use std::io::BufWriter;
use std::io::prelude::{Read, Write};
use std::{env, fs::File, path::PathBuf};

const RESULT_FOLDER_NAME: &str = ".resultImages";

pub async fn image_open() -> Result<PathBuf, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("画像パスを教えてください")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    Ok(handle.path().to_owned())
}
pub async fn save(path: Option<PathBuf>, image: ImageConverter) -> Result<PathBuf, Error> {
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

pub fn get_result_folder() -> Result<PathBuf, Error> {
    let mut result_folder_path = env::current_dir()
        .map_err(|error| error.kind())
        .map_err(Error::IOFailed)?;
    result_folder_path.push(RESULT_FOLDER_NAME);

    Ok(result_folder_path)
}

pub fn remove_all_temp_file(){
    let mut error_output = File::create("error.log").unwrap();
    let temp_dir = get_result_folder().unwrap();
    let read_dir = read_dir(temp_dir);
    let dir = read_dir.unwrap();

    for dir_entry in dir.into_iter(){
        let file_path = dir_entry.unwrap().path();
        let _ =remove_file(file_path).map_err(|error|{
            error_output.write(error.to_string().as_bytes());
        });

    }
    
}

#[derive(Debug)]
pub struct TextFile {
    // file: File,
    save_temp_path: PathBuf,
}

impl TextFile {
    pub fn new() ->(BufWriter<File>,PathBuf)   {
        let temp_text_file_name = format!("{}.txt", generate_strings());

        let path = get_result_folder().map(|mut save_path| {
            save_path.push(temp_text_file_name);
            save_path
        });
        (
            BufWriter::new(File::create(path.clone().unwrap()).unwrap()),
            path.unwrap()
        )
    }
    // pub fn open_text() -> Self {
    //     let temp_text_file_name = format!("{}.txt", generate_strings());

    //     let path = get_result_folder().map(|mut save_path| {
    //         save_path.push(temp_text_file_name);
    //         save_path
    //     });

    //     let text_file = TextFile {
    //         file: match File::create(path.clone().unwrap()) {
    //             Ok(file) => file,
    //             Err(error) => panic!("{}", error),
    //         },
    //         save_temp_path: path.unwrap(),
    //     };

    //     text_file
    // }

    pub fn read_text_file(&self) -> Result<String, Error> {
        let mut content = String::new();
        let _ = File::open(&self.save_temp_path)
            .map_err(|error| error.kind())
            .map_err(Error::IOFailed)?
            .read_to_string(&mut content);

        Ok(content)
    }

    // pub fn write_char(&mut self,output_char: &str) {
    //     self.file.write(output_char.as_bytes());
    // }
}

impl Drop for TextFile {
    fn drop(&mut self) {
        println!("Text File は閉じられました")
    }
}
