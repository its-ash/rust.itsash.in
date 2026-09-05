//! MUI-style `Button` variants: `primary`, `secondary`, `outlined`, `text`,
//! `danger`, `success`.

use iced::widget::button::Button;
use iced::widget::text;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

fn base<'a, Message>(label: impl text::IntoFragment<'a>) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::button(iced::widget::text(label).font(system_font(FontWeight::Medium))).padding([8, 16])
}

/// A primary, filled button — the main call to action.
pub fn primary<'a, Message>(label: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(label).on_press(on_press).style(|theme, status| {
        crate::style::button::primary()(theme, status)
    })
}

/// A secondary, filled button.
pub fn secondary<'a, Message>(label: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(label).on_press(on_press).style(|theme, status| {
        crate::style::button::secondary()(theme, status)
    })
}

/// An outlined button: transparent background, colored border and text.
pub fn outlined<'a, Message>(label: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(label).on_press(on_press).style(|theme, status| {
        crate::style::button::outlined()(theme, status)
    })
}

/// A text-only button: no background or border, useful for links/toolbars.
pub fn text_button<'a, Message>(label: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(label).on_press(on_press).style(|theme, status| {
        crate::style::button::text()(theme, status)
    })
}

/// A destructive/error action button.
pub fn danger<'a, Message>(label: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(label).on_press(on_press).style(|theme, status| {
        crate::style::button::danger()(theme, status)
    })
}

/// A positive/success action button.
pub fn success<'a, Message>(label: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(label).on_press(on_press).style(|theme, status| {
        crate::style::button::success()(theme, status)
    })
}
