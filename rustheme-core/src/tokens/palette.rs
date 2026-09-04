use iced_core::Color;

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
        Self::light()
    }
}
