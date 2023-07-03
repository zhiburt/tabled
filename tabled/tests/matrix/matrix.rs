use std::{
    fmt::{self, Display},
    iter::FromIterator,
    string::ToString,
};

use tabled::{
    grid::config::ColoredConfig,
    grid::dimension::CompleteDimensionVecRecords,
    grid::records::vec_records::{CellInfo, VecRecords},
    settings::{object::Segment, Alignment, Modify, TableOption},
    Table, Tabled,
};

use super::matrix_list::MatrixList;

/// A helper table factory.
///
/// It uses center alignment by default, because it's more complex and may spot more issues.
#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<Vec<String>>,
    size: (usize, usize),
}

impl Matrix {
    pub fn empty() -> Self {
        Self {
            data: vec![],
            size: (0, 0),
        }
    }

    pub fn with_no_frame(rows: usize, columns: usize) -> Self {
        Self {
            data: create_matrix(rows, columns),
            size: (rows, columns),
        }
    }

    pub fn new(rows: usize, columns: usize) -> Self {
        Self::with_no_frame(rows, columns)
            .with_header()
            .with_index()
    }

    pub fn vec(rows: usize, columns: usize) -> Vec<Vec<String>> {
        Self::new(rows, columns).to_vec()
    }

    pub fn table(rows: usize, columns: usize) -> Table {
        Self::new(rows, columns).to_table()
    }

    pub fn list<const ROWS: usize, const COLUMNS: usize>() -> Vec<MatrixList<COLUMNS, true>> {
        create_list::<ROWS, COLUMNS>()
    }

    pub fn iter<I, T>(iter: I) -> Table
    where
        I: IntoIterator<Item = T>,
        T: Tabled,
    {
        let mut table = tabled::Table::new(iter);
        table.with(Modify::new(Segment::all()).with(Alignment::center()));
        table
    }

    pub fn with_index(mut self) -> Self {
        set_index(&mut self.data);
        self
    }

    pub fn with_header(mut self) -> Self {
        set_header(&mut self.data, self.size.1);
        self
    }

    pub fn insert<V: ToString>(mut self, pos: tabled::grid::config::Position, value: V) -> Self {
        self.data[pos.0][pos.1] = value.to_string();
        self
    }

    pub fn to_table(&self) -> Table {
        let mut table = tabled::Table::from_iter(self.data.clone());
        table.with(Modify::new(Segment::all()).with(Alignment::center()));
        table
    }

    pub fn to_vec(&self) -> Vec<Vec<String>> {
        self.data.clone()
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

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.clone().to_table().fmt(f)
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

fn set_index(data: &mut [Vec<String>]) {
    if data.is_empty() {
        return;
    }

    data[0].insert(0, "N".to_owned());

    for (n, row) in data.iter_mut().skip(1).enumerate() {
        row.insert(0, n.to_string());
    }
}

fn create_list<const ROWS: usize, const COLUMNS: usize>() -> Vec<MatrixList<COLUMNS, true>> {
    let mut arr = Vec::with_capacity(ROWS);
    for row in 0..ROWS {
        let data = (0..COLUMNS)
            .map(|column| format!("{row}-{column}"))
            .collect::<Vec<_>>();
        let list = MatrixList::with_index(row, data);
        arr.push(list);
    }

    arr
}
