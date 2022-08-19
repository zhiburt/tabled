//! This module contains [`StyleCorrectSpan`] structure, which can be usefull when [`Span`] is used, and
//! you wan't to fix the intersections symbols which are left intact by default.
//!
//! [`Span`]: crate::Span

use papergrid::{records::Records, Position};

use crate::{Table, TableOption};

/// A correctnes function of style for [`Table`] which has [`Span`]s.
///
/// See [`Style::correct_spans`].
///
/// [`Table`]: crate::Table
/// [`Span`]: crate::Span
#[derive(Debug)]
pub struct StyleCorrectSpan;

impl<R> TableOption<R> for StyleCorrectSpan
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        correct_span_styles(table);
    }
}

fn correct_span_styles<R>(table: &mut Table<R>)
where
    R: Records,
{
    let spans = table.get_config().iter_column_spans().collect::<Vec<_>>();
    for &((row, c), span) in &spans {
        for col in c..c + span {
            if col == 0 {
                continue;
            }

            let is_first = col == c;
            let has_up = row > 0 && has_left(table, (row - 1, col));
            let has_down = row + 1 < table.shape().0 && has_left(table, (row + 1, col));

            let mut border = table.get_config().get_border((row, col), table.shape());
            let borders = table.get_config().get_borders();

            let has_top_border = border.left_top_corner.is_some() && border.top.is_some();
            if has_top_border {
                if has_up && is_first {
                    border.left_top_corner = borders.intersection;
                } else if has_up {
                    border.left_top_corner = borders.bottom_intersection;
                } else if is_first {
                    border.left_top_corner = borders.top_intersection;
                } else {
                    border.left_top_corner = border.top;
                }
            }

            let has_bottom_border = border.left_bottom_corner.is_some() && border.bottom.is_some();
            if has_bottom_border {
                if has_down && is_first {
                    border.left_bottom_corner = borders.intersection;
                } else if has_down {
                    border.left_bottom_corner = borders.top_intersection;
                } else if is_first {
                    border.left_bottom_corner = borders.bottom_intersection;
                } else {
                    border.left_bottom_corner = border.bottom;
                }
            }

            table.get_config_mut().set_border((row, col), border);
        }
    }

    let spans = table.get_config().iter_row_spans().collect::<Vec<_>>();
    for &((r, col), span) in &spans {
        for row in r + 1..r + span {
            let mut border = table.get_config().get_border((row, col), table.shape());
            let borders = table.get_config().get_borders();

            let has_left_border = border.left_top_corner.is_some();
            if has_left_border {
                let has_left = col > 0 && has_top(table, (row, col - 1));
                if has_left {
                    border.left_top_corner = borders.horizontal_right;
                } else {
                    border.left_top_corner = borders.vertical;
                }
            }

            let has_right_border = border.right_top_corner.is_some();
            if has_right_border {
                let has_right = col + 1 < table.shape().1 && has_top(table, (row, col + 1));
                if has_right {
                    border.right_top_corner = borders.horizontal_left;
                } else {
                    border.right_top_corner = borders.vertical;
                }
            }

            table.get_config_mut().set_border((row, col), border);
        }
    }

    let cells = iter_totaly_spanned_cells(table).collect::<Vec<_>>();
    for (row, col) in cells {
        if row == 0 {
            continue;
        }

        let mut border = table.get_config().get_border((row, col), table.shape());
        let borders = table.get_config().get_borders();

        let has_right = col + 1 < table.shape().1 && has_top(table, (row, col + 1));
        let has_up = has_left(table, (row - 1, col));
        if has_up && !has_right {
            border.right_top_corner = borders.horizontal_right;
        }

        let has_down = row + 1 < table.shape().0 && has_left(table, (row + 1, col));
        if has_down {
            border.left_bottom_corner = borders.top_intersection;
        }

        table.get_config_mut().set_border((row, col), border);
    }
}

fn has_left<R>(table: &Table<R>, pos: Position) -> bool
where
    R: Records,
{
    let cfg = table.get_config();
    if cfg.is_cell_covered_by_both_spans(pos) || cfg.is_cell_covered_by_column_span(pos) {
        return false;
    }

    let border = cfg.get_border(pos, table.shape());
    border.left.is_some() || border.left_top_corner.is_some() || border.left_bottom_corner.is_some()
}

fn has_top<R>(table: &Table<R>, pos: Position) -> bool
where
    R: Records,
{
    let cfg = table.get_config();
    if cfg.is_cell_covered_by_both_spans(pos) || cfg.is_cell_covered_by_row_span(pos) {
        return false;
    }

    let border = cfg.get_border(pos, table.shape());
    border.top.is_some() || border.left_top_corner.is_some() || border.right_top_corner.is_some()
}

fn iter_totaly_spanned_cells<R>(table: &Table<R>) -> impl Iterator<Item = Position> + '_
where
    R: Records,
{
    // todo: can be optimized
    let (count_rows, count_cols) = table.shape();
    (0..count_rows).flat_map(move |row| {
        (0..count_cols)
            .map(move |col| (row, col))
            .filter(move |&p| table.get_config().is_cell_covered_by_both_spans(p))
    })
}
