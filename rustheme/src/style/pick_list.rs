//! Named pick-list (select) style class: `outlined`.

use iced::widget::pick_list::{Status, Style, StyleFn};
use iced::{Background, Border};
use crate::theme::RustTheme;

pub fn outlined() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let border_color = match status {
            Status::Active => p.divider,
            Status::Hovered => p.text.secondary,
            Status::Opened => p.primary.main,
        };
        Style {
            text_color: p.text.primary,
            placeholder_color: p.text.disabled,
            handle_color: p.text.secondary,
            background: Background::Color(p.background.paper),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: theme.shape.extra_small.into(),
            },
        }
    })
}
