//! Elevation tokens following [Material 3's elevation
//! system](https://m3.material.io/styles/elevation/overview).
//!
//! M3 expresses elevation as dp levels (0–5) and renders them either as
//! tonal surface overlays (color gets darker/warmer with elevation) or as
//! shadows. M3 uses shadows more sparingly than M2 — only when a surface
//! needs to separate from its background or signal interactivity.
//!
//! Each level maps to ambient + key shadow values (from the M3 spec):
//!
//! | Level | Ambient `y/blur/alpha`     | Key `y/blur/alpha`       | Used by                       |
//! |-------|----------------------------|--------------------------|-------------------------------|
//! | 0     | 0/0/0                      | 0/0/0                    | Flat surfaces, backgrounds   |
//! | 1     | 1/3/0.04                   | 1/1/0.12                 | App bars, cards (resting)    |
//! | 2     | 1/4/0.06                   | 1/2/0.16                 | FAB (resting), dialogs       |
//! | 3     | 1/6/0.08                   | 2/3/0.18                 | FAB (hover), menus, snackbar |
//! | 4     | 2/8/0.10                   | 2/4/0.20                 | Dialogs, nav drawer          |
//! | 5     | 4/12/0.12                  | 4/6/0.24                 | Dialogs (top), full overlays |
//!
//! Each shadow is a pair: an **ambient** (soft, large blur, low opacity)
//! and a **key** (tight, small blur, higher opacity) light shadow, matching
//! how M3 models two-light illumination.

use iced_core::{Color, Shadow, Vector};

/// The six M3 elevation levels (0–5), each carrying its ambient + key
/// shadow. Use [`Elevation::shadow`] to produce the composed [`Shadow`]
// for a level — the ambient and key are blended into a single iced
/// `Shadow` (iced has one shadow per element, so we pick the key shadow
/// and fold the ambient's softness into the blur/alpha).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Elevation {
    /// Level 0 — no elevation. Flat surfaces, backgrounds.
    #[default]
    Level0,
    /// Level 1 — app bars, resting cards, sticky headers.
    Level1,
    /// Level 2 — FAB (resting), dialogs (scrim siblings), refresh indicators.
    Level2,
    /// Level 3 — FAB (hover), menus, snackbars, search bars.
    Level3,
    /// Level 4 — dialogs, navigation drawers, pickers.
    Level4,
    /// Level 5 — top-level dialogs, full-screen overlays, bottom sheets.
    Level5,
}

impl Elevation {
    /// The dp elevation value (0–5) for this level.
    pub const fn dp(self) -> u8 {
        match self {
            Self::Level0 => 0,
            Self::Level1 => 1,
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Level4 => 4,
            Self::Level5 => 5,
        }
    }

    /// Ambient shadow parameters: `(offset_y, blur, alpha)`.
    const fn ambient(self) -> (f32, f32, f32) {
        match self {
            Self::Level0 => (0.0, 0.0, 0.0),
            Self::Level1 => (1.0, 3.0, 0.04),
            Self::Level2 => (1.0, 4.0, 0.06),
            Self::Level3 => (1.0, 6.0, 0.08),
            Self::Level4 => (2.0, 8.0, 0.10),
            Self::Level5 => (4.0, 12.0, 0.12),
        }
    }

    /// Key shadow parameters: `(offset_y, blur, alpha)`.
    const fn key(self) -> (f32, f32, f32) {
        match self {
            Self::Level0 => (0.0, 0.0, 0.0),
            Self::Level1 => (1.0, 1.0, 0.12),
            Self::Level2 => (1.0, 2.0, 0.16),
            Self::Level3 => (2.0, 3.0, 0.18),
            Self::Level4 => (2.0, 4.0, 0.20),
            Self::Level5 => (4.0, 6.0, 0.24),
        }
    }

    /// Produces a single [`Shadow`] for this elevation level, blending the
    /// ambient + key M3 shadow into the one shadow iced supports per
    /// element. The key shadow dominates (tighter, higher-opacity) and the
    /// ambient contributes additional blur so the result reads as a
    /// two-light M3 shadow rather than a flat M2 drop.
    pub fn shadow(self) -> Shadow {
        let (ay, ab, aa) = self.ambient();
        let (ky, kb, ka) = self.key();
        let y = ky.max(ay);
        let blur = kb + ab * 0.5;
        let alpha = (ka + aa * 0.4).min(0.3);
        Shadow {
            color: Color { a: alpha, ..Color::BLACK },
            offset: Vector::new(0.0, y),
            blur_radius: blur,
        }
    }
}
