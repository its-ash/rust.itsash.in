//! MUI-style `Progress`: `linear` and `circular` (approximated with an arc-less
//! ring, since iced has no native circular progress widget).

use iced::widget::canvas::{self, Canvas};
use iced::widget::ProgressBar;
use iced::{Element, Length, Point, Rectangle, Renderer};
use crate::theme::RustTheme;

/// A linear progress bar, `value` in `0.0..=1.0`.
pub fn linear<'a, Message>(value: f32) -> ProgressBar<'a, RustTheme>
where
    Message: 'a,
{
    iced::widget::progress_bar(0.0..=1.0, value)
        .height(6)
        .style(|theme| crate::style::progress_bar::primary()(theme))
}

struct CircularProgress {
    value: f32,
    color: iced::Color,
    track_color: iced::Color,
}

impl<Message> canvas::Program<Message, RustTheme> for CircularProgress {
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
        let radius = (bounds.width.min(bounds.height) / 2.0) - 4.0;
        let stroke_width = 4.0;

        let track = canvas::Path::circle(center, radius);
        frame.stroke(
            &track,
            canvas::Stroke::default()
                .with_color(self.track_color)
                .with_width(stroke_width),
        );

        let start_angle = -std::f32::consts::FRAC_PI_2;
        let end_angle = start_angle + self.value.clamp(0.0, 1.0) * std::f32::consts::TAU;
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

/// A circular progress indicator, `value` in `0.0..=1.0`.
pub fn circular<'a, Message>(theme: &RustTheme, value: f32, diameter: f32) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    let program = CircularProgress {
        value,
        color: theme.palette.primary.main,
        track_color: theme.palette.divider,
    };
    Canvas::new(program)
        .width(Length::Fixed(diameter))
        .height(Length::Fixed(diameter))
        .into()
}
