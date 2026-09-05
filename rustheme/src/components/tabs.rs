//! MUI-style `Tabs`: a horizontal row of selectable tab buttons with an
//! active-tab indicator.

use iced::widget::{Column, Row};
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// A tab bar. `items` are the tab labels; `selected` is the active index;
/// `on_select` maps a clicked index to a `Message`.
pub fn tabs<'a, Message>(
    items: &[&'a str],
    selected: usize,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Column<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut row: Row<'a, Message, RustTheme> = iced::widget::row![].spacing(4);

    for (i, label) in items.iter().enumerate() {
        let is_active = i == selected;
        let message = on_select(i);
        let btn = iced::widget::button(
            iced::widget::text(*label)
                .size(14)
                .font(system_font(FontWeight::Medium))
                .align_x(iced::alignment::Horizontal::Center),
        )
        .padding([10, 16])
        .width(iced::Length::Fill)
        .on_press(message)
        .style(move |theme: &RustTheme, status| {
            let p = &theme.palette;
            let text_color = if is_active { p.primary.main } else { p.text.secondary };
            let text_color = match status {
                iced::widget::button::Status::Hovered if !is_active => p.text.primary,
                _ => text_color,
            };
            iced::widget::button::Style {
                background: None,
                text_color,
                border: iced::Border {
                    color: if is_active { p.primary.main } else { iced::Color::TRANSPARENT },
                    width: if is_active { 2.0 } else { 0.0 },
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow::default(),
            }
        });
        row = row.push(btn);
    }

    iced::widget::column![row, iced::widget::horizontal_rule(1)]
}
