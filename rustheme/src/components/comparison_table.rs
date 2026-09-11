//! `ComparisonTable`: a pricing/feature comparison grid — plan headers
//! across the top (one optionally highlighted as "recommended"), feature
//! rows down the side, and a checkmark/dash/text cell per plan.

use crate::components::icons::{self, Icon};
use crate::theme::RustTheme;
use iced::widget::{Column, Row};
use iced::{Background, Border, Element, Length, Shadow};
use rustheme_core::{heading_font, system_font, FontWeight};

/// One plan/column header.
#[derive(Debug, Clone)]
pub struct Plan {
    pub name: String,
    pub price: String,
    /// Highlights this column as the recommended choice (accent border +
    /// tinted header).
    pub highlighted: bool,
}

impl Plan {
    pub fn new(name: impl Into<String>, price: impl Into<String>, highlighted: bool) -> Self {
        Self { name: name.into(), price: price.into(), highlighted }
    }
}

/// One cell's content in a feature row.
#[derive(Debug, Clone)]
pub enum Cell {
    /// A green checkmark — the plan includes this feature.
    Included,
    /// A muted dash — the plan does not include this feature.
    Excluded,
    /// Free-form text (e.g. "Up to 5 seats", "Unlimited").
    Text(String),
}

/// A feature row: the row label plus one [`Cell`] per plan, in the same
/// order as the `plans` slice passed to [`comparison_table`].
#[derive(Debug, Clone)]
pub struct FeatureRow {
    pub label: String,
    pub cells: Vec<Cell>,
}

impl FeatureRow {
    pub fn new(label: impl Into<String>, cells: Vec<Cell>) -> Self {
        Self { label: label.into(), cells }
    }
}

/// Builds a plan comparison table: a header row of `plans` and a body of
/// `rows`, each row's `cells` aligned to the corresponding plan column.
pub fn comparison_table<'a, Message>(plans: &[Plan], rows: &[FeatureRow]) -> Column<'a, Message, RustTheme>
where
    Message: 'a,
{
    let label_width = Length::FillPortion(2);
    let plan_width = Length::FillPortion(2);

    let mut header: Row<'a, Message, RustTheme> = iced::widget::row![iced::widget::horizontal_space().width(label_width)].spacing(1);
    for plan in plans {
        let highlighted = plan.highlighted;
        header = header.push(
            iced::widget::container(
                iced::widget::column![
                    iced::widget::text(plan.name.clone())
                        .size(14)
                        .font(heading_font(FontWeight::Bold)),
                    iced::widget::text(plan.price.clone())
                        .size(13)
                        .font(system_font(FontWeight::Regular))
                        .style(move |theme: &RustTheme| iced::widget::text::Style {
                            color: Some(theme.palette.text.secondary),
                        }),
                ]
                .spacing(2)
                .align_x(iced::alignment::Horizontal::Center),
            )
            .padding(12)
            .width(plan_width)
            .align_x(iced::alignment::Horizontal::Center)
            .style(move |theme: &RustTheme| {
                if highlighted {
                    iced::widget::container::Style {
                        background: Some(Background::Color(iced::Color { a: 0.10, ..theme.palette.primary.main })),
                        text_color: Some(theme.palette.text.primary),
                        border: Border {
                            color: theme.palette.primary.main,
                            width: 1.0,
                            radius: theme.shape.small.into(),
                        },
                        shadow: Shadow::default(),
                    }
                } else {
                    iced::widget::container::Style::default()
                }
            }),
        );
    }

    let mut col = iced::widget::column![header].spacing(4);
    col = col.push(iced::widget::horizontal_rule(1));

    let last = rows.len().saturating_sub(1);
    for (i, feature_row) in rows.iter().enumerate() {
        let mut row: Row<'a, Message, RustTheme> = iced::widget::row![
            iced::widget::text(feature_row.label.clone())
                .size(13)
                .font(system_font(FontWeight::Medium))
                .width(label_width),
        ]
        .spacing(1)
        .align_y(iced::alignment::Vertical::Center);

        for cell in &feature_row.cells {
            let content: Element<'a, Message, RustTheme> = match cell {
                Cell::Included => icons::icon(Icon::Check, 18.0)
                    .style(|theme: &RustTheme| iced::widget::text::Style {
                        color: Some(theme.palette.success.main),
                    })
                    .into(),
                Cell::Excluded => iced::widget::text("—")
                    .size(14)
                    .style(|theme: &RustTheme| iced::widget::text::Style {
                        color: Some(theme.palette.text.disabled),
                    })
                    .into(),
                Cell::Text(text) => iced::widget::text(text.clone())
                    .size(13)
                    .font(system_font(FontWeight::Regular))
                    .into(),
            };
            row = row.push(
                iced::widget::container(content)
                    .padding(10)
                    .width(plan_width)
                    .align_x(iced::alignment::Horizontal::Center),
            );
        }

        col = col.push(row);
        if i != last {
            col = col.push(iced::widget::horizontal_rule(1));
        }
    }

    col
}
