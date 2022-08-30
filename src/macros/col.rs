/// Creates a parent [`Table`] with [`std::fmt::Display`] arguments nested within.
///
/// col! allows several tables to be displayed vertically.
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
#[macro_export]
macro_rules! col {
    // Vertical
    ( $($table:expr), * $(,)? ) => {{
        let mut builder = tabled::builder::Builder::default();

        $(
            builder.add_record([$table.to_string()]);
        )*

        builder.build()
    }};

    // Duplicate single item
    ( $table:expr; $N:expr) => {{
        let duplicates = vec![$table.to_string(); $N];

        let mut builder = Table::builder(duplicates);

        builder.remove_columns();

        builder.build()
    }};
}
