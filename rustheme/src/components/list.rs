//! MUI-style `List` / `ListItem`.

use iced::widget::{Column, Container};
use iced::Element;
use crate::theme::RustTheme;

/// A single, padded list-item row.
pub fn item<'a, Message>(content: impl Into<Element<'a, Message, RustTheme>>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(content)
        .padding([10, 16])
        .width(iced::Length::Fill)
}

/// A list container: a vertical column of items with dividers between them.
pub fn list<'a, Message>(items: Vec<Element<'a, Message, RustTheme>>) -> Column<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut col = iced::widget::column![];
    let len = items.len();
    for (i, el) in items.into_iter().enumerate() {
        col = col.push(el);
        if i + 1 < len {
            col = col.push(iced::widget::horizontal_rule(1));
        }
    }
    col
}
