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

pub use theme::{FontFamily, Palette, RustTheme, RustThemeBuilder};

// Flattened re-exports of every component module, so callers can write
// `rustheme::button::primary(...)`, `rustheme::card::elevated(...)`, etc.
// without reaching through `rustheme::components::`.
pub use components::{
    accordion, alert, app_bar, autocomplete, avatar, backdrop, badge, breadcrumbs, button, card,
    checkbox, chip, dialog, divider, drawer, fab, icon_button, layout, list, menu, pagination,
    progress, radio, rating, select, skeleton, slider, snackbar, speed_dial, stepper, switch,
    table, tabs, text_field, timeline, tooltip,
};

/// The commonly-needed imports for building a RustTheme-styled iced view:
/// the theme type, every component module, and iced's own `Element`.
pub mod prelude {
    pub use crate::theme::{FontFamily, Palette, RustTheme, RustThemeBuilder};
    pub use crate::{
        accordion, alert, app_bar, autocomplete, avatar, backdrop, badge, breadcrumbs, button,
        card, checkbox, chip, dialog, divider, drawer, fab, icon_button, layout, list, menu,
        pagination, progress, radio, rating, select, skeleton, slider, snackbar, speed_dial,
        stepper, switch, table, tabs, text_field, timeline, tooltip,
    };
    pub use iced::Element;
}
