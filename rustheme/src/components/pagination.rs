//! MUI-style `Pagination`: a row of circular page-number buttons plus
//! prev/next controls, with the current page highlighted.

use iced::widget::Row;
use crate::theme::RustTheme;

fn page_button<'a, Message>(
    label: String,
    is_active: bool,
    on_press: Option<Message>,
) -> iced::widget::Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut btn = iced::widget::button(
        iced::widget::text(label)
            .size(14)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .width(32)
    .height(32)
    .style(move |theme: &RustTheme, status| {
        let p = &theme.palette;
        if is_active {
            return iced::widget::button::Style {
                background: Some(iced::Background::Color(p.primary.main)),
                text_color: p.primary.contrast_text,
                border: iced::border::rounded(16),
                shadow: iced::Shadow::default(),
            };
        }
        let background = match status {
            iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => {
                Some(iced::Background::Color(iced::Color { a: 0.08, ..p.text.primary }))
            }
            _ => None,
        };
        let text_color = match status {
            iced::widget::button::Status::Disabled => p.text.disabled,
            _ => p.text.primary,
        };
        iced::widget::button::Style {
            background,
            text_color,
            border: iced::border::rounded(16),
            shadow: iced::Shadow::default(),
        }
    });
    if let Some(on_press) = on_press {
        btn = btn.on_press(on_press);
    }
    btn
}

/// A pagination control. `page` is the current 1-based page, `total` the
/// page count, and `on_select` maps a target 1-based page to a `Message`.
pub fn pagination<'a, Message>(
    page: usize,
    total: usize,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Row<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut row = iced::widget::row![].spacing(6).align_y(iced::alignment::Vertical::Center);

    row = row.push(page_button(
        "‹".to_string(),
        false,
        (page > 1).then(|| on_select(page - 1)),
    ));

    for p in 1..=total.max(1) {
        row = row.push(page_button(p.to_string(), p == page, Some(on_select(p))));
    }

    row = row.push(page_button(
        "›".to_string(),
        false,
        (page < total).then(|| on_select(page + 1)),
    ));

    row
}
