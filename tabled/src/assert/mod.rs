//! Assert module contains a help macros to compare and test tables conveniently.

/// Create a test function for table assertion.
///
/// # Example
///
/// ```
/// # fn main() {}
///
/// use tabled::Table;
/// use tabled::assert::test_table;
///
/// test_table!(
///     test_table,
///     Table::new([[1, 2, 3], [4, 5, 6]]),
///     "+---+---+---+"
///     "| 0 | 1 | 2 |"
///     "+---+---+---+"
///     "| 1 | 2 | 3 |"
///     "+---+---+---+"
///     "| 4 | 5 | 6 |"
///     "+---+---+---+"
/// );
/// ```
pub use testing_table::test_table;

/// Assert a table.
///
/// It's an analog of [`assert_eq`] but for tables.
///
/// # Example
///
/// ```
/// use tabled::Table;
/// use tabled::assert::assert_table;
///
/// let data = [[1, 2, 3], [4, 5, 6]];
/// let table = Table::new(data);
///
/// assert_table!(
///     table,
///     "+---+---+---+"
///     "| 0 | 1 | 2 |"
///     "+---+---+---+"
///     "| 1 | 2 | 3 |"
///     "+---+---+---+"
///     "| 4 | 5 | 6 |"
///     "+---+---+---+"
/// );
/// ```
pub use testing_table::assert_table;

/// Assert table width.
///
/// # Example
///
/// ```
/// use tabled::Table;
/// use tabled::assert::assert_width;
///
/// let data = [[1, 2, 3], [4, 5, 6]];
/// let table = Table::new(data);
///
/// assert_width!(table, 13);
/// ```
pub use testing_table::assert_width;

/// Construct a static table.
///
/// Usefull for assert functions.
///
/// # Example
///
/// ```
/// use tabled::Table;
/// use tabled::assert::static_table;
///
/// let data = [[1, 2, 3], [4, 5, 6]];
/// let table = Table::new(data);
///
/// assert_eq!(
///     table.to_string(),
///     static_table!(
///         "+---+---+---+"
///         "| 0 | 1 | 2 |"
///         "+---+---+---+"
///         "| 1 | 2 | 3 |"
///         "+---+---+---+"
///         "| 4 | 5 | 6 |"
///         "+---+---+---+"
///     ),
/// );
/// ```
pub use testing_table::static_table;
