use std::{io, num::ParseIntError};

use image::ImageError;


#[derive(Debug,Clone)]
pub enum Error{
    DialogClosed,
    IOFailed(io::ErrorKind),
    ParseError(ParseIntError),

}