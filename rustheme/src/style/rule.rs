//! Named rule (divider) style class: `default`.

use iced::widget::rule::{FillMode, Style, StyleFn};
use crate::theme::RustTheme;

pub fn default() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        color: theme.palette.divider,
        width: 1,
        radius: 0.0.into(),
        fill_mode: FillMode::Full,
    })
}
