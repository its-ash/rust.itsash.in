//! MUI-style `Rating`: a row of clickable star glyphs representing a 0..max
//! value.

use iced::widget::Row;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// A star rating row. `value` is the current rating (stars `<= value` render
/// filled); `max` is the star count (MUI default is 5); `on_rate` maps a
/// clicked star's 1-based position to a `Message`. Padding scales with
/// `theme.spacing`.
pub fn rating<'a, Message>(
    theme: &RustTheme,
    value: u8,
    max: u8,
    on_rate: impl Fn(u8) -> Message + 'a,
) -> Row<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut row = iced::widget::row![].spacing(2);

    for i in 1..=max {
        let filled = i <= value;
        let glyph = if filled { "★" } else { "☆" };
        let message = on_rate(i);
        row = row.push(
            iced::widget::button(iced::widget::text(glyph).size(22).font(system_font(FontWeight::Regular)))
                .padding(theme.spacing.unit(0.25))
                .on_press(message)
                .style(move |theme: &RustTheme, status| {
                    let p = &theme.palette;
                    let text_color = if filled { p.warning.main } else { p.text.disabled };
                    let background = match status {
                        iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => {
                            Some(iced::Background::Color(iced::Color { a: 0.08, ..p.warning.main }))
                        }
                        _ => None,
                    };
                    iced::widget::button::Style {
                        background,
                        text_color,
                        border: iced::border::rounded(4),
                        shadow: iced::Shadow::default(),
                    }
                }),
        );
    }

    row
}
