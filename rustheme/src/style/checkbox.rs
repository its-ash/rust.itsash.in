//! Named checkbox style class: `primary`.

use iced::widget::checkbox::{Status, Style, StyleFn};
use iced::{Background, Border, Color};
use crate::theme::RustTheme;

pub fn primary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let is_checked = matches!(
            status,
            Status::Active { is_checked: true }
                | Status::Hovered { is_checked: true }
                | Status::Disabled { is_checked: true }
        );

        let base = Style {
            background: Background::Color(if is_checked {
                p.primary.main
            } else {
                Color::TRANSPARENT
            }),
            icon_color: p.primary.contrast_text,
            border: Border {
                color: if is_checked { p.primary.main } else { p.text.secondary },
                width: 2.0,
                radius: (theme.shape.border_radius / 2.0).into(),
            },
            text_color: Some(p.text.primary),
        };

        match status {
            Status::Active { .. } => base,
            Status::Hovered { .. } => Style {
                border: Border {
                    color: p.primary.main,
                    ..base.border
                },
                ..base
            },
            Status::Disabled { .. } => Style {
                background: Background::Color(Color { a: 0.4, ..(if is_checked { p.primary.main } else { Color::TRANSPARENT }) }),
                border: Border {
                    color: Color { a: 0.4, ..base.border.color },
                    ..base.border
                },
                text_color: Some(p.text.disabled),
                ..base
            },
        }
    })
}
