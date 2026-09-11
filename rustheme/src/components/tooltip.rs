//! MUI-style `Tooltip`.

use iced::widget::tooltip::{Position, Tooltip};
use iced::widget::Container;
use iced::Element;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// Wraps `content` with a tooltip showing `label` above it on hover.
/// Padding scales with `theme.spacing`.
pub fn tooltip<'a, Message>(
    theme: &RustTheme,
    content: impl Into<Element<'a, Message, RustTheme>>,
    label: impl iced::widget::text::IntoFragment<'a>,
) -> Tooltip<'a, Message, RustTheme>
where
    Message: 'a,
{
    let bubble: Container<'a, Message, RustTheme> = iced::widget::container(iced::widget::text(label).size(13).font(system_font(FontWeight::Regular)))
        .padding(theme.spacing.unit(1.0))
        .style(|theme| crate::style::container::elevated()(theme));

    iced::widget::tooltip(content, bubble, Position::Top)
}
