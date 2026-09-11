//! MUI-style `SpeedDial`: a FAB that expands into a vertical stack of
//! secondary action FABs when toggled open.

use iced::widget::Column;
use crate::theme::RustTheme;

/// A single speed-dial action: a glyph and the message it fires.
pub struct SpeedDialAction<Message> {
    pub glyph: &'static str,
    pub on_press: Message,
}

impl<Message> SpeedDialAction<Message> {
    pub fn new(glyph: &'static str, on_press: Message) -> Self {
        Self { glyph, on_press }
    }
}

/// A speed dial. `is_open` shows/hides the `actions` stacked above the main
/// FAB; `main_glyph` is shown on the main FAB (conventionally a "+" that
/// visually rotates in MUI — kept static here); `on_toggle` fires when the
/// main FAB is pressed.
pub fn speed_dial<'a, Message>(
    theme: &RustTheme,
    main_glyph: impl iced::widget::text::IntoFragment<'a>,
    is_open: bool,
    actions: Vec<SpeedDialAction<Message>>,
    on_toggle: Message,
) -> Column<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut col = iced::widget::column![].spacing(12).align_x(iced::alignment::Horizontal::Center);

    if is_open {
        for action in actions {
            col = col.push(crate::components::fab::secondary(theme, action.glyph, action.on_press));
        }
    }

    col = col.push(crate::components::fab::primary(theme, main_glyph, on_toggle));

    col
}
