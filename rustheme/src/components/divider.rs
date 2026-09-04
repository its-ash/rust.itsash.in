//! MUI-style `Divider`.

use iced::widget::Rule;
use crate::theme::RustTheme;

/// A horizontal divider line.
pub fn horizontal<'a>(height: impl Into<iced::Pixels>) -> Rule<'a, RustTheme> {
    iced::widget::horizontal_rule(height)
}

/// A vertical divider line.
pub fn vertical<'a>(width: impl Into<iced::Pixels>) -> Rule<'a, RustTheme> {
    iced::widget::vertical_rule(width)
}
