/// Creates a [`Table`] with [`Display`] arguments nested within.
///
/// The macros allows several tables to be displayed vertically.
///
/// Companion to [`row!`].
///
/// # Examples
/// ```rust,no_run
/// # use tabled::{row, col, Table};
/// # let (table1, table2, table3) = (Table::new(&[String::new()]), Table::new(&[String::new()]), Table::new(&[String::new()]));
/// let new_table = col![table1, table2];
/// let new_table_of_clones = col![table1; 3];
/// let columns_and_rows = col![
///     table1,
///     row![table2, table3]
/// ];
/// ```
///
/// [`row!`]: crate::row
/// [`Table`]: crate::Table
/// [`Display`]: std::fmt::Display
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! col {
    // Vertical
    ( $($table:expr), * $(,)? ) => {{
        let mut builder = $crate::builder::Builder::default();

        $(
            builder.push_record([$table.to_string()]);
        )*

        builder.build()
    }};

    // Duplicate single item
    ( $table:expr; $N:expr) => {{
        let mut builder = $crate::builder::Builder::default();

        let n = $N;
        if n > 0 {
            let t = $table.to_string();
            for _ in 0..$N {
                builder.push_record([t.clone()]);
            }
        }

        builder.build()
    }};
}
