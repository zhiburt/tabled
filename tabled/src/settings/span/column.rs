use std::cmp::{self, Ordering};

use crate::{
    grid::{
        config::{ColoredConfig, Entity, Position, SpannedConfig},
        records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    },
    settings::CellOption,
};

/// Columns (horizontal) span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnSpan {
    size: isize,
}

impl ColumnSpan {
    /// Creates a new column (horizontal) span.
    pub fn new(size: isize) -> Self {
        Self { size }
    }

    /// Creates a new column (horizontal) span with a maximux value possible.
    pub fn max() -> Self {
        Self::new(isize::MAX)
    }

    /// Creates a new column (horizontal) span with a min value possible.
    pub fn min() -> Self {
        Self::new(isize::MIN)
    }

    /// Creates a new column (horizontal) to spread all the columns.
    pub fn spread() -> Self {
        Self::new(0)
    }
}

impl<R> CellOption<R, ColoredConfig> for ColumnSpan
where
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();
        let shape = (count_rows, count_cols).into();

        for pos in entity.iter(count_rows, count_cols) {
            set_col_span(records, cfg, self.size, pos, shape);
        }

        remove_false_spans(cfg);
    }
}

fn set_col_span<R>(
    recs: &mut R,
    cfg: &mut SpannedConfig,
    span: isize,
    pos: Position,
    shape: Position,
) where
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    if !shape.has_coverage(pos) {
        return;
    }

    match span.cmp(&0) {
        Ordering::Less => {
            // * got correct value [span, col]
            // * clean the route from col to pos.col
            // * set the (pos.row, col) to content from pos
            // * set span

            let span = span.unsigned_abs();
            let (col, span) = if span > pos.col {
                (0, pos.col)
            } else {
                (pos.col - span, span)
            };

            let content = recs.get_text(pos).to_string();

            for i in col..span + 1 {
                recs.set(Position::new(pos.row, i), String::new());
            }

            recs.set(Position::new(pos.row, col), content);
            cfg.set_column_span(Position::new(pos.row, col), span + 1);
        }
        Ordering::Equal => {
            let content = recs.get_text(pos).to_string();
            let span = recs.count_columns();

            for i in 0..recs.count_columns() {
                recs.set(Position::new(pos.row, i), String::new());
            }

            recs.set(Position::new(pos.row, 0), content);
            cfg.set_column_span(Position::new(pos.row, 0), span);
        }
        Ordering::Greater => {
            let span = cmp::min(span as usize, shape.col - pos.col);
            if span_has_intersections(cfg, pos, span) {
                return;
            }

            set_span_column(cfg, pos, span);
        }
    }
}

fn set_span_column(cfg: &mut SpannedConfig, p: Position, span: usize) {
    if span == 0 {
        if p.col == 0 {
            return;
        }

        if let Some(nearcol) = closest_visible(cfg, p - (0, 1)) {
            let span = p.col + 1 - nearcol;
            cfg.set_column_span((p.row, nearcol).into(), span);
        }
    }

    cfg.set_column_span(p, span);
}

fn closest_visible(cfg: &SpannedConfig, mut pos: Position) -> Option<usize> {
    loop {
        if cfg.is_cell_visible(pos) {
            return Some(pos.col);
        }

        if pos.col == 0 {
            return None;
        }

        pos -= (0, 1);
    }
}

fn span_has_intersections(cfg: &SpannedConfig, p: Position, span: usize) -> bool {
    for col in p.col..p.col + span {
        if !cfg.is_cell_visible((p.row, col).into()) {
            return true;
        }
    }

    false
}

fn remove_false_spans(cfg: &mut SpannedConfig) {
    for (pos, _) in cfg.get_column_spans() {
        if cfg.is_cell_visible(pos) {
            continue;
        }

        cfg.set_row_span(pos, 1);
        cfg.set_column_span(pos, 1);
    }

    for (pos, _) in cfg.get_row_spans() {
        if cfg.is_cell_visible(pos) {
            continue;
        }

        cfg.set_row_span(pos, 1);
        cfg.set_column_span(pos, 1);
    }
}
