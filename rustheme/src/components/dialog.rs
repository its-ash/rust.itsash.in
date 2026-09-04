//! MUI-style `Dialog`: a centered modal overlay with a title, body, and
//! action row, shown via `stack` when `is_open` is true.

use iced::widget::stack;
use iced::{Element, Length};
use crate::theme::RustTheme;

/// Overlays a modal dialog over `base_content` when `is_open`.
pub fn dialog<'a, Message>(
    base_content: impl Into<Element<'a, Message, RustTheme>>,
    is_open: bool,
    title: impl Into<String>,
    body: impl Into<Element<'a, Message, RustTheme>>,
    actions: Vec<Element<'a, Message, RustTheme>>,
) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    let base = base_content.into();
    if !is_open {
        return base;
    }

    let mut action_row = iced::widget::row![].spacing(8);
    for action in actions {
        action_row = action_row.push(action);
    }

    let panel = iced::widget::container(
        iced::widget::column![
            iced::widget::text(title.into()).size(20),
            body.into(),
            iced::widget::row![iced::widget::horizontal_space(), action_row],
        ]
        .spacing(16),
    )
    .padding(24)
    .width(Length::Fixed(420.0))
    .style(|theme| crate::style::container::elevated()(theme));

    let scrim = iced::widget::container(iced::widget::Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &RustTheme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color { a: 0.5, ..iced::Color::BLACK })),
            ..Default::default()
        });

    let centered = iced::widget::container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center);

    stack![base, scrim, centered].into()
}
