//! MUI-style `Table`: a header row plus striped body rows built from a
//! simple `&[String]` header and `&[Vec<String>]` body.

use iced::widget::Column;
use crate::theme::RustTheme;

/// A simple data table. `headers` labels the columns; `rows` is the body
/// data, each inner `Vec` a row of cell text in column order.
pub fn table<'a, Message>(headers: &[&'a str], rows: &[Vec<String>]) -> Column<'a, Message, RustTheme>
where
    Message: 'a,
{
    let mut header_row = iced::widget::row![].spacing(0);
    for h in headers {
        header_row = header_row.push(
            iced::widget::container(iced::widget::text(*h).size(13))
                .padding([10, 12])
                .width(iced::Length::FillPortion(1))
                .style(|theme: &RustTheme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(theme.palette.background.default)),
                    text_color: Some(theme.palette.text.secondary),
                    ..Default::default()
                }),
        );
    }

    let mut col = iced::widget::column![iced::widget::container(header_row)
        .style(|theme| crate::style::container::outlined()(theme))];

    for (i, row_data) in rows.iter().enumerate() {
        let striped = i % 2 == 1;
        let mut row = iced::widget::row![].spacing(0);
        for cell in row_data {
            row = row.push(
                iced::widget::container(iced::widget::text(cell.clone()).size(13))
                    .padding([10, 12])
                    .width(iced::Length::FillPortion(1)),
            );
        }
        col = col.push(iced::widget::container(row).width(iced::Length::Fill).style(move |theme: &RustTheme| {
            let p = &theme.palette;
            iced::widget::container::Style {
                background: Some(iced::Background::Color(if striped {
                    iced::Color { a: 0.4, ..p.background.default }
                } else {
                    p.background.paper
                })),
                text_color: Some(p.text.primary),
                border: iced::Border {
                    color: p.divider,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow::default(),
            }
        }));
    }

    col
}
