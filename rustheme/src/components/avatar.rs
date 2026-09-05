//! MUI-style `Avatar`: a circular badge showing initials.

use iced::widget::Container;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// A circular avatar showing up to two initial characters.
pub fn initials<'a, Message>(initials: impl Into<String>, diameter: f32) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    let initials = initials.into();
    iced::widget::container(
        iced::widget::text(initials)
            .size(diameter * 0.4)
            .font(system_font(FontWeight::Medium))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .width(diameter)
    .height(diameter)
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
    .style(move |theme: &RustTheme| {
        let p = &theme.palette;
        iced::widget::container::Style {
            background: Some(iced::Background::Color(p.secondary.main)),
            text_color: Some(p.secondary.contrast_text),
            border: iced::border::rounded(diameter / 2.0),
            shadow: iced::Shadow::default(),
        }
    })
}
