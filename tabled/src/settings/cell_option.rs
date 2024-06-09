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
    /// Modification function of a certail part of a grid targeted by [`Entity`].
    fn change(self, records: &mut R, cfg: &mut C, entity: Entity);

    /// A hint whether an [`TableOption`] is going to change table layout.
    ///
    /// Return [`None`] if no changes are being done.
    /// Otherwise return:
    ///
    /// - [Entity::Global] - a grand layout changed.
    /// - [Entity::Row] - a certain row was changed.
    /// - [Entity::Column] - a certain column was changed.
    /// - [Entity::Cell] - a certain cell was changed.
    ///
    /// By default it's considered to be a grand change.
    ///
    /// This methods primarily is used as an optimization,
    /// to not make unnecessary calculations if they're not needed, after using the [`TableOption`].
    ///
    /// [`TableOption`]: crate::settings::TableOption
    fn hint_change(&self) -> Option<Entity> {
        Some(Entity::Global)
    }
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
