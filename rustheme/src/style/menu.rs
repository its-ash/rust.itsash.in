//! The dropdown-menu appearance used by [`super::pick_list`]'s popover list.

use iced::widget::overlay::menu::{Style, StyleFn};
use iced::{Background, Border};
use crate::theme::RustTheme;

pub fn default() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme| {
        let p = &theme.palette;
        Style {
            background: Background::Color(p.background.paper),
            border: Border {
                color: p.divider,
                width: 1.0,
                radius: theme.shape.border_radius.into(),
            },
            text_color: p.text.primary,
            selected_text_color: p.primary.contrast_text,
            selected_background: Background::Color(p.primary.main),
        }
    })
}
