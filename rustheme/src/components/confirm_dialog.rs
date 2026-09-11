//! A pre-built confirmation dialog: wraps [`crate::dialog::dialog`] with the
//! standard "Cancel" + confirm action row already wired up, so a yes/no
//! prompt is one call instead of hand-building the button row each time.

use crate::components::{button, dialog};
use crate::theme::RustTheme;
use iced::widget::text;
use iced::Element;
use rustheme_core::system_font;
use rustheme_core::FontWeight;

/// A standard confirmation dialog: title, body message, a "Cancel" button
/// (ghost), and a confirm button (primary). Use [`destructive`] instead
/// when confirming an irreversible action, to render the confirm button in
/// the error color.
pub fn confirm_dialog<'a, Message>(
    theme: &RustTheme,
    base_content: impl Into<Element<'a, Message, RustTheme>>,
    is_open: bool,
    title: impl Into<String>,
    message: impl text::IntoFragment<'a>,
    confirm_label: impl text::IntoFragment<'a>,
    on_cancel: Message,
    on_confirm: Message,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    build(theme, base_content, is_open, title, message, confirm_label, on_cancel, on_confirm, false)
}

/// Same as [`confirm_dialog`], but the confirm button renders in the
/// destructive (error) color — use for irreversible actions like delete.
pub fn destructive<'a, Message>(
    theme: &RustTheme,
    base_content: impl Into<Element<'a, Message, RustTheme>>,
    is_open: bool,
    title: impl Into<String>,
    message: impl text::IntoFragment<'a>,
    confirm_label: impl text::IntoFragment<'a>,
    on_cancel: Message,
    on_confirm: Message,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    build(theme, base_content, is_open, title, message, confirm_label, on_cancel, on_confirm, true)
}

#[allow(clippy::too_many_arguments)]
fn build<'a, Message>(
    theme: &RustTheme,
    base_content: impl Into<Element<'a, Message, RustTheme>>,
    is_open: bool,
    title: impl Into<String>,
    message: impl text::IntoFragment<'a>,
    confirm_label: impl text::IntoFragment<'a>,
    on_cancel: Message,
    on_confirm: Message,
    is_destructive: bool,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let confirm = if is_destructive {
        button::destructive(theme, confirm_label, on_confirm)
    } else {
        button::primary(theme, confirm_label, on_confirm)
    };

    dialog::dialog(
        theme,
        base_content,
        is_open,
        title,
        iced::widget::text(message).font(system_font(FontWeight::Regular)),
        vec![
            button::ghost(theme, "Cancel", on_cancel).into(),
            confirm.into(),
        ],
    )
}
