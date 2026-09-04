//! MUI-style `Skeleton`: a loading placeholder shape (`text`, `rectangular`,
//! `circular`) filled with a muted color.
//!
//! True MUI skeletons pulse/shimmer via CSS animation; animating in iced
//! requires a `Subscription` driving a time-varying alpha, which is out of
//! scope here. A static muted fill is used instead — visually correct as a
//! placeholder, just without the shimmer motion.

use iced::widget::Container;
use iced::Length;
use crate::theme::RustTheme;

fn shaded<'a, Message>(width: Length, height: Length, radius: f32) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(iced::widget::Space::new(Length::Fill, Length::Fill))
        .width(width)
        .height(height)
        .style(move |theme: &RustTheme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color { a: 0.5, ..theme.palette.divider })),
            border: iced::border::rounded(radius),
            ..Default::default()
        })
}

/// A single-line text skeleton, `width` pixels wide.
pub fn text<'a, Message>(width: f32) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    shaded(Length::Fixed(width), Length::Fixed(16.0), 4.0)
}

/// A rectangular block skeleton of the given size.
pub fn rectangular<'a, Message>(width: f32, height: f32) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    shaded(Length::Fixed(width), Length::Fixed(height), 4.0)
}

/// A circular skeleton (e.g. standing in for an avatar) of the given diameter.
pub fn circular<'a, Message>(diameter: f32) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    shaded(Length::Fixed(diameter), Length::Fixed(diameter), diameter / 2.0)
}
