//! MUI-style `Stepper`: a horizontal sequence of numbered steps connected
//! by lines, each showing active/completed/upcoming state.

use iced::widget::Row;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

fn step_circle<'a, Message>(index: usize, is_active: bool, is_completed: bool) -> iced::widget::Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    let label = if is_completed { "✓".to_string() } else { (index + 1).to_string() };
    iced::widget::container(
        iced::widget::text(label)
            .size(13)
            .font(system_font(FontWeight::Medium))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .width(28)
    .height(28)
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
    .style(move |theme: &RustTheme| {
        let p = &theme.palette;
        let (background, text_color) = if is_completed || is_active {
            (p.primary.main, p.primary.contrast_text)
        } else {
            (p.divider, p.text.secondary)
        };
        iced::widget::container::Style {
            background: Some(iced::Background::Color(background)),
            text_color: Some(text_color),
            border: iced::border::rounded(14),
            shadow: iced::Shadow::default(),
        }
    })
}

/// A horizontal stepper. `labels` are the step titles; `active` is the
/// current 0-based step index — steps before it render as completed.
/// Padding scales with `theme.spacing`.
pub fn stepper<'a, Message>(theme: &RustTheme, labels: &[&'a str], active: usize) -> Row<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut row = iced::widget::row![].spacing(8).align_y(iced::alignment::Vertical::Center);
    let len = labels.len();

    for (i, label) in labels.iter().enumerate() {
        let is_completed = i < active;
        let is_active = i == active;

        let step = iced::widget::column![
            step_circle(i, is_active, is_completed),
            iced::widget::text(*label).size(12).font(system_font(FontWeight::Regular)),
        ]
        .spacing(4)
        .align_x(iced::alignment::Horizontal::Center);

        row = row.push(step);

        if i + 1 < len {
            row = row.push(
                iced::widget::container(iced::widget::horizontal_rule(2))
                    .width(iced::Length::Fixed(40.0))
                    .padding(iced::Padding {
                        top: 0.0,
                        right: 0.0,
                        bottom: theme.spacing.unit(1.75),
                        left: 0.0,
                    }),
            );
        }
    }

    row
}
