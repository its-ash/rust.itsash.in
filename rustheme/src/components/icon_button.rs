//! MUI-style `IconButton`: a compact circular button for icon-only actions.
//!
//! Icons are accepted as `impl text::IntoFragment` (a single glyph/symbol),
//! matching `Button`'s label parameter — this project has no icon asset
//! system, and reusing the same text-fragment pattern as `button.rs` keeps
//! the API consistent rather than introducing a separate `Element`-based
//! icon abstraction just for this one component.

use iced::widget::button::Button;
use iced::widget::text;
use crate::theme::RustTheme;

fn base<'a, Message>(glyph: impl text::IntoFragment<'a>) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::button(
        iced::widget::text(glyph)
            .size(18)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .padding(8)
    .width(40)
    .height(40)
}

/// A primary, filled circular icon button.
pub fn primary<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        let s = crate::style::button::primary()(theme, status);
        iced::widget::button::Style {
            border: iced::border::rounded(20),
            ..s
        }
    })
}

/// A secondary, filled circular icon button.
pub fn secondary<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        let s = crate::style::button::secondary()(theme, status);
        iced::widget::button::Style {
            border: iced::border::rounded(20),
            ..s
        }
    })
}

/// An outlined circular icon button.
pub fn outlined<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        let s = crate::style::button::outlined()(theme, status);
        iced::widget::button::Style {
            border: iced::Border {
                radius: 20.0.into(),
                ..s.border
            },
            ..s
        }
    })
}

/// A borderless, transparent icon button — the most common MUI `IconButton`.
pub fn text_icon<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        let s = crate::style::button::text()(theme, status);
        iced::widget::button::Style {
            border: iced::border::rounded(20),
            ..s
        }
    })
}
