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

#[cfg(feature = "std")]
macro_rules! tuple_trait_impl {
    ( $($name:ident)+ ) => {
        impl<R, C, D, $($name: TableOption<R, C, D>),+> TableOption<R, C, D> for ($($name,)+) {
            fn change(self, records: &mut R, cfg: &mut C, dimension: &mut D) {
                #![allow(non_snake_case)]
                let ($($name,)+) = self;
                $(
                    $name::change($name, records, cfg, dimension);
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

                hint_change_list(&list)
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

#[cfg(feature = "std")]
pub(crate) fn hint_change_list(list: &[Option<Entity>]) -> Option<Entity> {
    let mut entries = vec![];
    for e in list.iter().flatten() {
        entries.push(*e);
    }

    if entries.is_empty() {
        return None;
    }

    Some(combine_entity_list(&entries))
}

#[cfg(feature = "std")]
pub(crate) fn combine_entity_list(list: &[Entity]) -> Entity {
    if list.is_empty() {
        // must never happen
        return Entity::Global;
    }

    let mut entity = list[0];
    for e in &list[1..] {
        entity = crate::settings::settings_list::combine_entity(entity, *e);
    }

    entity
}
