//! `DataCardGrid`: a wrapping grid of cards ([`crate::layout::grid`]) with
//! loading and empty states baked in, so a typical "fetch a list, render
//! cards" screen doesn't need to hand-roll the loading/empty branches every
//! time.

use crate::components::{empty_state, layout, skeleton};
use crate::components::icons::Icon;
use crate::theme::RustTheme;
use iced::widget::Column;
use iced::Element;

/// Renders `items` as a wrapping grid of `columns` cards. While `loading`
/// is true, renders `columns * skeleton_rows` skeleton card placeholders
/// instead (ignoring `items`); when `loading` is false and `items` is
/// empty, renders an [`empty_state::empty_state`] with `empty_icon` /
/// `empty_title` / `empty_description` instead of an empty grid.
#[allow(clippy::too_many_arguments)]
pub fn data_card_grid<'a, Message>(
    theme: &RustTheme,
    columns: usize,
    items: Vec<Element<'a, Message, RustTheme>>,
    loading: bool,
    skeleton_rows: usize,
    empty_icon: Icon,
    empty_title: impl Into<String>,
    empty_description: impl Into<String>,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    if loading {
        let columns = columns.max(1);
        let placeholders = (0..columns * skeleton_rows.max(1))
            .map(|_| {
                iced::widget::column![
                    skeleton::rectangular(160.0, 90.0),
                    skeleton::text(120.0),
                    skeleton::text(80.0),
                ]
                .spacing(8)
                .into()
            })
            .collect::<Vec<Element<'a, Message, RustTheme>>>();
        return Element::from(layout::grid(theme, columns, 2.0, placeholders));
    }

    if items.is_empty() {
        return empty_state::empty_state(empty_icon, empty_title, empty_description, None::<(&str, Message)>).into();
    }

    let grid: Column<'a, Message, RustTheme> = layout::grid(theme, columns, 2.0, items);
    grid.into()
}
