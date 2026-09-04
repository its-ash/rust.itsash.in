//! MUI-style `Backdrop`: a full-screen semi-transparent scrim shown over
//! `base_content` via `stack`, reusable standalone (e.g. behind a spinner)
//! or as the dimming layer other overlay components compose with.

use iced::widget::stack;
use iced::{Element, Length};
use crate::theme::RustTheme;

/// The dimming scrim element alone, with no content on top — used by
/// [`backdrop`] and reusable by other overlay components (e.g. `dialog`).
pub fn scrim<'a, Message>() -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(iced::widget::Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &RustTheme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color { a: 0.5, ..iced::Color::BLACK })),
            ..Default::default()
        })
        .into()
}

/// Overlays a dimming backdrop (optionally with centered `content`, e.g. a
/// spinner) over `base_content` when `is_open`.
pub fn backdrop<'a, Message>(
    base_content: impl Into<Element<'a, Message, RustTheme>>,
    is_open: bool,
    content: Option<Element<'a, Message, RustTheme>>,
) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    let base = base_content.into();
    if !is_open {
        return base;
    }

    match content {
        Some(content) => {
            let centered = iced::widget::container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center);
            stack![base, scrim(), centered].into()
        }
        None => stack![base, scrim()].into(),
    }
}
