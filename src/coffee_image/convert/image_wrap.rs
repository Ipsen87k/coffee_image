use iced::{ widget::text_input};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use std::path::PathBuf;

use crate::coffee_image::{error::Error, coffee_image_io::get_result_folder, rng::generate_strings};



#[derive(Debug,Clone)]
pub struct ImageConverter {
    converted_image: Option<DynamicImage>,
    temp_converted_image_path:Option<PathBuf>,
}
type std_error = Box<dyn std::error::Error>;
impl ImageConverter {
    pub fn new() -> Self {
        Self {
            converted_image: None,
            temp_converted_image_path:None,
        }
    }
    pub fn gray_scale(mut self, path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let image = ImageReader::open(path)?.decode()?;

        let gray_image = image.grayscale();
        self.temp_converted_image_path=save_temp_converted_image(&gray_image).ok();
        self.converted_image = Some(gray_image);

        Ok(self)
    }

    pub fn bitwise_not(mut self,path:PathBuf) -> Result<Self,Box<dyn std::error::Error>> {
        let mut image = ImageReader::open(path)?.decode()?;
        
        image.invert();

        self.temp_converted_image_path = save_temp_converted_image(&image).ok();
        self.converted_image = Some(image);

        Ok(self)
    }

    pub fn rotate(mut self,path:PathBuf,rotate_value:i32) -> Result<Self,std_error>{
        let image = ImageReader::open(path)?.decode()?;
        let rotate_image = image.huerotate(rotate_value);

        self.temp_converted_image_path = save_temp_converted_image(&rotate_image).ok();
        self.converted_image = Some(rotate_image);

        Ok(self)
    }

    pub fn save_converted_image(self, path: &PathBuf) {
        self.converted_image
            .clone()
            .map(|result_image| result_image.save(path));
    }

    pub fn get_temp_result_path(self) -> Option<PathBuf> {
        self.temp_converted_image_path
    }
}
//https://www.youtube.com/watch?v=t4DmszQfD-Q
fn save_temp_converted_image(temp_image:&DynamicImage) -> Result<PathBuf,Error> {
    let file_name = format!("{}.jpg",generate_strings());

    let temp_image_path = get_result_folder().map(|mut path| {
        path.push(file_name);
        path
    })?;

    temp_image.save(temp_image_path.clone());

    Ok(temp_image_path)
}