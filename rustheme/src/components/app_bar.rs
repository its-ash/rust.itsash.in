//! MUI-style `AppBar`: a top-of-page header row with a title and trailing
//! action widgets.

use iced::widget::Container;
use iced::{Element, Length};
use crate::theme::RustTheme;
use rustheme_core::{heading_font, Elevation, FontWeight};

/// A top app bar with a title on the left and `actions` on the right.
/// Padding scales with `theme.spacing`.
pub fn app_bar<'a, Message>(
    theme: &RustTheme,
    title: impl Into<String>,
    actions: Vec<Element<'a, Message, RustTheme>>,
) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut action_row = iced::widget::row![].spacing(8).align_y(iced::alignment::Vertical::Center);
    for action in actions {
        action_row = action_row.push(action);
    }

    let content = iced::widget::row![
        iced::widget::text(title.into()).size(20).font(heading_font(FontWeight::Bold)),
        iced::widget::horizontal_space(),
        action_row,
    ]
    .align_y(iced::alignment::Vertical::Center)
    .width(Length::Fill);

    iced::widget::container(content)
        .padding([theme.spacing.unit(1.5), theme.spacing.unit(2.5)])
        .width(Length::Fill)
        .style(|theme: &RustTheme| {
            let p = &theme.palette;
            iced::widget::container::Style {
                background: Some(iced::Background::Color(p.background.default)),
                text_color: Some(p.text.primary),
                border: iced::Border::default(),
                shadow: Elevation::Level1.shadow(),
            }
        })
}
