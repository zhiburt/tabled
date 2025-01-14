use crate::{
    grid::{
        config::{ColoredConfig, Entity, Position, SpannedConfig},
        records::{ExactRecords, Records},
    },
    settings::CellOption,
};

/// Row (vertical) span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RowSpan {
    size: usize,
}

impl RowSpan {
    /// Creates a new row (vertical) span.
    pub const fn new(size: usize) -> Self {
        Self { size }
    }

    /// Creates a new row (vertical) span with a maximux value possible.
    pub const fn max() -> Self {
        Self::new(usize::MAX)
    }
}

impl<R> CellOption<R, ColoredConfig> for RowSpan
where
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        set_row_spans(cfg, self.size, entity, (count_rows, count_cols));
        remove_false_spans(cfg);
    }
}

fn set_row_spans(cfg: &mut SpannedConfig, span: usize, entity: Entity, shape: (usize, usize)) {
    for p in entity.iter(shape.0, shape.1) {
        if !p.is_covered(shape.into()) {
            continue;
        }

        let mut span = span;
        if !is_row_span_valid(p.row(), span, shape.0) {
            span = shape.0 - p.row();
        }

        if span_has_intersections(cfg, p, span) {
            continue;
        }

        set_span_row(cfg, p, span);
    }
}

fn set_span_row(cfg: &mut SpannedConfig, p: Position, span: usize) {
    if span == 0 {
        if p.row() == 0 {
            return;
        }

        if let Some(closerow) = closest_visible_row(cfg, p - (1, 0)) {
            let span = p.row() + 1 - closerow;
            cfg.set_row_span((closerow, p.col()).into(), span);
        }
    }

    cfg.set_row_span(p, span);
}

fn closest_visible_row(cfg: &SpannedConfig, mut pos: Position) -> Option<usize> {
    loop {
        if cfg.is_cell_visible(pos) {
            return Some(pos.row());
        }

        if pos.row() == 0 {
            // can happen if we have a above horizontal spanned cell
            return None;
        }

        pos -= (1, 0);
    }
}

fn is_row_span_valid(row: usize, span: usize, count_rows: usize) -> bool {
    span + row <= count_rows
}

fn span_has_intersections(cfg: &SpannedConfig, p: Position, span: usize) -> bool {
    for row in p.row()..p.row() + span {
        if !cfg.is_cell_visible((row, p.col()).into()) {
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
