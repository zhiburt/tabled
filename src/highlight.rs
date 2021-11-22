#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid, Settings};

pub use papergrid::Border;

pub struct Highlight {
    target: Target,
    border: Border,
}

impl Highlight {
    pub fn frame(border: Border) -> Self {
        Self::new(Target::Frame, border)
    }

    pub fn cell(row: usize, column: usize, border: Border) -> Self {
        Self::new(Target::Cell { row, column }, border)
    }

    pub fn row(row: usize, border: Border) -> Self {
        Self::row_range(row, row + 1, border)
    }

    pub fn row_range(start: usize, end: usize, border: Border) -> Self {
        assert!(end > start);
        Self::new(
            Target::Row {
                from: start,
                to: end,
            },
            border,
        )
    }

    pub fn column(column: usize, border: Border) -> Self {
        Self::column_range(column, column + 1, border)
    }

    pub fn column_range(start: usize, end: usize, border: Border) -> Self {
        assert!(end > start);
        Self::new(
            Target::Column {
                from: start,
                to: end,
            },
            border,
        )
    }

    fn new(target: Target, border: Border) -> Self {
        Self { target, border }
    }
}

pub enum Target {
    Cell { row: usize, column: usize },
    Row { from: usize, to: usize },
    Column { from: usize, to: usize },
    Frame,
}

impl TableOption for Highlight {
    fn change(&mut self, grid: &mut Grid) {
        match self.target {
            Target::Cell { row, column } => {
                let settings = Settings::default()
                    .border(self.border.clone())
                    .border_restriction(false);
                grid.set(&Entity::Cell(row, column), settings);
            }
            Target::Frame => {
                let settings = Settings::default()
                    .border(self.border.clone())
                    .border_restriction(false);
                grid.set(&Entity::Global, settings);
            }
            Target::Row { from, to } => {
                if to == from + 1 {
                    let settings = Settings::default()
                        .border(self.border.clone())
                        .border_restriction(false);
                    grid.set(&Entity::Row(from), settings);
                } else {
                    for row in from..to {
                        let mut border = self.border.clone();

                        let is_first_row = row == from;
                        let is_last_row = row + 1 == to;

                        if !is_first_row {
                            border.top = None;
                        }

                        if !is_last_row {
                            border.bottom = None;
                        }

                        let settings = Settings::default().border(border).border_restriction(false);
                        grid.set(&Entity::Row(row), settings);
                    }
                }
            }
            Target::Column { from, to } => {
                if to == from + 1 {
                    let settings = Settings::default()
                        .border(self.border.clone())
                        .border_restriction(false);
                    grid.set(&Entity::Column(from), settings);
                } else {
                    for column in from..to {
                        let mut border = self.border.clone();

                        let is_first_column = column == from;
                        let is_last_column = column + 1 == to;

                        if !is_first_column {
                            border.left = None;
                        }

                        if !is_last_column {
                            border.right = None;
                        }

                        let settings = Settings::default().border(border).border_restriction(false);
                        grid.set(&Entity::Column(column), settings);
                    }
                }
            }
        }
    }
}
