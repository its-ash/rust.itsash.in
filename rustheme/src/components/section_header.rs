//! A titled section header row: a title (optionally with supporting
//! subtitle text) and trailing action widgets, e.g. above a card grid or a
//! settings group. Pairs naturally with [`crate::divider::horizontal`]
//! placed underneath it.

use crate::theme::RustTheme;
use iced::widget::{Column, Row};
use iced::{Element, Length};
use rustheme_core::{heading_font, system_font, FontWeight};

/// A section title with no subtitle or actions.
pub fn section_header<'a, Message>(title: impl Into<String>) -> Column<'a, Message, RustTheme>
where
    Message: 'a,
{
    section_header_with(title, None::<String>, Vec::new())
}

/// A section title with supporting subtitle text and trailing action
/// widgets (e.g. a "See all" button or an icon button) right-aligned on the
/// same row as the title.
pub fn section_header_with<'a, Message>(
    title: impl Into<String>,
    subtitle: Option<impl Into<String>>,
    actions: Vec<Element<'a, Message, RustTheme>>,
) -> Column<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut header_row: Row<'a, Message, RustTheme> = iced::widget::row![
        iced::widget::text(title.into())
            .size(18)
            .font(heading_font(FontWeight::Bold))
            .width(Length::Fill),
    ]
    .align_y(iced::alignment::Vertical::Center)
    .spacing(12);

    for action in actions {
        header_row = header_row.push(action);
    }

    let mut col = iced::widget::column![header_row].spacing(4);

    if let Some(subtitle) = subtitle {
        col = col.push(
            iced::widget::text(subtitle.into())
                .size(13)
                .font(system_font(FontWeight::Regular))
                .style(|theme: &RustTheme| iced::widget::text::Style {
                    color: Some(theme.palette.text.secondary),
                }),
        );
    }

    col
}
