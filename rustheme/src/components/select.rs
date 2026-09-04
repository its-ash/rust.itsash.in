//! MUI-style `Select`, backed by iced's `PickList` widget.

use iced::widget::pick_list::PickList;
use crate::theme::RustTheme;
use std::borrow::Borrow;

/// An outlined select/dropdown field.
pub fn select<'a, T, L, V, Message>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, V, Message, RustTheme>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone + 'a,
{
    iced::widget::pick_list(options, selected, on_selected)
        .padding(10)
        .style(|theme, status| crate::style::pick_list::outlined()(theme, status))
}
