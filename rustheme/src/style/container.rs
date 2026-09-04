//! Named container style classes: `paper`, `card`, `elevated`, `outlined`,
//! `transparent`.

use iced::widget::container::{Style, StyleFn};
use iced::{Background, Border, Color, Shadow, Vector};
use crate::theme::RustTheme;

/// The default surface background (MUI `background.paper`), no border.
pub fn paper() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        background: Some(Background::Color(theme.palette.background.paper)),
        text_color: Some(theme.palette.text.primary),
        border: Border {
            radius: theme.shape.border_radius.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    })
}

/// A card surface with a subtle shadow, MUI `Card` default elevation.
pub fn card() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        background: Some(Background::Color(theme.palette.background.paper)),
        text_color: Some(theme.palette.text.primary),
        border: Border {
            radius: theme.shape.border_radius.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color { a: 0.2, ..Color::BLACK },
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        },
    })
}

/// A more prominent elevated surface with a stronger shadow.
pub fn elevated() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        background: Some(Background::Color(theme.palette.background.paper)),
        text_color: Some(theme.palette.text.primary),
        border: Border {
            radius: theme.shape.border_radius.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color { a: 0.3, ..Color::BLACK },
            offset: Vector::new(0.0, 4.0),
            blur_radius: 10.0,
        },
    })
}

/// A surface with a visible border and no fill, MUI `variant="outlined"`.
pub fn outlined() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        background: None,
        text_color: Some(theme.palette.text.primary),
        border: Border {
            radius: theme.shape.border_radius.into(),
            width: 1.0,
            color: theme.palette.divider,
        },
        shadow: Shadow::default(),
    })
}

/// A fully transparent container — layout only, no visual styling.
pub fn transparent() -> StyleFn<'static, RustTheme> {
    Box::new(|_theme: &RustTheme| Style::default())
}
