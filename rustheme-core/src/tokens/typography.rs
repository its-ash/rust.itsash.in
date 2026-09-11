use std::sync::OnceLock;
use iced_core::{font, Font};

/// Font weight, simplified to the values RustTheme's default scale uses.
/// Maps directly onto [`iced_core::font::Weight`] via [`FontWeight::to_iced`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Light,
    Regular,
    Medium,
    Bold,
}

impl FontWeight {
    pub const fn to_iced(self) -> font::Weight {
        match self {
            Self::Light => font::Weight::Light,
            Self::Regular => font::Weight::Normal,
            Self::Medium => font::Weight::Medium,
            Self::Bold => font::Weight::Bold,
        }
    }
}

/// The platform's native UI font family name, resolved at compile time per
/// OS so text renders in the same face as other apps on that platform
/// (San Francisco on macOS, Segoe UI on Windows, the desktop's configured
/// sans on Linux) instead of iced's generic bundled fallback.
#[cfg(target_os = "macos")]
pub const SYSTEM_FONT_NAME: &str = "SF Pro Text";
#[cfg(target_os = "windows")]
pub const SYSTEM_FONT_NAME: &str = "Segoe UI";
#[cfg(target_os = "linux")]
pub const SYSTEM_FONT_NAME: &str = "Cantarell";
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub const SYSTEM_FONT_NAME: &str = "Helvetica";

/// The platform's native monospace font family name, used for tabular
/// numerals, code, and other fixed-width contexts (e.g. table cells).
#[cfg(target_os = "macos")]
pub const SYSTEM_MONOSPACE_FONT_NAME: &str = "SF Mono";
#[cfg(target_os = "windows")]
pub const SYSTEM_MONOSPACE_FONT_NAME: &str = "Consolas";
#[cfg(target_os = "linux")]
pub const SYSTEM_MONOSPACE_FONT_NAME: &str = "Source Code Pro";
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub const SYSTEM_MONOSPACE_FONT_NAME: &str = "Courier New";

/// A font family: either the platform's native system font, or a specific
/// named family (e.g. a font the host application bundles/registers with
/// iced itself). Iced applies fonts by name at widget-construction time
/// rather than lazily through the `Catalog` styling system that colors use,
/// so there is no per-`RustTheme`-instance way to vary the font without
/// threading a theme reference through every component builder. Instead,
/// the *process-wide* default family is configured once via
/// [`set_global_font_family`] — typically in `main()`, before building any
/// UI — and every component reads it through [`system_font`] /
/// [`system_monospace_font`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    /// The OS-native UI font ([`SYSTEM_FONT_NAME`] for the current platform).
    System,
    /// A specific named font family.
    Named(&'static str),
}

impl FontFamily {
    fn to_iced(self) -> font::Family {
        match self {
            Self::System => font::Family::Name(SYSTEM_FONT_NAME),
            Self::Named(name) => font::Family::Name(name),
        }
    }
}

static GLOBAL_UI_FONT: OnceLock<FontFamily> = OnceLock::new();
static GLOBAL_MONO_FONT: OnceLock<FontFamily> = OnceLock::new();
static GLOBAL_HEADING_FONT: OnceLock<FontFamily> = OnceLock::new();

/// Sets the process-wide default UI font family used by every RustTheme
/// component from this point on. Call this once, before building any UI
/// (e.g. at the top of `main()`) — it has no effect if called again after
/// the first call (or after any component has already read the default).
///
/// ```no_run
/// use rustheme_core::{set_global_font_family, FontFamily};
///
/// set_global_font_family(FontFamily::Named("Inter"));
/// ```
pub fn set_global_font_family(family: FontFamily) {
    let _ = GLOBAL_UI_FONT.set(family);
}

/// Sets the process-wide default monospace font family (used for tabular
/// numerals, code, and other fixed-width content). Same one-shot,
/// call-before-building-UI semantics as [`set_global_font_family`].
pub fn set_global_monospace_font_family(family: FontFamily) {
    let _ = GLOBAL_MONO_FONT.set(family);
}

/// Sets the process-wide default **heading** font family — used for
/// `h1`..`h6` and `subtitle1`/`subtitle2` only, so a display/serif face can
/// be paired with a separate body font (e.g. Playfair Display for headings
/// alongside Roboto for everything else, RustTheme's bundled
/// default pairing — see [`crate::fonts`][fonts] in the `rustheme` crate,
/// which embeds and loads both). Falls back to the regular UI font
/// ([`set_global_font_family`]) if never set. Same one-shot,
/// call-before-building-UI semantics as [`set_global_font_family`].
///
/// [fonts]: https://docs.rs/rustheme (the `rustheme::fonts` module)
pub fn set_global_heading_font_family(family: FontFamily) {
    let _ = GLOBAL_HEADING_FONT.set(family);
}

/// Builds the current default UI [`Font`] at the given `weight`: the family
/// set via [`set_global_font_family`], or the platform system font if none
/// was configured. Text widgets should call `.font(...)` with this (via
/// [`Typography`]'s helpers) rather than relying on iced's generic default
/// font, which is what makes RustTheme apps read as "web-styled" instead of
/// native.
pub fn system_font(weight: FontWeight) -> Font {
    let family = GLOBAL_UI_FONT.get().copied().unwrap_or(FontFamily::System);
    Font {
        family: family.to_iced(),
        weight: weight.to_iced(),
        ..Font::DEFAULT
    }
}

/// Builds the current default monospace [`Font`] at the given `weight`: the
/// family set via [`set_global_monospace_font_family`], or the platform
/// system monospace font if none was configured.
pub fn system_monospace_font(weight: FontWeight) -> Font {
    let family = GLOBAL_MONO_FONT.get().copied().unwrap_or(FontFamily::System);
    Font {
        family: family.to_iced(),
        weight: weight.to_iced(),
        ..Font::MONOSPACE
    }
}

/// Builds the current default **heading** [`Font`] at the given `weight`:
/// the family set via [`set_global_heading_font_family`], falling back to
/// the regular UI font family if no distinct heading font was configured.
/// [`TypeStyle::font`] on `h1`..`h6`/`subtitle1`/`subtitle2` styles uses
/// this automatically.
pub fn heading_font(weight: FontWeight) -> Font {
    let family = GLOBAL_HEADING_FONT
        .get()
        .copied()
        .or_else(|| GLOBAL_UI_FONT.get().copied())
        .unwrap_or(FontFamily::System);
    Font {
        family: family.to_iced(),
        weight: weight.to_iced(),
        ..Font::DEFAULT
    }
}

/// Which font role a [`TypeStyle`] renders in: the regular UI/body font, or
/// the (optionally distinct) heading font — see
/// [`set_global_heading_font_family`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontRole {
    Body,
    Heading,
}

/// A single entry in the type scale: a pixel size, a weight, and which font
/// role it renders in. Call [`TypeStyle::font`] to get the matching [`Font`]
/// for `.font(...)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeStyle {
    pub size: f32,
    pub weight: FontWeight,
    pub role: FontRole,
}

impl TypeStyle {
    pub const fn new(size: f32, weight: FontWeight) -> Self {
        Self { size, weight, role: FontRole::Body }
    }

    pub const fn heading(size: f32, weight: FontWeight) -> Self {
        Self { size, weight, role: FontRole::Heading }
    }

    /// The current font for this style's role and weight (heading font for
    /// [`FontRole::Heading`], regular UI font otherwise).
    pub fn font(&self) -> Font {
        match self.role {
            FontRole::Body => system_font(self.weight),
            FontRole::Heading => heading_font(self.weight),
        }
    }
}

/// The Material 3 type scale — 15 named styles from Display Large to
/// Label Small, matching the [M3 type scale tokens](https://m3.material.io/styles/typography/type-scale-tokens).
///
/// Sizes are in px (1sp = 1px on desktop). Weights follow M3 defaults:
/// Display/Headline are Regular (400), Title is Medium (500), Body is
/// Regular (400), Label is Medium (500). Roboto is the default font; the
/// heading (brand) font is Playfair Display when
/// [`set_global_heading_font_family`] is configured.
///
/// The legacy MUI-style fields (`h1`..`h6`, `subtitle1`, `body1`, etc.)
/// are retained as aliases mapping to the closest M3 style for
/// backward-compatibility with existing components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeScale {
    // ── M3 Display (largest, for hero/empty-state text) ──
    pub display_large: TypeStyle,
    pub display_medium: TypeStyle,
    pub display_small: TypeStyle,
    // ── M3 Headline (for section headers, dialog titles) ──
    pub headline_large: TypeStyle,
    pub headline_medium: TypeStyle,
    pub headline_small: TypeStyle,
    // ── M3 Title (for app bar, dialog/card titles) ──
    pub title_large: TypeStyle,
    pub title_medium: TypeStyle,
    pub title_small: TypeStyle,
    // ── M3 Body (for paragraph/body text) ──
    pub body_large: TypeStyle,
    pub body_medium: TypeStyle,
    pub body_small: TypeStyle,
    // ── M3 Label (for buttons, chips, captions, overlines) ──
    pub label_large: TypeStyle,
    pub label_medium: TypeStyle,
    pub label_small: TypeStyle,

    // ── Legacy MUI-style aliases (map to the closest M3 style) ──
    pub h1: TypeStyle,
    pub h2: TypeStyle,
    pub h3: TypeStyle,
    pub h4: TypeStyle,
    pub h5: TypeStyle,
    pub h6: TypeStyle,
    pub subtitle1: TypeStyle,
    pub subtitle2: TypeStyle,
    pub body1: TypeStyle,
    pub body2: TypeStyle,
    pub button: TypeStyle,
    pub caption: TypeStyle,
    pub overline: TypeStyle,
}

impl Default for TypeScale {
    fn default() -> Self {
        // M3 baseline type scale (Roboto defaults):
        //   Display:   57/45/36  Regular
        //   Headline:  32/28/24  Regular
        //   Title:     22/16/14  Medium
        //   Body:      16/14/12  Regular
        //   Label:     14/12/11  Medium
        //
        // For desktop we scale Display down slightly (57px is enormous on a
        // desktop window) but keep the full scale intact for completeness.
        let display_large = TypeStyle::heading(36.0, FontWeight::Regular);
        let display_medium = TypeStyle::heading(32.0, FontWeight::Regular);
        let display_small = TypeStyle::heading(28.0, FontWeight::Regular);
        let headline_large = TypeStyle::heading(24.0, FontWeight::Regular);
        let headline_medium = TypeStyle::heading(20.0, FontWeight::Regular);
        let headline_small = TypeStyle::heading(17.0, FontWeight::Regular);
        let title_large = TypeStyle::heading(18.0, FontWeight::Medium);
        let title_medium = TypeStyle::new(15.0, FontWeight::Medium);
        let title_small = TypeStyle::new(14.0, FontWeight::Medium);
        let body_large = TypeStyle::new(16.0, FontWeight::Regular);
        let body_medium = TypeStyle::new(14.0, FontWeight::Regular);
        let body_small = TypeStyle::new(12.0, FontWeight::Regular);
        let label_large = TypeStyle::new(14.0, FontWeight::Medium);
        let label_medium = TypeStyle::new(12.0, FontWeight::Medium);
        let label_small = TypeStyle::new(11.0, FontWeight::Medium);
        Self {
            display_large,
            display_medium,
            display_small,
            headline_large,
            headline_medium,
            headline_small,
            title_large,
            title_medium,
            title_small,
            body_large,
            body_medium,
            body_small,
            label_large,
            label_medium,
            label_small,
            // Legacy aliases map onto the nearest M3 style.
            h1: display_small,
            h2: headline_large,
            h3: headline_medium,
            h4: headline_small,
            h5: title_large,
            h6: title_medium,
            subtitle1: body_medium,
            subtitle2: body_small,
            body1: body_medium,
            body2: body_small,
            button: label_large,
            caption: label_small,
            overline: label_small,
        }
    }
}

/// Typography tokens: the platform system font (UI + monospace variants)
/// plus the full type scale.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Typography {
    pub scale: TypeScale,
}

impl Typography {
    /// The current UI/body font at `weight`, for ordinary text.
    pub fn ui_font(&self, weight: FontWeight) -> Font {
        system_font(weight)
    }

    /// The current monospace font at `weight`, for tabular/numeric content.
    pub fn mono_font(&self, weight: FontWeight) -> Font {
        system_monospace_font(weight)
    }

    /// The current heading font at `weight` — see
    /// [`set_global_heading_font_family`].
    pub fn heading_font(&self, weight: FontWeight) -> Font {
        heading_font(weight)
    }
}
