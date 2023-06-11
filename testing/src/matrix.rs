use std::{iter::FromIterator, string::ToString};
use tabled::{
    grid::config::ColoredConfig,
    grid::dimension::CompleteDimensionVecRecords,
    grid::records::vec_records::{CellInfo, VecRecords},
    settings::{object::Segment, Alignment, Modify, TableOption},
    Table,
};

/// A helper table factory.
///
/// It uses center alignment by default, because it's more complex and may spot more issues.
pub struct Matrix {
    data: Vec<Vec<String>>,
    size: (usize, usize),
}

impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            data: create_matrix(rows, columns),
            size: (rows, columns),
        }
    }

    pub fn full(rows: usize, columns: usize) -> Self {
        Self::new(rows, columns).with_header().with_index()
    }

    pub fn with_index(mut self) -> Self {
        set_index(&mut self.data, self.size.1);
        self
    }

    pub fn with_header(mut self) -> Self {
        set_header(&mut self.data, self.size.0);
        self
    }

    pub fn insert<V: ToString>(mut self, pos: tabled::grid::config::Position, value: V) -> Self {
        self.data[pos.0][pos.1] = value.to_string();
        self
    }

    pub fn to_table(self) -> Table {
        let mut table = tabled::Table::from_iter(self.data);
        table.with(Modify::new(Segment::all()).with(Alignment::center()));
        table
    }

    pub fn to_vec(self) -> Vec<Vec<String>> {
        self.data
    }

    pub fn with<O>(self, opt: O) -> Table
    where
        O: TableOption<
            VecRecords<CellInfo<String>>,
            CompleteDimensionVecRecords<'static>,
            ColoredConfig,
        >,
    {
        let mut table = self.to_table();
        table.with(opt);
        table
    }
}

fn create_matrix(rows: usize, columns: usize) -> Vec<Vec<String>> {
    let mut arr = Vec::with_capacity(rows);
    for row in 0..rows {
        let mut data = Vec::with_capacity(columns);
        for column in 0..columns {
            let text = format!("{row}-{column}");
            data.push(text);
        }

        arr.push(data);
    }

    arr
}

fn set_header(data: &mut Vec<Vec<String>>, columns: usize) {
    data.insert(
        0,
        (0..columns)
            .map(|n| format!("column {n}"))
            .collect::<Vec<_>>(),
    );
}

fn set_index(data: &mut [Vec<String>], rows: usize) {
    if rows == 0 {
        return;
    }

    data[0].insert(0, "N".to_owned());

    (1..rows).for_each(|row| {
        data[row].insert(0, row.to_string());
    });
}
