use coffee_image::{
    convert::image_wrap::ImageConverter,
    error::Error,
    io::{
        coffee_image_io::{self, mkdir_result_temp_folder, remove_all_temp_file, save},
        dialog::error_dialog_show,
    },
    save_format::{self, SaveFormat},
};

use std::path::PathBuf;

use iced::{
    executor,
    widget::{button, column, container, horizontal_space, pick_list, row, text_input, Image},
    Application, Command, Element, Length, Settings, Theme,
};
use select_mode::SelectMode;
use text_viewer_::TextViewerState;

mod coffee_image;
mod components;
mod select_mode;
mod text_viewer_;
//https://github.com/iced-rs/iced
//https://docs.rs/iced/latest/iced/
//https://zenn.dev/tris/articles/e60efe7c60a770
fn main() -> iced::Result {
    init();
    ImageState::run(Settings::default())
}

#[derive(Debug, Clone)]
struct ImageState {
    image_path: Option<PathBuf>,
    error: Option<Error>,
    image_converter: ImageConverter,
    mode: SelectMode,
    save_format: SaveFormat,
    input_value: String,
    angle_value: i32,
    view_state: ViewState,
}
#[derive(Debug, Clone)]
struct ViewState {
    current_view: Views,
    text_view: Option<TextViewerState>,
}
#[derive(Debug, Clone)]
pub enum Views {
    Image,
    Text,
}
#[derive(Debug, Clone)]
pub enum Message {
    Open,
    ImageOpened(Result<PathBuf, Error>),
    Save,
    ImageSaved(Result<PathBuf, Error>),
    Convert,
    GrayConverted(Result<ImageConverter, Error>),
    Selected(SelectMode),
    InputChanged(String),
    ViewChanged(Views),
    SaveFormatSelected(SaveFormat),
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
                save_format: SaveFormat::default(),
                input_value: "please input angle value".to_string(),
                angle_value: 0,
                view_state: ViewState {
                    current_view: Views::Image,
                    text_view: None,
                },
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
                self.error = Some(error.clone());
                error_dialog_show(error);
                Command::none()
            }
            Message::Save => Command::perform(
                save(None, self.image_converter.clone(), self.save_format.clone()),
                Message::ImageSaved,
            ),
            Message::ImageSaved(Ok(path)) => Command::none(),
            Message::ImageSaved(Err(error)) => {
                self.error = Some(error.clone());
                error_dialog_show(error);
                Command::none()
            }
            Message::Convert => {
                let mut converter = self.image_converter.clone();
                match self.mode {
                    SelectMode::BitwiseNot => {
                        self.image_converter = converter
                            .bitwise_not(self.image_path.clone().unwrap())
                            .unwrap_or_else(|error| error.show_dialog_return_default());
                    }
                    SelectMode::Gray => {
                        self.image_converter = converter
                            .gray_scale(self.image_path.clone().unwrap())
                            .unwrap_or_else(|error| error.show_dialog_return_default());
                    }
                    SelectMode::HueRotate => {
                        self.image_converter = converter
                            .hue_rotate(self.image_path.clone().unwrap(), self.angle_value)
                            .unwrap_or_else(|error| error.show_dialog_return_default());
                    }
                    SelectMode::Blur => {
                        self.image_converter = converter
                            .blur(self.image_path.as_ref().unwrap(), 34.3)
                            .unwrap_or_else(|error| error.show_dialog_return_default());
                    }
                    SelectMode::ToAscii => {
                        let path = self
                            .image_converter
                            .clone()
                            .ascii_art(self.image_path.as_ref().unwrap(), 4);

                        self.view_state.text_view = Some(TextViewerState::new(
                            path.unwrap_or_else(|error| error.show_dialog_return_default()),
                        ));
                        self.view_state.current_view = Views::Text;
                    }
                }

                self.image_path = self.image_converter.clone().get_temp_result_path();
                Command::none()
            }
            Message::GrayConverted(result) => {
                match result {
                    Ok(image_converter) => self.image_converter = image_converter,
                    Err(error) => self.error = Some(error),
                }

                Command::none()
            }
            Message::Selected(mode) => {
                self.mode = mode;
                Command::none()
            }
            Message::SaveFormatSelected(save_format) => {
                self.save_format = save_format;
                Command::none()
            }
            Message::InputChanged(value) => {
                if &self.input_value == "please input angle value" {
                    self.input_value = String::new();
                }

                let angle_value = &value.parse::<i32>();
                match angle_value {
                    Ok(_angle_value) => {
                        self.input_value.push_str(&value);
                        self.angle_value = self.input_value.clone().parse::<i32>().unwrap();
                        println!("{}", self.angle_value);
                    }
                    Err(e) => {
                        //error_dialog_show(Some(e.to_owned()));
                    }
                }
                Command::none()
            }
            Message::ViewChanged(views) => {
                self.view_state.current_view = views;
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
        let save_format_list = pick_list(
            &SaveFormat::ALL[..],
            Some(self.save_format),
            Message::SaveFormatSelected,
        );

        let controlls = row![
            open_button,
            save_button,
            convert_button,
            horizontal_space(Length::Fill),
            save_format_list,
            select_mode_pick_list
        ]
        .padding(10);

        let image_path = self.image_path.clone().unwrap_or(PathBuf::from(""));

        let image = container(
            Image::new(image_path)
                .width(Length::Fill)
                .height(Length::Fill),
        );

        if self.mode == SelectMode::HueRotate {
            let input_angle_text =
                text_input(&self.input_value, "").on_input(Message::InputChanged);
            return container(column!(controlls, input_angle_text, image))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10)
                .into();
        }
        let image_view = container(column!(controlls, image))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into();

        match self.view_state.current_view {
            Views::Image => image_view,
            Views::Text => self.view_state.text_view.as_ref().unwrap().view(),
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

impl Drop for ImageState {
    fn drop(&mut self) {
        remove_all_temp_file();
    }
}

fn init() {
    let _ = mkdir_result_temp_folder();
}
