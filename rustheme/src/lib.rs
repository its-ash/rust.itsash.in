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
pub mod fonts;
pub mod style;
pub mod theme;

pub use theme::{Elevation, FontFamily, Palette, RustTheme, RustThemeBuilder, RustThemeConfig, Spacing};

// Flattened re-exports of every component module, so callers can write
// `rustheme::button::primary(...)`, `rustheme::card::elevated(...)`, etc.
// without reaching through `rustheme::components::`.
pub use components::{
    accordion, alert, app_bar, autocomplete, avatar, backdrop, badge, breadcrumbs, button, card,
    bottom_navigation, checkbox, chip, code_viewer, color_picker, comparison_table,
    confirm_dialog, context_menu, data_card_grid, dialog, divider, drawer, empty_state, fab,
    icon_button, icons, key_value_list, layout, list, menu, pagination, progress, radio, rating,
    search_bar, section_header, segmented, select, settings_row, sidebar_nav, skeleton, slider,
    snackbar, speed_dial, spinner, stat_card, stepper, switch, table, tabs, text_field, timeline,
    toast, toolbar, tooltip, tree_view,
};

/// The commonly-needed imports for building a RustTheme-styled iced view:
/// the theme type, every component module, and iced's own `Element`.
pub mod prelude {
    pub use crate::theme::{FontFamily, Palette, RustTheme, RustThemeBuilder, RustThemeConfig, Spacing};
    pub use crate::{
        accordion, alert, app_bar, autocomplete, avatar, backdrop, badge, breadcrumbs, button,
        bottom_navigation, card, checkbox, chip, code_viewer, color_picker, comparison_table,
        confirm_dialog, context_menu, data_card_grid, dialog, divider, drawer, empty_state, fab,
        icon_button, icons, key_value_list, layout, list, menu, pagination, progress, radio,
        rating, search_bar, section_header, segmented, select, settings_row, sidebar_nav,
        skeleton, slider, snackbar, speed_dial, spinner, stat_card, stepper, switch, table, tabs,
        text_field, timeline, toast, toolbar, tooltip, tree_view,
    };
    pub use iced::Element;
}
