//! MUI-style `Card` surfaces: `card` (subtle shadow), `elevated` (stronger
//! shadow), `outlined` (bordered, no fill).

use iced::widget::Container;
use iced::Element;
use crate::theme::RustTheme;

/// A card surface with a subtle shadow.
pub fn card<'a, Message>(content: impl Into<Element<'a, Message, RustTheme>>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(content)
        .padding(16)
        .style(|theme| crate::style::container::card()(theme))
}

/// A more prominent elevated card with a stronger shadow.
pub fn elevated<'a, Message>(content: impl Into<Element<'a, Message, RustTheme>>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(content)
        .padding(16)
        .style(|theme| crate::style::container::elevated()(theme))
}

/// A card with a visible border and no fill.
pub fn outlined<'a, Message>(content: impl Into<Element<'a, Message, RustTheme>>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(content)
        .padding(16)
        .style(|theme| crate::style::container::outlined()(theme))
}
