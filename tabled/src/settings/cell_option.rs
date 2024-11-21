use crate::grid::{
    config::Entity,
    records::{ExactRecords, Records, RecordsMut},
};

/// A trait for configuring a single cell.
///
/// A cell is represented by row and column indexes.
///
/// A cell can be targeted by [`Cell`].
///
/// [`Cell`]: crate::settings::object::Cell
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

#[cfg(feature = "std")]
macro_rules! tuple_trait_impl {
    ( $($name:ident)+ ) => {
        impl<R, C, $($name: CellOption<R, C>),+> CellOption<R, C> for ($($name,)+) {
            fn change(self, records: &mut R, cfg: &mut C, entity: Entity) {
                #![allow(non_snake_case)]
                let ($($name,)+) = self;
                $(
                    $name::change($name, records, cfg, entity);
                )+
            }

            fn hint_change(&self) -> Option<Entity> {
                #![allow(non_snake_case)]
                let ($($name,)+) = &self;
                let list = [
                    $(
                        $name::hint_change($name),
                    )+
                ];

                crate::settings::table_option::hint_change_list(&list)
            }
        }
    };
}

#[cfg(feature = "std")]
tuple_trait_impl!(T0 T1);
#[cfg(feature = "std")]
tuple_trait_impl!(T0 T1 T2);
#[cfg(feature = "std")]
tuple_trait_impl!(T0 T1 T2 T3);
#[cfg(feature = "std")]
tuple_trait_impl!(T0 T1 T2 T3 T4);
#[cfg(feature = "std")]
tuple_trait_impl!(T0 T1 T2 T3 T4 T5);
#[cfg(feature = "std")]
tuple_trait_impl!(T0 T1 T2 T3 T4 T5 T6);
#[cfg(feature = "std")]
tuple_trait_impl!(T0 T1 T2 T3 T4 T5 T6 T7);
#[cfg(feature = "std")]
tuple_trait_impl!(T0 T1 T2 T3 T4 T5 T6 T7 T8);
#[cfg(feature = "std")]
tuple_trait_impl!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9);
