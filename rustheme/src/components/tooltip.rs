//! MUI-style `Tooltip`.

use iced::widget::tooltip::{Position, Tooltip};
use iced::widget::Container;
use iced::Element;
use crate::theme::RustTheme;

/// Wraps `content` with a tooltip showing `label` above it on hover.
pub fn tooltip<'a, Message>(
    content: impl Into<Element<'a, Message, RustTheme>>,
    label: impl iced::widget::text::IntoFragment<'a>,
) -> Tooltip<'a, Message, RustTheme>
where
    Message: 'a,
{
    let bubble: Container<'a, Message, RustTheme> = iced::widget::container(iced::widget::text(label))
        .padding(8)
        .style(|theme| crate::style::container::elevated()(theme));

    iced::widget::tooltip(content, bubble, Position::Top)
}
