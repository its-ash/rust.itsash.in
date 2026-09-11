//! Material-style segmented controls for selecting one compact option.

use crate::theme::RustTheme;
use iced::widget::Container;
use iced::{Background, Border, Color, Length, Shadow};
use rustheme_core::{system_font, FontWeight};

/// A single-choice segmented control. `selected` is the active item index.
pub fn segmented<'a, Message>(
    labels: &[&'a str],
    selected: usize,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut row = iced::widget::row![].spacing(2);
    for (index, label) in labels.iter().enumerate() {
        let active = index == selected;
        let message = on_select(index);
        row = row.push(
            iced::widget::button(
                iced::widget::text(*label)
                    .size(13)
                    .font(system_font(FontWeight::Medium)),
            )
            .padding([8, 14])
            .width(Length::Fill)
            .on_press(message)
            .style(move |theme: &RustTheme, status| {
                let p = &theme.palette;
                let background = if active {
                    Some(Background::Color(p.primary.main))
                } else if matches!(status, iced::widget::button::Status::Hovered) {
                    Some(Background::Color(Color { a: 0.08, ..p.primary.main }))
                } else {
                    None
                };
                // Inset from the container's radius (which is padded by 3px)
                // so the active segment's rounded fill nests inside the
                // outer border instead of rendering square-cornered.
                let radius = (theme.shape.large - 3.0).max(0.0);
                iced::widget::button::Style {
                    background,
                    text_color: if active { p.primary.contrast_text } else { p.text.primary },
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: radius.into(),
                    },
                    shadow: Shadow::default(),
                }
            }),
        );
    }

    iced::widget::container(row)
        .padding(3)
        .width(Length::Fill)
        .style(|theme: &RustTheme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.background.default)),
            text_color: Some(theme.palette.text.primary),
            border: iced::Border {
                color: theme.palette.divider,
                width: 1.0,
                radius: theme.shape.large.into(),
            },
            shadow: Shadow::default(),
        })
}
