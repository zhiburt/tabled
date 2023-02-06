// todo: remove it. I believe it's unnessary and not usefull.

use std::borrow::Cow;

use crate::records::IterRecords;

use crate::{Table, Tabled};

/// A trait for [`IntoIterator`] whose Item type is bound to [`Tabled`].
/// Any type implements [`IntoIterator`] can call this function directly
///
/// ```rust
/// use tabled::{TableIteratorExt, Style};
///
/// let strings: &[&str] = &["Hello", "World"];
///
/// let mut table = strings.table();
/// table.with(Style::psql());
///
/// println!("{}", table);
/// ```
pub trait TableIteratorExt {
    /// Returns a [`Table`] instance from a given type
    fn table(self) -> Table;
}

impl<I, T> TableIteratorExt for I
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    fn table(self) -> Table {
        Table::new(self)
    }
}
