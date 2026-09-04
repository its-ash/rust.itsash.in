//! Named scrollable style class: `default`.

use iced::widget::container;
use iced::widget::scrollable::{Rail, Scroller, Status, Style, StyleFn};
use iced::{Background, Border, Color};
use crate::theme::RustTheme;

pub fn default() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let scroller_color = match status {
            Status::Hovered { .. } | Status::Dragged { .. } => p.text.secondary,
            Status::Active => Color { a: 0.5, ..p.text.secondary },
        };
        let rail = Rail {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border::default(),
            scroller: Scroller {
                color: scroller_color,
                border: Border {
                    radius: 4.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
            },
        };
        Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
        }
    })
}
