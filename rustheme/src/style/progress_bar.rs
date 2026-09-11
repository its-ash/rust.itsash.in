//! Named progress-bar style class: `primary`.

use iced::widget::progress_bar::{Style, StyleFn};
use iced::{Background, Border, Color};
use crate::theme::RustTheme;

pub fn primary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| {
        let p = &theme.palette;
        Style {
            background: Background::Color(p.divider),
            bar: Background::Color(p.primary.main),
            border: Border {
                radius: (theme.shape.extra_small / 2.0).into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        }
    })
}
