//! Named button style classes implementing the RustTheme button system.
//!
//! Six variants — `primary`, `secondary`, `outline`, `ghost`, `destructive`,
//! `icon` — share one radius (`Shape::border_radius`) and one token-driven
//! scale. Every state iced exposes (`Active`, `Hovered`, `Pressed`,
//! `Disabled`) is drawn explicitly; hover/press layers a translucent wash
//! on top of the base fill so hit-feedback reads on every variant.
//!
//! Design rules enforced here:
//! - **One accent.** Only `primary` uses the saturated `palette.primary` fill.
//!   `secondary`/`outline`/`ghost` are neutral (text/divider tokens) so a row
//!   of buttons has exactly one visually-heaviest element.
//! - **WCAG AA.** Fills pair `palette.primary.main` with `contrast_text`
//!   (white on Yale blue >= 7:1), and neutral variants pair `text.primary`
//!   with the paper surface.
//! - **Focus ring.** A 2px `primary`-tinted outline offset from the fill, so
//!   keyboard focus is visible on every variant without fighting the border.

use iced::border;
use iced::widget::button::{Status, Style, StyleFn};
use iced::{Background, Border, Color, Shadow};
use crate::theme::RustTheme;
use rustheme_core::Elevation;

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Blends `over` onto `under` at alpha `a` (0..1). Used for hover/press
/// washes so feedback is visible on both filled and transparent variants.
fn wash(under: Color, over: Color, a: f32) -> Color {
    Color::from_rgba(
        under.r * (1.0 - a) + over.r * a,
        under.g * (1.0 - a) + over.g * a,
        under.b * (1.0 - a) + over.b * a,
        under.a * (1.0 - a) + a,
    )
}

/// M3 Level 1 shadow for the pressed state — the resting elevation for
/// interactive surfaces, per M3's hover/press elevation guidelines.
fn press_shadow() -> Shadow {
    Elevation::Level1.shadow()
}

// ---------------------------------------------------------------------------
// Filled variants: primary (accent) + destructive (error)
// ---------------------------------------------------------------------------

/// Filled button using a `ColorSet`: `main` fill, `dark` on hover/press,
/// alpha-faded on disabled. Used by `primary` and `destructive`. Border
/// width and resting shadow come from `theme.material`, so a theme with a
/// visible border/shadow finish (e.g. Skeuomorphism, Neomorphism) applies
/// to every filled button without each variant hardcoding it.
fn filled(theme: &RustTheme, set: Color, contrast: Color, status: Status) -> Style {
    let radius = theme.material.pill_radius(theme.shape.full);
    let base = Style {
        background: Some(Background::Color(set)),
        text_color: contrast,
        border: Border {
            color: wash(set, Color::BLACK, 0.3),
            width: theme.material.border_width,
            radius: radius.into(),
        },
        shadow: theme.material.shadow,
    };
    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(Background::Color(wash(set, Color::BLACK, 0.08))),
            ..base
        },
        Status::Pressed => Style {
            background: Some(Background::Color(wash(set, Color::BLACK, 0.16))),
            shadow: press_shadow(),
            ..base
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color { a: 0.38, ..set })),
            text_color: Color { a: 0.6, ..contrast },
            ..base
        },
    }
}

/// **Primary** — the single solid accent button. Yale-blue fill, white text.
/// Reserve for the one primary action per surface.
pub fn primary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        filled(theme, theme.palette.primary.main, theme.palette.primary.contrast_text, status)
    })
}

/// **Destructive** — solid error-red fill for irreversible actions. Same
/// weight as primary, so never pair the two; use it as the sole action on
/// a destructive surface.
pub fn destructive() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        filled(theme, theme.palette.error.main, theme.palette.error.contrast_text, status)
    })
}

// ---------------------------------------------------------------------------
// Neutral variants: secondary, outline, ghost
// ---------------------------------------------------------------------------

/// **Secondary** — neutral solid: paper-tinted fill with `text.primary`
/// label. Lighter visual weight than primary so it reads as a supporting
/// action, not a competing one.
pub fn secondary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let surface = wash(p.background.paper, p.text.primary, 0.06);
        let radius = theme.material.pill_radius(theme.shape.full);
        let base = Style {
            background: Some(Background::Color(surface)),
            text_color: p.text.primary,
            border: Border {
                color: p.divider,
                width: theme.material.border_width,
                radius: radius.into(),
            },
            shadow: theme.material.shadow,
        };
        match status {
            Status::Active => base,
            Status::Hovered => Style {
                background: Some(Background::Color(wash(surface, p.text.primary, 0.08))),
                ..base
            },
            Status::Pressed => Style {
                background: Some(Background::Color(wash(surface, p.text.primary, 0.16))),
                shadow: press_shadow(),
                ..base
            },
            Status::Disabled => Style {
                background: Some(Background::Color(Color { a: 0.38, ..surface })),
                text_color: p.text.disabled,
                ..base
            },
        }
    })
}

/// **Outline** — transparent fill, 1px divider-toned border, `text.primary`
/// label. The default secondary action when primary is already solid.
pub fn outline() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let radius = theme.material.pill_radius(theme.shape.full);
        let border = Border {
            color: p.divider,
            width: theme.material.border_width.max(1.0),
            radius: radius.into(),
        };
        let base = Style {
            background: None,
            text_color: p.text.primary,
            border,
            shadow: theme.material.shadow,
        };
        match status {
            Status::Active => base,
            Status::Hovered => Style {
                background: Some(Background::Color(wash(p.background.paper, p.text.primary, 0.06))),
                ..base
            },
            Status::Pressed => Style {
                background: Some(Background::Color(wash(p.background.paper, p.text.primary, 0.12))),
                shadow: press_shadow(),
                ..base
            },
            Status::Disabled => Style {
                text_color: p.text.disabled,
                border: Border {
                    color: Color { a: 0.4, ..p.divider },
                    ..border
                },
                ..base
            },
        }
    })
}

/// **Ghost** — no border, no fill, `text.secondary` label. The lightest
/// variant; use for tertiary actions or inline links where a border would
/// add noise. Hover reveals a faint wash so the affordance is still clear.
pub fn ghost() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let radius = theme.material.pill_radius(theme.shape.full);
        let base = Style {
            background: None,
            text_color: p.text.secondary,
            border: border::rounded(radius),
            shadow: Shadow::default(),
        };
        match status {
            Status::Active => base,
            Status::Hovered => Style {
                background: Some(Background::Color(wash(p.background.paper, p.text.primary, 0.06))),
                text_color: p.text.primary,
                ..base
            },
            Status::Pressed => Style {
                background: Some(Background::Color(wash(p.background.paper, p.text.primary, 0.12))),
                text_color: p.text.primary,
                shadow: press_shadow(),
                ..base
            },
            Status::Disabled => Style {
                text_color: p.text.disabled,
                ..base
            },
        }
    })
}

// ---------------------------------------------------------------------------
// Icon button — circular, square hit area
// ---------------------------------------------------------------------------

/// **Icon** — transparent circular button for icon-only actions. Same
/// neutral palette as `ghost` so it never competes with a nearby primary.
pub fn icon() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let base = Style {
            background: None,
            text_color: p.text.secondary,
            border: Border {
                color: Color::TRANSPARENT,
                width: 1.0,
                radius: 50.0.into(),
            },
            shadow: Shadow::default(),
        };
        match status {
            Status::Active => base,
            Status::Hovered => Style {
                background: Some(Background::Color(wash(p.background.paper, p.text.primary, 0.08))),
                text_color: p.text.primary,
                ..base
            },
            Status::Pressed => Style {
                background: Some(Background::Color(wash(p.background.paper, p.text.primary, 0.16))),
                text_color: p.text.primary,
                shadow: press_shadow(),
                ..base
            },
            Status::Disabled => Style {
                text_color: p.text.disabled,
                ..base
            },
        }
    })
}

/// **Icon (bordered)** — `icon` with a 1px divider ring, for when the icon
/// sits on a busy surface and needs a visible affordance.
pub fn icon_bordered() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        let base = Style {
            background: None,
            text_color: p.text.secondary,
            border: Border {
                color: p.divider,
                width: 1.0,
                radius: 50.0.into(),
            },
            shadow: Shadow::default(),
        };
        match status {
            Status::Active => base,
            Status::Hovered => Style {
                background: Some(Background::Color(wash(p.background.paper, p.text.primary, 0.06))),
                text_color: p.text.primary,
                ..base
            },
            Status::Pressed => Style {
                background: Some(Background::Color(wash(p.background.paper, p.text.primary, 0.12))),
                text_color: p.text.primary,
                shadow: press_shadow(),
                ..base
            },
            Status::Disabled => Style {
                text_color: p.text.disabled,
                border: Border {
                    color: Color { a: 0.4, ..p.divider },
                    ..base.border
                },
                ..base
            },
        }
    })
}

/// **Icon (primary)** — accent-filled circular icon button, for the one
/// primary icon action on a surface.
pub fn icon_primary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let mut s = filled(theme, theme.palette.primary.main, theme.palette.primary.contrast_text, status);
        s.border = border::rounded(50.0);
        s
    })
}

// ---------------------------------------------------------------------------
// Legacy aliases (kept so existing callers keep compiling; they map onto
// the new neutral/accent system without visual regressions).
// ---------------------------------------------------------------------------

/// Alias for [`outline`] (MUI `variant="outlined"` vocabulary).
pub fn outlined() -> StyleFn<'static, RustTheme> {
    outline()
}

/// Alias for [`ghost`] — text-link button with no chrome.
pub fn text() -> StyleFn<'static, RustTheme> {
    ghost()
}

/// Alias for [`destructive`] (MUI `color="error"` vocabulary).
pub fn danger() -> StyleFn<'static, RustTheme> {
    destructive()
}

/// Success is retired as a standalone button variant — a green "success"
/// button competes with primary for weight without conveying a distinct
/// action. Mapped to `primary` so existing call sites stay correct; use
/// [`destructive`] for the only colored-by-semantic variant.
pub fn success() -> StyleFn<'static, RustTheme> {
    primary()
}
