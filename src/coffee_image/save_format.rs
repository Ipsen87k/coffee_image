use std::fmt::Display;

use image::ImageFormat;
pub const SAVEFORMATS:[&str;2] = ["png","jpg"];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SaveFormat {
    #[default]
    Png,
    Jpeg,
    // Gif,
    // Avif,
}

impl SaveFormat {
    pub const ALL: [SaveFormat; 2] = [
        SaveFormat::Png,
        SaveFormat::Jpeg,
        // SaveFormat::Gif,
        // SaveFormat::Avif,
    ];

    pub fn convert_to_imageformat(&self) -> ImageFormat {
        match self {
            SaveFormat::Png => ImageFormat::Png,
            SaveFormat::Jpeg => ImageFormat::Jpeg,
            // SaveFormat::Gif => ImageFormat::Gif,
            // SaveFormat::Avif => ImageFormat::Avif,
        }
    }
}

impl Display for SaveFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SaveFormat::Png => "png",
                SaveFormat::Jpeg => "jpeg",
                // SaveFormat::Gif => "gif",
                // SaveFormat::Avif => "avif",
            }
        )
    }
}
