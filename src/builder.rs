use std::fmt::Display;

use papergrid::{AlignmentHorizontal, Entity, Grid, Settings};

use crate::{Style, Table};

#[derive(Debug, Default, Clone)]
pub struct Builder {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    size: usize,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header<H: IntoIterator<Item = T>, T: Display>(mut self, header: H) -> Self {
        let header: Vec<String> = header.into_iter().map(|t| t.to_string()).collect();
        self.update_size(header.len());
        self.headers = header;

        self
    }

    pub fn add_row<R: IntoIterator<Item = T>, T: Display>(mut self, row: R) -> Self {
        let row: Vec<String> = row.into_iter().map(|t| t.to_string()).collect();
        self.update_size(row.len());
        self.rows.push(row);

        self
    }

    pub fn build(self) -> Table {
        let count_columns = self.size;
        let count_rows = self.rows.len() + 1;
        let mut grid = Grid::new(count_rows, count_columns);

        // it's crusial to set a global setting rather than a setting for an each cell
        // as it will be hard to override that since how Grid::style method works
        grid.set(
            &Entity::Global,
            Settings::new()
                .indent(1, 1, 0, 0)
                .alignment(AlignmentHorizontal::Center),
        );

        for (i, h) in self.headers.iter().enumerate() {
            grid.set(&Entity::Cell(0, i), Settings::new().text(h));
        }

        let mut row = 1;
        for fields in self.rows {
            // don't show off a empty data array
            if fields.is_empty() {
                continue;
            }

            for (column, field) in fields.into_iter().enumerate() {
                grid.set(&Entity::Cell(row, column), Settings::new().text(field));
            }

            row += 1;
        }

        let table = Table { grid };
        table.with(Style::ASCII)
    }

    fn update_size(&mut self, size: usize) {
        if size > self.size {
            self.size = size;
        }
    }
}
