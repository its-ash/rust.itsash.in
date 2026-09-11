//! Named text-input style classes: `outlined`, `filled`.

use iced::widget::text_input::{Status, Style, StyleFn};
use iced::{Background, Border, Color};
use crate::theme::RustTheme;

/// An outlined text field with a visible border, MUI `variant="outlined"` style.
pub fn outlined() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let border_color = match status {
            Status::Active | Status::Hovered => p.divider,
            Status::Focused => p.primary.main,
            Status::Disabled => Color { a: 0.5, ..p.divider },
        };
        Style {
            background: Background::Color(p.background.paper),
            border: Border {
                color: border_color,
                width: if matches!(status, Status::Focused) { 2.0 } else { 1.0 },
                radius: theme.shape.extra_small.into(),
            },
            icon: p.text.secondary,
            placeholder: p.text.disabled,
            value: p.text.primary,
            selection: Color { a: 0.3, ..p.primary.main },
        }
    })
}

/// A filled text field with a tinted background and no border, MUI
/// `variant="filled"` style.
pub fn filled() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let bg = match status {
            Status::Active => Color { a: 0.06, ..p.text.primary },
            Status::Hovered => Color { a: 0.09, ..p.text.primary },
            Status::Focused => Color { a: 0.09, ..p.text.primary },
            Status::Disabled => Color { a: 0.03, ..p.text.primary },
        };
        Style {
            background: Background::Color(Color {
                a: bg.a,
                ..p.background.default
            }),
            border: Border {
                color: if matches!(status, Status::Focused) {
                    p.primary.main
                } else {
                    Color::TRANSPARENT
                },
                width: if matches!(status, Status::Focused) { 2.0 } else { 0.0 },
                radius: theme.shape.extra_small.into(),
            },
            icon: p.text.secondary,
            placeholder: p.text.disabled,
            value: p.text.primary,
            selection: Color { a: 0.3, ..p.primary.main },
        }
    })
}
