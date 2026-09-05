//! The root [`RustTheme`] type: a bundle of design tokens (palette,
//! typography, spacing, shape) that every `Catalog` implementation in
//! [`crate::style`] reads from to produce widget styles.
//!
//! `RustTheme` lives in this crate (rather than in `rustheme-core`, which
//! owns the token types themselves) because Rust's orphan rules require
//! `impl <iced trait> for RustTheme` to live in the same crate that defines
//! `RustTheme`.

pub use rustheme_core::{
    hex, set_global_font_family, set_global_heading_font_family, set_global_monospace_font_family,
    BackgroundColors, ColorSet, FontFamily, FontWeight, Palette, Shape, Spacing, TextColors,
    TypeScale, TypeStyle, Typography,
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
    /// The RustTheme brand palette (light mode) — the default look.
    pub fn light() -> Self {
        Self::builder().build()
    }

    /// The RustTheme brand palette (dark mode).
    pub fn dark() -> Self {
        Self::builder().palette(Palette::brand_dark()).build()
    }

    /// Starts a [`RustThemeBuilder`] for configuring a custom palette and/or
    /// font, e.g. at application startup:
    ///
    /// ```no_run
    /// use rustheme::prelude::*;
    ///
    /// let theme = RustTheme::builder()
    ///     .palette(Palette::brand_dark())
    ///     .font("Inter")
    ///     .build();
    /// ```
    pub fn builder() -> RustThemeBuilder {
        RustThemeBuilder::default()
    }
}

impl Default for RustTheme {
    fn default() -> Self {
        Self::light()
    }
}

/// Builds a [`RustTheme`] with an optional custom [`Palette`], [`Typography`]
/// (spacing/shape scale), and global UI/monospace font family.
///
/// The font family is process-wide (see
/// [`rustheme_core::set_global_font_family`] for why: iced applies fonts at
/// widget-construction time, not lazily per theme instance like colors), so
/// [`RustThemeBuilder::font`] / [`RustThemeBuilder::monospace_font`] should
/// only be set on the *first* `RustTheme` built in a process — typically
/// once in `main()`, before constructing any UI. The palette, spacing, and
/// shape are ordinary per-instance data and can differ freely between
/// `RustTheme` values (e.g. a light and a dark variant built side by side).
#[derive(Debug, Clone, Default)]
pub struct RustThemeBuilder {
    palette: Option<Palette>,
    typography: Option<Typography>,
    spacing: Option<Spacing>,
    shape: Option<Shape>,
    font: Option<FontFamily>,
    monospace_font: Option<FontFamily>,
    heading_font: Option<FontFamily>,
}

impl RustThemeBuilder {
    /// Sets a custom color palette, overriding the RustTheme brand default.
    pub fn palette(mut self, palette: Palette) -> Self {
        self.palette = Some(palette);
        self
    }

    /// Sets a custom type scale, overriding the desktop-native default.
    pub fn typography(mut self, typography: Typography) -> Self {
        self.typography = Some(typography);
        self
    }

    /// Sets a custom base spacing unit, overriding the default (8px).
    pub fn spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = Some(spacing);
        self
    }

    /// Sets a custom corner radius, overriding the default (6px).
    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = Some(shape);
        self
    }

    /// Sets the process-wide default UI font family by name (e.g. `"Inter"`).
    /// See the [`RustThemeBuilder`] docs for why this is process-wide rather
    /// than per-instance, and call it only once, before building any UI.
    pub fn font(mut self, family_name: &'static str) -> Self {
        self.font = Some(FontFamily::Named(family_name));
        self
    }

    /// Sets the process-wide default monospace font family by name, used
    /// for tabular numerals and other fixed-width content (e.g. table
    /// cells). Same call-once-before-building-UI semantics as [`Self::font`].
    pub fn monospace_font(mut self, family_name: &'static str) -> Self {
        self.monospace_font = Some(FontFamily::Named(family_name));
        self
    }

    /// Sets the process-wide default **heading** font family by name, used
    /// only for `h1`..`h6`/`subtitle1`/`subtitle2` and structural titles
    /// (app bar, dialog) — everything else keeps using [`Self::font`]'s
    /// family. Falls back to the regular UI font if never set. Same
    /// call-once-before-building-UI semantics as [`Self::font`].
    ///
    /// RustTheme's bundled Playfair Display + Source Sans Pro pairing (see
    /// `rustheme::fonts`) is a ready-made example of this heading/body
    /// split, loaded via `rustheme::fonts::use_default_pairing()` +
    /// `rustheme::load_fonts!` rather than through this method, since those
    /// two fonts need their bytes embedded, not just their names set.
    pub fn heading_font(mut self, family_name: &'static str) -> Self {
        self.heading_font = Some(FontFamily::Named(family_name));
        self
    }

    /// Builds the configured [`RustTheme`], applying any global font
    /// selection.
    pub fn build(self) -> RustTheme {
        if let Some(font) = self.font {
            set_global_font_family(font);
        }
        if let Some(font) = self.monospace_font {
            set_global_monospace_font_family(font);
        }
        if let Some(font) = self.heading_font {
            set_global_heading_font_family(font);
        }
        RustTheme {
            palette: self.palette.unwrap_or_default(),
            typography: self.typography.unwrap_or_default(),
            spacing: self.spacing.unwrap_or_default(),
            shape: self.shape.unwrap_or_default(),
        }
    }
}
