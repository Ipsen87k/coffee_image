use std::{io, num::{ParseFloatError}};

use super::io::dialog::error_dialog_show;

//use image::ImageError;


#[derive(Debug,Clone)]
pub enum Error{
    DialogClosed,
    IOFailed(io::ErrorKind),
    ParseError(ParseFloatError),
    ImageError(String),
}
impl Into<String> for Error{
    fn into(self) -> String {
        match self {
            Error::DialogClosed => "DialogClosed".to_string(),
            Error::IOFailed(kind) => kind.to_string(),
            Error::ParseError(_kind) => "数値を入力してください".to_string(),
            Error::ImageError(kind) => kind,
        }
    }
}
impl Error{
    pub fn show_dialog_return_default<T:Default>(&self) ->T{
        error_dialog_show(self.clone());
        T::default()
    }
}