//! Named toggler (switch) style class: `primary`.

use iced::widget::toggler::{Status, Style, StyleFn};
use iced::Color;
use crate::theme::RustTheme;

pub fn primary() -> StyleFn<'static, RustTheme> {
    Box::new(|theme: &RustTheme, status: Status| {
        let p = &theme.palette;
        match status {
            Status::Active { is_toggled } | Status::Hovered { is_toggled } => Style {
                background: if is_toggled { p.primary.main } else { p.divider },
                background_border_width: 0.0,
                background_border_color: Color::TRANSPARENT,
                foreground: p.background.paper,
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
            },
            Status::Disabled => Style {
                background: Color { a: 0.4, ..p.divider },
                background_border_width: 0.0,
                background_border_color: Color::TRANSPARENT,
                foreground: Color { a: 0.7, ..p.background.paper },
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
            },
        }
    })
}
