//! MUI-style `Stack` and `Grid` layout helpers: thin wrappers over iced's
//! `row`/`column` that apply [`rustheme_core::Spacing`] as a consistent gap.
//! These are pure layout composition — they don't paint anything themselves,
//! so they don't need a `style::Catalog` entry.

use iced::widget::{Column, Row};
use iced::Element;
use crate::theme::RustTheme;

/// Direction for [`stack`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Row,
    Column,
}

/// A MUI `Stack`-equivalent: lays `children` out in `direction`, gapped by
/// `spacing_factor` multiples of the theme's base spacing unit.
pub fn stack<'a, Message>(
    theme: &RustTheme,
    direction: Direction,
    spacing_factor: f32,
    children: Vec<Element<'a, Message, RustTheme>>,
) -> Element<'a, Message, RustTheme>
where
    Message: 'a,
{
    let gap = theme.spacing.unit(spacing_factor);
    match direction {
        Direction::Row => {
            let mut row: Row<'a, Message, RustTheme> = iced::widget::row![].spacing(gap);
            for child in children {
                row = row.push(child);
            }
            row.into()
        }
        Direction::Column => {
            let mut col: Column<'a, Message, RustTheme> = iced::widget::column![].spacing(gap);
            for child in children {
                col = col.push(child);
            }
            col.into()
        }
    }
}

/// A MUI `Grid`-equivalent: wraps `children` into rows of `columns` items,
/// gapped by `spacing_factor` multiples of the theme's base spacing unit in
/// both axes.
pub fn grid<'a, Message>(
    theme: &RustTheme,
    columns: usize,
    spacing_factor: f32,
    children: Vec<Element<'a, Message, RustTheme>>,
) -> Column<'a, Message, RustTheme>
where
    Message: 'a,
{
    let gap = theme.spacing.unit(spacing_factor);
    let columns = columns.max(1);
    let mut col = iced::widget::column![].spacing(gap);
    let mut iter = children.into_iter().peekable();

    while iter.peek().is_some() {
        let mut row: Row<'a, Message, RustTheme> = iced::widget::row![].spacing(gap);
        for _ in 0..columns {
            match iter.next() {
                Some(child) => row = row.push(child),
                None => break,
            }
        }
        col = col.push(row);
    }

    col
}
