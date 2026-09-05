/// Shape tokens controlling corner rounding across components.
///
/// `6.0` matches the corner radius native desktop toolkits use for controls
/// (macOS/Windows/GTK all sit in the 4-8px range for buttons/fields/cards);
/// MUI's web-oriented default of `4.0` reads as flat/web on a desktop
/// window, while very round radii (`12.0`+) read as a mobile/web card
/// aesthetic rather than a native one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    pub border_radius: f32,
}

impl Default for Shape {
    fn default() -> Self {
        Self { border_radius: 6.0 }
    }
}
