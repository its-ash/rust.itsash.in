//! MUI-style `Timeline`: a vertical sequence of events, each with a dot
//! marker on a connecting line and content beside it.

use iced::widget::Column;
use crate::theme::RustTheme;

/// A single timeline entry: a title and a body description.
pub struct TimelineItem {
    pub title: String,
    pub body: String,
}

impl TimelineItem {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self { title: title.into(), body: body.into() }
    }
}

/// A vertical timeline built from `items`, in order from top to bottom.
pub fn timeline<'a, Message>(items: Vec<TimelineItem>) -> Column<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut col = iced::widget::column![].spacing(0);
    let len = items.len();

    for (i, item) in items.into_iter().enumerate() {
        let is_last = i + 1 == len;

        let dot = iced::widget::container(iced::widget::Space::new(10, 10)).style(|theme: &RustTheme| {
            iced::widget::container::Style {
                background: Some(iced::Background::Color(theme.palette.primary.main)),
                border: iced::border::rounded(5),
                ..Default::default()
            }
        });

        let mut marker_col = iced::widget::column![iced::widget::container(dot).padding(iced::Padding {
            top: 4.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })]
            .width(iced::Length::Fixed(10.0))
            .align_x(iced::alignment::Horizontal::Center);

        if !is_last {
            marker_col = marker_col.push(
                iced::widget::container(
                    iced::widget::vertical_rule(2),
                )
                .height(iced::Length::Fixed(48.0))
                .padding([4, 0]),
            );
        }

        let content = iced::widget::column![
            iced::widget::text(item.title).size(15),
            iced::widget::text(item.body).size(13).style(|theme: &RustTheme| iced::widget::text::Style {
                color: Some(theme.palette.text.secondary),
            }),
        ]
        .spacing(4)
        .padding(iced::Padding {
            top: 0.0,
            right: 0.0,
            bottom: 16.0,
            left: 0.0,
        });

        col = col.push(
            iced::widget::row![marker_col, content]
                .spacing(16),
        );
    }

    col
}
