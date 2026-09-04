//! # RustTheme
//!
//! A MUI-inspired theming framework and component library for the [Iced]
//! GUI toolkit. `rustheme-core` provides the design tokens and [`RustTheme`]
//! type; this crate (`rustheme`) implements iced's `Catalog` styling traits
//! for `RustTheme` and exposes an ergonomic component library built on top.
//!
//! [Iced]: https://iced.rs
//!
//! ```no_run
//! use rustheme::prelude::*;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Save,
//! }
//!
//! fn view(_state: &()) -> iced::Element<'_, Message, RustTheme> {
//!     rustheme::button::primary("Save", Message::Save).into()
//! }
//! ```

pub mod components;
pub mod style;
pub mod theme;

pub use theme::RustTheme;

// Flattened re-exports of every component module, so callers can write
// `rustheme::button::primary(...)`, `rustheme::card::elevated(...)`, etc.
// without reaching through `rustheme::components::`.
pub use components::{
    app_bar, avatar, badge, button, card, checkbox, chip, dialog, divider, list, menu, progress,
    radio, select, slider, snackbar, switch, tabs, text_field, tooltip,
};

/// The commonly-needed imports for building a RustTheme-styled iced view:
/// the theme type, every component module, and iced's own `Element`.
pub mod prelude {
    pub use crate::theme::RustTheme;
    pub use crate::{
        app_bar, avatar, badge, button, card, checkbox, chip, dialog, divider, list, menu,
        progress, radio, select, slider, snackbar, switch, tabs, text_field, tooltip,
    };
    pub use iced::Element;
}
