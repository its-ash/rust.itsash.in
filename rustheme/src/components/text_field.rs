//! MUI-style `TextField` variants: `outlined`, `filled`.

use iced::widget::text_input::TextInput;
use crate::theme::RustTheme;

/// An outlined text field with a visible border.
pub fn outlined<'a, Message>(
    placeholder: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
) -> TextInput<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::text_input(placeholder, value)
        .on_input(on_input)
        .padding(10)
        .style(|theme, status| crate::style::text_input::outlined()(theme, status))
}

/// A filled text field with a tinted background and no border.
pub fn filled<'a, Message>(
    placeholder: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
) -> TextInput<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::text_input(placeholder, value)
        .on_input(on_input)
        .padding(10)
        .style(|theme, status| crate::style::text_input::filled()(theme, status))
}
