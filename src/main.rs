use std::path::PathBuf;

use coffee_image::{
    coffee_image_io::{self, save},
    convert::image_wrap::ImageConverter,
    error::Error,
};

use iced::{
    executor,
    widget::{button, column, container, horizontal_space, pick_list, row, Image},
    Application, Command, Length, Settings, Theme,
};
use select_mode::SelectMode;

mod coffee_image;
mod components;
mod select_mode;
//https://github.com/iced-rs/iced
//https://docs.rs/iced/latest/iced/
//https://zenn.dev/tris/articles/e60efe7c60a770
fn main() -> iced::Result {
    ImageState::run(Settings::default())
}

#[derive(Debug, Clone)]
struct ImageState {
    image_path: Option<PathBuf>,
    error: Option<Error>,
    image_converter: ImageConverter,
    mode: SelectMode,
}

#[derive(Debug, Clone)]
pub enum Message {
    Open,
    ImageOpened(Result<PathBuf, Error>),
    Save,
    ImageSaved(Result<PathBuf, Error>),
    Convert,
    Selected(SelectMode),
}

impl Application for ImageState {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Message>) {
        (
            Self {
                image_path: None,
                error: None,
                image_converter: ImageConverter::new(),
                mode: SelectMode::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "coffee-image".to_string()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Open => Command::perform(coffee_image_io::image_open(), Message::ImageOpened),
            Message::ImageOpened(Ok(path)) => {
                self.image_path = Some(path);

                Command::none()
            }
            Message::ImageOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::Save => Command::perform(
                save(None, self.image_converter.clone()),
                Message::ImageSaved,
            ),
            Message::ImageSaved(Ok(path)) => Command::none(),
            Message::ImageSaved(Err(error)) => Command::none(),
            Message::Convert => {
                match self.mode {
                    SelectMode::BitwiseNot => {
                        self.image_converter = self
                            .image_converter
                            .clone()
                            .bitwise_not(self.image_path.clone().unwrap())
                            .unwrap_or(ImageConverter::new());
                    }
                    SelectMode::Gray => {
                        self.image_converter = self
                            .image_converter
                            .clone()
                            .gray_scale(self.image_path.clone().unwrap())
                            .unwrap_or(ImageConverter::new());
                    }
                }

                self.image_path = self.image_converter.clone().get_temp_result_path();
                Command::none()
            }
            Message::Selected(mode) => {
                self.mode = mode;
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let open_button = button("Open").on_press(Message::Open);
        let convert_button = button("Convert").on_press(Message::Convert);
        let save_button = button("Save").on_press(Message::Save);

        let select_mode_pick_list =
            pick_list(&SelectMode::ALL[..], Some(self.mode), Message::Selected);
        let controlls = row![
            open_button,
            save_button,
            convert_button,
            horizontal_space(Length::Fill),
            select_mode_pick_list
        ]
        .padding(10);

        let image_path = self.image_path.clone().unwrap_or(PathBuf::from(""));

        let image = container(
            Image::new(image_path)
                .width(Length::Fill)
                .height(Length::Fill),
        );

        container(column!(controlls, image))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
