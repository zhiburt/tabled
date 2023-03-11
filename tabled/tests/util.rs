#![cfg(feature = "std")]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

use std::{
    borrow::Cow,
    ops::{Index, IndexMut},
};

use tabled::{
    grid::config::Position,
    grid::util::string::string_width_multiline,
    settings::{object::SegmentAll, Alignment, Modify},
    Table, Tabled,
};

/// A helper table factory.
///
/// It uses center alignment by default, because it's more complex and may spot more issues.
pub fn create_table<const ROWS: usize, const COLUMNS: usize>() -> Table {
    init_table::<ROWS, COLUMNS, _, &str>(std::iter::empty())
}

pub fn init_table<const ROWS: usize, const COLUMNS: usize, I, S>(init: I) -> Table
where
    I: IntoIterator<Item = (Position, S)>,
    S: Into<String>,
{
    let mut data = create_vector::<ROWS, COLUMNS>();
    for (pos, value) in init {
        data[pos.0][pos.1] = value.into();
    }

    new_table(data)
}

pub fn new_table<'a, T: Tabled>(iter: impl IntoIterator<Item = T> + 'a) -> Table {
    let mut table = Table::new(iter);
    table.with(Modify::new(SegmentAll).with(Alignment::center()));
    table
}

pub fn create_vector<const ROWS: usize, const COLUMNS: usize>() -> Vec<Obj<COLUMNS>> {
    let mut arr = Vec::with_capacity(ROWS);
    for row in 0..ROWS {
        let mut data = Vec::with_capacity(COLUMNS);
        for column in 0..COLUMNS {
            let text = format!("{row}-{column}");
            data.push(text);
        }

        arr.push(Obj::new(row, data));
    }

    arr
}

pub fn create_matrix<const ROWS: usize, const COLUMNS: usize>() -> Vec<Vec<String>> {
    let mut arr = Vec::with_capacity(ROWS);
    for row in 0..ROWS {
        let mut data = Vec::with_capacity(COLUMNS);
        for column in 0..COLUMNS {
            let text = format!("{row}-{column}");
            data.push(text);
        }

        arr.push(data);
    }

    arr
}

#[derive(Debug)]
pub struct Obj<const N: usize> {
    data: Vec<String>,
}

impl<const N: usize> Obj<N> {
    fn new(index: usize, mut data: Vec<String>) -> Self {
        assert_eq!(data.len(), N);
        data.insert(0, index.to_string());
        Self { data }
    }
}

impl<const N: usize> Index<usize> for Obj<N> {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> IndexMut<usize> for Obj<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize> Tabled for Obj<N> {
    const LENGTH: usize = N + 1;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        self.data.iter().cloned().map(Cow::Owned).collect()
    }

    fn headers() -> Vec<Cow<'static, str>> {
        std::iter::once("N".to_owned())
            .chain((0..N).map(|n| format!("column {n}")))
            .map(Cow::Owned)
            .collect()
    }
}

pub fn is_lines_equal(s: &str, width: usize) -> bool {
    string_width_multiline(s) == width
}

macro_rules! static_table {
    ($($line:expr)*) => {
        concat!(
            $($line, "\n",)*
        )
        .trim_end_matches('\n')
    };
}

pub(crate) use static_table;

macro_rules! test_table {
    ($test:ident, $table:expr, $($line:expr)*) => {
        #[test]
        fn $test() {
            let table = $table.to_string();
            println!("{}", table);
            assert_eq!(table, crate::util::static_table!($($line)*));
        }
    };
}

pub(crate) use test_table;

macro_rules! assert_table {
    ($table:expr, $($line:expr)*) => {
        let table = $table.to_string();
        println!("{}", table);
        assert_eq!(table, crate::util::static_table!($($line)*));
    };
}

pub(crate) use assert_table;
