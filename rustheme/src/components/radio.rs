//! MUI-style `Radio` button.

use iced::widget::radio::Radio;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

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
        .font(system_font(FontWeight::Regular))
        .style(|theme, status| crate::style::radio::primary()(theme, status))
}
