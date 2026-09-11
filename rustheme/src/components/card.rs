//! MUI-style `Card` surfaces: `card` (subtle shadow), `elevated` (stronger
//! shadow), `outlined` (bordered, no fill).

use iced::widget::Container;
use iced::Element;
use crate::theme::RustTheme;

/// A card surface with a subtle shadow. Padding scales with `theme.spacing`.
pub fn card<'a, Message>(theme: &RustTheme, content: impl Into<Element<'a, Message, RustTheme>>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(content)
        .padding(theme.spacing.unit(2.0))
        .style(|theme| crate::style::container::card()(theme))
}

/// A more prominent elevated card with a stronger shadow. Padding scales
/// with `theme.spacing`.
pub fn elevated<'a, Message>(theme: &RustTheme, content: impl Into<Element<'a, Message, RustTheme>>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(content)
        .padding(theme.spacing.unit(2.0))
        .style(|theme| crate::style::container::elevated()(theme))
}

/// A card with a visible border and no fill. Padding scales with
/// `theme.spacing`.
pub fn outlined<'a, Message>(theme: &RustTheme, content: impl Into<Element<'a, Message, RustTheme>>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(content)
        .padding(theme.spacing.unit(2.0))
        .style(|theme| crate::style::container::outlined()(theme))
}
