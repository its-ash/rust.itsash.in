//! A search input pre-wired with a leading search glyph and an optional
//! trailing clear ("×") button — the common search-box pattern, built on
//! [`crate::text_field`].

use crate::components::icons::{self, Icon};
use crate::theme::RustTheme;
use iced::widget::Container;
use iced::{Background, Border, Length, Shadow};

/// A filled search bar. `on_input` fires as the query changes; `on_clear`,
/// if given, shows a trailing "×" button (only when `value` is non-empty)
/// that fires when pressed.
pub fn search_bar<'a, Message>(
    placeholder: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
    on_clear: Option<Message>,
) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let field = iced::widget::text_input(placeholder, value)
        .on_input(on_input)
        .padding([10, 4])
        .size(14)
        .style(|theme: &RustTheme, _status| iced::widget::text_input::Style {
            background: Background::Color(iced::Color::TRANSPARENT),
            border: Border::default(),
            icon: theme.palette.text.secondary,
            placeholder: theme.palette.text.disabled,
            value: theme.palette.text.primary,
            selection: iced::Color { a: 0.3, ..theme.palette.primary.main },
        });

    let mut row = iced::widget::row![
        icons::icon(Icon::Search, 18.0)
            .style(|theme: &RustTheme| iced::widget::text::Style {
                color: Some(theme.palette.text.secondary),
            }),
        field.width(Length::Fill),
    ]
    .spacing(8)
    .align_y(iced::alignment::Vertical::Center);

    if let (Some(on_clear), false) = (on_clear.clone(), value.is_empty()) {
        row = row.push(
            iced::widget::button(icons::icon(Icon::Close, 16.0))
                .padding(2)
                .on_press(on_clear)
                .style(|theme: &RustTheme, status| {
                    let color = match status {
                        iced::widget::button::Status::Hovered
                        | iced::widget::button::Status::Pressed => theme.palette.text.primary,
                        _ => theme.palette.text.secondary,
                    };
                    iced::widget::button::Style {
                        background: None,
                        text_color: color,
                        border: Border::default(),
                        shadow: Shadow::default(),
                    }
                }),
        );
    }

    iced::widget::container(row)
        .padding([4, 14])
        .width(Length::Fill)
        .style(|theme: &RustTheme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.background.paper)),
            text_color: Some(theme.palette.text.primary),
            border: Border {
                color: theme.palette.divider,
                width: 1.0,
                radius: theme.material.pill_radius(theme.shape.full).into(),
            },
            shadow: Shadow::default(),
        })
}
