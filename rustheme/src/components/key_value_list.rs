//! `KeyValueList`: a two-column label/value detail list — the "Order #1234
//! / Status: Shipped / Total: $42.00" pattern for a detail or summary panel.
//! Distinct from [`crate::table`], which is for multi-row tabular data
//! rather than a single record's properties.

use crate::theme::RustTheme;
use iced::widget::Column;
use iced::Length;
use rustheme_core::{system_font, system_monospace_font, FontWeight};

/// One label/value pair.
#[derive(Debug, Clone)]
pub struct KeyValue {
    pub label: String,
    pub value: String,
}

impl KeyValue {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into() }
    }
}

/// A vertical label/value list, each row divided by a hairline. Values
/// render in the monospace font role, matching how numeric/structured data
/// (IDs, prices, dates) is read elsewhere in RustTheme (e.g. [`crate::table`]).
pub fn key_value_list<'a, Message>(pairs: Vec<KeyValue>) -> Column<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut col = iced::widget::column![];
    let len = pairs.len();
    for (i, pair) in pairs.into_iter().enumerate() {
        let row = iced::widget::row![
            iced::widget::text(pair.label)
                .size(13)
                .font(system_font(FontWeight::Regular))
                .style(|theme: &RustTheme| iced::widget::text::Style {
                    color: Some(theme.palette.text.secondary),
                })
                .width(Length::FillPortion(2)),
            iced::widget::text(pair.value)
                .size(13)
                .font(system_monospace_font(FontWeight::Medium))
                .width(Length::FillPortion(3)),
        ]
        .spacing(12);

        col = col.push(iced::widget::container(row).padding([8, 0]));
        if i + 1 < len {
            col = col.push(iced::widget::horizontal_rule(1));
        }
    }
    col
}
