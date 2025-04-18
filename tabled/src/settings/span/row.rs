use std::cmp::{self, Ordering};

use crate::{
    grid::{
        config::{ColoredConfig, Entity, Position, SpannedConfig},
        records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    },
    settings::CellOption,
};

/// Row (vertical) span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RowSpan {
    size: isize,
}

impl RowSpan {
    /// Creates a new row (vertical) span.
    pub const fn new(size: isize) -> Self {
        Self { size }
    }

    /// Creates a new row (vertical) span with a maximux value possible.
    pub const fn max() -> Self {
        Self::new(isize::MAX)
    }

    /// Creates a new row (vertical) span with a min value possible.
    pub fn min() -> Self {
        Self::new(isize::MIN)
    }

    /// Creates a new row (vertical) to spread on all rows.
    pub fn spread() -> Self {
        Self::new(0)
    }
}

impl<R> CellOption<R, ColoredConfig> for RowSpan
where
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();
        let shape = (count_rows, count_cols).into();

        for pos in entity.iter(count_rows, count_cols) {
            set_span(records, cfg, self.size, pos, shape);
        }

        remove_false_spans(cfg);
    }
}

fn set_span<R>(recs: &mut R, cfg: &mut SpannedConfig, span: isize, pos: Position, shape: Position)
where
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    if !shape.has_coverage(pos) {
        return;
    }

    match span.cmp(&0) {
        Ordering::Less => {
            // * got correct value [span, row]
            // * clean the route from row to pos.row
            // * set the (row, pos.col) to content from pos
            // * set span

            let span = span.unsigned_abs();
            let (row, span) = if span > pos.row {
                (0, pos.row)
            } else {
                (pos.row - span, span)
            };

            let content = recs.get_text(pos).to_string();

            for i in row..span + 1 {
                recs.set(Position::new(i, pos.col), String::new());
            }

            recs.set(Position::new(row, pos.col), content);
            cfg.set_row_span(Position::new(row, pos.col), span + 1);
        }
        Ordering::Equal => {
            let content = recs.get_text(pos).to_string();
            let span = recs.count_rows();

            for i in 0..recs.count_rows() {
                recs.set(Position::new(i, pos.col), String::new());
            }

            recs.set(Position::new(0, pos.col), content);
            cfg.set_row_span(Position::new(0, pos.col), span);
        }
        Ordering::Greater => {
            let span = cmp::min(span as usize, shape.row - pos.row);
            if span_has_intersections(cfg, pos, span) {
                return;
            }

            set_span_row(cfg, pos, span);
        }
    }
}

fn set_span_row(cfg: &mut SpannedConfig, p: Position, span: usize) {
    if span == 0 {
        if p.col == 0 {
            return;
        }

        if let Some(nearcol) = closest_visible(cfg, p - (0, 1)) {
            let span = p.col + 1 - nearcol;
            cfg.set_row_span((p.row, nearcol).into(), span);
        }
    }

    cfg.set_row_span(p, span);
}

fn closest_visible(cfg: &SpannedConfig, mut pos: Position) -> Option<usize> {
    loop {
        if cfg.is_cell_visible(pos) {
            return Some(pos.row);
        }

        if pos.row == 0 {
            // can happen if we have a above horizontal spanned cell
            return None;
        }

        pos -= (1, 0);
    }
}

fn span_has_intersections(cfg: &SpannedConfig, p: Position, span: usize) -> bool {
    for row in p.row..p.row + span {
        if !cfg.is_cell_visible((row, p.col).into()) {
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
