use iced::widget::Image;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use std::path::PathBuf;
use std::io::prelude::Write;
                          
use crate::coffee_image::io::coffee_image_io::get_result_folder;
use crate::coffee_image::io::text::TextFile;
use crate::coffee_image::string_art::ascii::get_byte_ascii;
use crate::coffee_image::{
    error::Error, rng::generate_strings
};

//https://docs.rs/image/latest/image/
#[derive(Debug, Clone)]
pub struct ImageConverter {
    converted_image: Option<DynamicImage>,
    temp_converted_image_path: Option<PathBuf>,
}
type StdError = Box<dyn std::error::Error>;

//Convert Methods
impl ImageConverter {

    pub fn gray_scale(mut self, path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let image = ImageReader::open(path)?.decode()?;

        let gray_image = image.grayscale();

        Ok(self.save_temp_result_image(gray_image))
    }

    pub fn blur(&mut self,path:&PathBuf,blur_value:f32) ->Result<Self,StdError>{
        let image = self.get_dynamic_image(path)?;

        let bulred_image= image.blur(blur_value);

        Ok(self.save_temp_result_image(bulred_image))
    }

    pub fn bitwise_not(&mut self, path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut image = ImageReader::open(path)?.decode()?;

        image.invert();

        Ok(self.save_temp_result_image(image))
    }

    pub fn hue_rotate(&mut self, path: PathBuf, rotate_value: i32) -> Result<Self, StdError> {
        let image = ImageReader::open(path)?.decode()?;
        let rotate_image = image.huerotate(rotate_value);

        Ok(self.save_temp_result_image(rotate_image))
    }

    pub fn ascii_art(self, path: &PathBuf, scale: u32) -> Result<TextFile, StdError> {
        let image = self.get_dynamic_image(path)?;
        let (width, height) = image.dimensions();

        let (text_file,mut output) = TextFile::new();

        for y in 0..height {
            for x in 0..width {
                if y % (scale * 2) == 0 && x % scale == 0 {
                    let pixel = image.get_pixel(x, y);
                    let mut intent = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
                    if pixel[3] == 0 {
                        intent = 0;
                    }
                    output.write(get_byte_ascii(intent));
                }
            }
            if y % (scale * 2) == 0 {
                output.write(b"\n");
            }
        }
        Ok(text_file)
    }


}
//https://www.youtube.com/watch?v=t4DmszQfD-Q
//汎用的なメッソド
impl ImageConverter{
    pub fn new() -> Self {
        Self {
            converted_image: None,
            temp_converted_image_path: None,
        }
    }
    pub fn save_converted_image(&self, path: &PathBuf) {
        self.converted_image
            .clone()
            .map(|result_image| result_image.save(path));
    }

    pub fn get_temp_result_path(self) -> Option<PathBuf> {
        self.temp_converted_image_path
    }

    fn get_dynamic_image(&self, path: &PathBuf) -> Result<DynamicImage, StdError> {
        let image = ImageReader::open(path)?.decode()?;
        Ok(image)
    }
    fn save_temp_result_image(&mut self,temp_image: DynamicImage) -> Self{
        let file_name = format!("{}.jpg",generate_strings());

        let temp_image_path = get_result_folder().map(|mut path| {
            path.push(file_name);
            path
        }).ok();

        let _ =&temp_image.save(temp_image_path.as_ref().unwrap());

        self.temp_converted_image_path = temp_image_path;
        self.converted_image = Some(temp_image);

        self.clone()
    }
}