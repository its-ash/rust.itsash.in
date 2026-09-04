//! MUI-style `Switch`, backed by iced's `Toggler` widget.

use iced::widget::toggler::Toggler;
use crate::theme::RustTheme;

/// A primary-colored switch, optionally with a text label.
pub fn switch<'a, Message>(is_toggled: bool, on_toggle: impl Fn(bool) -> Message + 'a) -> Toggler<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::toggler(is_toggled)
        .on_toggle(on_toggle)
        .style(|theme, status| crate::style::toggler::primary()(theme, status))
}
