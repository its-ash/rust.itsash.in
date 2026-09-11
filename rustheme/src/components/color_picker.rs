//! `ColorPicker`: a swatch grid plus a hex text field, the pattern used by
//! `examples/gallery`'s Create section generalized into a reusable
//! component. Fully controlled: the caller owns the selected `Color` and
//! the hex field's live text, and reacts to `on_swatch`/`on_hex_changed`.

use crate::theme::RustTheme;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Border, Color, Element, Length, Shadow};

/// One named swatch in a [`color_picker`] grid.
#[derive(Debug, Clone, Copy)]
pub struct Swatch {
    pub name: &'static str,
    pub color: Color,
}

impl Swatch {
    pub const fn new(name: &'static str, color: Color) -> Self {
        Self { name, color }
    }
}

/// A swatch grid + hex input. `swatches` are the selectable preset colors;
/// `selected` is the currently active color (used to ring-highlight a
/// matching swatch); `hex_value` is the live text of the hex field, which
/// the caller parses and applies on submit via `on_hex_changed`.
pub fn color_picker<'a, Message>(
    swatches: &[Swatch],
    selected: Option<Color>,
    hex_value: &'a str,
    on_swatch: impl Fn(usize) -> Message + 'a,
    on_hex_changed: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let grid = row(swatches.iter().enumerate().map(|(index, swatch)| {
        let is_selected = selected == Some(swatch.color);
        let color = swatch.color;
        button(text(""))
            .width(Length::Fixed(28.0))
            .height(Length::Fixed(28.0))
            .on_press(on_swatch(index))
            .style(move |theme: &RustTheme, status| {
                let hovered = matches!(status, iced::widget::button::Status::Hovered);
                button::Style {
                    background: Some(iced::Background::Color(color)),
                    text_color: Color::TRANSPARENT,
                    border: Border {
                        color: if is_selected { theme.palette.text.primary } else { Color::TRANSPARENT },
                        width: if is_selected { 2.0 } else { 0.0 },
                        radius: theme.shape.small.into(),
                    },
                    shadow: if hovered { Shadow { color: Color::BLACK, offset: iced::Vector::new(0.0, 1.0), blur_radius: 3.0 } } else { Shadow::default() },
                }
            })
            .into()
    }))
    .spacing(8)
    .wrap();

    let hex_field = text_input("#RRGGBB", hex_value)
        .on_input(on_hex_changed)
        .padding(10)
        .width(Length::Fixed(140.0))
        .style(|theme, status| crate::style::text_input::outlined()(theme, status));

    let preview = container(text(""))
        .width(Length::Fixed(36.0))
        .height(Length::Fixed(36.0))
        .style(move |theme: &RustTheme| container::Style {
            background: Some(iced::Background::Color(selected.unwrap_or(theme.palette.divider))),
            border: Border { color: theme.palette.divider, width: 1.0, radius: theme.shape.small.into() },
            ..Default::default()
        });

    column![
        grid,
        row![preview, hex_field].spacing(10).align_y(iced::alignment::Vertical::Center),
    ]
    .spacing(12)
    .into()
}

/// A small neutral palette suitable as a default swatch set (accent hues
/// only — not tied to any particular preset system).
pub const DEFAULT_SWATCHES: [Swatch; 8] = [
    Swatch::new("Slate", Color::from_rgb(0.29, 0.33, 0.41)),
    Swatch::new("Blue", Color::from_rgb(0.23, 0.51, 0.96)),
    Swatch::new("Teal", Color::from_rgb(0.0, 0.42, 0.42)),
    Swatch::new("Green", Color::from_rgb(0.13, 0.55, 0.33)),
    Swatch::new("Amber", Color::from_rgb(0.96, 0.62, 0.04)),
    Swatch::new("Orange", Color::from_rgb(0.92, 0.35, 0.10)),
    Swatch::new("Red", Color::from_rgb(0.86, 0.15, 0.15)),
    Swatch::new("Purple", Color::from_rgb(0.49, 0.23, 0.93)),
];
