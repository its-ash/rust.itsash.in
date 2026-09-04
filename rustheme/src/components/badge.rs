//! MUI-style `Badge`: a small count/dot overlaid on the corner of content.

use iced::widget::{stack, Container};
use iced::{Element, Length};
use crate::theme::RustTheme;

fn pill<'a, Message>(text: String) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(
        iced::widget::text(text)
            .size(11)
            .align_x(iced::alignment::Horizontal::Center),
    )
    .padding([1, 6])
    .style(|theme: &RustTheme| {
        let p = &theme.palette;
        iced::widget::container::Style {
            background: Some(iced::Background::Color(p.error.main)),
            text_color: Some(p.error.contrast_text),
            border: iced::border::rounded(8),
            shadow: iced::Shadow::default(),
        }
    })
}

/// Overlays a numeric count badge on the top-right corner of `content`.
/// A count of `0` renders no badge.
pub fn count<'a, Message>(content: impl Into<Element<'a, Message, RustTheme>>, count: u32) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    if count == 0 {
        return content.into();
    }
    let label = if count > 99 { "99+".to_string() } else { count.to_string() };

    stack![
        content.into(),
        iced::widget::container(pill(label))
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right)
    ]
    .into()
}

/// Overlays a small colored dot (no label) on the top-right corner of `content`.
pub fn dot<'a, Message>(content: impl Into<Element<'a, Message, RustTheme>>) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    let marker = iced::widget::container(iced::widget::Space::new(8, 8)).style(|theme: &RustTheme| {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(theme.palette.error.main)),
            border: iced::border::rounded(4),
            ..Default::default()
        }
    });

    stack![
        content.into(),
        iced::widget::container(marker)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right)
    ]
    .into()
}
