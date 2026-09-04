//! MUI-style `Radio` button.

use iced::widget::radio::Radio;
use crate::theme::RustTheme;

/// A primary-colored radio button.
pub fn radio<'a, Message, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl FnOnce(V) -> Message,
) -> Radio<'a, Message, RustTheme>
where
    Message: Clone + 'a,
    V: Copy + Eq + 'a,
{
    iced::widget::radio(label, value, selected, on_click)
        .style(|theme, status| crate::style::radio::primary()(theme, status))
}
