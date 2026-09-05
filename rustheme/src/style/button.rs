//! Named button style classes: `primary`, `secondary`, `outlined`, `text`,
//! `danger`, `success`.

use iced::border;
use iced::widget::button::{Status, Style, StyleFn};
use iced::{Background, Border, Color, Shadow};
use crate::theme::RustTheme;
use rustheme_core::ColorSet;

fn from_color_set(set: ColorSet, radius: f32, status: Status) -> Style {
    let base = Style {
        background: Some(Background::Color(set.main)),
        text_color: set.contrast_text,
        border: border::rounded(radius),
        shadow: Shadow::default(),
    };
    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(Background::Color(set.dark)),
            ..base
        },
        Status::Pressed => Style {
            background: Some(Background::Color(set.dark)),
            shadow: Shadow {
                color: Color { a: 0.15, ..Color::BLACK },
                offset: iced::Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            ..base
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color {
                a: 0.4,
                ..set.main
            })),
            text_color: Color {
                a: 0.6,
                ..set.contrast_text
            },
            ..base
        },
    }
}

/// A primary, filled button — the main call to action.
pub fn primary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        from_color_set(theme.palette.primary, theme.shape.border_radius, status)
    })
}

/// A secondary, filled button.
pub fn secondary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        from_color_set(theme.palette.secondary, theme.shape.border_radius, status)
    })
}

/// A destructive/error action button.
pub fn danger() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        from_color_set(theme.palette.error, theme.shape.border_radius, status)
    })
}

/// A positive/success action button.
pub fn success() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        from_color_set(theme.palette.success, theme.shape.border_radius, status)
    })
}

/// An outlined button: transparent background, colored border and text.
pub fn outlined() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let primary = theme.palette.primary;
        let border = Border {
            color: primary.main,
            width: 1.0,
            radius: theme.shape.border_radius.into(),
        };
        match status {
            Status::Active => Style {
                background: None,
                text_color: primary.main,
                border,
                shadow: Shadow::default(),
            },
            Status::Hovered => Style {
                background: Some(Background::Color(Color {
                    a: 0.08,
                    ..primary.main
                })),
                text_color: primary.main,
                border,
                shadow: Shadow::default(),
            },
            Status::Pressed => Style {
                background: Some(Background::Color(Color {
                    a: 0.16,
                    ..primary.main
                })),
                text_color: primary.dark,
                border,
                shadow: Shadow::default(),
            },
            Status::Disabled => Style {
                background: None,
                text_color: Color {
                    a: 0.4,
                    ..primary.main
                },
                border: Border {
                    color: Color { a: 0.3, ..border.color },
                    ..border
                },
                shadow: Shadow::default(),
            },
        }
    })
}

/// A text-only button: no background or border, useful for links/toolbars.
pub fn text() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let primary = theme.palette.primary;
        let base = Style {
            background: None,
            text_color: primary.main,
            border: Border::default(),
            shadow: Shadow::default(),
        };
        match status {
            Status::Active => base,
            Status::Hovered => Style {
                background: Some(Background::Color(Color { a: 0.08, ..primary.main })),
                ..base
            },
            Status::Pressed => Style {
                background: Some(Background::Color(Color { a: 0.16, ..primary.main })),
                text_color: primary.dark,
                ..base
            },
            Status::Disabled => Style {
                text_color: Color { a: 0.4, ..primary.main },
                ..base
            },
        }
    })
}
