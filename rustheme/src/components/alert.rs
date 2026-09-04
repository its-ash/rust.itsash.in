//! MUI-style `Alert`: a tinted severity banner with an optional title and
//! dismiss action. Each variant reads its [`rustheme_core::ColorSet`] from
//! the live `RustTheme` at style-time, so it follows light/dark mode.

use iced::widget::Container;
use iced::Length;
use crate::theme::RustTheme;

/// The four MUI alert severities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Success,
    Info,
    Warning,
    Error,
}

fn banner<'a, Message>(
    severity: Severity,
    title: Option<String>,
    body: impl Into<String>,
    on_dismiss: Option<Message>,
) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let mut text_col = iced::widget::column![].spacing(4);
    if let Some(title) = title {
        text_col = text_col.push(iced::widget::text(title).size(15));
    }
    text_col = text_col.push(iced::widget::text(body.into()).size(14));

    let mut row = iced::widget::row![text_col.width(Length::Fill)]
        .spacing(12)
        .align_y(iced::alignment::Vertical::Center);

    if let Some(on_dismiss) = on_dismiss {
        row = row.push(
            iced::widget::button(iced::widget::text("✕").size(14))
                .padding(4)
                .on_press(on_dismiss)
                .style(move |theme: &RustTheme, status| {
                    let set = severity_color(theme, severity);
                    let a = match status {
                        iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => 0.15,
                        _ => 0.0,
                    };
                    iced::widget::button::Style {
                        background: Some(iced::Background::Color(iced::Color { a, ..set.contrast_text })),
                        text_color: set.contrast_text,
                        border: iced::border::rounded(12),
                        shadow: iced::Shadow::default(),
                    }
                }),
        );
    }

    iced::widget::container(row)
        .padding([12, 16])
        .width(Length::Fill)
        .style(move |theme: &RustTheme| {
            let set = severity_color(theme, severity);
            iced::widget::container::Style {
                background: Some(iced::Background::Color(set.light)),
                text_color: Some(set.contrast_text),
                border: iced::border::rounded(theme.shape.border_radius),
                shadow: iced::Shadow::default(),
            }
        })
}

/// A success-severity alert banner.
pub fn success<'a, Message>(body: impl Into<String>, on_dismiss: Option<Message>) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    banner(Severity::Success, None, body, on_dismiss)
}

/// An info-severity alert banner.
pub fn info<'a, Message>(body: impl Into<String>, on_dismiss: Option<Message>) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    banner(Severity::Info, None, body, on_dismiss)
}

/// A warning-severity alert banner.
pub fn warning<'a, Message>(body: impl Into<String>, on_dismiss: Option<Message>) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    banner(Severity::Warning, None, body, on_dismiss)
}

/// An error-severity alert banner.
pub fn error<'a, Message>(body: impl Into<String>, on_dismiss: Option<Message>) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    banner(Severity::Error, None, body, on_dismiss)
}

/// A severity alert banner with a title above the body text.
pub fn with_title<'a, Message>(
    severity: Severity,
    title: impl Into<String>,
    body: impl Into<String>,
    on_dismiss: Option<Message>,
) -> Container<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    banner(severity, Some(title.into()), body, on_dismiss)
}

/// Returns the [`rustheme_core::ColorSet`] for each MUI severity, read from `theme`.
pub fn severity_color(theme: &RustTheme, severity: Severity) -> rustheme_core::ColorSet {
    match severity {
        Severity::Success => theme.palette.success,
        Severity::Info => theme.palette.info,
        Severity::Warning => theme.palette.warning,
        Severity::Error => theme.palette.error,
    }
}
