//! Named radio style class: `primary`.

use iced::widget::radio::{Status, Style, StyleFn};
use iced::{Background, Color};
use crate::theme::RustTheme;

pub fn primary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let is_selected = match status {
            Status::Active { is_selected } => is_selected,
            Status::Hovered { is_selected } => is_selected,
        };
        let border_color = if is_selected || matches!(status, Status::Hovered { .. }) {
            p.primary.main
        } else {
            p.text.secondary
        };
        Style {
            background: Background::Color(Color::TRANSPARENT),
            dot_color: p.primary.main,
            border_width: 2.0,
            border_color,
            text_color: Some(p.text.primary),
        }
    })
}
