/// Build a static table.
///
/// # Example
///
/// ```text
/// static_table!(
///     "|--|--|"
///     "|XX|XY|"
///     "|--|--|"
/// )
/// ```
#[macro_export]
macro_rules! static_table {
    ($($line:expr)*) => {
        concat!(
            $($line, "\n",)*
        )
        .trim_end_matches('\n')
    };
}

/// Create a test for a given table.
///
/// # Example
///
/// ```text
/// test_table!(
///     test_name,
///     Table::new([[1, 2, 3], [4, 5, 6]]),
///     "|--|--|"
///     "|XX|XY|"
///     "|--|--|"
/// )
/// ```
#[macro_export]
macro_rules! test_table {
    ($test:ident, $table:expr, $($line:expr)*) => {
        #[test]
        fn $test() {
            $crate::assert_table!($table, $($line)*);
        }
    };
    ($test:ident, $table:expr, $expected:expr,) => {
        #[test]
        fn $test() {
            let table = $table.to_string();
            let expected = $expected.to_string();
            assert_eq!(table, expected);
        }
    };
}

/// Assert a given table.
///
/// # Example
///
/// ```text
/// assert_table!(
///     Table::new([[1, 2, 3], [4, 5, 6]]),
///     "|--|--|"
///     "|XX|XY|"
///     "|--|--|"
/// )
/// ```
#[macro_export]
macro_rules! assert_table {
    ($table:expr, $($line:expr)*) => {
        let table = $table.to_string();
        assert_eq!(table, $crate::static_table!($($line)*));
    };
}

/// Assert a given table width.
///
/// # Example
///
/// ```text
/// assert_width!(Table::new([[1, 2, 3], [4, 5, 6]]), 10);
/// ```
#[macro_export]
macro_rules! assert_width {
    ($table:expr, $expected:expr) => {
        let expected = $expected;
        let table = $table.to_string();
        let width = $crate::get_text_width(&table);
        assert_eq!(width, expected);
    };
}
