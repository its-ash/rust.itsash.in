//! A compact metric/KPI card: a label, a large value, and an optional
//! trend indicator (e.g. "+12.4%") tinted success/error by sign — the
//! pattern used in dashboards for headline numbers.

use crate::theme::RustTheme;
use iced::widget::Container;
use iced::Length;
use rustheme_core::{heading_font, system_font, system_monospace_font, FontWeight};

/// Which direction a [`stat_card`]'s trend value moved, controlling its tint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    /// Tinted with `palette.success` (e.g. "+12.4%").
    Up,
    /// Tinted with `palette.error` (e.g. "-3.1%").
    Down,
    /// Tinted with `palette.text.secondary` — no directional change.
    Neutral,
}

/// A metric card with just a label and value — no trend indicator.
pub fn stat_card<'a, Message>(label: impl Into<String>, value: impl Into<String>) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    build(label.into(), value.into(), None)
}

/// A metric card with a label, value, and a tinted trend line underneath
/// (e.g. `stat_card::with_trend("Revenue", "$48.2k", "+12.4% vs last month", stat_card::Trend::Up)`).
pub fn with_trend<'a, Message>(
    label: impl Into<String>,
    value: impl Into<String>,
    trend_label: impl Into<String>,
    trend: Trend,
) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    build(label.into(), value.into(), Some((trend_label.into(), trend)))
}

fn build<'a, Message>(
    label: String,
    value: String,
    trend: Option<(String, Trend)>,
) -> Container<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut content = iced::widget::column![
        iced::widget::text(label)
            .size(13)
            .font(system_font(FontWeight::Medium))
            .style(|theme: &RustTheme| iced::widget::text::Style {
                color: Some(theme.palette.text.secondary),
            }),
        iced::widget::text(value)
            .size(28)
            .font(heading_font(FontWeight::Bold)),
    ]
    .spacing(4);

    if let Some((trend_label, trend)) = trend {
        content = content.push(
            iced::widget::text(trend_label)
                .size(12)
                .font(system_monospace_font(FontWeight::Medium))
                .style(move |theme: &RustTheme| iced::widget::text::Style {
                    color: Some(match trend {
                        Trend::Up => theme.palette.success.main,
                        Trend::Down => theme.palette.error.main,
                        Trend::Neutral => theme.palette.text.secondary,
                    }),
                }),
        );
    }

    iced::widget::container(content)
        .padding(18)
        .width(Length::Fill)
        .style(|theme| crate::style::container::card()(theme))
}
