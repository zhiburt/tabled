use crate::grid::GridConfig;

pub mod height;
pub mod width;

pub trait Estimate<R> {
    fn estimate(&mut self, records: R, cfg: &GridConfig);
    fn get(&self, i: usize) -> Option<usize>;
    fn total(&self) -> usize;
}
