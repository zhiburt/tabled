use crate::grid::config::Entity;

/// A trait which is responsible for configuration of a [`Table`].
///
/// [`Table`]: crate::Table
pub trait TableOption<R, C, D> {
    /// The function modificaties of records and a grid configuration.
    fn change(self, records: &mut R, cfg: &mut C, dimension: &mut D);

    /// A hint whether an [`TableOption`] is going to change table layout.
    ///
    /// Return [`None`] if no changes are being done.
    /// Otherwise return:
    ///
    /// - [Entity::Global] - a grand layout changed. (a change which MIGHT require height/width update)
    /// - [Entity::Row] - a certain row was changed. (a change which MIGHT require height update)
    /// - [Entity::Column] - a certain column was changed. (a change which MIGHT require width update)
    /// - [Entity::Cell] - a certain cell was changed. (a local change, no width/height update)
    ///
    /// By default it's considered to be a grand change.
    ///
    /// This methods primarily is used as an optimization,
    /// to not make unnecessary calculations if they're not needed, after using the [`TableOption`].
    fn hint_change(&self) -> Option<Entity> {
        Some(Entity::Global)
    }
}

// todo: probably we could add one more hint but it likely require Vec<Entity>,
// so, as I am not sure about exact interface it's better be commented.
// /// A hint which layout part a [`TableOption`] is going to change.
// ///
// /// Return [`None`] if no part are being changed.
// /// Otherwise return:
// ///
// /// - [Entity::Global] - a total layout affection.
// /// - [Entity::Row] - a certain row affection.
// /// - [Entity::Column] - a certain column affection.
// /// - [Entity::Cell] - a certain cell affection.
// ///
// /// By default it's considered to be a grand change.
// ///
// /// This methods primarily is used as an optimization,
// /// to not make unnecessary calculations if they're not needed, after using the [`TableOption`].
// fn hint_target(&self, records: &R) -> Option<Vec<Entity>> {
//     let _ = records;
//     Some(vec![Entity::Global])
// }

impl<T, R, C, D> TableOption<R, C, D> for &[T]
where
    for<'a> &'a T: TableOption<R, C, D>,
{
    fn change(self, records: &mut R, cfg: &mut C, dimension: &mut D) {
        for opt in self {
            opt.change(records, cfg, dimension)
        }
    }
}

#[cfg(feature = "std")]
impl<T, R, D, C> TableOption<R, C, D> for Vec<T>
where
    T: TableOption<R, C, D>,
{
    fn change(self, records: &mut R, cfg: &mut C, dimension: &mut D) {
        for opt in self {
            opt.change(records, cfg, dimension)
        }
    }
}
