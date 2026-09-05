//! Bundled font files for RustTheme's default Playfair Display + Source
//! Sans Pro pairing, both SIL Open Font License (OFL) fonts from Google
//! Fonts. Embedding the actual font bytes (rather than only naming a
//! family via [`rustheme_core::FontFamily::Named`]) is required because
//! iced only renders a named font correctly if its bytes are loaded into
//! the process — neither font ships with any OS by default, so naming them
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
//! fn main() -> iced::Result {
//!     rustheme::fonts::use_default_pairing();
//!
//!     rustheme::load_fonts!(iced::application("My App", update, view)).run()
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Save,
//! }
//!
//! fn update(_state: &mut u64, _message: Message) {}
//!
//! fn view(_state: &u64) -> Element<'_, Message, RustTheme> {
//!     button::primary("Save", Message::Save).into()
//! }
//! ```
//!
//! Only static weights are bundled (Regular/Medium/Bold for Playfair
//! Display; Regular/SemiBold/Bold for Source Sans Pro, which has no
//! separate static Medium weight) rather than each family's variable font,
//! since iced's text shaping (via `cosmic-text`) resolves distinct static
//! weight files more reliably than a single variable font's weight axis.

use rustheme_core::FontFamily;

/// Playfair Display's font family name as embedded here.
pub const PLAYFAIR_DISPLAY: &str = "Playfair Display";
/// Source Sans Pro's font family name as embedded here.
pub const SOURCE_SANS_PRO: &str = "Source Sans Pro";

/// Playfair Display Regular, embedded from `assets/fonts/`.
pub const PLAYFAIR_DISPLAY_REGULAR: &[u8] = include_bytes!("../assets/fonts/PlayfairDisplay-Regular.ttf");
/// Playfair Display Medium, embedded from `assets/fonts/`.
pub const PLAYFAIR_DISPLAY_MEDIUM: &[u8] = include_bytes!("../assets/fonts/PlayfairDisplay-Medium.ttf");
/// Playfair Display Bold, embedded from `assets/fonts/`.
pub const PLAYFAIR_DISPLAY_BOLD: &[u8] = include_bytes!("../assets/fonts/PlayfairDisplay-Bold.ttf");
/// Source Sans Pro Regular, embedded from `assets/fonts/`.
pub const SOURCE_SANS_PRO_REGULAR: &[u8] = include_bytes!("../assets/fonts/SourceSansPro-Regular.ttf");
/// Source Sans Pro SemiBold, embedded from `assets/fonts/` (there is no
/// separate static Medium weight for this family).
pub const SOURCE_SANS_PRO_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/SourceSansPro-SemiBold.ttf");
/// Source Sans Pro Bold, embedded from `assets/fonts/`.
pub const SOURCE_SANS_PRO_BOLD: &[u8] = include_bytes!("../assets/fonts/SourceSansPro-Bold.ttf");

/// All six bundled static font files (3 weights x 2 families), in the byte
/// form `iced::application(...).font(bytes)` accepts. Prefer
/// [`crate::load_fonts!`] over calling `.font(...)` on each of these
/// yourself, unless you need to load only some of the six.
pub fn default_pairing_bytes() -> [&'static [u8]; 6] {
    [
        PLAYFAIR_DISPLAY_REGULAR,
        PLAYFAIR_DISPLAY_MEDIUM,
        PLAYFAIR_DISPLAY_BOLD,
        SOURCE_SANS_PRO_REGULAR,
        SOURCE_SANS_PRO_SEMIBOLD,
        SOURCE_SANS_PRO_BOLD,
    ]
}

/// Loads all six bundled font files (see [`default_pairing_bytes`]) onto an
/// `iced::application(...)` builder expression. See the [`crate::fonts`]
/// module docs for why this is a macro rather than a function.
#[macro_export]
macro_rules! load_fonts {
    ($application:expr) => {{
        let mut app = $application;
        for bytes in $crate::fonts::default_pairing_bytes() {
            app = app.font(bytes);
        }
        app
    }};
}

/// Points RustTheme's global typography at the bundled Playfair Display +
/// Source Sans Pro pairing: Playfair Display for headings
/// ([`rustheme_core::FontRole::Heading`] — `h1`..`h6`, `subtitle1`/`2`),
/// Source Sans Pro for everything else. Call once, before building any UI
/// (alongside [`load_default_pairing`] in `main()`) — see
/// [`rustheme_core::set_global_font_family`] for why this is process-wide.
pub fn use_default_pairing() {
    rustheme_core::set_global_font_family(FontFamily::Named(SOURCE_SANS_PRO));
    rustheme_core::set_global_heading_font_family(FontFamily::Named(PLAYFAIR_DISPLAY));
}
