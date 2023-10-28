use std::path::PathBuf;

use coffee_image::{
    coffee_image_io::{self, save},
    convert::image_wrap::ImageConverter,
    error::Error,
};

use iced::{
    executor,
    theme::TextInput,
    widget::{button, column, container, horizontal_space, pick_list, row, text_input, Image},
    Application, Command, Length, Settings, Theme, Renderer,
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
    input_value: String,
    angle_value: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Open,
    ImageOpened(Result<PathBuf, Error>),
    Save,
    ImageSaved(Result<PathBuf, Error>),
    Convert,
    Selected(SelectMode),
    InputChanged(String),
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
                input_value: "please input angle value".to_string(),
                angle_value: 0,
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
                    SelectMode::Rotate => {
                        self.image_converter = self
                            .image_converter
                            .clone()
                            .rotate(self.image_path.clone().unwrap(), self.angle_value)
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
            Message::InputChanged(value) => {
                if &self.input_value == "please input angle value"{
                    self.input_value = String::new();
                }

                let angle_value = &value.parse::<i32>();
                match angle_value {
                    Ok(_angle_value) => {
                        self.input_value.push_str(&value);
                        self.angle_value = self.input_value.clone().parse::<i32>().unwrap();
                        println!("{}",self.angle_value);
                    }
                    Err(e) => {
                    println!("{}",e);
                    }
                }
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

        if self.mode == SelectMode::Rotate {
            let input_angle_text =
                text_input(&self.input_value,"").on_input(Message::InputChanged);
            return container(column!(controlls, input_angle_text, image))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10)
                .into();
        }
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
