//! A settings-list row: leading icon, title, optional subtitle, and a
//! trailing control (switch, chevron, text, or any custom widget) — the
//! row pattern used across most "Settings"/"Preferences" screens. Stack
//! several inside [`crate::list::list`] for dividers between rows, or wrap
//! a group in [`crate::card::card`].

use crate::components::icons::{self, Icon};
use crate::theme::RustTheme;
use iced::widget::Container;
use iced::{Element, Length};
use rustheme_core::{system_font, FontWeight};

/// A settings row with a leading icon, title, optional subtitle, and a
/// trailing widget (e.g. `switch::switch(...)`, `icons::icon(Icon::ChevronRight, 20.0)`,
/// or a status `text`).
pub fn settings_row<'a, Message>(
    icon: Icon,
    title: impl Into<String>,
    subtitle: Option<impl Into<String>>,
    trailing: impl Into<Element<'a, Message, RustTheme>>,
) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut label_col = iced::widget::column![
        iced::widget::text(title.into())
            .size(14)
            .font(system_font(FontWeight::Medium)),
    ]
    .spacing(2);

    if let Some(subtitle) = subtitle {
        label_col = label_col.push(
            iced::widget::text(subtitle.into())
                .size(12)
                .font(system_font(FontWeight::Regular))
                .style(|theme: &RustTheme| iced::widget::text::Style {
                    color: Some(theme.palette.text.secondary),
                }),
        );
    }

    let row = iced::widget::row![
        icons::icon(icon, 20.0)
            .style(|theme: &RustTheme| iced::widget::text::Style {
                color: Some(theme.palette.text.secondary),
            })
            .width(Length::Fixed(24.0)),
        label_col.width(Length::Fill),
        trailing.into(),
    ]
    .spacing(14)
    .align_y(iced::alignment::Vertical::Center);

    iced::widget::container(row)
        .padding([12, 16])
        .width(Length::Fill)
}

/// A settings row with no leading icon — a narrower layout for a group
/// where the icon column would be wasted space (e.g. a dense preferences
/// panel where every row already sits under an iconed [`section_header`]).
///
/// [`section_header`]: crate::section_header::section_header
pub fn settings_row_compact<'a, Message>(
    title: impl Into<String>,
    subtitle: Option<impl Into<String>>,
    trailing: impl Into<Element<'a, Message, RustTheme>>,
) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut label_col = iced::widget::column![
        iced::widget::text(title.into())
            .size(14)
            .font(system_font(FontWeight::Medium)),
    ]
    .spacing(2);

    if let Some(subtitle) = subtitle {
        label_col = label_col.push(
            iced::widget::text(subtitle.into())
                .size(12)
                .font(system_font(FontWeight::Regular))
                .style(|theme: &RustTheme| iced::widget::text::Style {
                    color: Some(theme.palette.text.secondary),
                }),
        );
    }

    let row = iced::widget::row![label_col.width(Length::Fill), trailing.into()]
        .spacing(14)
        .align_y(iced::alignment::Vertical::Center);

    iced::widget::container(row)
        .padding([12, 16])
        .width(Length::Fill)
}
