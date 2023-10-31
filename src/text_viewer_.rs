use iced::{
    widget::{container, text},
    Length,
};
use std::io::prelude::Read;
use std::path::PathBuf;

use crate::{coffee_image::error::Error, Message};

#[derive(Debug, Clone)]
pub struct TextViewerState {
    content_or_error: Result<String, Error>,
    text_path: Option<PathBuf>,
}

impl TextViewerState {
    pub fn new(path: PathBuf) -> Self {
        let result = read_text_file(path.clone());
        Self {
            content_or_error: result,
            text_path: Some(path),
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let content_or_error_text = match &self.content_or_error {
            Ok(content) => text(content).size(8),
            Err(error) => {
                let error = format!("{:?}", error);
                text(error)
            }
        };

        container(content_or_error_text)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn read_text_file(path: PathBuf) -> Result<String, Error> {
    let mut content = String::new();
    let _ = std::fs::File::open(path)
        .map_err(|error| error.kind())
        .map_err(Error::IOFailed)?
        .read_to_string(&mut content);

    Ok(content)
}

