use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use image::{GenericImage, Rgba};
use std::io::prelude::Write;

use std::path::PathBuf;

use crate::coffee_image::io::coffee_image_io::get_result_folder;
use crate::coffee_image::io::text::TextFile;
use crate::coffee_image::save_format::SaveFormat;
use crate::coffee_image::string_art::ascii::get_byte_ascii;
use crate::coffee_image::{error::Error, rng::generate_strings};

//https://docs.rs/image/latest/image/
#[derive(Debug, Clone, Default)]
pub struct ImageConverter {
    temp_converted_image_path: Option<PathBuf>,
    orgin_image_path: PathBuf,
    pub save_format: SaveFormat,
    mask:Option<DynamicImage>,
}

#[allow(dead_code)]
type StdError = Box<dyn std::error::Error>;

//Convert Methods
impl ImageConverter {
    pub fn gray_scale(&mut self) -> Result<DynamicImage, Error> {
        let image = get_dynamic_image(&self.orgin_image_path)?;

        let gray_image = image.grayscale();

        Ok(gray_image)
    }

    pub fn blur(&mut self, blur_value: f32) -> Result<DynamicImage, Error> {
        let image = get_dynamic_image(&self.orgin_image_path)?;

        let bulred_image = image.blur(blur_value);

        Ok(bulred_image)
    }

    pub async fn async_blur(&mut self, blur_value: f32) -> Result<(), Error> {
        let image = get_dynamic_image(&self.orgin_image_path)?;

        let bulred_image = image.blur(blur_value);

        Ok(())
    }

    pub fn bitwise_not(&mut self) -> Result<DynamicImage, Error> {
        let mut image = get_dynamic_image(&self.orgin_image_path)?;

        image.invert();

        Ok(image)
    }

    pub fn hue_rotate(&mut self, rotate_value: i32) -> Result<DynamicImage, Error> {
        let image = get_dynamic_image(&self.orgin_image_path)?;
        let rotate_image = image.huerotate(rotate_value);

        Ok(rotate_image)
    }

    pub fn ascii_art(self, scale: u32) -> Result<TextFile, Error> {
        let image = get_dynamic_image(&self.orgin_image_path)?;
        let (width, height) = image.dimensions();

        let (text_file, mut output) = TextFile::new();

        for y in 0..height {
            for x in 0..width {
                if y % (scale * 2) == 0 && x % scale == 0 {
                    let pixel = image.get_pixel(x, y);
                    let mut intent = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
                    if pixel[3] == 0 {
                        intent = 0;
                    }
                    let _ = output.write(get_byte_ascii(intent));
                }
            }
            if y % (scale * 2) == 0 {
                let _ = output.write(b"\n");
            }
        }
        Ok(text_file)
    }
    //https://qiita.com/yaju/items/680086b39bec5db93366
    pub fn rotate(&mut self, angle: f32) -> Result<DynamicImage, Error> {
        let radian = angle.to_radians();
        let (sin, cos) = radian.sin_cos();

        let image = get_dynamic_image(&self.orgin_image_path)?;
        let (width, height) = image.dimensions();

        let new_width = (cos.abs() * width as f32 + sin.abs() * height as f32).abs() as u32;
        let new_height = (sin.abs() * width as f32 + cos.abs() * height as f32).abs() as u32;

        let mut rotated_image = DynamicImage::new_rgba8(new_width, new_height);

        let new_width_center = new_width as f32 / 2.0;
        let new_height_center = new_height as f32 / 2.0;
        let orgin_width_center = width as f32 / 2.0;
        let orgin_height_center = height as f32 / 2.0;

        for y in 0..new_height {
            for x in 0..new_width {
                let orgin_x = (cos * (x as f32 - new_width_center)
                    - sin * (y as f32 - new_height_center)
                    + orgin_width_center)
                    .round() as i32;
                let origin_y = (sin * (x as f32 - new_width_center)
                    + cos * (y as f32 - new_height_center)
                    + orgin_height_center)
                    .round() as i32;

                if orgin_x >= 0
                    && orgin_x < width as i32
                    && origin_y >= 0
                    && origin_y < height as i32
                {
                    let pixel = image.get_pixel(orgin_x as u32, origin_y as u32);
                    rotated_image.put_pixel(x, y, Rgba(pixel.0));
                }
            }
        }

        Ok(rotated_image)
    }
    pub fn add_images(&mut self, image_path2: &PathBuf) -> Result<DynamicImage, Error> {
        let img1 = get_dynamic_image(&self.orgin_image_path)?;
        let img2 = get_dynamic_image(&image_path2)?;
        if !self.is_image_width_height_equal(&img1, &img2) {
            return Err(Error::WidthHeightNotEqualError);
        }
        let (width, height) = img1.dimensions();
        let mut result_image = DynamicImage::new_rgba8(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel1 = img1.get_pixel(x, y);
                let pixel2 = img2.get_pixel(x, y);

                let new_pixel = [
                    pixel1[0].saturating_add(pixel2[0]),
                    pixel1[1].saturating_add(pixel2[1]),
                    pixel1[2].saturating_add(pixel2[2]),
                    pixel1[3].saturating_add(pixel2[3]),
                ];
                result_image.put_pixel(x, y, Rgba(new_pixel));
            }
        }
        Ok(result_image)
    }
    //https://whitewell.sakura.ne.jp/OpenCV/py_tutorials/py_core/py_image_arithmetics/py_image_arithmetics.html
    //TODO 画像の合成
    fn threshold(&mut self) ->Result<DynamicImage,Error>{
        let gray_image=self.gray_scale()?;
        let threshold_value=127;

        let (width,height)=gray_image.dimensions();
        let mut dst_image= self.new_image_create(&gray_image);

        for y in 0..height{
            for x in 0..width{
                let pixel=gray_image.get_pixel(x, y);
                if pixel[0] > threshold_value{
                    dst_image.put_pixel(x, y, Rgba([0,0,0,255]));
                }else{
                    dst_image.put_pixel(x, y, Rgba([255,255,255,255]));
                }
            }
        }

        Ok(dst_image)
    }

    fn create_mask(&mut self) ->Result<DynamicImage,Error>{
        let mut thresholded_image = self.threshold()?;
        thresholded_image.invert();
        self.mask=Some(thresholded_image);
        Ok(self.mask.clone().unwrap())
    }

    fn new_image_create(&mut self,image:&DynamicImage) ->DynamicImage{
        let (width,height) = image.dimensions();
        DynamicImage::new_rgb8(width, height)
    }
}
//https://www.youtube.com/watch?v=t4DmszQfD-Q
//汎用的なメッソド
impl ImageConverter {
    pub fn new() -> Self {
        Self {
            temp_converted_image_path: None,
            orgin_image_path: PathBuf::from(""),
            save_format: SaveFormat::Png,
            mask:None,
        }
    }
    pub fn save_converted_image(&self, path: &PathBuf, save_format: SaveFormat) {
        let result_image = get_dynamic_image(self.temp_converted_image_path.as_ref().unwrap());
        let _ = result_image.map(|result_image| {
            result_image.save_with_format(path, save_format.convert_to_imageformat())
        });
    }

    pub fn get_temp_result_path(&self) -> Option<PathBuf> {
        self.temp_converted_image_path.clone()
    }

    pub fn set_image_path(&mut self, image_path: PathBuf) {
        self.orgin_image_path = image_path
    }

    pub fn is_result_temp_path(&self) -> bool {
        self.temp_converted_image_path.is_some()
    }

    pub fn save_temp_result_image(&mut self, temp_image: DynamicImage) {
        let file_name = format!("{}", generate_strings());

        let temp_image_path = get_result_folder()
            .map(|mut path| {
                path.push(file_name + "." + self.save_format.to_string().as_ref());
                path
            })
            .ok();

        let _ = &temp_image.save_with_format(
            temp_image_path.as_ref().unwrap(),
            self.save_format.convert_to_imageformat(),
        );

        self.temp_converted_image_path = temp_image_path;
    }

    fn is_image_width_height_equal(&self, image1: &DynamicImage, image2: &DynamicImage) -> bool {
        let (width1, height1) = image1.dimensions();
        let (width2, height2) = image2.dimensions();

        width1 == width2 && height1 == height2
    }
}
pub fn get_dynamic_image(path: &PathBuf) -> Result<DynamicImage, Error> {
    let image = ImageReader::open(path)
        .map_err(|error| error.kind())
        .map_err(Error::IOFailed)?
        .decode()
        .map_err(|error| error.to_string())
        .map_err(Error::ImageError)?;
    Ok(image)
}
pub fn async_blur(path: Option<PathBuf>, blur_value: f32) -> Result<PathBuf, Error> {
    let image = get_dynamic_image(path.as_ref().unwrap())?;

    let blured_image = image.blur(blur_value);

    let file_name = format!("{}.jpg", generate_strings());

    let temp_image_path = get_result_folder()
        .map(|mut path| {
            path.push(file_name);
            path
        })
        .ok();

    let _ = &blured_image.save(temp_image_path.as_ref().unwrap());

    Ok(temp_image_path.unwrap())
}

#[cfg(test)]
mod test{
    use super::*;
    fn create_imageconverter_helper() ->ImageConverter{
        let mut ic = ImageConverter::new();
        ic.set_image_path(PathBuf::from("examplesImages/people.jpg"));
        ic
    }
    #[test]
    fn threshold_test(){
        let mut ic = ImageConverter::new();
        ic.set_image_path(PathBuf::from("examplesImages/people.jpg"));
        let mask=ic.threshold().unwrap();
        let _=mask.save("test.jpg");
    }

    #[test]
    fn create_mask_test(){
        let mut ic = create_imageconverter_helper();
        let mask = ic.create_mask().unwrap();
        let _=mask.save("cm.jpg");
    }
}
