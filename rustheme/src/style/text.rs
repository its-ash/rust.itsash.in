//! Named text style classes: `primary`, `secondary`, `disabled`, `error`.
//!
//! The default class ([`primary`]) intentionally *inherits* rather than
//! forcing `text.primary`: plain `text(...)` nested inside a container that
//! sets its own `text_color` (a snackbar, alert, filled chip/badge, a
//! colored button label) must pick up that ambient color, not override it.
//! Forcing `text.primary` here previously made any such text render in the
//! theme's normal foreground color regardless of the surface it sat on —
//! e.g. dark text on a dark snackbar surface in dark mode.

use iced::widget::text::{Style, StyleFn};
use crate::theme::RustTheme;

pub fn primary() -> StyleFn<'static, RustTheme> {
    Box::new(|_theme: &RustTheme| Style { color: None })
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
