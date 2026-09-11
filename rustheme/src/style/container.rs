//! Named container style classes following Material 3 surface elevation.
//!
//! - `paper` — M3 Level 0: flat surface, no shadow, M3 Medium radius.
//! - `card` — M3 Level 0 outlined variant: flat with a 1px outline.
//! - `elevated` — M3 Level 1: tonal-elevation surface with the M3
//!   Level 1 shadow (ambient + key).
//! - `outlined` — M3 outlined card: transparent fill, 1px divider border.

use iced::widget::container::{Style, StyleFn};
use iced::{Background, Border, Color, Shadow};
use crate::theme::RustTheme;
use rustheme_core::Elevation;

/// The default surface background (M3 surface), no border, no shadow.
pub fn paper() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        background: Some(Background::Color(theme.palette.background.paper)),
        text_color: Some(theme.palette.text.primary),
        border: Border {
            radius: theme.shape.medium.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    })
}

/// A card surface: M3 filled card — flat, M3 Large radius (16dp), with a
/// thin outline so it reads as a distinct surface on the background.
/// Border width and resting shadow follow `theme.material`, so a theme's
/// finish (e.g. Neomorphism's soft shadow, Brutalism's thick border)
/// applies here too.
pub fn card() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        background: Some(Background::Color(theme.palette.background.paper)),
        text_color: Some(theme.palette.text.primary),
        border: Border {
            radius: theme.shape.large.into(),
            width: theme.material.border_width.max(1.0),
            color: theme.palette.divider,
        },
        shadow: theme.material.shadow,
    })
}

/// An elevated surface: M3 Level 1 elevation with the M3 Level 1 shadow
/// (ambient + key blended), or `theme.material`'s shadow when the active
/// theme defines one heavier than the M3 default. M3 Large radius for
/// card/sheet use.
pub fn elevated() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| {
        let m3_shadow = Elevation::Level1.shadow();
        let shadow = if theme.material.shadow.blur_radius > m3_shadow.blur_radius {
            theme.material.shadow
        } else {
            m3_shadow
        };
        Style {
            background: Some(Background::Color(theme.palette.background.paper)),
            text_color: Some(theme.palette.text.primary),
            border: Border {
                radius: theme.shape.large.into(),
                width: theme.material.border_width,
                color: theme.palette.divider,
            },
            shadow,
        }
    })
}

/// A surface with a visible border and no fill, M3 outlined card variant.
pub fn outlined() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| Style {
        background: None,
        text_color: Some(theme.palette.text.primary),
        border: Border {
            radius: theme.shape.large.into(),
            width: theme.material.border_width.max(1.0),
            color: theme.palette.divider,
        },
        shadow: Shadow::default(),
    })
}

/// A fully transparent container — layout only, no visual styling.
pub fn transparent() -> StyleFn<'static, RustTheme> {
    Box::new(|_theme: &RustTheme| Style::default())
}
