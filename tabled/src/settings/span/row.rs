use crate::{
    grid::{
        config::{Entity, GridConfig, Position},
        grid_projection::GridProjection,
    },
    records::{ExactRecords, Records},
    settings::CellOption,
};

/// Row (vertical) span.
#[derive(Debug)]
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

impl<R> CellOption<R> for RowSpan
where
    R: Records + ExactRecords,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        set_row_spans(cfg, self.size, entity, (count_rows, count_cols));
    }
}

fn set_row_spans(cfg: &mut GridConfig, span: usize, entity: Entity, shape: (usize, usize)) {
    for pos in entity.iter(shape.0, shape.1) {
        if !is_valid_pos(pos, shape) {
            continue;
        }

        let mut span = span;
        if !is_row_span_valid(pos.0, span, shape.0) {
            span = shape.0 - pos.0;
        }

        if span_has_intersections(cfg, pos, span, shape) {
            continue;
        }

        set_span_row(cfg, pos, span);
    }
}

fn set_span_row(cfg: &mut GridConfig, pos: (usize, usize), span: usize) {
    if span == 0 {
        let (row, col) = pos;
        if row == 0 {
            return;
        }

        let closerow = closest_visible_row(cfg, (row - 1, col));
        let span = row + 1 - closerow;

        cfg.set_row_span((closerow, col), span);

        return;
    }

    cfg.set_row_span(pos, span);
}

fn closest_visible_row(cfg: &GridConfig, mut pos: Position) -> usize {
    loop {
        if GridProjection::new(cfg).is_cell_visible(pos) {
            return pos.0;
        }

        if pos.0 == 0 {
            unreachable!("must never happen");
        }

        pos.0 -= 1;
    }
}

fn is_row_span_valid(row: usize, span: usize, count_rows: usize) -> bool {
    span + row <= count_rows
}

fn is_valid_pos((row, col): Position, (count_rows, count_cols): (usize, usize)) -> bool {
    row < count_rows && col < count_cols
}

fn span_has_intersections(
    cfg: &GridConfig,
    (row, col): Position,
    span: usize,
    shape: (usize, usize),
) -> bool {
    let gp = GridProjection::with_shape(cfg, shape);

    for row in row..row + span {
        if !gp.is_cell_visible((row, col)) {
            return true;
        }
    }

    false
}
