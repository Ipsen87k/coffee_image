use iced::{
    widget::{container, text, button, row, column},
    Length,
};

use std::path::PathBuf;

use crate::{coffee_image::{error::Error, io::text::TextFile}, Message};

#[derive(Debug, Clone)]
pub struct TextViewerState {
    content_or_error: Result<String, Error>,
    text_path: Option<PathBuf>,
}

impl TextViewerState {
    pub fn new(text_file:TextFile) -> Self {
        let result = text_file.read_text_file();
        Self {
            content_or_error: result,
            text_path: Some(text_file.get_result_text_file()),
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let change_view_button=button("Return Main").on_press(Message::ViewChanged(crate::Views::Image));
        let controll = row![change_view_button];
        
        let content_or_error_text = match &self.content_or_error {
            Ok(content) => text(content).size(8),
            Err(error) => {
                let error = format!("{:?}", error);
                text(error)
            }
        };

        container(column![controll,content_or_error_text])
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

