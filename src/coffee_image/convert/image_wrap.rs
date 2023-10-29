use iced::widget::{text_input, Image};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use std::{path::PathBuf};
                          
use crate::coffee_image::{
    coffee_image_io::get_result_folder, error::Error, rng::generate_strings,
};

//https://docs.rs/image/latest/image/
#[derive(Debug, Clone)]
pub struct ImageConverter {
    converted_image: Option<DynamicImage>,
    temp_converted_image_path: Option<PathBuf>,
}
type StdError = Box<dyn std::error::Error>;
impl ImageConverter {
    pub fn new() -> Self {
        Self {
            converted_image: None,
            temp_converted_image_path: None,
        }
    }
    pub fn gray_scale(mut self, path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let image = ImageReader::open(path)?.decode()?;

        let gray_image = image.grayscale();
        self.temp_converted_image_path = save_temp_converted_image(&gray_image).ok();
        self.converted_image = Some(gray_image);

        Ok(self)
    }

    pub fn bitwise_not(mut self, path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut image = ImageReader::open(path)?.decode()?;

        image.invert();

        self.temp_converted_image_path = save_temp_converted_image(&image).ok();
        self.converted_image = Some(image);

        Ok(self)
    }

    pub fn hue_rotate(mut self, path: PathBuf, rotate_value: i32) -> Result<Self, StdError> {
        let image = ImageReader::open(path)?.decode()?;
        let rotate_image = image.huerotate(rotate_value);

        self.temp_converted_image_path = save_temp_converted_image(&rotate_image).ok();
        self.converted_image = Some(rotate_image);

        Ok(self)
    }

    pub fn ascii_art(self, path: PathBuf, scale: u32) -> Result<(), StdError> {
        let image = self.get_dynamic_image(path)?;
        let (width, height) = image.dimensions();

        for y in 0..height {
            for x in 0..width {
                if y % (scale * 2) == 0 && x % scale == 0 {
                    let pixel = image.get_pixel(x, y);
                    let mut intent = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
                    if pixel[3] == 0 {
                        intent = 0;
                    }
                    print!("{}", get_str_ascii(intent));
                }
            }
            if y % (scale * 2) == 0 {
                println!("");
            }
        }
        Ok(())
    }

    pub fn save_converted_image(self, path: &PathBuf) {
        self.converted_image
            .clone()
            .map(|result_image| result_image.save(path));
    }

    pub fn get_temp_result_path(self) -> Option<PathBuf> {
        self.temp_converted_image_path
    }

    fn get_dynamic_image(self, path: PathBuf) -> Result<DynamicImage, StdError> {
        let image = ImageReader::open(path)?.decode()?;
        Ok(image)
    }
}
//https://www.youtube.com/watch?v=t4DmszQfD-Q
fn save_temp_converted_image(temp_image: &DynamicImage) -> Result<PathBuf, Error> {
    let file_name = format!("{}.jpg", generate_strings());

    let temp_image_path = get_result_folder().map(|mut path| {
        path.push(file_name);
        path
    })?;

    temp_image.save(temp_image_path.clone());

    Ok(temp_image_path)
}

fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 32;
    let ascii = ["", ".", ",", "-", "~", "+", "=", "@"];

    return ascii[index as usize];
}

