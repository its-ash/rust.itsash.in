//! A centered empty-state surface with an optional primary action.

use crate::components::button;
use crate::components::icons::{font as icon_font, glyph, Icon};
use crate::theme::RustTheme;
use iced::widget::Container;
use iced::{Background, Length, Shadow};
use rustheme_core::{system_font, FontWeight};

/// Builds an empty state with a semantic icon, title, supporting copy, and
/// an optional primary action.
pub fn empty_state<'a, Message>(
    theme: &RustTheme,
    icon: Icon,
    title: impl Into<String>,
    description: impl Into<String>,
    action: Option<(&'a str, Message)>,
) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut content = iced::widget::column![
        iced::widget::text(glyph(icon)).size(40).font(icon_font()),
        iced::widget::text(title.into()).size(18).font(system_font(FontWeight::Medium)),
        iced::widget::text(description.into()).size(14).font(system_font(FontWeight::Regular)),
    ]
    .spacing(8)
    .align_x(iced::alignment::Horizontal::Center);

    if let Some((label, message)) = action {
        content = content.push(button::primary(theme, label, message));
    }

    iced::widget::container(content)
        .padding(24)
        .width(Length::Fill)
        .style(|theme: &RustTheme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.background.paper)),
            text_color: Some(theme.palette.text.secondary),
            border: iced::Border {
                color: theme.palette.divider,
                width: 1.0,
                radius: theme.shape.large.into(),
            },
            shadow: Shadow::default(),
        })
}
