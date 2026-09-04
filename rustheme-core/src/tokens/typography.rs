/// Font weight, simplified to the two values RustTheme's default scale uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Light,
    Regular,
    Medium,
    Bold,
}

/// A single entry in the type scale: a pixel size and a weight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeStyle {
    pub size: f32,
    pub weight: FontWeight,
}

impl TypeStyle {
    pub const fn new(size: f32, weight: FontWeight) -> Self {
        Self { size, weight }
    }
}

/// Mirrors MUI's default typography scale (h1..h6, subtitle1/2, body1/2,
/// button, caption, overline).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeScale {
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
        Self {
            h1: TypeStyle::new(96.0, FontWeight::Light),
            h2: TypeStyle::new(60.0, FontWeight::Light),
            h3: TypeStyle::new(48.0, FontWeight::Regular),
            h4: TypeStyle::new(34.0, FontWeight::Regular),
            h5: TypeStyle::new(24.0, FontWeight::Regular),
            h6: TypeStyle::new(20.0, FontWeight::Medium),
            subtitle1: TypeStyle::new(16.0, FontWeight::Regular),
            subtitle2: TypeStyle::new(14.0, FontWeight::Medium),
            body1: TypeStyle::new(16.0, FontWeight::Regular),
            body2: TypeStyle::new(14.0, FontWeight::Regular),
            button: TypeStyle::new(14.0, FontWeight::Medium),
            caption: TypeStyle::new(12.0, FontWeight::Regular),
            overline: TypeStyle::new(12.0, FontWeight::Regular),
        }
    }
}

/// Typography tokens: a font family label plus the full type scale.
///
/// The `font_family` field is a display label only (iced's default font is
/// always used for rendering); it exists so themes/consumers can document
/// or branch on intended family.
#[derive(Debug, Clone, PartialEq)]
pub struct Typography {
    pub font_family: String,
    pub scale: TypeScale,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "Roboto, Helvetica, Arial, sans-serif".to_string(),
            scale: TypeScale::default(),
        }
    }
}
