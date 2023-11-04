use std::{path::PathBuf, io::BufWriter, fs::File};
use std::io::prelude::Read;

use crate::coffee_image::{rng::generate_strings, error::Error};

use super::coffee_image_io::get_result_folder;

#[derive(Debug)]
pub struct TextFile {
    save_temp_path: PathBuf,
}

impl TextFile {
    pub fn new() ->(Self,BufWriter<File>)   {
        let temp_text_file_name = format!("{}.txt", generate_strings());

        let path = get_result_folder().map(|mut save_path| {
            save_path.push(temp_text_file_name);
            save_path
        });
        (
            Self{
                save_temp_path:path.clone().unwrap(),
            },
            BufWriter::new(File::create(path.unwrap()).unwrap()),
        )
    }

    pub fn get_result_text_file(&self)->PathBuf{
        self.save_temp_path.clone()
    }
    
    pub fn read_text_file(&self) -> Result<String, Error> {
        let mut content = String::new();
        let _ = File::open(&self.save_temp_path)
            .map_err(|error| error.kind())
            .map_err(Error::IOFailed)?
            .read_to_string(&mut content);

        Ok(content)
    }

}

impl Drop for TextFile {
    fn drop(&mut self) {
        println!("Text File は閉じられました")
    }
}
