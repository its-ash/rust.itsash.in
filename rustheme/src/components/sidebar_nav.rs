//! `SidebarNav`: a persistent left-hand navigation column with icon+label
//! destinations and a collapse toggle — the desktop-app equivalent of
//! [`crate::bottom_navigation`]. Unlike [`crate::drawer`] (a transient
//! overlay dismissed by a scrim), this renders inline as a fixed layout
//! column alongside your page content.

use crate::components::icons::{self, Icon};
use crate::theme::RustTheme;
use iced::widget::Container;
use iced::{Background, Border, Element, Length, Shadow};
use rustheme_core::{system_font, FontWeight};

/// One destination in a [`sidebar_nav`] column.
#[derive(Debug, Clone, Copy)]
pub struct SidebarNavItem<'a> {
    pub label: &'a str,
    pub icon: Icon,
}

/// Builds a sidebar navigation column. `collapsed` shows icon-only (56px
/// wide); expanded shows icon + label (220px wide). `on_select` maps a
/// clicked destination's index to a `Message`; `on_toggle_collapse` fires
/// when the collapse/expand control at the bottom is pressed.
pub fn sidebar_nav<'a, Message>(
    items: &[SidebarNavItem<'a>],
    selected: usize,
    collapsed: bool,
    on_select: impl Fn(usize) -> Message + 'a,
    on_toggle_collapse: Message,
) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let width = if collapsed { 56.0 } else { 220.0 };

    let mut col = iced::widget::column![].spacing(4).width(Length::Fixed(width));

    for (index, item) in items.iter().enumerate() {
        let active = index == selected;
        let message = on_select(index);

        let icon_el = icons::icon(item.icon, 20.0);
        let row: Element<'a, Message, RustTheme> = if collapsed {
            iced::widget::container(icon_el)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .into()
        } else {
            iced::widget::row![
                icon_el,
                iced::widget::text(item.label).size(14).font(system_font(FontWeight::Medium)),
            ]
            .spacing(12)
            .align_y(iced::alignment::Vertical::Center)
            .into()
        };

        col = col.push(
            iced::widget::button(row)
                .padding(if collapsed { [10, 8] } else { [10, 14] })
                .width(Length::Fill)
                .on_press(message)
                .style(move |theme: &RustTheme, status| {
                    let p = &theme.palette;
                    let background = if active {
                        Some(Background::Color(iced::Color { a: 0.14, ..p.primary.main }))
                    } else if matches!(status, iced::widget::button::Status::Hovered) {
                        Some(Background::Color(iced::Color { a: 0.06, ..p.primary.main }))
                    } else {
                        None
                    };
                    iced::widget::button::Style {
                        background,
                        text_color: if active { p.primary.main } else { p.text.secondary },
                        border: Border {
                            color: iced::Color::TRANSPARENT,
                            width: 0.0,
                            radius: theme.shape.small.into(),
                        },
                        shadow: Shadow::default(),
                    }
                }),
        );
    }

    let collapse_glyph = if collapsed { Icon::ChevronRight } else { Icon::ChevronLeft };
    let collapse_row: Element<'a, Message, RustTheme> = if collapsed {
        iced::widget::container(icons::icon(collapse_glyph, 18.0))
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .into()
    } else {
        iced::widget::row![
            icons::icon(collapse_glyph, 18.0),
            iced::widget::text("Collapse").size(13).font(system_font(FontWeight::Regular)),
        ]
        .spacing(12)
        .align_y(iced::alignment::Vertical::Center)
        .into()
    };

    let collapse_button = iced::widget::button(collapse_row)
        .padding(if collapsed { [10, 8] } else { [10, 14] })
        .width(Length::Fill)
        .on_press(on_toggle_collapse)
        .style(|theme: &RustTheme, status| {
            let p = &theme.palette;
            let text_color = match status {
                iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => p.text.primary,
                _ => p.text.secondary,
            };
            iced::widget::button::Style {
                background: None,
                text_color,
                border: Border::default(),
                shadow: Shadow::default(),
            }
        });

    let content = iced::widget::column![
        iced::widget::scrollable(col).height(Length::Fill),
        iced::widget::horizontal_rule(1),
        collapse_button,
    ]
    .spacing(8);

    iced::widget::container(content)
        .padding([16, 8])
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .style(|theme: &RustTheme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.background.paper)),
            text_color: Some(theme.palette.text.primary),
            border: Border {
                color: theme.palette.divider,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
        })
}
