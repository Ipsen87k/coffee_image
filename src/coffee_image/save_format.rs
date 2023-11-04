use std::fmt::Display;

use image::ImageFormat;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SaveFormat{
    #[default]
    Png,
    Jpeg,
}

impl SaveFormat{
    pub const ALL:[SaveFormat;2] = [SaveFormat::Png,SaveFormat::Jpeg];

    pub fn convert_to_imageformat(&self)->ImageFormat{
        match self {
            SaveFormat::Png => ImageFormat::Png,
            SaveFormat::Jpeg => ImageFormat::Jpeg,
        }
    }
}

impl Display for SaveFormat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",match self {
            SaveFormat::Png=>"png",
            SaveFormat::Jpeg=>"jpeg",

        })
    }
}
