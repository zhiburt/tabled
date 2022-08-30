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
/// [`Style::correct_spans`]: crate::Style::correct_spans
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
            let has_up = row > 0 && has_vertical(table, &spans, (row - 1, col));
            let has_down = row + 1 < table.shape().0 && has_vertical(table, &spans, (row + 1, col));

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
}

fn has_vertical<R>(grid: &Table<R>, spans: &[(Position, usize)], pos: Position) -> bool
where
    R: Records,
{
    if is_in_span_range(spans, pos) {
        return spans.iter().any(|&(p, _)| p == pos);
    }

    if grid.get_config().is_cell_visible(pos) {
        let border = grid.get_config().get_border(pos, grid.shape());
        return border.left.is_some()
            || border.left_top_corner.is_some()
            || border.left_bottom_corner.is_some();
    }

    false
}

fn is_in_span_range(spans: &[(Position, usize)], pos: Position) -> bool {
    spans
        .iter()
        .any(|&((row, col), span)| row == pos.0 && pos.1 > col && pos.1 < col + span)
}
