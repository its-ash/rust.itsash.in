//! Material tokens: the surface-finish knobs a design style controls beyond
//! color and radius — border weight and resting shadow. Distinct from
//! [`crate::Shape`] (radius only) and [`crate::Elevation`] (a fixed M3
//! shadow scale): `Material` is the one place a whole design language
//! (Flat, Neomorphism, Glassmorphism, Brutalism, ...) can override "does
//! this surface have a border, and does it cast a shadow at rest."

use iced_core::{Color, Shadow, Vector};

/// Border + shadow finish applied to buttons, cards, and other bordered
/// surfaces across the whole theme.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    /// Border width (px) for outlined/bordered surfaces. `0.0` means no
    /// visible border even where a component would normally draw one.
    pub border_width: f32,
    /// Resting shadow for filled buttons and elevated surfaces.
    /// `Shadow::default()`-equivalent (fully transparent) means flat, no
    /// shadow at rest.
    pub shadow: Shadow,
    /// Corner-radius override for components that are pill-shaped by
    /// default (buttons, chips, the bottom-nav active pill, the search
    /// bar). `None` keeps the existing pill/fully-rounded look
    /// (RustTheme's own standard finish). `Some(radius)` replaces it with
    /// that exact radius everywhere those components would otherwise use
    /// [`crate::Shape::full`] — e.g. `Some(0.0)` for square-cornered
    /// styles like Brutalism, Bauhaus, or 8-bit/pixel-art, where a pill
    /// button would contradict the whole design language.
    pub pill_radius_override: Option<f32>,
}

impl Material {
    /// Flat: no border, no shadow, pill shape unchanged. RustTheme's own
    /// standard finish.
    pub const fn flat() -> Self {
        Self {
            border_width: 0.0,
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
            pill_radius_override: None,
        }
    }

    /// Sets the corner-radius override (see
    /// [`Material::pill_radius_override`]), builder-style.
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.pill_radius_override = Some(radius);
        self
    }

    /// A soft resting shadow plus a hairline border, for skeuomorphic /
    /// classic elevated finishes.
    pub fn soft(shadow_alpha: f32, blur: f32, offset_y: f32, border_width: f32) -> Self {
        Self {
            border_width,
            shadow: Shadow {
                color: Color { a: shadow_alpha, ..Color::BLACK },
                offset: Vector::new(0.0, offset_y),
                blur_radius: blur,
            },
            pill_radius_override: None,
        }
    }

    /// Resolves the radius a pill-shaped component should render at: the
    /// override when set, otherwise `shape.full` (the existing pill look).
    pub fn pill_radius(self, shape_full: f32) -> f32 {
        self.pill_radius_override.unwrap_or(shape_full)
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::flat()
    }
}
