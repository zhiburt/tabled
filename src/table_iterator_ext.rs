use papergrid::records::{cell_info::CellInfo, vec_records::VecRecords};

use crate::{Table, Tabled};

/// A trait for [`IntoIterator`] whose Item type is bound to [`Tabled`].
/// Any type implements [`IntoIterator`] can call this function directly
///
/// ```rust
/// use tabled::{TableIteratorExt, Style};
///
/// let strings: &[&str] = &["Hello", "World"];
///
/// let table = strings.table().with(Style::psql());
///
/// println!("{}", table);
/// ```
pub trait TableIteratorExt {
    /// A underline [`Records`],
    type Records;

    /// Returns a [`Table`] instance from a given type
    fn table(self) -> Table<Self::Records>;
}

impl<I, T> TableIteratorExt for I
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    type Records = VecRecords<CellInfo<'static>>;

    fn table(self) -> Table<Self::Records> {
        Table::new(self)
    }
}
