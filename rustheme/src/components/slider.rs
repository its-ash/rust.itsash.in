//! MUI-style `Slider`.

use iced::widget::slider::Slider;
use crate::theme::RustTheme;
use std::ops::RangeInclusive;

/// A primary-colored slider.
pub fn slider<'a, Message>(
    range: RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> Slider<'a, f32, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::slider(range, value, on_change)
        .style(|theme, status| crate::style::slider::primary()(theme, status))
}
