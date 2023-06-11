
#[macro_export]
macro_rules! static_table {
    ($($line:expr)*) => {
        concat!(
            $($line, "\n",)*
        )
        .trim_end_matches('\n')
    };
}

#[macro_export]
macro_rules! test_table {
    ($test:ident, $table:expr, $($line:expr)*) => {
        #[test]
        fn $test() {
            $crate::assert_table!($table, $($line)*)
        }
    };
}

#[macro_export]
macro_rules! assert_table {
    ($table:expr, $($line:expr)*) => {
        let table = $table.to_string();
        println!("{}", table);
        assert_eq!(table, $crate::static_table!($($line)*));
    };
}
