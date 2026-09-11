/// MUI-style spacing unit. `Spacing(8.0)` is the default MUI base unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spacing(pub f32);

impl Spacing {
    /// Returns `factor` multiples of the base spacing unit, e.g.
    /// `spacing.unit(2.0)` with a default `Spacing(8.0)` yields `16.0`.
    pub fn unit(&self, factor: f32) -> f32 {
        self.0 * factor
    }

    /// A tighter base unit (4px) for dense, information-heavy UIs (data
    /// tables, dashboards).
    pub const fn compact() -> Self {
        Self(4.0)
    }

    /// The default MUI base unit (8px), balanced for general-purpose UI.
    pub const fn normal() -> Self {
        Self(8.0)
    }

    /// A looser base unit (12px) for airier, touch-friendly, or marketing-
    /// style UIs.
    pub const fn spacious() -> Self {
        Self(12.0)
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self(8.0)
    }
}
