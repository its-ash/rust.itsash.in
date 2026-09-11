//! `Spinner`: an indeterminate loading indicator — a rotating arc rendered
//! on a `Canvas`. Unlike [`crate::progress::circular`] (a determinate ring
//! for a known `0.0..=1.0` value), the spinner has no value: it just spins
//! to signal "working" with no known completion time.
//!
//! Rotation needs a continuously advancing angle, which only the host
//! application can drive (a `Canvas::Program` has no timer of its own) —
//! so `angle` is owned by application state, advanced on each tick of an
//! `iced::time::every` subscription. See `examples/gallery` for the wiring:
//! a `SpinnerTick(Instant)` message advances `self.spinner_angle` and
//! `subscription()` schedules the ticks.

use iced::widget::canvas::{self, Canvas};
use iced::{Element, Length, Point, Rectangle, Renderer};
use crate::theme::RustTheme;

struct SpinnerProgram {
    angle: f32,
    color: iced::Color,
}

impl<Message> canvas::Program<Message, RustTheme> for SpinnerProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &RustTheme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = (bounds.width.min(bounds.height) / 2.0) - 3.0;
        let stroke_width = 3.0;

        // A 270-degree arc (not a full circle) sweeping with `angle` reads
        // as spinning motion rather than a static ring.
        let sweep = std::f32::consts::TAU * 0.75;
        let start_angle = self.angle;
        let end_angle = start_angle + sweep;

        let arc = canvas::Path::new(|builder| {
            builder.arc(canvas::path::Arc {
                center,
                radius,
                start_angle: iced::Radians(start_angle),
                end_angle: iced::Radians(end_angle),
            });
        });
        frame.stroke(
            &arc,
            canvas::Stroke::default()
                .with_color(self.color)
                .with_width(stroke_width)
                .with_line_cap(canvas::LineCap::Round),
        );

        vec![frame.into_geometry()]
    }
}

/// A spinning indeterminate loading indicator, `diameter` pixels wide,
/// colored with the theme's primary color. `angle` (radians) is owned and
/// advanced by the host application — see the module docs.
pub fn spinner<'a, Message>(theme: &RustTheme, angle: f32, diameter: f32) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    spinner_colored(angle, diameter, theme.palette.primary.main)
}

/// Like [`spinner`], but with an explicit color instead of the theme's
/// primary — useful on a dark overlay/scrim where the primary color may
/// not read well.
pub fn spinner_colored<'a, Message>(angle: f32, diameter: f32, color: iced::Color) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    Canvas::new(SpinnerProgram { angle, color })
        .width(Length::Fixed(diameter))
        .height(Length::Fixed(diameter))
        .into()
}
