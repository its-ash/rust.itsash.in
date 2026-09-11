//! Bundled font files for RustTheme's default type pairing, all downloaded
//! from Google Fonts and embedded at compile time so no external files are
//! needed at runtime.
//!
//! **Type pairing**
//! - **Roboto** (Apache 2.0) — the primary UI/body font, used for all
//!   buttons, labels, inputs, and body text.
//! - **Playfair Display** (OFL) — the heading font, used for `h1`..`h6` and
//!   `subtitle1`/`subtitle2` so a display/serif face pairs with Roboto.
//! - **Material Symbols Rounded** (OFL) — the icon font, used by
//!   [`crate::icons`] and every icon-bearing component. Codepoints are in
//!   the Material Symbols private-use range (U+E000–U+F8FF).
//!
//! Embedding the actual font bytes (rather than only naming a family via
//! [`rustheme_core::FontFamily::Named`]) is required because iced only
//! renders a named font correctly if its bytes are loaded into the
//! process — none of these ship with any OS by default, so naming them
//! without loading them would silently fall back to a generic font.
//!
//! Call [`use_default_pairing`] once in `main()`, before building any UI,
//! to point RustTheme's typography at these families, and chain the
//! [`load_fonts!`] macro onto your `iced::application(...)` builder to
//! actually load their bytes (a plain function can't do this: `.font()`
//! lives on `iced::application::Application<P>`, generic over iced's
//! private `Program` trait, which downstream crates can't name as a bound —
//! a macro expands inline at the call site instead, where `P` is already
//! known):
//!
//! ```no_run
//! use rustheme::prelude::*;
//!
//! fn update(_state: &mut (), _message: ()) {}
//!
//! fn view(_state: &()) -> Element<'_, (), RustTheme> {
//!     iced::widget::text("Hello, RustTheme").into()
//! }
//!
//! fn main() -> iced::Result {
//!     rustheme::fonts::use_default_pairing();
//!
//!     rustheme::load_fonts!(iced::application("My App", update, view)).run()
//! }
//! ```

use rustheme_core::FontFamily;

// ---------------------------------------------------------------------------
// Family names
// ---------------------------------------------------------------------------

/// Roboto's font family name as embedded here — the primary UI/body font.
pub const ROBOTO: &str = "Roboto";
/// Playfair Display's font family name as embedded here — the heading font.
pub const PLAYFAIR_DISPLAY: &str = "Playfair Display";
/// Material Symbols Rounded's font family name as embedded here — the icon font.
pub const MATERIAL_SYMBOLS: &str = "Material Symbols Rounded";

// ---------------------------------------------------------------------------
// Roboto — primary UI/body font (Regular, Medium, Bold)
// ---------------------------------------------------------------------------

/// Roboto Regular, embedded from `assets/fonts/`.
pub const ROBOTO_REGULAR: &[u8] = include_bytes!("../assets/fonts/Roboto-Regular.ttf");
/// Roboto Medium, embedded from `assets/fonts/`.
pub const ROBOTO_MEDIUM: &[u8] = include_bytes!("../assets/fonts/Roboto-Medium.ttf");
/// Roboto Bold, embedded from `assets/fonts/`.
pub const ROBOTO_BOLD: &[u8] = include_bytes!("../assets/fonts/Roboto-Bold.ttf");

// ---------------------------------------------------------------------------
// Playfair Display — heading font (Regular, Medium, Bold)
// ---------------------------------------------------------------------------

/// Playfair Display Regular, embedded from `assets/fonts/`.
pub const PLAYFAIR_DISPLAY_REGULAR: &[u8] = include_bytes!("../assets/fonts/PlayfairDisplay-Regular.ttf");
/// Playfair Display Medium, embedded from `assets/fonts/`.
pub const PLAYFAIR_DISPLAY_MEDIUM: &[u8] = include_bytes!("../assets/fonts/PlayfairDisplay-Medium.ttf");
/// Playfair Display Bold, embedded from `assets/fonts/`.
pub const PLAYFAIR_DISPLAY_BOLD: &[u8] = include_bytes!("../assets/fonts/PlayfairDisplay-Bold.ttf");

// ---------------------------------------------------------------------------
// Material Symbols Rounded — icon font (single variable file, all weights)
// ---------------------------------------------------------------------------

/// Material Symbols Rounded (filled, variable), embedded from `assets/fonts/`.
pub const MATERIAL_SYMBOLS_ROUNDED: &[u8] = include_bytes!("../assets/fonts/MaterialSymbolsRounded.ttf");

// ---------------------------------------------------------------------------
// Byte sets
// ---------------------------------------------------------------------------

/// All bundled text font files (3 weights × 2 families), in the byte form
/// `iced::application(...).font(bytes)` accepts.
pub fn text_font_bytes() -> [&'static [u8]; 6] {
    [
        ROBOTO_REGULAR,
        ROBOTO_MEDIUM,
        ROBOTO_BOLD,
        PLAYFAIR_DISPLAY_REGULAR,
        PLAYFAIR_DISPLAY_MEDIUM,
        PLAYFAIR_DISPLAY_BOLD,
    ]
}

/// The bundled icon font file — Material Symbols Rounded. Loaded
/// separately so icon rendering works even if a caller only wants the icon
/// font without the text pairing.
pub fn icon_font_bytes() -> [&'static [u8]; 1] {
    [MATERIAL_SYMBOLS_ROUNDED]
}

/// All bundled font files (text + icon), in the byte form
/// `iced::application(...).font(bytes)` accepts. Prefer
/// [`load_fonts!`] over calling `.font(...)` on each of these yourself.
pub fn all_font_bytes() -> [&'static [u8]; 7] {
    [
        ROBOTO_REGULAR,
        ROBOTO_MEDIUM,
        ROBOTO_BOLD,
        PLAYFAIR_DISPLAY_REGULAR,
        PLAYFAIR_DISPLAY_MEDIUM,
        PLAYFAIR_DISPLAY_BOLD,
        MATERIAL_SYMBOLS_ROUNDED,
    ]
}

/// Loads all bundled font files (text + icon, see [`all_font_bytes`]) onto
/// an `iced::application(...)` builder expression. See the [`crate::fonts`]
/// module docs for why this is a macro rather than a function.
#[macro_export]
macro_rules! load_fonts {
    ($application:expr) => {{
        let mut app = $application;
        for bytes in $crate::fonts::all_font_bytes() {
            app = app.font(bytes);
        }
        app
    }};
}

/// Points RustTheme's global typography at the bundled Roboto + Playfair
/// Display pairing: **Roboto** for body/UI text
/// ([`rustheme_core::FontRole::Body`]), **Playfair Display** for headings
/// ([`rustheme_core::FontRole::Heading`] — `h1`..`h6`, `subtitle1`/`2`).
/// Call once, before building any UI (alongside [`load_fonts!`] in
/// `main()`) — see [`rustheme_core::set_global_font_family`] for why this
/// is process-wide. The Material Symbols icon font is loaded by
/// [`load_fonts!`] but doesn't need a global-family call — it's applied
/// per-icon via [`crate::icons::icon`].
pub fn use_default_pairing() {
    rustheme_core::set_global_font_family(FontFamily::Named(ROBOTO));
    rustheme_core::set_global_heading_font_family(FontFamily::Named(PLAYFAIR_DISPLAY));
}

/// Points RustTheme's global typography at **Roboto only** — body/UI text
/// and headings (`h1`..`h6`, `subtitle1`/`2`, plus structural titles like
/// the app bar and dialog) all render in Roboto, with no separate display
/// face. Use this instead of [`use_default_pairing`] for a single
/// consistent typeface throughout the app rather than a heading/body
/// pairing. Call once, before building any UI (alongside [`load_fonts!`] in
/// `main()`) — see [`rustheme_core::set_global_font_family`] for why this
/// is process-wide.
pub fn use_roboto() {
    rustheme_core::set_global_font_family(FontFamily::Named(ROBOTO));
    rustheme_core::set_global_heading_font_family(FontFamily::Named(ROBOTO));
}

/// Builds the [`iced_core::Font`] for Material Symbols Rounded at the given
/// weight, for use by [`crate::icons`] and icon-bearing components.
pub fn material_symbols_font() -> iced::Font {
    iced::Font {
        family: iced::font::Family::Name(MATERIAL_SYMBOLS),
        weight: iced::font::Weight::Normal,
        ..iced::Font::DEFAULT
    }
}
