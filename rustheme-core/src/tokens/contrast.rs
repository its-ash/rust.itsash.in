//! WCAG 2.1 contrast utilities.
//!
//! Every text-on-background pair in RustTheme must clear **WCAG AA
//! (4.5:1)** for normal text or **AA-large (3:1)** for disabled/large text.
//! The [`contrast_ratio`] function computes the exact ratio; [`assert_aa`]
//! and [`assert_aa_large`] panic at construction time if a pair fails, so a
//! non-compliant palette can never ship silently.
//!
//! See <https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum>.

use iced_core::Color;

/// Computes the sRGB channel value in the 0..1 range from a `Color` channel
/// (which iced stores as a 0..1 float already).
fn channel(c: f32) -> f32 {
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Computes the [relative luminance](https://www.w3.org/TR/WCAG21/#dfn-relative-luminance)
/// of a [`Color`] (0 = darkest black, 1 = brightest white).
pub fn relative_luminance(color: Color) -> f32 {
    0.2126 * channel(color.r)
        + 0.7152 * channel(color.g)
        + 0.0722 * channel(color.b)
}

/// Computes the [WCAG contrast ratio](https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio)
/// between two colors. Returns a value in the range 1.0 (identical) to
/// 21.0 (black vs white). AA requires ≥4.5 for normal text, ≥3.0 for
/// large text (≥18px or ≥14px bold). AAA requires ≥7.0 / ≥4.5.
pub fn contrast_ratio(foreground: Color, background: Color) -> f32 {
    let l1 = relative_luminance(foreground);
    let l2 = relative_luminance(background);
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Asserts that `foreground` on `background` clears WCAG AA (4.5:1).
/// Panics with a descriptive message if it doesn't — call this in palette
/// constructors so non-compliant palettes fail at startup, not in
/// production.
pub fn assert_aa(foreground: Color, background: Color, label: &str) {
    let ratio = contrast_ratio(foreground, background);
    assert!(
        ratio >= 4.5,
        "WCAG AA violation: {label} contrast ratio is {ratio:.2}:1 (needs >= 4.5:1)"
    );
}

/// Asserts that `foreground` on `background` clears WCAG AA-large (3:1),
/// the threshold for disabled text and text ≥18px or ≥14px bold.
pub fn assert_aa_large(foreground: Color, background: Color, label: &str) {
    let ratio = contrast_ratio(foreground, background);
    assert!(
        ratio >= 3.0,
        "WCAG AA-large violation: {label} contrast ratio is {ratio:.2}:1 (needs >= 3.0:1)"
    );
}