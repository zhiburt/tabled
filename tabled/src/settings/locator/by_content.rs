use crate::{
    grid::config::Entity,
    grid::{
        config::Position,
        records::{ExactRecords, PeekableRecords, Records},
    },
    settings::locator::Locator,
    settings::object::Object,
};

/// The structure is an implementation of [`Locator`] to search for cells with a specified content by it's name.
/// A name is considered be a value in a first row.
///
/// So even if in reality there's no header, the first row will be considered to be one.
#[derive(Debug, Clone, Copy)]
pub struct ByContent<S>(S);

impl<S> ByContent<S> {
    /// Constructs a new object of the structure.
    pub fn new(text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(text)
    }
}

impl<R, S> Locator<R> for ByContent<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + PeekableRecords,
{
    type Coordinate = Position;
    type IntoIter = Vec<Position>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        // todo: can be optimized by creating Iterator
        let text = self.0.as_ref();

        let mut out = vec![];
        for row in 0..records.count_rows() {
            for col in 0..records.count_columns() {
                let cell = records.get_text((row, col));
                if cell.eq(text) {
                    out.push((row, col));
                }
            }
        }

        out
    }
}

impl<S, R> Object<R> for ByContent<S>
where
    S: AsRef<str>,
    R: Records + PeekableRecords + ExactRecords,
{
    type Iter = std::vec::IntoIter<Entity>;

    fn cells(&self, records: &R) -> Self::Iter {
        // todo: can be optimized by creating Iterator
        let text = self.0.as_ref();

        let mut out = vec![];
        for row in 0..records.count_rows() {
            for col in 0..records.count_columns() {
                let cell = records.get_text((row, col));
                if cell.eq(text) {
                    out.push(Entity::Cell(row, col));
                }
            }
        }

        out.into_iter()
    }
}
