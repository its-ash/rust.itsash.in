//! A horizontal action toolbar: leading content (e.g. a title or breadcrumb),
//! a flexible spacer, and trailing action widgets — the row above a table
//! or list that holds bulk actions, filters, or a "New" button.

use crate::theme::RustTheme;
use iced::widget::{Container, Row};
use iced::{Element, Length};

/// A toolbar row. `leading` sits on the left; `trailing` actions are packed
/// on the right with the remaining space between them.
pub fn toolbar<'a, Message>(
    leading: impl Into<Element<'a, Message, RustTheme>>,
    trailing: Vec<Element<'a, Message, RustTheme>>,
) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut row: Row<'a, Message, RustTheme> = iced::widget::row![leading.into()]
        .spacing(12)
        .align_y(iced::alignment::Vertical::Center)
        .width(Length::Fill);

    row = row.push(iced::widget::horizontal_space());

    for action in trailing {
        row = row.push(action);
    }

    iced::widget::container(row.spacing(8))
        .padding([8, 4])
        .width(Length::Fill)
}
