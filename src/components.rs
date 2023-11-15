use iced::{
    theme,
    widget::{button, container, tooltip},
    Element,
};

use crate::{Message};

pub fn button_component_font<'a>(
    content: Element<'a, Message>,
    label: &str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    tooltip(
        button(container(content).width(30).center_x())
            .on_press_maybe(on_press)
            .padding([5, 10]),
        label,
        tooltip::Position::FollowCursor,
    )
    .style(theme::Container::Box)
    .into()
}

pub fn button_component<'a,Message:Clone+'a>(
    label:&'a str,
    on_press:Option<Message>,
) -> Element<'a,Message>{
    let button = button(label);

    if let Some(on_press) = on_press {
        button.on_press(on_press).into()
    }else{
        button.style(iced::theme::Button::Secondary).into()
    }
}