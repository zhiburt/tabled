#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

use std::ops::{Index, IndexMut};

use tabled::Tabled;

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
    const LENGTH: usize = N;

    fn fields(&self) -> Vec<String> {
        self.data.clone()
    }

    fn headers() -> Vec<String> {
        std::iter::once("N".to_owned())
            .chain((0..N).map(|n| format!("column {}", n)))
            .collect()
    }
}

pub fn create_vector<const ROWS: usize, const COLUMNS: usize>() -> Vec<Obj<COLUMNS>> {
    let mut arr = Vec::with_capacity(ROWS);
    for row in 0..ROWS {
        let mut data = Vec::with_capacity(COLUMNS);
        for column in 0..COLUMNS {
            let text = format!("{}-{}", row, column);
            data.push(text);
        }

        arr.push(Obj::new(row, data))
    }

    arr
}

pub fn is_lines_equal(s: &str, width: usize) -> bool {
    papergrid::string_width_multiline(s) == width
}

macro_rules! static_table {
    ($($line:expr)*) => {
        concat!(
            $($line, "\n",)*
        )
    };
}

pub(crate) use static_table;
