//! `Tabs`: a horizontal row of selectable, card-style rounded tab buttons.
//!
//! Design — rounded "pill card" tabs, not outlined:
//! - The entire tab bar sits inside a rounded container with a subtle
//!   background (the `default` surface), like a card holding the tabs.
//! - Each tab is a rounded pill (`shape.full`). The active tab gets a
//!   solid accent fill; inactive tabs are transparent with muted text.
//! - Hover on an inactive tab shows a faint wash; pressed shows a deeper
//!   wash. No outlines or bottom-border indicators.

use iced::widget::{Column, Row};
use iced::{Background, Border, Color, Length, Shadow};
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// A rounded, card-style tab bar. `items` are the tab labels; `selected`
/// is the active index; `on_select` maps a clicked index to a `Message`.
/// Padding scales with `theme.spacing`.
pub fn tabs<'a, Message>(
    theme: &RustTheme,
    items: &[&'a str],
    selected: usize,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Column<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut row: Row<'a, Message, RustTheme> = iced::widget::row![].spacing(4);
    let tab_padding = [theme.spacing.unit(1.25), theme.spacing.unit(2.5)];
    let bar_padding = theme.spacing.unit(0.5);

    for (i, label) in items.iter().enumerate() {
        let is_active = i == selected;
        let message = on_select(i);
        let btn = iced::widget::button(
            iced::widget::text(*label)
                .size(14)
                .font(system_font(FontWeight::Medium))
                .align_x(iced::alignment::Horizontal::Center),
        )
        .padding(tab_padding)
        .width(Length::Fill)
        .on_press(message)
        .style(move |theme: &RustTheme, status| {
            let p = &theme.palette;
            let radius = theme.shape.full;
            if is_active {
                // Active tab: solid accent fill, accent contrast text.
                let bg = match status {
                    iced::widget::button::Status::Hovered
                    | iced::widget::button::Status::Pressed => {
                        Background::Color(blend(p.primary.main, Color::BLACK, 0.08))
                    }
                    _ => Background::Color(p.primary.main),
                };
                iced::widget::button::Style {
                    background: Some(bg),
                    text_color: Color::WHITE,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: radius.into(),
                    },
                    shadow: Shadow::default(),
                }
            } else {
                // Inactive tab: subtle surface fill so each tab reads as a
                // distinct clickable pill against the container background.
                let text_color = match status {
                    iced::widget::button::Status::Hovered => p.text.primary,
                    iced::widget::button::Status::Pressed => p.text.primary,
                    _ => p.text.secondary,
                };
                let bg = match status {
                    iced::widget::button::Status::Hovered => {
                        Background::Color(blend(p.background.paper, p.text.primary, 0.06))
                    }
                    iced::widget::button::Status::Pressed => {
                        Background::Color(blend(p.background.paper, p.text.primary, 0.12))
                    }
                    _ => Background::Color(blend(p.background.paper, p.text.primary, 0.03)),
                };
                iced::widget::button::Style {
                    background: Some(bg),
                    text_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: radius.into(),
                    },
                    shadow: Shadow::default(),
                }
            }
        });
        row = row.push(btn);
    }

    // Wrap the tab row in a rounded container with a subtle background,
    // like a card holding the tabs.
    iced::widget::column![
        iced::widget::container(row)
            .padding(bar_padding)
            .width(Length::Fill)
            .style(|theme: &RustTheme| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.background.default)),
                text_color: Some(theme.palette.text.primary),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: theme.shape.large.into(),
                },
                shadow: Shadow::default(),
            })
    ]
}

/// Blends `over` onto `under` at alpha `a` (0..1).
fn blend(under: Color, over: Color, a: f32) -> Color {
    Color::from_rgba(
        under.r * (1.0 - a) + over.r * a,
        under.g * (1.0 - a) + over.g * a,
        under.b * (1.0 - a) + over.b * a,
        under.a,
    )
}
