use iced::{
    theme,
    widget::{button, container, tooltip},
    Element,
};

use crate::{Message};

pub fn button_component<'a>(
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

