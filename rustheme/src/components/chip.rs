//! MUI-style `Chip`: `filled`, `outlined`.

use iced::widget::Container;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// A filled, pill-shaped chip.
pub fn filled<'a, Message>(label: impl Into<String>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(iced::widget::text(label.into()).size(13).font(system_font(FontWeight::Medium)))
        .padding([4, 12])
        .style(|theme: &RustTheme| {
            let p = &theme.palette;
            iced::widget::container::Style {
                background: Some(iced::Background::Color(p.primary.main)),
                text_color: Some(p.primary.contrast_text),
                border: iced::border::rounded(16),
                shadow: iced::Shadow::default(),
            }
        })
}

/// An outlined, pill-shaped chip.
pub fn outlined<'a, Message>(label: impl Into<String>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(iced::widget::text(label.into()).size(13).font(system_font(FontWeight::Medium)))
        .padding([4, 12])
        .style(|theme: &RustTheme| {
            let p = &theme.palette;
            iced::widget::container::Style {
                background: None,
                text_color: Some(p.text.primary),
                border: iced::Border {
                    color: p.divider,
                    width: 1.0,
                    radius: 16.0.into(),
                },
                shadow: iced::Shadow::default(),
            }
        })
}
