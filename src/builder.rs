use std::{fmt::Display, iter::FromIterator};

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

    pub fn header<H, T>(mut self, header: H) -> Self
    where
        H: IntoIterator<Item = T>,
        T: Display,
    {
        let header: Vec<String> = header.into_iter().map(|t| t.to_string()).collect();
        self.update_size(header.len());
        self.headers = header;

        self
    }

    pub fn add_row<R, T>(mut self, row: R) -> Self
    where
        R: IntoIterator<Item = T>,
        T: Display,
    {
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

impl<R, V> FromIterator<R> for Builder
where
    R: IntoIterator<Item = V>,
    V: Display,
{
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        let mut builder = Self::default();

        let mut iterator = iter.into_iter();
        if let Some(header) = iterator.next() {
            builder = builder.header(header);
        }

        for row in iterator {
            builder = builder.add_row(row);
        }

        builder
    }
}
