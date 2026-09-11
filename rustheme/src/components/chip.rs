//! MUI-style `Chip`: `filled`, `outlined`.

use iced::widget::Container;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// A filled, pill-shaped chip. Padding scales with `theme.spacing`.
pub fn filled<'a, Message>(theme: &RustTheme, label: impl Into<String>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(iced::widget::text(label.into()).size(13).font(system_font(FontWeight::Medium)))
        .padding([theme.spacing.unit(0.5), theme.spacing.unit(1.5)])
        .style(|theme: &RustTheme| {
            let p = &theme.palette;
            iced::widget::container::Style {
                background: Some(iced::Background::Color(p.primary.main)),
                text_color: Some(p.primary.contrast_text),
                border: iced::border::rounded(theme.shape.full),
                shadow: iced::Shadow::default(),
            }
        })
}

/// An outlined, pill-shaped chip. Padding scales with `theme.spacing`.
pub fn outlined<'a, Message>(theme: &RustTheme, label: impl Into<String>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(iced::widget::text(label.into()).size(13).font(system_font(FontWeight::Medium)))
        .padding([theme.spacing.unit(0.5), theme.spacing.unit(1.5)])
        .style(|theme: &RustTheme| {
            let p = &theme.palette;
            iced::widget::container::Style {
                background: None,
                text_color: Some(p.text.primary),
                border: iced::Border {
                    color: p.divider,
                    width: 1.0,
                    radius: theme.shape.full.into(),
                },
                shadow: iced::Shadow::default(),
            }
        })
}
