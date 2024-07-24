//! This module contains a [`PaddingExpand`] setting to expand cells padding to its limit a cell.

use papergrid::{
    config::{AlignmentHorizontal, AlignmentVertical},
    dimension::spanned::SpannedGridDimension,
    records::{ExactRecords, IntoRecords, PeekableRecords, Records},
};

#[cfg(feature = "std")]
use crate::{
    grid::{config::ColoredConfig, config::Entity},
    settings::CellOption,
};

use super::TableOption;

/// PaddingExpand is able to expand padding to its limit a cell.
/// Maybe usefull in cases were its colorization is supposed to be used next.
///
/// # Example
///
#[cfg_attr(feature = "ansi", doc = "```")]
#[cfg_attr(not(feature = "ansi"), doc = "```ignore")]
/// use std::iter::FromIterator;
///
/// use tabled::{
///     settings::{Padding, Style, Color, PaddingColor},
///     Table,
/// };
///
/// let char_split = |s: &str| s.chars().map(|c| c.to_string()).collect::<Vec<_>>();
/// let data = vec![
///     char_split("2021"),
///     char_split("2022"),
///     char_split("2023"),
///     char_split("2024"),
/// ];
///
/// let table = Table::from_iter(&data)
///     .with(Style::ascii())
///     .with(PaddingColor::filled(Color::BG_BLUE))
///     .modify((2, 1), Padding::new(2, 2, 3, 3))
///     .with(Padding::expand(true))
///     .with(Padding::expand(false))
///     .to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         "+---+-----+---+---+\n",
///         "|2\u{1b}[44m  \u{1b}[49m|0\u{1b}[44m    \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|1\u{1b}[44m  \u{1b}[49m|\n",
///         "+---+-----+---+---+\n",
///         "|2\u{1b}[44m  \u{1b}[49m|0\u{1b}[44m    \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|\n",
///         "+---+-----+---+---+\n",
///         "|2\u{1b}[44m  \u{1b}[49m|0\u{1b}[44m    \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|3\u{1b}[44m  \u{1b}[49m|\n",
///         "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
///         "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
///         "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
///         "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
///         "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
///         "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
///         "+---+-----+---+---+\n",
///         "|2\u{1b}[44m  \u{1b}[49m|0\u{1b}[44m    \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|4\u{1b}[44m  \u{1b}[49m|\n",
///         "+---+-----+---+---+",
///     ),
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PaddingExpand {
    /// Horizontal expansion of padding (LEFT and RIGHT)
    Horizontal,
    /// Vertical expansion of padding (TOP and BOTTOM)
    Vertical,
}

impl<R, D> TableOption<R, ColoredConfig, D> for PaddingExpand
where
    R: Records + ExactRecords + PeekableRecords,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        <Self as CellOption<R, ColoredConfig>>::change(self, records, cfg, Entity::Global)
    }
}

impl<R> CellOption<R, ColoredConfig> for PaddingExpand
where
    R: Records + ExactRecords + PeekableRecords,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        match self {
            PaddingExpand::Vertical => expand_vertical(records, cfg, entity),
            PaddingExpand::Horizontal => expand_horizontal(records, cfg, entity),
        }
    }
}

fn expand_horizontal<R>(records: &mut R, cfg: &mut ColoredConfig, entity: Entity)
where
    R: Records + ExactRecords + PeekableRecords,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let widths = SpannedGridDimension::width(&*records, cfg);

    let count_rows = records.count_rows();
    let count_cols = records.count_columns();

    for (row, col) in entity.iter(count_rows, count_cols) {
        let pos = Entity::Cell(row, col);

        let column_width = widths[col];
        let width = records.get_width((row, col));

        if width < column_width {
            let alignment = *cfg.get_alignment_horizontal(pos);

            let available_width = column_width - width;
            let (left, right) = split_horizontal_space(alignment, available_width);

            let mut pad = cfg.get_padding(pos);
            pad.left.size = left;
            pad.right.size = right;

            cfg.set_padding(pos, pad);
        }
    }
}

fn expand_vertical<R>(records: &mut R, cfg: &mut ColoredConfig, entity: Entity)
where
    R: Records + ExactRecords + PeekableRecords,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let heights = SpannedGridDimension::height(&*records, cfg);

    let count_rows = records.count_rows();
    let count_cols = records.count_columns();

    for (row, col) in entity.iter(count_rows, count_cols) {
        let pos = Entity::Cell(row, col);

        let row_height = heights[row];
        let cell_height = records.count_lines((row, col));

        if cell_height < row_height {
            let alignment = *cfg.get_alignment_vertical(pos);

            let available_width = row_height - cell_height;
            let (top, bottom) = split_vertical_space(alignment, available_width);

            let mut pad = cfg.get_padding(pos);
            pad.top.size = top;
            pad.bottom.size = bottom;

            cfg.set_padding(pos, pad);
        }
    }
}

fn split_horizontal_space(al: AlignmentHorizontal, space: usize) -> (usize, usize) {
    match al {
        AlignmentHorizontal::Center => {
            let left = space / 2;
            let right = space - left;
            (left, right)
        }
        AlignmentHorizontal::Left => (0, space),
        AlignmentHorizontal::Right => (space, 0),
    }
}

fn split_vertical_space(al: AlignmentVertical, space: usize) -> (usize, usize) {
    match al {
        AlignmentVertical::Center => {
            let left = space / 2;
            let right = space - left;
            (left, right)
        }
        AlignmentVertical::Top => (0, space),
        AlignmentVertical::Bottom => (space, 0),
    }
}
