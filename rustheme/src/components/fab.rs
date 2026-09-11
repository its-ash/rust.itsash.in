//! MUI-style `Fab` (Floating Action Button): a large circular elevated
//! button, plus an `extended` variant that also shows a label.

use iced::widget::button::Button;
use iced::widget::text;
use crate::theme::RustTheme;
use rustheme_core::{system_font, Elevation, FontWeight};

/// A circular, elevated primary FAB containing a single glyph. Padding
/// scales with `theme.spacing`.
pub fn primary<'a, Message>(theme: &RustTheme, glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::button(
        iced::widget::text(glyph)
            .size(22)
            .font(system_font(FontWeight::Medium))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .padding(theme.spacing.unit(1.5))
    .width(56)
    .height(56)
    .on_press(on_press)
    .style(|theme: &RustTheme, status| {
        let s = crate::style::button::primary()(theme, status);
        iced::widget::button::Style {
            border: iced::border::rounded(28),
            shadow: Elevation::Level3.shadow(),
            ..s
        }
    })
}

/// A circular, elevated secondary FAB containing a single glyph. Padding
/// scales with `theme.spacing`.
pub fn secondary<'a, Message>(theme: &RustTheme, glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::button(
        iced::widget::text(glyph)
            .size(22)
            .font(system_font(FontWeight::Medium))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .padding(theme.spacing.unit(1.5))
    .width(56)
    .height(56)
    .on_press(on_press)
    .style(|theme: &RustTheme, status| {
        let s = crate::style::button::secondary()(theme, status);
        iced::widget::button::Style {
            border: iced::border::rounded(28),
            shadow: Elevation::Level3.shadow(),
            ..s
        }
    })
}

/// An extended, pill-shaped FAB combining a leading glyph and a text label.
/// Padding scales with `theme.spacing`.
pub fn extended<'a, Message>(
    theme: &RustTheme,
    glyph: impl text::IntoFragment<'a>,
    label: impl Into<String>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let content = iced::widget::row![
        iced::widget::text(glyph).size(18).font(system_font(FontWeight::Medium)),
        iced::widget::text(label.into()).font(system_font(FontWeight::Medium))
    ]
        .spacing(8)
        .align_y(iced::alignment::Vertical::Center);

    iced::widget::button(content)
        .padding([theme.spacing.unit(2.0), theme.spacing.unit(3.0)])
        .height(48)
        .on_press(on_press)
        .style(|theme: &RustTheme, status| {
            let s = crate::style::button::primary()(theme, status);
            iced::widget::button::Style {
                border: iced::border::rounded(24),
                shadow: Elevation::Level3.shadow(),
                ..s
            }
        })
}
