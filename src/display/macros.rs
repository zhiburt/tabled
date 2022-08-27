//! This module contains macro functions for dynamic [`Table`] displays.

/// Creates a parent [`Table`] with [`std::fmt::Display`] arguments nested within.
///
/// group! allows several tables to be displayed in parallel, or as a greater table to maximize data visualization.
///
/// ### Parallel Display
///
/// Easily compare multiple tables by grouping them together.
///
/// ```rust,no_run
/// # use tabled::{group, Table};
/// # let (table_a, table_b) = (Table::new(&[1]), Table::new(&[1]));
/// println!("{}", group!(table_a, table_b));
/// /*
/// +------------------------+------------------------+
/// | .--------------------. | ┌─────────┬──────────┐ |
/// | | name    | based_on | | │ name    │ based_on │ |
/// | | Debian  |          | | ├─────────┼──────────┤ |
/// | | Arch    |          | | │ Debian  │          │ |
/// | | Manjaro | Arch     | | ├─────────┼──────────┤ |
/// | '--------------------' | │ Arch    │          │ |
/// |                        | ├─────────┼──────────┤ |
/// |                        | │ Manjaro │ Arch     │ |
/// |                        | └─────────┴──────────┘ |
/// +------------------------+------------------------+
/// */
/// ```
///
/// ### Table Duplication
///
/// Akin to the `vec![T; N]` pattern for repetition.
///
/// ```rust,no_run
/// # use tabled::{group, Table};
/// # let (table_a) = (Table::new(&[1]));
/// println!("{}", group!(table_a; 3));
/// /*
/// +------------------------+------------------------+------------------------+
/// | .--------------------. | .--------------------. | .--------------------. |
/// | | name    | based_on | | | name    | based_on | | | name    | based_on | |
/// | | Debian  |          | | | Debian  |          | | | Debian  |          | |
/// | | Arch    |          | | | Arch    |          | | | Arch    |          | |
/// | | Manjaro | Arch     | | | Manjaro | Arch     | | | Manjaro | Arch     | |
/// | '--------------------' | '--------------------' | '--------------------' |
/// +------------------------+------------------------+------------------------+
/// */
/// ```
///
/// ### Rows
///
/// - Each rows height is calculated separately.
/// - Each columns width is calculated separately.
/// - Rows are chunked by the defined increment.
/// - Empty cells default to blank.
///
/// ```rust,no_run
/// # use tabled::{group, Table};
/// # let (table_a, table_b, table_c) = (Table::new(&[1]), Table::new(&[1]), Table::new(&[1]));
/// println!("{}", group!(table_a, table_b, table_c; 2));
/// /*
/// +-------------------------------------+------------------------+
/// | .--------------------.              | ┌─────────┬──────────┐ |
/// | | name    | based_on |              | │ name    │ based_on │ |
/// | | Debian  |          |              | ├─────────┼──────────┤ |
/// | | Arch    |          |              | │ Debian  │          │ |
/// | | Manjaro | Arch     |              | ├─────────┼──────────┤ |
/// | '--------------------'              | │ Arch    │          │ |
/// |                                     | ├─────────┼──────────┤ |
/// |                                     | │ Manjaro │ Arch     │ |
/// |                                     | └─────────┴──────────┘ |
/// +-------------------------------------+------------------------+
/// | | name                 | based_on | |                        |
/// | |----------------------|----------| |                        |
/// | | Super Long Name Here |          | |                        |
/// | | Arch                 |          | |                        |
/// | | Manjaro              | Arch     | |                        |
/// |                                     |                        |
/// |                                     |                        |
/// |                                     |                        |
/// |                                     |                        |
/// +-------------------------------------+------------------------+
/// */
/// ```
#[macro_export]
macro_rules! group {
    // Parallel Display
    ( $($table:expr), * ) => {{
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

    // Divide items into rows
    ( $($table:expr), *; $N:expr) => {{
        let tables = &[$($table.to_string(),)*];
        let groups = tables
            .chunks($N)
            .map(|chunk| {
                chunk.iter()
                    .map(ToString::to_string)
                    .chain(std::iter::repeat(String::default()))
                    .take($N)
            });

        // todo: (yet not merged) builder.hint_size($N);
        let mut builder = tabled::builder::Builder::default();

        for group in groups {
            builder.add_record(group);
        }

        builder.build()
    }};
}
