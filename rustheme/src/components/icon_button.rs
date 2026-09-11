//! MUI-style `IconButton`: a compact circular button for icon-only actions.
//!
//! Icons are accepted as `impl text::IntoFragment` (a single glyph/symbol).
//! All variants enforce a 44x44px minimum hit area. The default
//! [`primary`]/[`secondary`] now map onto the new neutral icon styles so a
//! row of icon buttons never competes with a nearby primary button; use
//! [`icon_primary`] for the one accent-filled icon action.

use iced::widget::button::Button;
use iced::widget::text;
use iced::{Alignment, Length};
use crate::theme::RustTheme;

/// Minimum tap target — 44px on a side, clearing the touch ergonomics bar.
const HIT: u16 = 44;

fn base<'a, Message>(glyph: impl text::IntoFragment<'a>) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::button(
        iced::widget::text(glyph)
            .size(18)
            .font(crate::components::icons::font())
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(0)
    .width(Length::Fixed(HIT as f32))
    .height(Length::Fixed(HIT as f32))
}

/// A neutral filled icon button (was: accent primary). Reads as a supporting
/// control, not the main action.
pub fn primary<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        crate::style::button::icon()(theme, status)
    })
}

/// A neutral filled icon button — alias for [`primary`].
pub fn secondary<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        crate::style::button::icon()(theme, status)
    })
}

/// A bordered icon button — 1px divider ring on a transparent fill.
pub fn outlined<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        crate::style::button::icon_bordered()(theme, status)
    })
}

/// A borderless, transparent icon button — the lightest variant.
pub fn text_icon<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        crate::style::button::icon()(theme, status)
    })
}

/// An accent-filled icon button — the one primary icon action on a surface.
pub fn icon_primary<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        crate::style::button::icon_primary()(theme, status)
    })
}

/// A destructive (error-red) icon button.
pub fn destructive<'a, Message>(glyph: impl text::IntoFragment<'a>, on_press: Message) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    base(glyph).on_press(on_press).style(|theme, status| {
        let mut s = crate::style::button::destructive()(theme, status);
        s.border = iced::border::rounded(50.0);
        s
    })
}
