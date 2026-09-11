//! A compact bottom navigation bar with icon-and-label destinations.

use crate::components::icons::{self, Icon};
use crate::theme::RustTheme;
use iced::widget::Container;
use iced::{Background, Border, Color, Length, Shadow};
use rustheme_core::{system_font, Elevation, FontWeight};

/// One destination in a [`bottom_navigation`] bar.
#[derive(Debug, Clone, Copy)]
pub struct BottomNavigationItem<'a> {
    pub label: &'a str,
    pub icon: Icon,
}

/// Builds a bottom navigation bar. The selected destination receives a
/// tinted accent container and stronger label treatment.
pub fn bottom_navigation<'a, Message>(
    items: &[BottomNavigationItem<'a>],
    selected: usize,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut row = iced::widget::row![].spacing(4);
    for (index, item) in items.iter().enumerate() {
        let active = index == selected;
        let message = on_select(index);
        row = row.push(
            iced::widget::button(
                iced::widget::column![
                    icons::icon(item.icon, 22.0).width(Length::Fill),
                    iced::widget::text(item.label)
                        .size(12)
                        .font(system_font(FontWeight::Medium))
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center),
                ]
                .spacing(3)
                .align_x(iced::alignment::Horizontal::Center),
            )
            .padding([8, 12])
            .width(Length::Fill)
            .on_press(message)
            .style(move |theme: &RustTheme, status| {
                let p = &theme.palette;
                let background = if active {
                    Some(Background::Color(Color { a: 0.14, ..p.primary.main }))
                } else if matches!(status, iced::widget::button::Status::Hovered) {
                    Some(Background::Color(Color { a: 0.06, ..p.primary.main }))
                } else {
                    None
                };
                iced::widget::button::Style {
                    background,
                    text_color: if active { p.primary.main } else { p.text.secondary },
                    border: iced::border::rounded(theme.material.pill_radius(theme.shape.full)),
                    shadow: Shadow::default(),
                }
            }),
        );
    }

    iced::widget::container(row)
        .padding(8)
        .width(Length::Fill)
        .style(|theme: &RustTheme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.background.paper)),
            text_color: Some(theme.palette.text.primary),
            border: Border {
                color: theme.palette.divider,
                width: 1.0,
                radius: theme.shape.large.into(),
            },
            shadow: Elevation::Level1.shadow(),
        })
}
