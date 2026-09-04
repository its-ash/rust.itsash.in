//! MUI-style `Accordion`: an expandable/collapsible section with a
//! clickable header and a body shown only when expanded.
//!
//! iced doesn't animate layout automatically, so expansion is a plain
//! conditional render rather than a height transition.

use iced::widget::Column;
use iced::Element;
use crate::theme::RustTheme;

/// An accordion section. `is_expanded` controls whether `body` renders;
/// clicking the header fires `on_toggle`.
pub fn accordion<'a, Message>(
    title: impl Into<String>,
    body: impl Into<Element<'a, Message, RustTheme>>,
    is_expanded: bool,
    on_toggle: Message,
) -> Column<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let chevron = if is_expanded { "⌄" } else { "›" };
    let header = iced::widget::button(
        iced::widget::row![
            iced::widget::text(title.into()).size(15),
            iced::widget::horizontal_space(),
            iced::widget::text(chevron).size(15),
        ]
        .align_y(iced::alignment::Vertical::Center),
    )
    .padding([12, 16])
    .width(iced::Length::Fill)
    .on_press(on_toggle)
    .style(|theme: &RustTheme, status| {
        let p = &theme.palette;
        let background = match status {
            iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => {
                Some(iced::Background::Color(iced::Color { a: 0.06, ..p.text.primary }))
            }
            _ => None,
        };
        iced::widget::button::Style {
            background,
            text_color: p.text.primary,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        }
    });

    let mut col = iced::widget::column![iced::widget::container(header).style(|theme| crate::style::container::outlined()(theme))];

    if is_expanded {
        col = col.push(
            iced::widget::container(body)
                .padding([12, 16])
                .width(iced::Length::Fill)
                .style(|theme: &RustTheme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(theme.palette.background.paper)),
                    text_color: Some(theme.palette.text.secondary),
                    border: iced::Border {
                        color: theme.palette.divider,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                }),
        );
    }

    col
}
