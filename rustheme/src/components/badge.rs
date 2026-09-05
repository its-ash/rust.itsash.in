//! MUI-style `Badge`: a small count/dot/standard marker overlaid on a
//! corner of content, inset so it overlaps the corner rather than floating
//! outside it (matching MUI's default badge position).

use iced::widget::{stack, Container};
use iced::{Element, Length};
use crate::theme::RustTheme;
use rustheme_core::{system_font, ColorSet, FontWeight};

/// Which corner of the content the badge is anchored to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

/// The semantic color of a badge, mirroring MUI's `color` prop. Resolve to
/// a [`ColorSet`] via [`Color::resolve`], so the badge follows whichever
/// `RustTheme` (light/dark, or a custom palette) it's drawn with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Primary,
    Secondary,
    Error,
    Warning,
    Info,
    Success,
}

impl Color {
    pub fn resolve(self, theme: &RustTheme) -> ColorSet {
        let p = &theme.palette;
        match self {
            Self::Primary => p.primary,
            Self::Secondary => p.secondary,
            Self::Error => p.error,
            Self::Warning => p.warning,
            Self::Info => p.info,
            Self::Success => p.success,
        }
    }
}

/// How far the badge insets into its anchored corner, in logical pixels.
/// MUI overlaps a badge onto its corner rather than floating it fully
/// outside the content — this is the offset from the edges that achieves
/// that overlap for the ~20px pill / ~8px dot markers this module draws.
const INSET: f32 = 6.0;

fn positioned<'a, Message>(
    content: Element<'a, Message, RustTheme>,
    marker: Element<'a, Message, RustTheme>,
    position: Position,
) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    use iced::alignment::{Horizontal, Vertical};

    let (h, v, pad) = match position {
        Position::TopRight => (Horizontal::Right, Vertical::Top, [0.0, INSET, 0.0, 0.0]),
        Position::TopLeft => (Horizontal::Left, Vertical::Top, [0.0, 0.0, 0.0, INSET]),
        Position::BottomRight => (Horizontal::Right, Vertical::Bottom, [0.0, INSET, INSET, 0.0]),
        Position::BottomLeft => (Horizontal::Left, Vertical::Bottom, [0.0, 0.0, INSET, INSET]),
    };
    let [top, right, bottom, left] = pad;

    stack![
        content,
        iced::widget::container(marker)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(h)
            .align_y(v)
            .padding(iced::Padding { top, right, bottom, left }),
    ]
    .into()
}

fn pill<'a, Message>(text: String, color: Color) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    iced::widget::container(
        iced::widget::text(text)
            .size(11)
            .font(system_font(FontWeight::Medium))
            .align_x(iced::alignment::Horizontal::Center),
    )
    .padding([1, 6])
    .style(move |theme: &RustTheme| {
        let set = color.resolve(theme);
        iced::widget::container::Style {
            background: Some(iced::Background::Color(set.main)),
            text_color: Some(set.contrast_text),
            border: iced::border::rounded(8),
            shadow: iced::Shadow::default(),
        }
    })
}

/// Overlays a numeric count badge, inset into `position`, on `content`. A
/// count of `0` renders no badge; counts above `max` render as `"{max}+"`
/// (MUI's default `max` is 99).
pub fn count<'a, Message>(
    content: impl Into<Element<'a, Message, RustTheme>>,
    count: u32,
    max: u32,
    color: Color,
    position: Position,
) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    if count == 0 {
        return content.into();
    }
    let label = if count > max { format!("{max}+") } else { count.to_string() };
    positioned(content.into(), pill(label, color).into(), position)
}

/// Overlays a numeric count badge on the top-right corner with the default
/// `error` color and a max of 99 — the common case, matching this
/// function's previous signature.
pub fn simple_count<'a, Message>(
    content: impl Into<Element<'a, Message, RustTheme>>,
    count: u32,
) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    self::count(content, count, 99, Color::Error, Position::TopRight)
}

/// Overlays a small colored dot (no label), inset into `position`, on
/// `content`.
pub fn dot<'a, Message>(
    content: impl Into<Element<'a, Message, RustTheme>>,
    color: Color,
    position: Position,
) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    let marker = iced::widget::container(iced::widget::Space::new(8, 8)).style(move |theme: &RustTheme| {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(color.resolve(theme).main)),
            border: iced::border::rounded(4),
            ..Default::default()
        }
    });
    positioned(content.into(), marker.into(), position)
}

/// Overlays a small `error`-colored dot on the top-right corner — the
/// common case, matching this function's previous signature.
pub fn simple_dot<'a, Message>(content: impl Into<Element<'a, Message, RustTheme>>) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    dot(content, Color::Error, Position::TopRight)
}

/// A standalone badge pill with no anchored content, MUI's
/// `variant="standard"` used on its own (e.g. next to a nav label rather
/// than overlaid on an icon).
pub fn standard<'a, Message>(label: impl Into<String>, color: Color) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    pill(label.into(), color)
}
