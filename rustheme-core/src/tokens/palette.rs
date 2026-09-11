use iced_core::Color;

// ---------------------------------------------------------------------------
// Platform-native surface/text colors with RustTheme's brand accent
//
// Each platform keeps its familiar default surfaces and text. RustTheme uses
// one consistent deep-teal accent instead of the OS blue so primary actions
// retain a clear product identity and always carry light text in light mode.
//
// Disabled text colors are tuned to clear WCAG AA (4.5:1) on the matching
// paper surface; secondary text clears AA on both paper and default bg.
// The deep-teal accent clears WCAG AA with its paired text in both modes.
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod native {
    pub const LIGHT_BG: &str = "#F6F6F6";
    pub const LIGHT_PAPER: &str = "#FFFFFF";
    pub const LIGHT_TEXT_PRIMARY: &str = "#000000";
    pub const LIGHT_TEXT_SECONDARY: &str = "#3A3A3A";
    pub const LIGHT_TEXT_DISABLED: &str = "#7A6E6C";
    pub const LIGHT_DIVIDER: &str = "#D1D1D1";
    pub const DARK_BG: &str = "#1E1E1E";
    pub const DARK_PAPER: &str = "#2D2D2D";
    pub const DARK_TEXT_PRIMARY: &str = "#FFFFFF";
    pub const DARK_TEXT_SECONDARY: &str = "#CCCCCC";
    pub const DARK_TEXT_DISABLED: &str = "#8A8A8A";
    pub const DARK_DIVIDER: &str = "#3A3A3A";
    pub const ACCENT_LIGHT: &str = "#006A6A";
    pub const ACCENT_DARK: &str = "#4FD8D5";
    pub const ACCENT_CONTRAST_LIGHT: &str = "#FFFFFF";
    pub const ACCENT_CONTRAST_DARK: &str = "#003737";
}

#[cfg(target_os = "windows")]
mod native {
    pub const LIGHT_BG: &str = "#F3F3F3";
    pub const LIGHT_PAPER: &str = "#FFFFFF";
    pub const LIGHT_TEXT_PRIMARY: &str = "#000000";
    pub const LIGHT_TEXT_SECONDARY: &str = "#3A3A3A";
    pub const LIGHT_TEXT_DISABLED: &str = "#767676";
    pub const LIGHT_DIVIDER: &str = "#E5E5E5";
    pub const DARK_BG: &str = "#202020";
    pub const DARK_PAPER: &str = "#2B2B2B";
    pub const DARK_TEXT_PRIMARY: &str = "#FFFFFF";
    pub const DARK_TEXT_SECONDARY: &str = "#C7C7C7";
    pub const DARK_TEXT_DISABLED: &str = "#7A7A7A";
    pub const DARK_DIVIDER: &str = "#3F3F3F";
    pub const ACCENT_LIGHT: &str = "#006A6A";
    pub const ACCENT_DARK: &str = "#4FD8D5";
    pub const ACCENT_CONTRAST_LIGHT: &str = "#FFFFFF";
    pub const ACCENT_CONTRAST_DARK: &str = "#003737";
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod native {
    // GTK/Adwaita defaults (Linux and other platforms)
    pub const LIGHT_BG: &str = "#FAFAFA";
    pub const LIGHT_PAPER: &str = "#FFFFFF";
    pub const LIGHT_TEXT_PRIMARY: &str = "#000000";
    pub const LIGHT_TEXT_SECONDARY: &str = "#3A3A3A";
    pub const LIGHT_TEXT_DISABLED: &str = "#7A7A7A";
    pub const LIGHT_DIVIDER: &str = "#D4D4D4";
    pub const DARK_BG: &str = "#242424";
    pub const DARK_PAPER: &str = "#303030";
    pub const DARK_TEXT_PRIMARY: &str = "#FFFFFF";
    pub const DARK_TEXT_SECONDARY: &str = "#C9C9C9";
    pub const DARK_TEXT_DISABLED: &str = "#7C7C7C";
    pub const DARK_DIVIDER: &str = "#3D3D3D";
    pub const ACCENT_LIGHT: &str = "#006A6A";
    pub const ACCENT_DARK: &str = "#4FD8D5";
    pub const ACCENT_CONTRAST_LIGHT: &str = "#FFFFFF";
    pub const ACCENT_CONTRAST_DARK: &str = "#003737";
}

use native as n;

/// Light-mode native background `default` color for the current platform.
pub const SYSTEM_LIGHT_BG: &str = n::LIGHT_BG;
/// Light-mode native `paper` (card/elevated surface) color.
pub const SYSTEM_LIGHT_PAPER: &str = n::LIGHT_PAPER;
/// Light-mode native primary text color (near-black on all platforms).
pub const SYSTEM_LIGHT_TEXT_PRIMARY: &str = n::LIGHT_TEXT_PRIMARY;
/// Light-mode native secondary text color.
pub const SYSTEM_LIGHT_TEXT_SECONDARY: &str = n::LIGHT_TEXT_SECONDARY;
/// Light-mode disabled text color (WCAG AA on paper).
pub const SYSTEM_LIGHT_TEXT_DISABLED: &str = n::LIGHT_TEXT_DISABLED;
/// Light-mode divider/hairline color.
pub const SYSTEM_LIGHT_DIVIDER: &str = n::LIGHT_DIVIDER;
/// Light-mode RustTheme primary accent: deep teal #006A6A.
pub const SYSTEM_LIGHT_ACCENT: &str = n::ACCENT_LIGHT;
/// Light-mode contrast text for the primary accent fill.
pub const SYSTEM_LIGHT_ACCENT_CONTRAST: &str = n::ACCENT_CONTRAST_LIGHT;

/// Dark-mode native background `default` color for the current platform.
pub const SYSTEM_DARK_BG: &str = n::DARK_BG;
/// Dark-mode native `paper` (card/elevated surface) color.
pub const SYSTEM_DARK_PAPER: &str = n::DARK_PAPER;
/// Dark-mode native primary text color (white on all platforms).
pub const SYSTEM_DARK_TEXT_PRIMARY: &str = n::DARK_TEXT_PRIMARY;
/// Dark-mode native secondary text color.
pub const SYSTEM_DARK_TEXT_SECONDARY: &str = n::DARK_TEXT_SECONDARY;
/// Dark-mode disabled text color (WCAG AA on paper).
pub const SYSTEM_DARK_TEXT_DISABLED: &str = n::DARK_TEXT_DISABLED;
/// Dark-mode divider/hairline color.
pub const SYSTEM_DARK_DIVIDER: &str = n::DARK_DIVIDER;
/// Dark-mode RustTheme primary accent: bright teal #4FD8D5.
pub const SYSTEM_DARK_ACCENT: &str = n::ACCENT_DARK;
/// Dark-mode contrast text for the primary accent fill.
pub const SYSTEM_DARK_ACCENT_CONTRAST: &str = n::ACCENT_CONTRAST_DARK;

/// Parses a `#rrggbb` or `#rrggbbaa` hex string into an [`iced_core::Color`].
///
/// Panics on malformed input — this is only ever called with hardcoded
/// literals in this module, so a panic here indicates a programming error.
pub fn hex(s: &str) -> Color {
    let s = s.trim_start_matches('#');
    let (r, g, b, a) = match s.len() {
        6 => (
            u8::from_str_radix(&s[0..2], 16).unwrap(),
            u8::from_str_radix(&s[2..4], 16).unwrap(),
            u8::from_str_radix(&s[4..6], 16).unwrap(),
            255u8,
        ),
        8 => (
            u8::from_str_radix(&s[0..2], 16).unwrap(),
            u8::from_str_radix(&s[2..4], 16).unwrap(),
            u8::from_str_radix(&s[4..6], 16).unwrap(),
            u8::from_str_radix(&s[6..8], 16).unwrap(),
        ),
        _ => panic!("invalid hex color: {s}"),
    };
    Color::from_rgba8(r, g, b, a as f32 / 255.0)
}

/// Non-panicking counterpart to [`hex`], for parsing live user input (e.g.
/// a color picker's hex text field) where malformed text is expected and
/// must not crash the app. Returns `None` instead of panicking.
pub fn parse_hex(s: &str) -> Option<Color> {
    let s = s.trim_start_matches('#');
    let component = |range: std::ops::Range<usize>| s.get(range).and_then(|h| u8::from_str_radix(h, 16).ok());
    match s.len() {
        6 => Some(Color::from_rgb8(component(0..2)?, component(2..4)?, component(4..6)?)),
        8 => Some(Color::from_rgba8(
            component(0..2)?,
            component(2..4)?,
            component(4..6)?,
            component(6..8)? as f32 / 255.0,
        )),
        _ => None,
    }
}

/// Formats a [`Color`] back to a `#rrggbb` hex string (alpha is dropped —
/// use this for opaque swatches/pickers, not for round-tripping translucent
/// colors).
pub fn to_hex(color: Color) -> String {
    let [r, g, b, _] = color.into_rgba8();
    format!("#{r:02X}{g:02X}{b:02X}")
}

/// A MUI-style color group: a main color plus lighter/darker variants and
/// the text color that should be used on top of `main`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorSet {
    pub main: Color,
    pub light: Color,
    pub dark: Color,
    pub contrast_text: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BackgroundColors {
    pub default: Color,
    pub paper: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextColors {
    pub primary: Color,
    pub secondary: Color,
    pub disabled: Color,
}

fn blend(under: Color, over: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);
    Color::from_rgba(
        under.r * (1.0 - amount) + over.r * amount,
        under.g * (1.0 - amount) + over.g * amount,
        under.b * (1.0 - amount) + over.b * amount,
        1.0,
    )
}

impl ColorSet {
    /// Builds a complete semantic color set from one main color. The light
    /// and dark tones are derived for hover/tinted treatments, and the
    /// contrast text is selected automatically for the best readable pair.
    pub fn from_main(main: Color) -> Self {
        let white = Color::WHITE;
        let black = Color::BLACK;
        let contrast_text = if crate::tokens::contrast::contrast_ratio(white, main)
            >= crate::tokens::contrast::contrast_ratio(black, main)
        {
            white
        } else {
            black
        };

        Self {
            main,
            light: blend(main, white, 0.22),
            dark: blend(main, black, 0.22),
            contrast_text,
        }
    }
}

/// The full set of semantic colors used across RustTheme components,
/// modeled after Material UI's default palette.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub primary: ColorSet,
    pub secondary: ColorSet,
    pub error: ColorSet,
    pub warning: ColorSet,
    pub info: ColorSet,
    pub success: ColorSet,
    pub background: BackgroundColors,
    pub text: TextColors,
    pub divider: Color,
}

impl Palette {
    /// Replaces the global primary color while deriving matching light, dark,
    /// and contrast-text tokens.
    pub fn with_primary(mut self, color: Color) -> Self {
        self.primary = ColorSet::from_main(color);
        self
    }

    /// Replaces the global text color and derives secondary and disabled
    /// tones against the current paper surface.
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text = TextColors {
            primary: color,
            secondary: blend(color, self.background.paper, 0.35),
            disabled: blend(color, self.background.paper, 0.60),
        };
        self
    }

    /// Replaces the global canvas and surface colors together.
    pub fn with_surface_color(mut self, color: Color) -> Self {
        self.background = BackgroundColors {
            default: color,
            paper: color,
        };
        self
    }

    /// Replaces the global divider color.
    pub fn with_divider_color(mut self, color: Color) -> Self {
        self.divider = color;
        self
    }
    /// The native platform palette (light mode): surfaces, text, and accent
    /// colors that match the OS's default window appearance so a RustTheme
    /// app blends fully with native apps on the current platform.
    ///
    /// - **Primary** = RustTheme's deep teal accent (#006A6A), used for
    ///   primary buttons, links, and focus rings with white text.
    /// - **Secondary** = a neutral secondary derived from the text color,
    ///   not a competing brand color.
    /// - Backgrounds, text, and dividers are the platform's native window
    ///   chrome colors.
    ///
    /// Error/warning/info/success keep standard semantic hues.
    pub fn brand_light() -> Self {
        let p = Self {
            primary: ColorSet {
                main: hex(SYSTEM_LIGHT_ACCENT),
                light: hex(SYSTEM_LIGHT_ACCENT),
                dark: hex(SYSTEM_LIGHT_ACCENT),
                contrast_text: hex(SYSTEM_LIGHT_ACCENT_CONTRAST),
            },
            secondary: ColorSet {
                main: hex(SYSTEM_LIGHT_TEXT_SECONDARY),
                light: hex(SYSTEM_LIGHT_TEXT_SECONDARY),
                dark: hex(SYSTEM_LIGHT_TEXT_SECONDARY),
                contrast_text: hex(SYSTEM_LIGHT_PAPER),
            },
            error: ColorSet {
                main: hex("#d93b3b"),
                light: hex("#ff6b6b"),
                dark: hex("#a51e1e"),
                contrast_text: hex("#ffffff"),
            },
            warning: ColorSet {
                main: hex("#e8862a"),
                light: hex("#ffab5c"),
                dark: hex("#b5641a"),
                contrast_text: hex("#1a0e00"),
            },
            info: ColorSet {
                main: hex("#008857"),
                light: hex("#3aab7e"),
                dark: hex("#00663f"),
                contrast_text: hex("#ffffff"),
            },
            success: ColorSet {
                main: hex("#2fa06a"),
                light: hex("#5cc494"),
                dark: hex("#1c7048"),
                contrast_text: hex("#ffffff"),
            },
            background: BackgroundColors {
                default: hex(SYSTEM_LIGHT_BG),
                paper: hex(SYSTEM_LIGHT_PAPER),
            },
            text: TextColors {
                primary: hex(SYSTEM_LIGHT_TEXT_PRIMARY),
                secondary: hex(SYSTEM_LIGHT_TEXT_SECONDARY),
                disabled: hex(SYSTEM_LIGHT_TEXT_DISABLED),
            },
            divider: hex(SYSTEM_LIGHT_DIVIDER),
        };
        crate::tokens::contrast::assert_aa(p.text.primary, p.background.paper, "light text.primary on paper");
        crate::tokens::contrast::assert_aa(p.text.primary, p.background.default, "light text.primary on default bg");
        crate::tokens::contrast::assert_aa(p.text.secondary, p.background.paper, "light text.secondary on paper");
        crate::tokens::contrast::assert_aa_large(p.text.disabled, p.background.paper, "light text.disabled on paper");
        crate::tokens::contrast::assert_aa(p.primary.contrast_text, p.primary.main, "light primary.contrast_text on primary");
        crate::tokens::contrast::assert_aa(p.error.contrast_text, p.error.main, "light error.contrast_text on error");
        p
    }

    /// The native platform palette in dark mode: RustTheme's bright teal
    /// accent on the platform's native dark window surfaces. The accent uses
    /// a dark teal text color for readable contrast.
    pub fn brand_dark() -> Self {
        let p = Self {
            primary: ColorSet {
                main: hex(SYSTEM_DARK_ACCENT),
                light: hex(SYSTEM_DARK_ACCENT),
                dark: hex(SYSTEM_DARK_ACCENT),
                contrast_text: hex(SYSTEM_DARK_ACCENT_CONTRAST),
            },
            secondary: ColorSet {
                main: hex(SYSTEM_DARK_TEXT_SECONDARY),
                light: hex(SYSTEM_DARK_TEXT_SECONDARY),
                dark: hex(SYSTEM_DARK_TEXT_SECONDARY),
                contrast_text: hex(SYSTEM_DARK_BG),
            },
            error: ColorSet {
                main: hex("#ff6b6b"),
                light: hex("#ff9d9d"),
                dark: hex("#d93b3b"),
                contrast_text: hex("#3a0000"),
            },
            warning: ColorSet {
                main: hex("#ffab5c"),
                light: hex("#ffc88a"),
                dark: hex("#e8862a"),
                contrast_text: hex("#2a1600"),
            },
            info: ColorSet {
                main: hex("#3aab7e"),
                light: hex("#6dc89f"),
                dark: hex("#008857"),
                contrast_text: hex("#002a16"),
            },
            success: ColorSet {
                main: hex("#5cc494"),
                light: hex("#8dd9b0"),
                dark: hex("#2fa06a"),
                contrast_text: hex("#002a16"),
            },
            background: BackgroundColors {
                default: hex(SYSTEM_DARK_BG),
                paper: hex(SYSTEM_DARK_PAPER),
            },
            text: TextColors {
                primary: hex(SYSTEM_DARK_TEXT_PRIMARY),
                secondary: hex(SYSTEM_DARK_TEXT_SECONDARY),
                disabled: hex(SYSTEM_DARK_TEXT_DISABLED),
            },
            divider: hex(SYSTEM_DARK_DIVIDER),
        };
        crate::tokens::contrast::assert_aa(p.text.primary, p.background.paper, "dark text.primary on paper");
        crate::tokens::contrast::assert_aa(p.text.primary, p.background.default, "dark text.primary on default bg");
        crate::tokens::contrast::assert_aa(p.text.secondary, p.background.paper, "dark text.secondary on paper");
        crate::tokens::contrast::assert_aa_large(p.text.disabled, p.background.paper, "dark text.disabled on paper");
        crate::tokens::contrast::assert_aa(p.primary.contrast_text, p.primary.main, "dark primary.contrast_text on primary");
        crate::tokens::contrast::assert_aa(p.error.contrast_text, p.error.main, "dark error.contrast_text on error");
        p
    }

    /// A modern, vibrant Material 3 -inspired light palette: dark text on
    /// light/white surfaces, and white text on saturated color fills.
    pub fn light() -> Self {
        Self {
            primary: ColorSet {
                main: hex("#6750a4"),
                light: hex("#9a82db"),
                dark: hex("#4f378b"),
                contrast_text: hex("#ffffff"),
            },
            secondary: ColorSet {
                main: hex("#00897b"),
                light: hex("#4ebaaa"),
                dark: hex("#005b4f"),
                contrast_text: hex("#ffffff"),
            },
            error: ColorSet {
                main: hex("#ba1a1a"),
                light: hex("#ff5449"),
                dark: hex("#93000a"),
                contrast_text: hex("#ffffff"),
            },
            warning: ColorSet {
                main: hex("#f9a825"),
                light: hex("#ffd95a"),
                dark: hex("#c17900"),
                contrast_text: hex("#3e2723"),
            },
            info: ColorSet {
                main: hex("#0061a4"),
                light: hex("#5eb3ff"),
                dark: hex("#00447c"),
                contrast_text: hex("#ffffff"),
            },
            success: ColorSet {
                main: hex("#2e7d32"),
                light: hex("#60ad5e"),
                dark: hex("#005005"),
                contrast_text: hex("#ffffff"),
            },
            background: BackgroundColors {
                default: hex("#fef7ff"),
                paper: hex("#ffffff"),
            },
            text: TextColors {
                primary: hex("#1c1b1f"),
                secondary: hex("#49454f"),
                disabled: hex("#79747e"),
            },
            divider: hex("#cac4d0"),
        }
    }

    /// A modern, vibrant Material 3 -inspired dark palette: light text on
    /// dark surfaces, and dark text on the brighter, desaturated color
    /// fills M3 uses in dark mode for contrast and reduced eye strain.
    pub fn dark() -> Self {
        Self {
            primary: ColorSet {
                main: hex("#d0bcff"),
                light: hex("#eaddff"),
                dark: hex("#9a82db"),
                contrast_text: hex("#381e72"),
            },
            secondary: ColorSet {
                main: hex("#4ebaaa"),
                light: hex("#a2f2e2"),
                dark: hex("#00897b"),
                contrast_text: hex("#00382e"),
            },
            error: ColorSet {
                main: hex("#ffb4ab"),
                light: hex("#ffdad6"),
                dark: hex("#ff5449"),
                contrast_text: hex("#690005"),
            },
            warning: ColorSet {
                main: hex("#ffd95a"),
                light: hex("#ffe999"),
                dark: hex("#f9a825"),
                contrast_text: hex("#402d00"),
            },
            info: ColorSet {
                main: hex("#9fcaff"),
                light: hex("#d1e4ff"),
                dark: hex("#5eb3ff"),
                contrast_text: hex("#003258"),
            },
            success: ColorSet {
                main: hex("#8bd689"),
                light: hex("#c2f2c0"),
                dark: hex("#4ca84a"),
                contrast_text: hex("#00390a"),
            },
            background: BackgroundColors {
                default: hex("#141218"),
                paper: hex("#211f26"),
            },
            text: TextColors {
                primary: hex("#e6e1e5"),
                secondary: hex("#cac4d0"),
                disabled: hex("#79747e"),
            },
            divider: hex("#49454f"),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::brand_light()
    }
}

#[cfg(test)]
mod tests {
    use super::{ColorSet, Palette};
    use crate::tokens::contrast::contrast_ratio;

    #[test]
    fn derived_primary_selects_readable_contrast_text() {
        let set = ColorSet::from_main(super::hex("#3F51B5"));
        assert!(contrast_ratio(set.main, set.contrast_text) >= 4.5);
    }

    #[test]
    fn palette_overrides_keep_unrelated_semantic_colors() {
        let palette = Palette::brand_light();
        let adjusted = palette.with_primary(super::hex("#B23A48"));
        assert_eq!(adjusted.error, palette.error);
        assert_ne!(adjusted.primary, palette.primary);
    }
}
