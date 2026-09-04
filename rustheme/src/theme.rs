//! The root [`RustTheme`] type: a bundle of design tokens (palette,
//! typography, spacing, shape) that every `Catalog` implementation in
//! [`crate::style`] reads from to produce widget styles.
//!
//! `RustTheme` lives in this crate (rather than in `rustheme-core`, which
//! owns the token types themselves) because Rust's orphan rules require
//! `impl <iced trait> for RustTheme` to live in the same crate that defines
//! `RustTheme`.

pub use rustheme_core::{
    hex, BackgroundColors, ColorSet, FontWeight, Palette, Shape, Spacing, TextColors, TypeScale,
    TypeStyle, Typography,
};

/// The root theme type for RustTheme: a bundle of design tokens that every
/// styled component reads from.
#[derive(Debug, Clone, PartialEq)]
pub struct RustTheme {
    pub palette: Palette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub shape: Shape,
}

impl RustTheme {
    pub fn light() -> Self {
        Self {
            palette: Palette::light(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            shape: Shape::default(),
        }
    }

    pub fn dark() -> Self {
        Self {
            palette: Palette::dark(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            shape: Shape::default(),
        }
    }
}

impl Default for RustTheme {
    fn default() -> Self {
        Self::light()
    }
}
