//! MUI-style `Drawer`: a sidebar panel anchored left or right, shown/hidden
//! via `stack` over `base_content` — the same overlay pattern as
//! [`crate::dialog`].

use iced::widget::stack;
use iced::{Element, Length};
use crate::theme::RustTheme;

/// Which edge the drawer slides in from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Left,
    Right,
}

/// Overlays a `content` sidebar panel over `base_content` when `is_open`,
/// anchored to `anchor`. Clicking the scrim behind it fires `on_dismiss`.
pub fn drawer<'a, Message>(
    base_content: impl Into<Element<'a, Message, RustTheme>>,
    is_open: bool,
    anchor: Anchor,
    content: impl Into<Element<'a, Message, RustTheme>>,
    on_dismiss: Message,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let base = base_content.into();
    if !is_open {
        return base;
    }

    let panel = iced::widget::container(content)
        .padding(20)
        .width(Length::Fixed(280.0))
        .height(Length::Fill)
        .style(|theme| crate::style::container::elevated()(theme));

    let scrim = iced::widget::mouse_area(
        iced::widget::container(iced::widget::Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &RustTheme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color { a: 0.5, ..iced::Color::BLACK })),
                ..Default::default()
            }),
    )
    .on_press(on_dismiss);

    let horizontal_align = match anchor {
        Anchor::Left => iced::alignment::Horizontal::Left,
        Anchor::Right => iced::alignment::Horizontal::Right,
    };

    let positioned = iced::widget::container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(horizontal_align);

    stack![base, scrim, positioned].into()
}
