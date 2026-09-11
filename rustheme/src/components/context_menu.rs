//! `ContextMenu`: a right-click-triggered popover menu anchored to its
//! target content — the same overlay mechanics as [`crate::menu`], but
//! opened via a right-press on `target` instead of a left-press on a
//! trigger button.
//!
//! Anchoring is to the wrapped `target`'s corner, not the literal cursor
//! position (iced 0.13 doesn't expose click coordinates to a plain
//! component without a custom `canvas`/event subscription) — for most
//! right-click menus (on a list row, a card, a canvas item) this reads the
//! same as cursor-anchoring since the target is usually small.

use crate::theme::RustTheme;
use iced::widget::stack;
use iced::{Element, Length};
use rustheme_core::{system_font, FontWeight};

/// Wraps `target` so right-clicking it opens a popover `items` menu
/// (tracked externally via `is_open`). `on_open` fires on right-press;
/// `on_select` maps a clicked item index to a `Message`; `on_dismiss` fires
/// when the scrim behind the menu is clicked.
pub fn context_menu<'a, Message>(
    target: impl Into<Element<'a, Message, RustTheme>>,
    is_open: bool,
    items: &[&'a str],
    on_open: Message,
    on_select: impl Fn(usize) -> Message + 'a,
    on_dismiss: Message,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let target = iced::widget::mouse_area(target.into()).on_right_press(on_open);

    if !is_open {
        return target.into();
    }

    let mut list = iced::widget::column![].width(Length::Fixed(200.0));
    for (i, label) in items.iter().enumerate() {
        let message = on_select(i);
        list = list.push(
            iced::widget::button(iced::widget::text(*label).font(system_font(FontWeight::Regular)))
                .width(Length::Fill)
                .padding([8, 16])
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

    let popover = iced::widget::container(list).style(|theme| crate::style::container::elevated()(theme));

    let scrim = iced::widget::mouse_area(
        iced::widget::container(iced::widget::Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .on_press(on_dismiss.clone())
    .on_right_press(on_dismiss);

    stack![
        target,
        scrim,
        iced::widget::container(popover).padding(4),
    ]
    .into()
}
