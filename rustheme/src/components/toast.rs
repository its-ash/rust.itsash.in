//! `Toast`: a stack of multiple simultaneous notifications anchored to a
//! screen corner, each with a severity tint and dismiss button — the
//! multi-item upgrade to [`crate::snackbar`], which only shows one message
//! at a time. Use this when several notifications can be in flight at once
//! (e.g. background task completions); use `snackbar` for a single
//! transient confirmation.

use crate::components::alert::Severity;
use crate::theme::RustTheme;
use iced::widget::stack;
use iced::{Element, Length};
use rustheme_core::{system_font, FontWeight};

/// One queued toast notification. Give each a stable `id` (e.g. an
/// incrementing counter in your app state) so [`Toast`] messages can target
/// the right entry for dismissal.
#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u64,
    pub severity: Severity,
    pub message: String,
}

impl Toast {
    pub fn new(id: u64, severity: Severity, message: impl Into<String>) -> Self {
        Self { id, severity, message: message.into() }
    }
}

/// Which corner the toast stack anchors to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    TopRight,
    #[default]
    BottomRight,
    BottomLeft,
    TopLeft,
}

/// Overlays a vertically-stacked list of `toasts` over `base_content`,
/// anchored to `position`. `on_dismiss` maps a clicked toast's `id` to a
/// `Message`. Renders nothing when `toasts` is empty.
pub fn toast_stack<'a, Message>(
    base_content: impl Into<Element<'a, Message, RustTheme>>,
    toasts: &[Toast],
    position: Position,
    on_dismiss: impl Fn(u64) -> Message + 'a,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let base = base_content.into();
    if toasts.is_empty() {
        return base;
    }

    let mut col = iced::widget::column![].spacing(8).width(Length::Fixed(320.0));
    for toast in toasts {
        let id = toast.id;
        let on_dismiss_msg = on_dismiss(id);
        let severity = toast.severity;
        let dismiss = iced::widget::button(iced::widget::text("×").size(16))
            .padding(2)
            .on_press(on_dismiss_msg)
            .style(move |theme: &RustTheme, status| {
                let set = crate::components::alert::severity_color(theme, severity);
                let color = match status {
                    iced::widget::button::Status::Hovered
                    | iced::widget::button::Status::Pressed => set.dark,
                    _ => set.main,
                };
                iced::widget::button::Style {
                    background: None,
                    text_color: color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                }
            });

        let row = iced::widget::row![
            iced::widget::text(toast.message.clone())
                .size(13)
                .font(system_font(FontWeight::Regular))
                .width(Length::Fill),
            dismiss,
        ]
        .spacing(12)
        .align_y(iced::alignment::Vertical::Center);

        col = col.push(
            iced::widget::container(row)
                .padding([10, 14])
                .width(Length::Fill)
                .style(move |theme: &RustTheme| {
                    let set = crate::components::alert::severity_color(theme, severity);
                    iced::widget::container::Style {
                        background: Some(iced::Background::Color(iced::Color { a: 0.14, ..set.main })),
                        text_color: Some(theme.palette.text.primary),
                        border: iced::Border {
                            color: iced::Color { a: 0.4, ..set.main },
                            width: 1.0,
                            radius: theme.shape.small.into(),
                        },
                        shadow: rustheme_core::Elevation::Level2.shadow(),
                    }
                }),
        );
    }

    let (h_align, v_align) = match position {
        Position::TopRight => (iced::alignment::Horizontal::Right, iced::alignment::Vertical::Top),
        Position::BottomRight => (iced::alignment::Horizontal::Right, iced::alignment::Vertical::Bottom),
        Position::BottomLeft => (iced::alignment::Horizontal::Left, iced::alignment::Vertical::Bottom),
        Position::TopLeft => (iced::alignment::Horizontal::Left, iced::alignment::Vertical::Top),
    };

    let positioned = iced::widget::container(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .align_x(h_align)
        .align_y(v_align);

    stack![base, positioned].into()
}
