use crate::{
    grid::{
        config::{Entity, Position},
        spanned::GridConfig,
    },
    records::{ExactRecords, Records},
    settings::CellOption,
};

/// Columns (Vertical) span.
#[derive(Debug)]
pub struct ColumnSpan {
    size: usize,
}

impl ColumnSpan {
    /// Creates a new column (vertical) span.
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    /// Creates a new column (vertical) span with a maximux value possible.
    pub fn max() -> Self {
        Self::new(usize::MAX)
    }
}

impl<R> CellOption<R, GridConfig> for ColumnSpan
where
    R: Records + ExactRecords,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        set_col_spans(cfg, self.size, entity, (count_rows, count_cols));
    }
}

fn set_col_spans(cfg: &mut GridConfig, span: usize, entity: Entity, shape: (usize, usize)) {
    for pos in entity.iter(shape.0, shape.1) {
        if !is_valid_pos(pos, shape) {
            continue;
        }

        let mut span = span;
        if !is_column_span_valid(pos.1, span, shape.1) {
            span = shape.1 - pos.1;
        }

        if span_has_intersections(cfg, pos, span) {
            continue;
        }

        set_span_column(cfg, pos, span);
    }
}

fn set_span_column(cfg: &mut GridConfig, pos: (usize, usize), span: usize) {
    if span == 0 {
        let (row, col) = pos;
        if col == 0 {
            return;
        }

        if let Some(closecol) = closest_visible(cfg, (row, col - 1)) {
            let span = col + 1 - closecol;
            cfg.set_column_span((row, closecol), span);
        }
    }

    cfg.set_column_span(pos, span);
}

fn closest_visible(cfg: &GridConfig, mut pos: Position) -> Option<usize> {
    loop {
        if cfg.is_cell_visible(pos) {
            return Some(pos.1);
        }

        if pos.1 == 0 {
            return None;
        }

        pos.1 -= 1;
    }
}

fn is_column_span_valid(col: usize, span: usize, count_cols: usize) -> bool {
    span + col <= count_cols
}

fn is_valid_pos((row, col): Position, (count_rows, count_cols): (usize, usize)) -> bool {
    row < count_rows && col < count_cols
}

fn span_has_intersections(cfg: &GridConfig, (row, col): Position, span: usize) -> bool {
    for col in col..col + span {
        if !cfg.is_cell_visible((row, col)) {
            return true;
        }
    }

    false
}
