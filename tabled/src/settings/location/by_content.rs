use crate::{
    grid::config::Entity,
    grid::{
        config::Position,
        records::{ExactRecords, PeekableRecords, Records},
    },
    settings::location::Location,
    settings::object::Object,
};

/// The structure is an implementation of [`Location`] to search for cells with a given content.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

impl<R, S> Location<R> for ByContent<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + PeekableRecords,
{
    type Coordinate = Position;
    type IntoIter = Vec<Position>;

    fn locate(&mut self, records: &R) -> Self::IntoIter {
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
