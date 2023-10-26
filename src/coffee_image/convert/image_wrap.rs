use std::path::PathBuf;
use image::{io::Reader as ImageReader, GenericImageView};

//https://www.youtube.com/watch?v=t4DmszQfD-Q
pub fn gray_scale(path:PathBuf) -> Result<(),Box<dyn std::error::Error>> {
    let image = ImageReader::open(path)?.decode()?;

    let (width,height) = image.dimensions();
    println!("{width},{height}");
    
    Ok(())
}
