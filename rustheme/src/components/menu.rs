//! MUI-style popover `Menu`: a floating list of options shown via `stack`
//! when `is_open` is true.

use iced::widget::stack;
use iced::{Element, Length};
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// Overlays a dropdown `items` menu below `base_content` when `is_open`.
/// `on_select` maps a clicked item index to a `Message`; `on_dismiss` fires
/// when the scrim behind the menu is clicked.
pub fn menu<'a, Message>(
    theme: &RustTheme,
    base_content: impl Into<Element<'a, Message, RustTheme>>,
    is_open: bool,
    items: &[&'a str],
    on_select: impl Fn(usize) -> Message + 'a,
    on_dismiss: Message,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let base = base_content.into();
    if !is_open {
        return base;
    }

    let mut list = iced::widget::column![].width(Length::Fixed(200.0));
    for (i, label) in items.iter().enumerate() {
        let message = on_select(i);
        list = list.push(
            iced::widget::button(iced::widget::text(*label).font(system_font(FontWeight::Regular)))
                .width(Length::Fill)
                .padding([theme.spacing.unit(1.0), theme.spacing.unit(2.0)])
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
    .on_press(on_dismiss);

    stack![base, scrim, iced::widget::container(popover).padding(theme.spacing.unit(0.5))].into()
}
