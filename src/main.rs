use std::path::PathBuf;

use coffee_image::{coffee_image_io, error::Error, convert::image_wrap::gray_scale};
use iced::{
    executor,
    widget::{button, column, container, Image, row},
    Application, Command, Length, Settings, Theme,
};


mod coffee_image;

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
}

#[derive(Debug, Clone)]
enum Message {
    Open,
    ImageOpened(Result<PathBuf, Error>),
    Convert,
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
            Message::Convert => {
                let _ =gray_scale(self.image_path.clone().unwrap());
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let open_button = button("Open").on_press(Message::Open);
        let convert_button = button("Convert").on_press(Message::Convert);

        let controlls = row![
            open_button,
            convert_button
        ].padding(10);

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
