//! MUI-style `Checkbox`.

use iced::widget::checkbox::Checkbox;
use crate::theme::RustTheme;

/// A primary-colored checkbox with a text label.
pub fn checkbox<'a, Message>(
    label: impl Into<String>,
    is_checked: bool,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> Checkbox<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::checkbox(label, is_checked)
        .on_toggle(on_toggle)
        .style(|theme, status| crate::style::checkbox::primary()(theme, status))
}
