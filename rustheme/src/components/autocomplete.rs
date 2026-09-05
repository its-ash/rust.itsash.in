//! MUI-style `Autocomplete`: a text field bound to a query string, with a
//! filtered dropdown of matches shown below it via `stack`/`column`.
//!
//! Filtering itself (matching `query` against a candidate list) is left to
//! the caller — this component only renders whatever already-filtered
//! `suggestions` it's given, since plain substring filtering is simple
//! enough to live in the consumer's `update` and keeps this component
//! generic over how matching works.

use iced::widget::Column;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// An autocomplete field: a text input plus a dropdown of `suggestions`
/// shown when non-empty. `on_input` fires as the query changes; `on_select`
/// maps a clicked suggestion's index to a `Message`.
pub fn autocomplete<'a, Message>(
    placeholder: &'a str,
    query: &'a str,
    suggestions: &[String],
    on_input: impl Fn(String) -> Message + 'a,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Column<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let field = iced::widget::text_input(placeholder, query)
        .on_input(on_input)
        .padding(10)
        .style(|theme, status| crate::style::text_input::outlined()(theme, status));

    let mut col = iced::widget::column![field].spacing(0);

    if !suggestions.is_empty() {
        let mut list = iced::widget::column![].width(iced::Length::Fill);
        for (i, suggestion) in suggestions.iter().enumerate() {
            let message = on_select(i);
            list = list.push(
                iced::widget::button(iced::widget::text(suggestion.clone()).size(14).font(system_font(FontWeight::Regular)))
                    .width(iced::Length::Fill)
                    .padding([8, 12])
                    .on_press(message)
                    .style(|theme: &RustTheme, status| {
                        let p = &theme.palette;
                        let background = match status {
                            iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => {
                                Some(iced::Background::Color(iced::Color { a: 0.08, ..p.primary.main }))
                            }
                            _ => None,
                        };
                        iced::widget::button::Style {
                            background,
                            text_color: p.text.primary,
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        }
                    }),
            );
        }

        col = col.push(
            iced::widget::container(list)
                .padding(4)
                .style(|theme| crate::style::container::elevated()(theme)),
        );
    }

    col
}
