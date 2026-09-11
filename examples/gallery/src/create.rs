//! "Create" — a pro-level button theme builder. Nineteen named design-style
//! presets (Flat Design, Neomorphism, Glassmorphism, Cyberpunk, ...) each
//! carry their own border, shadow, height, and color config; selecting one
//! loads its preset into a single editable [`ButtonConfig`] that every
//! preview button on the canvas renders from — no per-button theme
//! selection. A small color palette lets the accent hue be swapped
//! independently of the structural preset.

use iced::widget::button::{Status, Style};
use iced::{Background, Border, Color, Shadow, Vector};
use rustheme_core::{hex, Material};

/// Fully editable visual config for the preview buttons. Every field here
/// is a knob the "Advanced" panel exposes; theme presets are just canned
/// values for this same struct.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonConfig {
    pub background: Color,
    pub text_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub radius: f32,
    pub shadow_color: Color,
    pub shadow_blur: f32,
    pub shadow_offset: (f32, f32),
    pub height: f32,
    pub uppercase: bool,
}

impl ButtonConfig {
    /// The "Default" preset — RustTheme's own standard button: filled,
    /// fully rounded, no shadow, no border. Every other preset is a
    /// deliberate departure from this baseline.
    pub const STANDARD: Self = Self {
        background: Color::from_rgb(0.0, 0.42, 0.42),
        text_color: Color::WHITE,
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        radius: 999.0,
        shadow_color: Color::TRANSPARENT,
        shadow_blur: 0.0,
        shadow_offset: (0.0, 0.0),
        height: 44.0,
        uppercase: false,
    };

    /// Swap only the accent hue, keeping every structural property (border,
    /// shadow, height, radius) from the current preset. Border and shadow
    /// colors that visibly derive from the accent (non-transparent, not a
    /// plain neutral) are re-derived too, so the swatch reads as "this
    /// button's color" rather than only recoloring the fill.
    pub fn with_accent(mut self, accent: Color) -> Self {
        let had_tinted_border = self.border_color != Color::TRANSPARENT && self.border_color != self.background;
        self.background = accent;
        self.text_color = contrasting_text(accent);
        if had_tinted_border {
            self.border_color = accent;
        }
        self
    }

    /// Projects this preset onto the app-wide [`Material`] token (border
    /// width, resting shadow, and pill-radius override), so selecting a
    /// preset can drive `RustThemeConfig::material` and restyle every
    /// component in the app — not just the Create section's own preview
    /// buttons — including components that are pill-shaped by default
    /// (buttons, chips, bottom nav, search bar). Without the radius
    /// override those stayed permanently pill-shaped regardless of the
    /// preset's own `radius`, which broke square-cornered styles like
    /// Brutalism, Bauhaus, and Retro/8-Bit.
    pub fn app_material(self) -> Material {
        Material {
            border_width: self.border_width,
            shadow: self.to_style().shadow,
            pill_radius_override: Some(self.radius),
        }
    }

    fn to_style(self) -> Style {
        Style {
            background: Some(Background::Color(self.background)),
            text_color: self.text_color,
            border: Border {
                color: self.border_color,
                width: self.border_width,
                radius: self.radius.into(),
            },
            shadow: Shadow {
                color: self.shadow_color,
                offset: Vector::new(self.shadow_offset.0, self.shadow_offset.1),
                blur_radius: self.shadow_blur,
            },
        }
    }

    /// The `iced::widget::button::Style` closure for this config, with
    /// simple hover/press/disabled feedback layered on top.
    pub fn style_fn(self) -> impl Fn(&rustheme::theme::RustTheme, Status) -> Style {
        move |_theme, status| {
            let base = self.to_style();
            match status {
                Status::Active => base,
                Status::Hovered => Style {
                    background: base.background.map(|bg| match bg {
                        Background::Color(c) => Background::Color(wash(c, Color::BLACK, 0.08)),
                        other => other,
                    }),
                    ..base
                },
                Status::Pressed => Style {
                    background: base.background.map(|bg| match bg {
                        Background::Color(c) => Background::Color(wash(c, Color::BLACK, 0.16)),
                        other => other,
                    }),
                    shadow: Shadow { blur_radius: base.shadow.blur_radius * 0.5, ..base.shadow },
                    ..base
                },
                Status::Disabled => Style {
                    background: base.background.map(|bg| match bg {
                        Background::Color(c) => Background::Color(Color { a: 0.38, ..c }),
                        other => other,
                    }),
                    text_color: Color { a: 0.6, ..base.text_color },
                    ..base
                },
            }
        }
    }
}

fn wash(under: Color, over: Color, a: f32) -> Color {
    Color::from_rgba(
        under.r * (1.0 - a) + over.r * a,
        under.g * (1.0 - a) + over.g * a,
        under.b * (1.0 - a) + over.b * a,
        under.a,
    )
}

/// Relative luminance (WCAG) pick between black and white text for a given
/// fill, so an accent swap always stays legible.
fn contrasting_text(c: Color) -> Color {
    let luminance = 0.2126 * c.r + 0.7152 * c.g + 0.0722 * c.b;
    if luminance > 0.55 { Color::BLACK } else { Color::WHITE }
}

/// One named design-style preset: a display label plus a concrete
/// [`ButtonConfig`]. Values are deliberate stylistic choices, not derived
/// from tokens — that is the point of a style gallery.
#[derive(Debug, Clone, Copy)]
pub struct ThemePreset {
    pub name: &'static str,
    pub config: ButtonConfig,
}

macro_rules! preset {
    ($name:expr, $bg:expr, $text:expr, $border_color:expr, $border_width:expr, $radius:expr, $shadow_color:expr, $shadow_blur:expr, $shadow_off:expr, $height:expr, $uppercase:expr) => {
        ThemePreset {
            name: $name,
            config: ButtonConfig {
                background: $bg,
                text_color: $text,
                border_color: $border_color,
                border_width: $border_width,
                radius: $radius,
                shadow_color: $shadow_color,
                shadow_blur: $shadow_blur,
                shadow_offset: $shadow_off,
                height: $height,
                uppercase: $uppercase,
            },
        }
    };
}

/// All nineteen presets, in the order the spec listed them. `Default` is
/// [`ButtonConfig::STANDARD`]; every other preset departs from it along the
/// axis that defines that design movement. `radius` is load-bearing, not
/// cosmetic: it now drives the app-wide pill-radius override (see
/// [`ButtonConfig::app_material`]), so a style whose whole identity is
/// sharp corners (Brutalism, Bauhaus, Retro/8-Bit) MUST use `0.0` here —
/// a nonzero radius silently turns it back into a rounded/pill button
/// everywhere in the app.
pub fn presets() -> Vec<ThemePreset> {
    vec![
        ThemePreset { name: "Default", config: ButtonConfig::STANDARD },

        // Flat Design — solid fill, no border, no shadow, gently rounded.
        // The one deliberately "boring" preset: flat color is the entire idea.
        preset!("Flat Design", hex("#3B82F6"), Color::WHITE, Color::TRANSPARENT, 0.0, 6.0, Color::TRANSPARENT, 0.0, (0.0, 0.0), 42.0, false),

        // Material Design — M3 filled button: saturated purple, small
        // radius, a real (if subtle) key+ambient elevation shadow, no
        // border. Distinct from Flat by its shadow and from Skeuomorphism
        // by staying matte/flat-shaded rather than gradient-lit.
        preset!("Material Design", hex("#6750A4"), Color::WHITE, Color::TRANSPARENT, 0.0, 20.0, Color::from_rgba(0.0, 0.0, 0.0, 0.30), 5.0, (0.0, 2.0), 40.0, false),

        // Neomorphism — same-tone fill as its surroundings (soft gray),
        // no border, a big soft dual-toned shadow standing in for the
        // light/dark emboss pair. Heavily rounded, low contrast text.
        preset!("Neomorphism", hex("#E0E5EC"), hex("#5C6470"), Color::TRANSPARENT, 0.0, 16.0, Color::from_rgba(0.0, 0.0, 0.0, 0.15), 16.0, (8.0, 8.0), 48.0, false),

        // Glassmorphism — translucent white fill, thin bright edge
        // (stands in for the frosted highlight), soft diffuse shadow to
        // read as floating glass. Large radius (glass panels are soft).
        preset!("Glassmorphism", Color::from_rgba(1.0, 1.0, 1.0, 0.22), Color::WHITE, Color::from_rgba(1.0, 1.0, 1.0, 0.45), 1.0, 18.0, Color::from_rgba(0.0, 0.0, 0.0, 0.18), 24.0, (0.0, 10.0), 46.0, false),

        // Brutalism / Neo-Brutalism — sharp corners (radius 0, the whole
        // point), thick pure-black border, a hard offset shadow with NO
        // blur (a drawn box, not a soft glow) so it reads as a distinct
        // silhouette rather than blur bleed.
        preset!("Brutalism / Neo-Brutalism", hex("#FFDE03"), Color::BLACK, Color::BLACK, 3.0, 0.0, Color::BLACK, 0.0, (6.0, 6.0), 46.0, true),

        // Maximalism — clashing saturated magenta fill against a
        // saturated yellow border, on a purple hard shadow: three
        // unrelated hues at once is the point. Big soft radius so it
        // doesn't read as Brutalist.
        preset!("Maximalism", hex("#FF2E93"), Color::WHITE, hex("#FFD400"), 4.0, 26.0, hex("#7B2FF7"), 0.0, (5.0, 5.0), 50.0, false),

        // Skeuomorphism — a believable "physical button": mid-blue fill,
        // darker-blue edge, soft directional drop shadow, small radius
        // (real buttons aren't pills). Distinct from the Classic 3D gold
        // variant below by hue and by a lighter, shorter shadow.
        preset!("Skeuomorphism", hex("#4A90D9"), Color::WHITE, hex("#2F5F8F"), 1.0, 8.0, Color::from_rgba(0.0, 0.0, 0.0, 0.35), 4.0, (0.0, 3.0), 44.0, false),

        // Skeuominimalism — skeuomorphism's restraint: a near-white fill,
        // a hairline border for edge definition, a barely-there shadow.
        // Small radius, no color — the "skeu" is in the subtlety, not a hue.
        preset!("Skeuominimalism", hex("#F5F5F7"), hex("#1D1D1F"), hex("#D9D9DE"), 1.0, 10.0, Color::from_rgba(0.0, 0.0, 0.0, 0.10), 4.0, (0.0, 1.0), 44.0, false),

        // Dark Mode / High Contrast — pure black fill, pure white 2px
        // border, pure white text: maximum contrast, zero decoration.
        // Small radius so it reads as a deliberate UI chrome, not a pill.
        preset!("Dark Mode / High Contrast", Color::BLACK, Color::WHITE, Color::WHITE, 2.0, 4.0, Color::TRANSPARENT, 0.0, (0.0, 0.0), 44.0, false),

        // Retro / 8-Bit (Pixel Art) — radius 0 is mandatory (pixels don't
        // round), thick black pixel-outline border, a hard offset "pixel
        // shadow" with zero blur so it looks stepped, not soft.
        preset!("Retro / 8-Bit (Pixel Art)", hex("#00B140"), Color::BLACK, Color::BLACK, 4.0, 0.0, Color::BLACK, 0.0, (4.0, 4.0), 44.0, true),

        // Cyberpunk / Synthwave — near-black fill so the neon border and
        // glow shadow (same cyan hue, wide blur, no offset) do the work.
        // Small radius keeps a technical/HUD edge rather than a soft pill.
        preset!("Cyberpunk / Synthwave (Neon Glow)", hex("#0D0221"), hex("#00F0FF"), hex("#FF00E5"), 2.0, 6.0, hex("#00F0FF"), 22.0, (0.0, 0.0), 46.0, true),

        // Memorphism (3D Cartoon / Claymorphism) — a candy-pink fill,
        // very large radius (clay is soft and round), no border, a big
        // soft shadow pushed well down to read as a puffy 3D blob.
        preset!("Memorphism (3D Cartoon / Claymorphism)", hex("#FFB4D6"), hex("#5B2A3D"), Color::TRANSPARENT, 0.0, 30.0, Color::from_rgba(0.85, 0.4, 0.6, 0.55), 20.0, (0.0, 12.0), 52.0, false),

        // Bauhaus / Swiss Style — primary red fill, pure-black hairline
        // grid border, radius 0 (Bauhaus is geometric, never rounded), no
        // shadow (flat print-like color blocking).
        preset!("Bauhaus / Swiss Style (Grid-Based)", hex("#E30613"), Color::WHITE, Color::BLACK, 2.0, 0.0, Color::TRANSPARENT, 0.0, (0.0, 0.0), 44.0, true),

        // Organic / Biomorphic — muted sage green, no border, a very
        // large (near-blob) radius, soft diffuse shadow. Distinct from
        // Memorphism by a desaturated earthy palette instead of candy tones.
        preset!("Organic / Biomorphic (Fluid Shapes)", hex("#8FB996"), hex("#1F3327"), Color::TRANSPARENT, 0.0, 34.0, Color::from_rgba(0.1, 0.3, 0.2, 0.20), 14.0, (0.0, 7.0), 48.0, false),

        // Typographic / Text-First — no fill, no border, no shadow, no
        // radius (there's no visible shape at all): the label alone,
        // colored ink on the page. The most stripped-down preset.
        preset!("Typographic / Text-First", Color::TRANSPARENT, hex("#111111"), Color::TRANSPARENT, 0.0, 0.0, Color::TRANSPARENT, 0.0, (0.0, 0.0), 40.0, false),

        // Minimalism (Monochrome) — pure white fill, a single thin
        // near-black hairline border, small radius, no shadow, no color
        // anywhere: black/white/gray only, by definition.
        preset!("Minimalism (Monochrome)", Color::WHITE, Color::BLACK, hex("#111111"), 1.0, 4.0, Color::TRANSPARENT, 0.0, (0.0, 0.0), 42.0, false),

        // Papercut / Layered Vector — a warm cream fill (paper, not
        // screen-blue) with a soft brown ink-line border standing in for
        // a printed cut edge, and an offset-but-crisp shadow (short blur,
        // clear offset) reading as one paper layer sitting on another —
        // deliberately NOT a diffuse M3 shadow, which is what made it
        // read as "just Material" before. No color saturation, no gloss.
        preset!("Papercut / Layered Vector", hex("#FFF3DC"), hex("#6B4423"), hex("#C9A671"), 2.0, 14.0, hex("#B8875A"), 0.0, (4.0, 6.0), 44.0, false),

        // Skeuomorphism (Classic 3D) — warm gold/brass fill, a darker
        // bronze edge, and a heavier, longer shadow than the plain
        // Skeuomorphism preset above (more offset, more blur) so it
        // reads as an older, heavier "real hardware" button — a distinct
        // era/material from the cool-blue modern skeuomorphic preset.
        preset!("Skeuomorphism (Classic 3D)", hex("#C9A227"), hex("#3B2F0B"), hex("#8A6D1B"), 2.0, 8.0, Color::from_rgba(0.0, 0.0, 0.0, 0.5), 7.0, (0.0, 4.0), 46.0, false),
    ]
}

/// A small, theme-agnostic accent swatch list. Picking one sets only the
/// accent hue on the current [`ButtonConfig`] via [`ButtonConfig::with_accent`].
pub const SWATCHES: [(&str, Color); 8] = [
    ("Slate", Color::from_rgb(0.29, 0.33, 0.41)),
    ("Blue", Color::from_rgb(0.23, 0.51, 0.96)),
    ("Teal", Color::from_rgb(0.0, 0.42, 0.42)),
    ("Green", Color::from_rgb(0.13, 0.55, 0.33)),
    ("Amber", Color::from_rgb(0.96, 0.62, 0.04)),
    ("Orange", Color::from_rgb(0.92, 0.35, 0.10)),
    ("Red", Color::from_rgb(0.86, 0.15, 0.15)),
    ("Purple", Color::from_rgb(0.49, 0.23, 0.93)),
];
