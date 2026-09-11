//! The root [`RustTheme`] type: a bundle of design tokens (palette,
//! typography, spacing, shape) that every `Catalog` implementation in
//! [`crate::style`] reads from to produce widget styles.
//!
//! `RustTheme` lives in this crate (rather than in `rustheme-core`, which
//! owns the token types themselves) because Rust's orphan rules require
//! `impl <iced trait> for RustTheme` to live in the same crate that defines
//! `RustTheme`.

pub use rustheme_core::{
    hex, set_global_font_family, set_global_heading_font_family,
    set_global_monospace_font_family, BackgroundColors, ColorSet, Elevation, FontFamily,
    FontWeight, Material, Palette, Shape, Spacing, TextColors, TypeScale, TypeStyle, Typography,
};

/// The root theme type for RustTheme: a bundle of design tokens that every
/// styled component reads from.
#[derive(Debug, Clone, PartialEq)]
pub struct RustTheme {
    pub palette: Palette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub shape: Shape,
    /// The default elevation level for resting surfaces (M3 Level1 for
    /// cards/app bars; components override this via
    /// [`rustheme_core::Elevation`] directly).
    pub elevation: Elevation,
    /// Border width + resting shadow finish, shared by every bordered
    /// surface (buttons, cards, inputs). Defaults to [`Material::flat`].
    pub material: Material,
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
    elevation: Option<Elevation>,
    material: Option<Material>,
    font: Option<FontFamily>,
    monospace_font: Option<FontFamily>,
    heading_font: Option<FontFamily>,
    primary_color: Option<iced::Color>,
    text_color: Option<iced::Color>,
    surface_color: Option<iced::Color>,
    divider_color: Option<iced::Color>,
    border_radius: Option<f32>,
}

impl RustThemeBuilder {
    /// Sets a custom color palette, overriding the RustTheme brand default.
    pub fn palette(mut self, palette: Palette) -> Self {
        self.palette = Some(palette);
        self
    }

    /// Sets the global primary color, deriving its lighter, darker, and
    /// contrast-text tokens automatically. Apply this to either a light or
    /// dark palette to configure that mode's primary color.
    pub fn primary_color(mut self, color: iced::Color) -> Self {
        self.primary_color = Some(color);
        self
    }

    /// Sets the global primary text color and derives secondary/disabled
    /// text tones from the configured surface.
    pub fn text_color(mut self, color: iced::Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Sets the global canvas and paper surface color.
    pub fn surface_color(mut self, color: iced::Color) -> Self {
        self.surface_color = Some(color);
        self
    }

    /// Sets the global divider color.
    pub fn divider_color(mut self, color: iced::Color) -> Self {
        self.divider_color = Some(color);
        self
    }

    /// Applies one finite corner radius to every component shape token.
    /// Pills and circles remain fully rounded.
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
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

    /// Sets a custom corner radius, overriding the default (12px / M3 Medium).
    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = Some(shape);
        self
    }

    /// Sets the default resting elevation level, overriding the default
    /// (M3 Level1). Components that need a specific level pass an
    /// [`Elevation`] directly to their style function.
    pub fn elevation(mut self, elevation: Elevation) -> Self {
        self.elevation = Some(elevation);
        self
    }

    /// Sets the border width + resting shadow finish applied to buttons,
    /// cards, and other bordered surfaces, overriding the default
    /// ([`Material::flat`] — no border, no shadow).
    pub fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
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
    /// RustTheme's bundled Playfair Display + Roboto pairing (see
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
        let mut palette = self.palette.unwrap_or_default();
        if let Some(color) = self.surface_color {
            palette = palette.with_surface_color(color);
        }
        if let Some(color) = self.primary_color {
            palette = palette.with_primary(color);
        }
        if let Some(color) = self.text_color {
            palette = palette.with_text_color(color);
        }
        if let Some(color) = self.divider_color {
            palette = palette.with_divider_color(color);
        }

        RustTheme {
            palette,
            typography: self.typography.unwrap_or_default(),
            spacing: self.spacing.unwrap_or_default(),
            shape: self
                .border_radius
                .map(Shape::uniform)
                .or(self.shape)
                .unwrap_or_default(),
            elevation: self.elevation.unwrap_or(Elevation::Level1),
            material: self.material.unwrap_or_default(),
        }
    }
}

/// App-wide theme configuration: one place to set global tokens for **both**
/// light and dark mode at once, with optional per-mode overrides layered on
/// top. This is the recommended entry point for apps that want a single
/// "design settings" struct (e.g. driven by user preferences or a settings
/// screen) rather than building light/dark `RustTheme`s by hand.
///
/// Precedence per mode: per-mode setter (e.g. [`Self::dark_primary_color`])
/// wins over the shared setter (e.g. [`Self::primary_color`]) wins over the
/// brand default.
///
/// ```no_run
/// use rustheme::prelude::*;
/// use rustheme_core::hex;
///
/// let config = RustThemeConfig::new()
///     .radius(10.0)                       // both modes
///     .primary_color(hex("#3F51B5"))      // both modes
///     .dark_primary_color(hex("#B7C4FF")) // dark-mode-only override
///     .dark_radius(6.0);                  // dark-mode-only override
///
/// let light = config.light();
/// let dark = config.dark();
/// ```
#[derive(Debug, Clone, Default)]
pub struct RustThemeConfig {
    radius: Option<f32>,
    primary_color: Option<iced::Color>,
    text_color: Option<iced::Color>,
    surface_color: Option<iced::Color>,
    divider_color: Option<iced::Color>,
    material: Option<Material>,
    spacing: Option<Spacing>,

    light_radius: Option<f32>,
    light_primary_color: Option<iced::Color>,
    light_text_color: Option<iced::Color>,
    light_surface_color: Option<iced::Color>,
    light_divider_color: Option<iced::Color>,
    light_material: Option<Material>,

    dark_radius: Option<f32>,
    dark_primary_color: Option<iced::Color>,
    dark_text_color: Option<iced::Color>,
    dark_surface_color: Option<iced::Color>,
    dark_divider_color: Option<iced::Color>,
    dark_material: Option<Material>,

    font: Option<FontFamily>,
    monospace_font: Option<FontFamily>,
    heading_font: Option<FontFamily>,
}

impl RustThemeConfig {
    /// Starts an empty config; every token falls back to the brand default
    /// until overridden.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the global corner radius applied to both light and dark mode.
    /// Overridden per-mode by [`Self::light_radius`]/[`Self::dark_radius`].
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the global primary/brand color applied to both light and dark
    /// mode. Overridden per-mode by [`Self::light_primary_color`]/
    /// [`Self::dark_primary_color`].
    pub fn primary_color(mut self, color: iced::Color) -> Self {
        self.primary_color = Some(color);
        self
    }

    /// Sets the global text color applied to both light and dark mode.
    /// Overridden per-mode by [`Self::light_text_color`]/
    /// [`Self::dark_text_color`].
    pub fn text_color(mut self, color: iced::Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Sets the global surface (background/paper) color applied to both
    /// light and dark mode. Overridden per-mode by
    /// [`Self::light_surface_color`]/[`Self::dark_surface_color`].
    pub fn surface_color(mut self, color: iced::Color) -> Self {
        self.surface_color = Some(color);
        self
    }

    /// Sets the global divider color applied to both light and dark mode.
    /// Overridden per-mode by [`Self::light_divider_color`]/
    /// [`Self::dark_divider_color`].
    pub fn divider_color(mut self, color: iced::Color) -> Self {
        self.divider_color = Some(color);
        self
    }

    /// Sets the border width + resting shadow finish applied to both light
    /// and dark mode. Overridden per-mode by [`Self::light_material`]/
    /// [`Self::dark_material`].
    pub fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }

    /// Sets the base spacing unit (applied to both light and dark mode —
    /// spacing is a layout density preference, not a color, so it has no
    /// per-mode override). Defaults to `Spacing(8.0)` (MUI's base unit);
    /// [`Spacing::compact`]/[`Spacing::normal`]/[`Spacing::spacious`] are
    /// ready-made presets for a density picker.
    pub fn spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = Some(spacing);
        self
    }

    /// Light-mode-only corner radius override, taking precedence over
    /// [`Self::radius`] when [`Self::light`] is built.
    pub fn light_radius(mut self, radius: f32) -> Self {
        self.light_radius = Some(radius);
        self
    }

    /// Light-mode-only primary color override, taking precedence over
    /// [`Self::primary_color`] when [`Self::light`] is built.
    pub fn light_primary_color(mut self, color: iced::Color) -> Self {
        self.light_primary_color = Some(color);
        self
    }

    /// Light-mode-only text color override, taking precedence over
    /// [`Self::text_color`] when [`Self::light`] is built.
    pub fn light_text_color(mut self, color: iced::Color) -> Self {
        self.light_text_color = Some(color);
        self
    }

    /// Light-mode-only surface color override, taking precedence over
    /// [`Self::surface_color`] when [`Self::light`] is built.
    pub fn light_surface_color(mut self, color: iced::Color) -> Self {
        self.light_surface_color = Some(color);
        self
    }

    /// Light-mode-only divider color override, taking precedence over
    /// [`Self::divider_color`] when [`Self::light`] is built.
    pub fn light_divider_color(mut self, color: iced::Color) -> Self {
        self.light_divider_color = Some(color);
        self
    }

    /// Light-mode-only material (border/shadow) override, taking precedence
    /// over [`Self::material`] when [`Self::light`] is built.
    pub fn light_material(mut self, material: Material) -> Self {
        self.light_material = Some(material);
        self
    }

    /// Dark-mode-only corner radius override, taking precedence over
    /// [`Self::radius`] when [`Self::dark`] is built.
    pub fn dark_radius(mut self, radius: f32) -> Self {
        self.dark_radius = Some(radius);
        self
    }

    /// Dark-mode-only primary color override, taking precedence over
    /// [`Self::primary_color`] when [`Self::dark`] is built.
    pub fn dark_primary_color(mut self, color: iced::Color) -> Self {
        self.dark_primary_color = Some(color);
        self
    }

    /// Dark-mode-only text color override, taking precedence over
    /// [`Self::text_color`] when [`Self::dark`] is built.
    pub fn dark_text_color(mut self, color: iced::Color) -> Self {
        self.dark_text_color = Some(color);
        self
    }

    /// Dark-mode-only surface color override, taking precedence over
    /// [`Self::surface_color`] when [`Self::dark`] is built.
    pub fn dark_surface_color(mut self, color: iced::Color) -> Self {
        self.dark_surface_color = Some(color);
        self
    }

    /// Dark-mode-only divider color override, taking precedence over
    /// [`Self::divider_color`] when [`Self::dark`] is built.
    pub fn dark_divider_color(mut self, color: iced::Color) -> Self {
        self.dark_divider_color = Some(color);
        self
    }

    /// Dark-mode-only material (border/shadow) override, taking precedence
    /// over [`Self::material`] when [`Self::dark`] is built.
    pub fn dark_material(mut self, material: Material) -> Self {
        self.dark_material = Some(material);
        self
    }

    /// Sets the process-wide UI font family for both modes. See
    /// [`RustThemeBuilder::font`] for call-once-before-building-UI semantics.
    pub fn font(mut self, family_name: &'static str) -> Self {
        self.font = Some(FontFamily::Named(family_name));
        self
    }

    /// Sets the process-wide monospace font family for both modes. See
    /// [`RustThemeBuilder::monospace_font`].
    pub fn monospace_font(mut self, family_name: &'static str) -> Self {
        self.monospace_font = Some(FontFamily::Named(family_name));
        self
    }

    /// Sets the process-wide heading font family for both modes. See
    /// [`RustThemeBuilder::heading_font`].
    pub fn heading_font(mut self, family_name: &'static str) -> Self {
        self.heading_font = Some(FontFamily::Named(family_name));
        self
    }

    /// Builds the light-mode [`RustTheme`], applying shared tokens with any
    /// light-mode-only overrides layered on top.
    pub fn light(&self) -> RustTheme {
        self.build(Palette::brand_light(), self.light_radius, self.light_primary_color, self.light_text_color, self.light_surface_color, self.light_divider_color, self.light_material)
    }

    /// Builds the dark-mode [`RustTheme`], applying shared tokens with any
    /// dark-mode-only overrides layered on top.
    pub fn dark(&self) -> RustTheme {
        self.build(Palette::brand_dark(), self.dark_radius, self.dark_primary_color, self.dark_text_color, self.dark_surface_color, self.dark_divider_color, self.dark_material)
    }

    #[allow(clippy::too_many_arguments)]
    fn build(
        &self,
        palette: Palette,
        mode_radius: Option<f32>,
        mode_primary: Option<iced::Color>,
        mode_text: Option<iced::Color>,
        mode_surface: Option<iced::Color>,
        mode_divider: Option<iced::Color>,
        mode_material: Option<Material>,
    ) -> RustTheme {
        let mut builder = RustTheme::builder().palette(palette);

        if let Some(radius) = mode_radius.or(self.radius) {
            builder = builder.border_radius(radius);
        }
        if let Some(color) = mode_surface.or(self.surface_color) {
            builder = builder.surface_color(color);
        }
        if let Some(color) = mode_primary.or(self.primary_color) {
            builder = builder.primary_color(color);
        }
        if let Some(color) = mode_text.or(self.text_color) {
            builder = builder.text_color(color);
        }
        if let Some(color) = mode_divider.or(self.divider_color) {
            builder = builder.divider_color(color);
        }
        if let Some(material) = mode_material.or(self.material) {
            builder = builder.material(material);
        }
        if let Some(spacing) = self.spacing {
            builder = builder.spacing(spacing);
        }
        if let Some(font) = self.font {
            builder = builder.font(match font {
                FontFamily::Named(name) => name,
                FontFamily::System => rustheme_core::SYSTEM_FONT_NAME,
            });
        }
        if let Some(font) = self.monospace_font {
            builder = builder.monospace_font(match font {
                FontFamily::Named(name) => name,
                FontFamily::System => rustheme_core::SYSTEM_MONOSPACE_FONT_NAME,
            });
        }
        if let Some(font) = self.heading_font {
            builder = builder.heading_font(match font {
                FontFamily::Named(name) => name,
                FontFamily::System => rustheme_core::SYSTEM_FONT_NAME,
            });
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{Palette, RustTheme, RustThemeConfig};
    use rustheme_core::hex;

    #[test]
    fn builder_applies_global_color_and_radius_overrides() {
        let theme = RustTheme::builder()
            .palette(Palette::brand_dark())
            .primary_color(hex("#B7C4FF"))
            .text_color(hex("#F2F2F2"))
            .border_radius(20.0)
            .build();

        assert_eq!(theme.palette.primary.main, hex("#B7C4FF"));
        assert_eq!(theme.palette.text.primary, hex("#F2F2F2"));
        assert_eq!(theme.shape.large, 20.0);
    }

    #[test]
    fn config_applies_shared_tokens_to_both_modes() {
        let config = RustThemeConfig::new()
            .radius(10.0)
            .primary_color(hex("#3F51B5"));

        let light = config.light();
        let dark = config.dark();

        assert_eq!(light.shape.large, 10.0);
        assert_eq!(dark.shape.large, 10.0);
        assert_eq!(light.palette.primary.main, hex("#3F51B5"));
        assert_eq!(dark.palette.primary.main, hex("#3F51B5"));
    }

    #[test]
    fn config_per_mode_override_wins_over_shared() {
        let config = RustThemeConfig::new()
            .radius(10.0)
            .primary_color(hex("#3F51B5"))
            .dark_radius(4.0)
            .dark_primary_color(hex("#B7C4FF"));

        let light = config.light();
        let dark = config.dark();

        assert_eq!(light.shape.large, 10.0);
        assert_eq!(light.palette.primary.main, hex("#3F51B5"));
        assert_eq!(dark.shape.large, 4.0);
        assert_eq!(dark.palette.primary.main, hex("#B7C4FF"));
    }
}
