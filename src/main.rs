use coffee_image::{
    convert::image_wrap::{get_dynamic_image, ImageConverter},
    error::Error,
    io::{
        coffee_image_io::{self, mkdir_result_temp_folder, remove_all_temp_file, save, image_open},
        dialog::error_dialog_show,
    },
    save_format::{self, SaveFormat},
};

use std::path::PathBuf;

use iced::{
    executor,
    keyboard::{self, KeyCode, Modifiers},
    widget::{button, column, container, horizontal_space, pick_list, row, text_input, Image},
    Application, Command, Event, Length, Settings, Theme,
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
    iamge_path2:Option<PathBuf>,
    error: Option<Error>,
    image_converter: ImageConverter,
    mode: SelectMode,
    input_value: String,
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
    EventOccurred(Event),
    Exit,
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
                iamge_path2:None,
                error: None,
                image_converter: ImageConverter::new(),
                mode: SelectMode::default(),
                input_value: "".to_string(),
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
                if self.mode==SelectMode::Add{
                    self.iamge_path2=Some(path)
                }else{
                    self.image_path = Some(path);
                }
                
                Command::none()
            }
            Message::ImageOpened(Err(error)) => {
                self.error = Some(error.clone());
                error_dialog_show(error);
                Command::none()
            }
            Message::Save => Command::perform(
                save(
                    None,
                    self.image_converter.clone(),
                    self.image_converter.save_format.clone(),
                ),
                Message::ImageSaved,
            ),
            Message::ImageSaved(Ok(_path)) => Command::none(),
            Message::ImageSaved(Err(error)) => {
                self.error = Some(error.clone());
                error_dialog_show(error);
                Command::none()
            }
            Message::Convert => {

                let converted_image = match self.mode {
                    SelectMode::BitwiseNot => self
                        .image_converter
                        .bitwise_not(self.image_path.clone().unwrap()),
                    SelectMode::Gray => self
                        .image_converter
                        .gray_scale(self.image_path.clone().unwrap()),
                    SelectMode::HueRotate => self.image_converter.hue_rotate(
                        self.image_path.clone().unwrap(),
                        self.convert_input_value_to_float() as i32,
                    ),
                    SelectMode::Blur => self.image_converter.blur(
                        self.image_path.as_ref().unwrap(),
                        self.convert_input_value_to_float(),
                    ),
                    SelectMode::Add=>{
                        self.image_converter.add_images(self.image_path.as_ref().unwrap(), self.iamge_path2.as_ref().unwrap())
                    },
                    SelectMode::ToAscii => {
                        let path = self
                            .image_converter
                            .clone()
                            .ascii_art(self.image_path.as_ref().unwrap(), 4);

                        self.view_state.text_view = Some(TextViewerState::new(
                            path.unwrap_or_else(|error| error.show_dialog_return_default()),
                        ));
                        self.view_state.current_view = Views::Text;
                        get_dynamic_image(self.image_path.as_ref().unwrap())
                    }
                    SelectMode::Rotate => self.image_converter.rotate(
                        self.image_path.as_ref().unwrap(),
                        self.convert_input_value_to_float(),
                    ),
                };
                let converted_image =
                    converted_image.unwrap_or_else(|error| error.show_dialog_return_default());
                self.image_converter.save_temp_result_image(converted_image);
                self.image_path = self.image_converter.get_temp_result_path();

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
                if self.mode==SelectMode::Add{
                    Command::perform(image_open(), Message::ImageOpened)
                }else{
                    Command::none()
                }
                
            }
            Message::SaveFormatSelected(save_format) => {
                self.image_converter.save_format = save_format;
                Command::none()
            }
            Message::InputChanged(value) => {
                self.input_value = value;
                Command::none()
            }
            Message::ViewChanged(views) => {
                self.view_state.current_view = views;
                Command::none()
            }
            Message::EventOccurred(event) => {
                match event {
                    Event::Window(window_event) => {
                        if let iced::window::Event::FileDropped(dropped_image_path) = window_event {
                            self.image_path = Some(dropped_image_path)
                        }
                    }
                    Event::Keyboard(key_event) => {
                        if let iced::keyboard::Event::KeyPressed {
                            key_code,
                            modifiers,
                        } = key_event.clone()
                        {
                            if key_code == KeyCode::B && modifiers.command() {
                                println!("keyevnet");
                                return iced::window::close();
                            } else if key_code == KeyCode::S && modifiers.command() {
                                return Command::perform(
                                    save(
                                        None,
                                        self.image_converter.clone(),
                                        self.image_converter.save_format.clone(),
                                    ),
                                    Message::ImageSaved,
                                );
                            } else if key_code == KeyCode::O && modifiers.command() {
                                return Command::perform(
                                    coffee_image_io::image_open(),
                                    Message::ImageOpened,
                                );
                            }
                        }
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::Exit => iced::window::close(),
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let open_button = button("Open").on_press(Message::Open);
        let convert_button = components::button_component(
            "Convert",
            self.image_path.is_some().then_some(Message::Convert),
        );
        let save_button = components::button_component(
            "Save",
            self.image_converter
                .is_result_temp_path()
                .then_some(Message::Save),
        );

        let select_mode_pick_list =
            pick_list(&SelectMode::ALL[..], Some(self.mode), Message::Selected);
        let save_format_list = pick_list(
            &SaveFormat::ALL[..],
            Some(self.image_converter.save_format),
            Message::SaveFormatSelected,
        );
        //TODO リファクタリング
        let controlls =if self.mode==SelectMode::Add{
            let reselect_button=components::button_component("Reselect", Some(Message::Open));
            row![
            open_button,
            save_button,
            convert_button,
            reselect_button,
            horizontal_space(Length::Fill),
            save_format_list,
            select_mode_pick_list
        ]
        }else{
            row![
            open_button,
            save_button,
            convert_button,
            horizontal_space(Length::Fill),
            save_format_list,
            select_mode_pick_list
        ]
        }
        .padding(10);
        
        let image_path = self.image_path.clone().unwrap_or(PathBuf::from(""));

        let image = container(
            Image::new(image_path)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center);

        if self.mode == SelectMode::HueRotate
            || self.mode == SelectMode::Blur
            || self.mode == SelectMode::Rotate
        {
            let input_angle_text =
                text_input("", &self.input_value).on_input(Message::InputChanged);
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

impl ImageState {
    fn convert_input_value_to_float(&self) -> f32 {
        let float_value = &self.input_value.parse::<f32>().map_err(Error::ParseError);
        match float_value {
            Ok(value) => value.to_owned(),
            Err(error) => error.show_dialog_return_default::<f32>(),
        }
    }
}

fn init() {
    let _ = mkdir_result_temp_folder();
}
