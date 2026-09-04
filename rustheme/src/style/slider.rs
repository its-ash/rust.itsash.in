//! Named slider style class: `primary`.

use iced::widget::slider::{Handle, HandleShape, Rail, Status, Style, StyleFn};
use iced::{Background, Border, Color};
use crate::theme::RustTheme;

pub fn primary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let handle_radius = match status {
            Status::Dragged => 7.0,
            _ => 6.0,
        };
        Style {
            rail: Rail {
                backgrounds: (
                    Background::Color(p.primary.main),
                    Background::Color(p.divider),
                ),
                width: 4.0,
                border: Border {
                    radius: 2.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: handle_radius },
                background: Background::Color(p.primary.main),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    })
}
