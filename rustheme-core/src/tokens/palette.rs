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
    pub fn light() -> Self {
        Self {
            primary: ColorSet {
                main: hex("#1976d2"),
                light: hex("#42a5f5"),
                dark: hex("#1565c0"),
                contrast_text: hex("#ffffff"),
            },
            secondary: ColorSet {
                main: hex("#9c27b0"),
                light: hex("#ba68c8"),
                dark: hex("#7b1fa2"),
                contrast_text: hex("#ffffff"),
            },
            error: ColorSet {
                main: hex("#d32f2f"),
                light: hex("#ef5350"),
                dark: hex("#c62828"),
                contrast_text: hex("#ffffff"),
            },
            warning: ColorSet {
                main: hex("#ed6c02"),
                light: hex("#ff9800"),
                dark: hex("#e65100"),
                contrast_text: hex("#ffffff"),
            },
            info: ColorSet {
                main: hex("#0288d1"),
                light: hex("#03a9f4"),
                dark: hex("#01579b"),
                contrast_text: hex("#ffffff"),
            },
            success: ColorSet {
                main: hex("#2e7d32"),
                light: hex("#4caf50"),
                dark: hex("#1b5e20"),
                contrast_text: hex("#ffffff"),
            },
            background: BackgroundColors {
                default: hex("#f5f5f5"),
                paper: hex("#ffffff"),
            },
            text: TextColors {
                primary: hex("#212121"),
                secondary: hex("#616161"),
                disabled: hex("#9e9e9e"),
            },
            divider: hex("#e0e0e0"),
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: ColorSet {
                main: hex("#90caf9"),
                light: hex("#e3f2fd"),
                dark: hex("#42a5f5"),
                contrast_text: hex("#0a0a0a"),
            },
            secondary: ColorSet {
                main: hex("#ce93d8"),
                light: hex("#f3e5f5"),
                dark: hex("#ab47bc"),
                contrast_text: hex("#0a0a0a"),
            },
            error: ColorSet {
                main: hex("#f44336"),
                light: hex("#e57373"),
                dark: hex("#d32f2f"),
                contrast_text: hex("#0a0a0a"),
            },
            warning: ColorSet {
                main: hex("#ffa726"),
                light: hex("#ffb74d"),
                dark: hex("#f57c00"),
                contrast_text: hex("#0a0a0a"),
            },
            info: ColorSet {
                main: hex("#29b6f6"),
                light: hex("#4fc3f7"),
                dark: hex("#0288d1"),
                contrast_text: hex("#0a0a0a"),
            },
            success: ColorSet {
                main: hex("#66bb6a"),
                light: hex("#81c784"),
                dark: hex("#388e3c"),
                contrast_text: hex("#0a0a0a"),
            },
            background: BackgroundColors {
                default: hex("#121212"),
                paper: hex("#1e1e1e"),
            },
            text: TextColors {
                primary: hex("#ffffff"),
                secondary: hex("#b0b0b0"),
                disabled: hex("#6c6c6c"),
            },
            divider: hex("#333333"),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::light()
    }
}
