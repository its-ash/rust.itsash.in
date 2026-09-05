//! `rustheme-core` provides the design-token primitives (palette, typography,
//! spacing, shape) that the `rustheme` crate bundles into its `RustTheme`
//! type and styles Iced widgets from.
//!
//! This crate is intentionally free of any dependency on the full `iced`
//! crate or its widget system — it only depends on `iced_core` for the
//! [`iced_core::Color`] type, so it can be reused independently of iced's
//! rendering/windowing stack if needed.
//!
//! Note: the `RustTheme` bundling type itself lives in the `rustheme` crate
//! rather than here, because Rust's orphan rules require `impl SomeIcedTrait
//! for RustTheme` to live in the same crate that defines `RustTheme` — and
//! `rustheme` is where all the iced `Catalog` trait implementations live.

pub mod tokens;

pub use tokens::{
    hex, system_font, system_monospace_font, BackgroundColors, ColorSet, FontWeight, Palette,
    Shape, Spacing, TextColors, TypeScale, TypeStyle, Typography, SYSTEM_FONT_NAME,
    SYSTEM_MONOSPACE_FONT_NAME,
};
