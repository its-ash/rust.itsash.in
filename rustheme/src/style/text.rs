//! Named text style classes: `primary`, `secondary`, `disabled`, `error`.

use iced::widget::text::{Style, StyleFn};
use crate::theme::RustTheme;

pub fn primary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        color: Some(theme.palette.text.primary),
    })
}

pub fn secondary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        color: Some(theme.palette.text.secondary),
    })
}

pub fn disabled() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        color: Some(theme.palette.text.disabled),
    })
}

pub fn error() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        color: Some(theme.palette.error.main),
    })
}
