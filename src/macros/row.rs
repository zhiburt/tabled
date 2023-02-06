/// Creates a [`Table`] with [`Display`] arguments nested within.
///
/// The macros allows several tables to be displayed horizontally.
///
/// Companion to [`col!`].
///
/// # Examples
/// ```rust,no_run
/// # use tabled::{row, col, Table};
/// # let (table1, table2, table3) = (Table::new(&[String::new()]), Table::new(&[String::new()]), Table::new(&[String::new()]));
/// let new_table = row![table1, table2];
/// let new_table_of_clones = row![table1; 3];
/// let rows_and_columns = row![
///     table1,
///     col![table2, table3]
/// ];
/// ```
///
/// [`col!`]: crate::col
/// [`Table`]: crate::Table
/// [`Display`]: std::fmt::Display
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! row {
    // Horizontal Display
    ( $($table:expr), * $(,)? ) => {{
        let mut builder = $crate::builder::Builder::default();

        let record = [ $($table.to_string(),)* ];
        builder.push_record(record);

        builder.build()
    }};

    // Duplicate single item
    ( $table:expr; $N:expr) => {{
        let mut builder = $crate::builder::Builder::default();

        let duplicates = vec![$table.to_string(); $N];
        builder.push_record(duplicates);

        builder.build()
    }};
}
