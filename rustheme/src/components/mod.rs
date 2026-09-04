//! Ergonomic, pre-styled widget constructors — the component library layer
//! of RustTheme. Each module exposes free functions that build an iced
//! widget already wired to a named [`crate::style`] class, mirroring MUI's
//! component API (e.g. `Button variant="contained" color="primary"`).

pub mod app_bar;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod dialog;
pub mod divider;
pub mod list;
pub mod menu;
pub mod progress;
pub mod radio;
pub mod select;
pub mod slider;
pub mod snackbar;
pub mod switch;
pub mod tabs;
pub mod text_field;
pub mod tooltip;
