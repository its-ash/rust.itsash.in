/// MUI-style spacing unit. `Spacing(8.0)` is the default MUI base unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spacing(pub f32);

impl Spacing {
    /// Returns `factor` multiples of the base spacing unit, e.g.
    /// `spacing.unit(2.0)` with a default `Spacing(8.0)` yields `16.0`.
    pub fn unit(&self, factor: f32) -> f32 {
        self.0 * factor
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self(8.0)
    }
}
