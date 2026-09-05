//! MUI-style `Breadcrumbs`: a horizontal path of clickable segments
//! separated by a "›" glyph, with the last segment shown as current/disabled.

use iced::widget::Row;
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// A breadcrumb trail. `items` are the segment labels in order; `on_select`
/// maps a clicked segment index to a `Message`. The final segment is
/// rendered as plain (non-interactive) text, matching MUI's default.
pub fn breadcrumbs<'a, Message>(
    items: &[&'a str],
    on_select: impl Fn(usize) -> Message + 'a,
) -> Row<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut row = iced::widget::row![].spacing(8).align_y(iced::alignment::Vertical::Center);
    let len = items.len();

    for (i, label) in items.iter().enumerate() {
        let is_last = i + 1 == len;
        if is_last {
            row = row.push(iced::widget::text(*label).size(14).font(system_font(FontWeight::Regular)).style(|theme: &RustTheme| iced::widget::text::Style {
                color: Some(theme.palette.text.primary),
            }));
        } else {
            let message = on_select(i);
            row = row.push(
                iced::widget::button(iced::widget::text(*label).size(14).font(system_font(FontWeight::Regular)))
                    .padding(0)
                    .on_press(message)
                    .style(|theme: &RustTheme, status| {
                        let p = &theme.palette;
                        let text_color = match status {
                            iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => {
                                p.primary.dark
                            }
                            _ => p.primary.main,
                        };
                        iced::widget::button::Style {
                            background: None,
                            text_color,
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        }
                    }),
            );
            row = row.push(iced::widget::text("›").size(14).font(system_font(FontWeight::Regular)).style(|theme: &RustTheme| iced::widget::text::Style {
                color: Some(theme.palette.text.disabled),
            }));
        }
    }

    row
}
