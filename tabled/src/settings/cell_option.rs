use crate::grid::{
    config::Entity,
    records::{ExactRecords, Records, RecordsMut},
};

/// A trait for configuring a single cell.
///
/// ~~~~ Where cell represented by 'row' and 'column' indexes. ~~~~
///
/// A cell can be targeted by [`Cell`].
///
/// [`Cell`]: crate::object::Cell
pub trait CellOption<R, C> {
    /// Modification function of a single cell.
    fn change(self, records: &mut R, cfg: &mut C, entity: Entity);
}

#[cfg(feature = "std")]
impl<R, C> CellOption<R, C> for String
where
    R: Records + ExactRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, cfg: &mut C, entity: Entity) {
        (&self).change(records, cfg, entity);
    }
}

#[cfg(feature = "std")]
impl<R, C> CellOption<R, C> for &String
where
    R: Records + ExactRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, _: &mut C, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for pos in entity.iter(count_rows, count_cols) {
            records.set(pos, self.clone());
        }
    }
}

impl<'a, R, C> CellOption<R, C> for &'a str
where
    R: Records + ExactRecords + RecordsMut<&'a str>,
{
    fn change(self, records: &mut R, _: &mut C, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for pos in entity.iter(count_rows, count_cols) {
            records.set(pos, self);
        }
    }
}
