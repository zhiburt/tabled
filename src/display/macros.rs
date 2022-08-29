//! This module contains macro functions for dynamic [`Table`] displays.

/// Creates a parent [`Table`] with [`std::fmt::Display`] arguments nested within.
///
/// row! allows several tables to be displayed horizontally.
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
#[macro_export]
macro_rules! row {
    // Horizontal Display
    ( $($table:expr), * $(,)? ) => {{
        let mut builder = Table::builder([( $($table.to_string(),)*) ]);
        builder.remove_columns();
        builder.build()
    }};

    // Duplicate single item
    ( $table:expr; $N:expr) => {{
        let duplicates = vec![$table.to_string(); $N];

        let mut builder = tabled::builder::Builder::default();

        builder.add_record(duplicates);

        builder.build()
    }};
}

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
        let mut builder = Table::builder([ $($table.to_string(),)* ]);
        builder.remove_columns();
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
