//! MUI-style `Fab` (Floating Action Button): a large circular elevated
//! button, plus an `extended` variant that also shows a label.

use iced::widget::button::Button;
use iced::widget::text;
use crate::theme::RustTheme;

/// A circular, elevated primary FAB containing a single glyph.
pub fn primary<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::button(
        iced::widget::text(glyph)
            .size(22)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .padding(12)
    .width(56)
    .height(56)
    .on_press(on_press)
    .style(|theme: &RustTheme, status| {
        let s = crate::style::button::primary()(theme, status);
        iced::widget::button::Style {
            border: iced::border::rounded(28),
            shadow: iced::Shadow {
                color: iced::Color { a: 0.35, ..iced::Color::BLACK },
                offset: iced::Vector::new(0.0, 3.0),
                blur_radius: 8.0,
            },
            ..s
        }
    })
}

/// A circular, elevated secondary FAB containing a single glyph.
pub fn secondary<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::button(
        iced::widget::text(glyph)
            .size(22)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .padding(12)
    .width(56)
    .height(56)
    .on_press(on_press)
    .style(|theme: &RustTheme, status| {
        let s = crate::style::button::secondary()(theme, status);
        iced::widget::button::Style {
            border: iced::border::rounded(28),
            shadow: iced::Shadow {
                color: iced::Color { a: 0.35, ..iced::Color::BLACK },
                offset: iced::Vector::new(0.0, 3.0),
                blur_radius: 8.0,
            },
            ..s
        }
    })
}

/// An extended, pill-shaped FAB combining a leading glyph and a text label.
pub fn extended<'a, Message>(
    glyph: impl text::IntoFragment<'a>,
    label: impl Into<String>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let content = iced::widget::row![iced::widget::text(glyph).size(18), iced::widget::text(label.into())]
        .spacing(8)
        .align_y(iced::alignment::Vertical::Center);

    iced::widget::button(content)
        .padding([16, 24])
        .height(48)
        .on_press(on_press)
        .style(|theme: &RustTheme, status| {
            let s = crate::style::button::primary()(theme, status);
            iced::widget::button::Style {
                border: iced::border::rounded(24),
                shadow: iced::Shadow {
                    color: iced::Color { a: 0.35, ..iced::Color::BLACK },
                    offset: iced::Vector::new(0.0, 3.0),
                    blur_radius: 8.0,
                },
                ..s
            }
        })
}
