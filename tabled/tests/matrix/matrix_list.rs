use std::{
    borrow::Cow,
    iter::once,
    ops::{Index, IndexMut},
};

use tabled::Tabled;

#[derive(Debug)]
pub struct MatrixList<const N: usize, const INDEX: bool> {
    data: Vec<String>,
}

impl<const N: usize> MatrixList<N, false> {
    #[allow(dead_code)]
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }
}

impl<const N: usize> MatrixList<N, true> {
    pub fn with_index(index: usize, mut data: Vec<String>) -> Self {
        assert_eq!(data.len(), N);
        data.insert(0, index.to_string());
        Self { data }
    }
}

impl<const N: usize, const INDEX: bool> Index<usize> for MatrixList<N, INDEX> {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, const INDEX: bool> IndexMut<usize> for MatrixList<N, INDEX> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize, const INDEX: bool> Tabled for MatrixList<N, INDEX> {
    const LENGTH: usize = N + 1;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        self.data.iter().cloned().map(Cow::Owned).collect()
    }

    fn headers() -> Vec<Cow<'static, str>> {
        let header = (0..N).map(|n| format!("column {n}"));

        match INDEX {
            true => once("N".to_owned()).chain(header).map(Cow::Owned).collect(),
            false => header.map(Cow::Owned).collect(),
        }
    }
}
