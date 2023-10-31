use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, PeekableRecords, Records},
    settings::location::Location,
    settings::object::Object,
};

/// The structure is an implementation of [`Location`] to search for a column by it's name.
/// A name is considered be a value in a first row.
///
/// So even if in reality there's no header, the first row will be considered to be one.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByColumnName<S>(S);

impl<S> ByColumnName<S> {
    /// Constructs a new object of the structure.
    pub fn new(text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(text)
    }
}

impl<R, S> Location<R> for ByColumnName<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + PeekableRecords,
{
    type Coordinate = usize;
    type IntoIter = Vec<usize>;

    fn locate(&mut self, records: &R) -> Self::IntoIter {
        // todo: can be optimized by creating Iterator
        (0..records.count_columns())
            .filter(|col| records.get_text((0, *col)) == self.0.as_ref())
            .collect::<Vec<_>>()
    }
}

impl<S, R> Object<R> for ByColumnName<S>
where
    S: AsRef<str>,
    R: Records + PeekableRecords + ExactRecords,
{
    type Iter = std::vec::IntoIter<Entity>;

    fn cells(&self, records: &R) -> Self::Iter {
        // todo: can be optimized by creating Iterator
        (0..records.count_columns())
            .filter(|col| records.get_text((0, *col)) == self.0.as_ref())
            .map(Entity::Column)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
