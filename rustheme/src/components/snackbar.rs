//! MUI-style `Snackbar`: a small message bar anchored to the bottom of the
//! screen, shown via `stack` when `is_visible` is true.

use iced::widget::stack;
use iced::{Element, Length};
use crate::theme::RustTheme;

/// Overlays a bottom-anchored snackbar over `base_content` when `is_visible`,
/// with a trailing "Dismiss" action that fires `on_dismiss`.
pub fn snackbar<'a, Message>(
    base_content: impl Into<Element<'a, Message, RustTheme>>,
    is_visible: bool,
    message: impl Into<String>,
    on_dismiss: Message,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let base = base_content.into();
    if !is_visible {
        return base;
    }

    let dismiss = iced::widget::button(iced::widget::text("Dismiss").size(13))
        .padding([4, 8])
        .on_press(on_dismiss)
        .style(|theme: &RustTheme, status| {
            let p = &theme.palette;
            let text_color = match status {
                iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => {
                    p.primary.light
                }
                _ => p.background.paper,
            };
            iced::widget::button::Style {
                background: None,
                text_color,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            }
        });

    let bar = iced::widget::container(
        iced::widget::row![iced::widget::text(message.into()), dismiss]
            .spacing(16)
            .align_y(iced::alignment::Vertical::Center),
    )
    .padding([12, 20])
    .style(|theme: &RustTheme| {
        let p = &theme.palette;
        iced::widget::container::Style {
            background: Some(iced::Background::Color(p.text.primary)),
            text_color: Some(p.background.paper),
            border: iced::border::rounded(theme.shape.border_radius),
            shadow: iced::Shadow {
                color: iced::Color { a: 0.3, ..iced::Color::BLACK },
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 6.0,
            },
        }
    });

    let positioned = iced::widget::container(bar)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Bottom);

    stack![base, positioned].into()
}
