//! Shape tokens controlling corner rounding across components, following
//! the [Material 3 corner radius scale](https://m3.material.io/styles/shape/corner-radius-scale).
//!
//! M3 defines a 10-level scale based on roundedness, not component size:
//! ```text
//!  None  0dp   ·  ExtraSmall  4dp  ·  Small  8dp  ·  Medium  12dp
//!  Large  16dp ·  LargeIncreased 20dp ·  ExtraLarge  28dp
//!  ExtraLargeIncreased  32dp  ·  ExtraExtraLarge  48dp  ·  Full  ∞
//! ```
//! Each component maps to the appropriate level (buttons → Full, cards →
//! Large, text fields → ExtraSmall, dialogs → ExtraLarge, FAB → Large).
//! `border_radius` is kept as the legacy default for simple widgets; the
//! named levels below are the primary API.

/// Shape tokens: the full M3 10-level corner radius scale plus the legacy
/// `border_radius` alias (defaults to `Medium`, 12dp — M3's general-purpose
/// rounding).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    /// Legacy default radius for widgets that take a single radius value.
    /// Defaults to 12.0 (M3 Medium). Prefer the named levels below.
    pub border_radius: f32,
    /// M3 None — 0dp. Dividers, full-bleed images.
    pub none: f32,
    /// M3 ExtraSmall — 4dp. Text fields, snackbars, small UI elements.
    pub extra_small: f32,
    /// M3 Small — 8dp. Menus, list items, small cards.
    pub small: f32,
    /// M3 Medium — 12dp. General-purpose containers, small FABs.
    pub medium: f32,
    /// M3 Large — 16dp. Cards, dialogs (inner), sheets.
    pub large: f32,
    /// M3 LargeIncreased — 20dp. Large FABs, search bars.
    pub large_increased: f32,
    /// M3 ExtraLarge — 28dp. Dialogs, bottom sheets, search bars (expanded).
    pub extra_large: f32,
    /// M3 ExtraLargeIncreased — 32dp. Large dialogs, full-screen sheets.
    pub extra_large_increased: f32,
    /// M3 ExtraExtraLarge — 48dp. Bottom sheet drag handles, large cards.
    pub extra_extra_large: f32,
    /// M3 Full — fully rounded (use 99999.0 or component-specific rounding).
    /// For pill/rounded-full components, use the component's own dimension / 2.
    pub full: f32,
}

impl Shape {
    /// M3 ExtraSmall — 4dp. Alias for [`Shape::extra_small`].
    pub const EXTRA_SMALL: f32 = 4.0;
    /// M3 Small — 8dp.
    pub const SMALL: f32 = 8.0;
    /// M3 Medium — 12dp.
    pub const MEDIUM: f32 = 12.0;
    /// M3 Large — 16dp.
    pub const LARGE: f32 = 16.0;
    /// M3 ExtraLarge — 28dp.
    pub const EXTRA_LARGE: f32 = 28.0;
    /// M3 Full — sentinel for fully-rounded (use dimension/2 at call site).
    pub const FULL: f32 = 99999.0;

    /// Creates a global corner-radius override. Every finite shape token is
    /// set to `radius`; `none` and `full` retain their semantic meanings.
    pub fn uniform(radius: f32) -> Self {
        let radius = radius.max(0.0);
        Self {
            border_radius: radius,
            none: 0.0,
            extra_small: radius,
            small: radius,
            medium: radius,
            large: radius,
            large_increased: radius,
            extra_large: radius,
            extra_large_increased: radius,
            extra_extra_large: radius,
            full: Self::FULL,
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            border_radius: Self::MEDIUM,
            none: 0.0,
            extra_small: Self::EXTRA_SMALL,
            small: Self::SMALL,
            medium: Self::MEDIUM,
            large: Self::LARGE,
            large_increased: 20.0,
            extra_large: Self::EXTRA_LARGE,
            extra_large_increased: 32.0,
            extra_extra_large: 48.0,
            full: Self::FULL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Shape;

    #[test]
    fn uniform_preserves_semantic_none_and_full() {
        let shape = Shape::uniform(18.0);
        assert_eq!(shape.none, 0.0);
        assert_eq!(shape.full, Shape::FULL);
        assert_eq!(shape.border_radius, 18.0);
        assert_eq!(shape.extra_small, 18.0);
        assert_eq!(shape.extra_extra_large, 18.0);
    }
}
