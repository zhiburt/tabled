//! This module contains [`StyleCorrectSpan`] structure, which can be usefull when [`Span`] is used, and
//! you wan't to fix the intersections symbols which are left intact by default.
//!
//! [`Span`]: crate::Span

use crate::{
    grid::{
        config::{GridConfig, Position},
        grid_projection::GridProjection,
    },
    records::{ExactRecords, Records},
    settings::TableOption,
};

/// A correctness function of style for [`Table`] which has [`Span`]s.
///
/// Try to fix the style when table contains spans.
///
/// By default [`Style`] doesn't implies any logic to better render split lines when
/// [`Span`] is used.
///
/// So this function can be used to set the split lines in regard of spans used.
///
/// # Example
///
/// ```
/// use tabled::{
///     Table,
///     settings::{
///         Modify, style::{Style, BorderSpanCorrection},
///         format::Format, span::Span, object::Cell
///     }
/// };
///
/// let data = vec![
///     ("09", "June", "2022"),
///     ("10", "July", "2022"),
/// ];
///
/// let mut table = Table::new(data);
/// table.with(Modify::new((0, 0)).with("date").with(Span::horizontal(3)));
///
/// assert_eq!(
///     table.to_string(),
///     concat!(
///         "+----+------+------+\n",
///         "| date             |\n",
///         "+----+------+------+\n",
///         "| 09 | June | 2022 |\n",
///         "+----+------+------+\n",
///         "| 10 | July | 2022 |\n",
///         "+----+------+------+",
///     )
/// );
///
/// table.with(BorderSpanCorrection);
///
/// assert_eq!(
///     table.to_string(),
///     concat!(
///         "+------------------+\n",
///         "| date             |\n",
///         "+----+------+------+\n",
///         "| 09 | June | 2022 |\n",
///         "+----+------+------+\n",
///         "| 10 | July | 2022 |\n",
///         "+----+------+------+",
///     )
/// );
/// ```
/// See [`BorderSpanCorrection`].
///
/// [`Table`]: crate::Table
/// [`Span`]: crate::Span
/// [`Style::correct_spans`]: crate::Style::correct_spans
#[derive(Debug)]
pub struct BorderSpanCorrection;

impl<R, D> TableOption<R, D> for BorderSpanCorrection
where
    R: Records + ExactRecords,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, _: &mut D) {
        let shape = (records.count_rows(), records.count_columns());
        correct_span_styles(cfg, shape);
    }
}

fn correct_span_styles(cfg: &mut GridConfig, shape: (usize, usize)) {
    let spans = cfg.iter_span_columns().collect::<Vec<_>>();
    for &((row, c), span) in &spans {
        for col in c..c + span {
            if col == 0 {
                continue;
            }

            let is_first = col == c;
            let has_up = row > 0 && has_left(cfg, (row - 1, col), shape);
            let has_down = row + 1 < shape.0 && has_left(cfg, (row + 1, col), shape);

            let borders = cfg.get_borders();

            let gp = GridProjection::with_shape(cfg, shape);
            let mut border = gp.get_border((row, col));

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

            cfg.set_border((row, col), border);
        }
    }

    let spans = cfg.iter_span_rows().collect::<Vec<_>>();
    for &((r, col), span) in &spans {
        for row in r + 1..r + span {
            let gp = GridProjection::with_shape(cfg, shape);
            let mut border = gp.get_border((row, col));
            let borders = cfg.get_borders();

            let has_left_border = border.left_top_corner.is_some();
            if has_left_border {
                let has_left = col > 0 && has_top(cfg, (row, col - 1), shape);
                if has_left {
                    border.left_top_corner = borders.horizontal_right;
                } else {
                    border.left_top_corner = borders.vertical;
                }
            }

            let has_right_border = border.right_top_corner.is_some();
            if has_right_border {
                let has_right = col + 1 < shape.1 && has_top(cfg, (row, col + 1), shape);
                if has_right {
                    border.right_top_corner = borders.horizontal_left;
                } else {
                    border.right_top_corner = borders.vertical;
                }
            }

            cfg.set_border((row, col), border);
        }
    }

    let cells = iter_totaly_spanned_cells(cfg, shape).collect::<Vec<_>>();
    for (row, col) in cells {
        if row == 0 {
            continue;
        }

        let gp = GridProjection::with_shape(cfg, shape);
        let mut border = gp.get_border((row, col));
        let borders = cfg.get_borders();

        let has_right = col + 1 < shape.1 && has_top(cfg, (row, col + 1), shape);
        let has_up = has_left(cfg, (row - 1, col), shape);
        if has_up && !has_right {
            border.right_top_corner = borders.horizontal_right;
        }

        let has_down = row + 1 < shape.0 && has_left(cfg, (row + 1, col), shape);
        if has_down {
            border.left_bottom_corner = borders.top_intersection;
        }

        cfg.set_border((row, col), border);
    }
}

fn has_left(cfg: &GridConfig, pos: Position, shape: (usize, usize)) -> bool {
    let gp = GridProjection::with_shape(cfg, shape);
    if gp.is_cell_covered_by_both_spans(pos) || gp.is_cell_covered_by_column_span(pos) {
        return false;
    }

    let border = gp.get_border(pos);
    border.left.is_some() || border.left_top_corner.is_some() || border.left_bottom_corner.is_some()
}

fn has_top(cfg: &GridConfig, pos: Position, shape: (usize, usize)) -> bool {
    let gp = GridProjection::with_shape(cfg, shape);
    if gp.is_cell_covered_by_both_spans(pos) || gp.is_cell_covered_by_row_span(pos) {
        return false;
    }

    let border = gp.get_border(pos);
    border.top.is_some() || border.left_top_corner.is_some() || border.right_top_corner.is_some()
}

fn iter_totaly_spanned_cells(
    cfg: &GridConfig,
    shape: (usize, usize),
) -> impl Iterator<Item = Position> + '_ {
    // todo: can be optimized
    let (count_rows, count_cols) = shape;
    (0..count_rows).flat_map(move |row| {
        (0..count_cols)
            .map(move |col| (row, col))
            .filter(move |&p| {
                let gp = GridProjection::with_shape(cfg, shape);
                gp.is_cell_covered_by_both_spans(p)
            })
    })
}
