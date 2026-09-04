//! Ergonomic, pre-styled widget constructors — the component library layer
//! of RustTheme. Each module exposes free functions that build an iced
//! widget already wired to a named [`crate::style`] class, mirroring MUI's
//! component API (e.g. `Button variant="contained" color="primary"`).

pub mod accordion;
pub mod alert;
pub mod app_bar;
pub mod autocomplete;
pub mod avatar;
pub mod backdrop;
pub mod badge;
pub mod breadcrumbs;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod dialog;
pub mod divider;
pub mod drawer;
pub mod fab;
pub mod icon_button;
pub mod layout;
pub mod list;
pub mod menu;
pub mod pagination;
pub mod progress;
pub mod radio;
pub mod rating;
pub mod select;
pub mod skeleton;
pub mod slider;
pub mod snackbar;
pub mod speed_dial;
pub mod stepper;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod text_field;
pub mod timeline;
pub mod tooltip;
