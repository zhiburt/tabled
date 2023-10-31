use crate::{
    grid::config::Entity,
    grid::{
        config::Position,
        records::{ExactRecords, PeekableRecords, Records},
    },
    settings::location::Location,
    settings::object::Object,
};

/// The structure is an implementation of [`Location`] to search for cells with a specified condition.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByCondition<F>(F);

impl<F> ByCondition<F> {
    /// Constructs a new object of the structure.
    pub fn new(search: F) -> Self
    where
        F: Fn(&str) -> bool,
    {
        Self(search)
    }
}

impl<F, R> Location<R> for ByCondition<F>
where
    F: Fn(&str) -> bool,
    R: Records + ExactRecords + PeekableRecords,
{
    type Coordinate = Position;
    type IntoIter = Vec<Position>;

    fn locate(&mut self, records: &R) -> Self::IntoIter {
        // todo: can be optimized by creating Iterator
        let cond = &self.0;

        let mut out = vec![];
        for row in 0..records.count_rows() {
            for col in 0..records.count_columns() {
                let text = records.get_text((row, col));
                if cond(text) {
                    out.push((row, col));
                }
            }
        }

        out
    }
}

impl<F, R> Object<R> for ByCondition<F>
where
    F: Fn(&str) -> bool,
    R: Records + ExactRecords + PeekableRecords,
{
    type Iter = std::vec::IntoIter<Entity>;

    fn cells(&self, records: &R) -> Self::Iter {
        // todo: can be optimized by creating Iterator
        let cond = &self.0;

        let mut out = vec![];
        for row in 0..records.count_rows() {
            for col in 0..records.count_columns() {
                let text = records.get_text((row, col));
                if cond(text) {
                    out.push(Entity::Cell(row, col));
                }
            }
        }

        out.into_iter()
    }
}
