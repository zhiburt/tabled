#![cfg(feature = "std")]

#[cfg(feature = "color")]
use owo_colors::{AnsiColors, OwoColorize};

use tabled::{tables::ExtendedTable, Tabled};

use crate::matrix::Matrix;
use testing_table::{static_table, test_table};

macro_rules! assert_expanded_display {
    ( $data:expr, $expected:expr ) => {
        let table = ExtendedTable::new($data).to_string();
        assert_eq!(table, $expected);
    };
}

macro_rules! build_tabled_type {
    ( $name:ident, $length:expr, $fields:expr, $headers:expr ) => {
        #[derive(Debug, Clone, Copy)]
        struct $name;

        impl Tabled for $name {
            const LENGTH: usize = $length;

            fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
                $fields.iter().map(|s| s.to_string().into()).collect()
            }

            fn headers() -> Vec<std::borrow::Cow<'static, str>> {
                $headers.iter().map(|s| s.to_string().into()).collect()
            }
        }
    };
}

test_table!(
    display,
    ExtendedTable::from(Matrix::vec(3, 3)),
    "-[ RECORD 0 ]-"
    "N        | 0"
    "column 0 | 0-0"
    "column 1 | 0-1"
    "column 2 | 0-2"
    "-[ RECORD 1 ]-"
    "N        | 1"
    "column 0 | 1-0"
    "column 1 | 1-1"
    "column 2 | 1-2"
    "-[ RECORD 2 ]-"
    "N        | 2"
    "column 0 | 2-0"
    "column 1 | 2-1"
    "column 2 | 2-2"
);

#[test]
fn display_empty_records() {
    build_tabled_type!(TestType, 3, ["He", "123", "asd"], ["1", "2", "3"]);
    let data: Vec<TestType> = vec![];
    assert_expanded_display!(data, "");
}

#[test]
fn display_empty() {
    build_tabled_type!(
        TestType,
        3,
        {
            let d: Vec<String> = vec![];
            d
        },
        {
            let d: Vec<String> = vec![];
            d
        }
    );
    let data: Vec<TestType> = vec![];
    assert_expanded_display!(data, "");
}

#[test]
fn display_empty_2() {
    build_tabled_type!(EmptyType, 0, [""; 0], [""; 0]);
    assert_expanded_display!(&[EmptyType], "-[ RECORD 0 ]-");
}

#[test]
fn display_dynamic_header_template() {
    {
        build_tabled_type!(TestType, 3, ["He", "123", "asd"], ["1", "2", "3"]);
        assert_expanded_display!(
            &[TestType],
            static_table!(
                "-[ RECORD 0 ]-"
                "1 | He"
                "2 | 123"
                "3 | asd"
            )
        );
    }
    {
        build_tabled_type!(TestType, 3, ["He", "123", "asd"], ["11", "2222222", "3"]);
        assert_expanded_display!(
            &[TestType],
            static_table!(
                "-[ RECORD 0 ]-"
                "11      | He"
                "2222222 | 123"
                "3       | asd"
            )
        );
    }
    {
        build_tabled_type!(
            TestType,
            3,
            ["HeheHehe", "123", "asd"],
            ["11", "2222222", "3"]
        );
        assert_expanded_display!(
            &[TestType],
            static_table!(
                "-[ RECORD 0 ]-----"
                "11      | HeheHehe"
                "2222222 | 123"
                "3       | asd"
            )
        );
    }
    {
        build_tabled_type!(TestType, 3, ["He", "123", "asd"], ["11111111111", "2", "3"]);
        assert_expanded_display!(
            &[TestType],
            static_table!(
                "-[ RECORD 0 ]----"
                "11111111111 | He"
                "2           | 123"
                "3           | asd"
            )
        );
    }
    {
        build_tabled_type!(
            TestType,
            3,
            ["He", "123", "asd"],
            ["1111111111111", "2", "3"]
        );
        assert_expanded_display!(
            &[TestType],
            static_table!(
                "-[ RECORD 0 ]-+----"
                "1111111111111 | He"
                "2             | 123"
                "3             | asd"
            )
        );
    }
    {
        build_tabled_type!(
            TestType,
            3,
            ["He", "123", "asd"],
            ["11111111111111111111111111111", "2", "3"]
        );
        assert_expanded_display!(
            &[TestType],
            static_table!(
                "-[ RECORD 0 ]-----------------+----"
                "11111111111111111111111111111 | He"
                "2                             | 123"
                "3                             | asd"
            )
        );
    }
    {
        build_tabled_type!(TestType, 3, ["22"], ["11111111111"]);
        assert_expanded_display!(
            std::iter::repeat(TestType).take(11),
            static_table!(
                "-[ RECORD 0 ]---"
                "11111111111 | 22"
                "-[ RECORD 1 ]---"
                "11111111111 | 22"
                "-[ RECORD 2 ]---"
                "11111111111 | 22"
                "-[ RECORD 3 ]---"
                "11111111111 | 22"
                "-[ RECORD 4 ]---"
                "11111111111 | 22"
                "-[ RECORD 5 ]---"
                "11111111111 | 22"
                "-[ RECORD 6 ]---"
                "11111111111 | 22"
                "-[ RECORD 7 ]---"
                "11111111111 | 22"
                "-[ RECORD 8 ]---"
                "11111111111 | 22"
                "-[ RECORD 9 ]---"
                "11111111111 | 22"
                "-[ RECORD 10 ]--"
                "11111111111 | 22"
            )
        );
    }
}

#[test]
fn display_multiline_field() {
    build_tabled_type!(TestType, 3, ["1", "2", "3"], ["Hello\nWorld", "123", "asd"]);
    assert_expanded_display!(
        [TestType],
        static_table!(
            "-[ RECORD 0 ]---"
            "Hello\\nWorld | 1"
            "123          | 2"
            "asd          | 3"
        )
    );
}

#[test]
fn display_multiline_record_value() {
    let mut data = Matrix::list::<2, 3>();
    data[0][0] = "Hello\nWorld".to_string();
    data[0][1] = "123".to_string();
    data[0][2] = "asd".to_string();

    assert_expanded_display!(
        data,
        static_table!(
            "-[ RECORD 0 ]----------"
            "N        | Hello\\nWorld"
            "column 0 | 123"
            "column 1 | asd"
            "column 2 | 0-2"
            "-[ RECORD 1 ]----------"
            "N        | 1"
            "column 0 | 1-0"
            "column 1 | 1-1"
            "column 2 | 1-2"
        )
    );
}

test_table!(
    display_with_truncate,
    {
        let data = Matrix::new(3, 3).insert((1, 0), "a long string").to_vec();
        let mut table = ExtendedTable::from(data);
        table.truncate(14, "");
        table.to_string()
    },
    "-[ RECORD 0 ]-"
    "N        | a l"
    "column 0 | 0-0"
    "column 1 | 0-1"
    "column 2 | 0-2"
    "-[ RECORD 1 ]-"
    "N        | 1"
    "column 0 | 1-0"
    "column 1 | 1-1"
    "column 2 | 1-2"
    "-[ RECORD 2 ]-"
    "N        | 2"
    "column 0 | 2-0"
    "column 1 | 2-1"
    "column 2 | 2-2"
);

test_table!(
    truncate_with_suffix,
    {
        let data = Matrix::new(3, 3).insert((1, 0), "a long string").to_vec();
        let mut table = ExtendedTable::from(data);
        table.truncate(15, "..");
        table.to_string()
    },
    "-[ RECORD 0 ]-"
    "N        | .."
    "column 0 | .."
    "column 1 | .."
    "column 2 | .."
    "-[ RECORD 1 ]-"
    "N        | .."
    "column 0 | .."
    "column 1 | .."
    "column 2 | .."
    "-[ RECORD 2 ]-"
    "N        | .."
    "column 0 | .."
    "column 1 | .."
    "column 2 | .."
);

#[test]
fn truncate_big_fields() {
    build_tabled_type!(
        TestType,
        3,
        ["1", "2", "3"],
        ["A quite big field", "123", "asd"]
    );
    let data: Vec<TestType> = vec![TestType, TestType];

    let mut table = ExtendedTable::new(&data);
    table.truncate(14, "..");
    let table = table.to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]-"
            "A quite.. | .."
            "123       | .."
            "asd       | .."
            "-[ RECORD 1 ]-"
            "A quite.. | .."
            "123       | .."
            "asd       | .."
        )
    );

    let mut table = ExtendedTable::new(&data);
    table.truncate(15, "..");
    let table = table.to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]--"
            "A quite .. | .."
            "123        | .."
            "asd        | .."
            "-[ RECORD 1 ]--"
            "A quite .. | .."
            "123        | .."
            "asd        | .."
        )
    );

    let mut table = ExtendedTable::new(&data);
    table.truncate(0, "..");
    let table = table.to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]-----+--"
            "A quite big field | 1"
            "123               | 2"
            "asd               | 3"
            "-[ RECORD 1 ]-----+--"
            "A quite big field | 1"
            "123               | 2"
            "asd               | 3"
        )
    );

    let mut table = ExtendedTable::new(&data);
    table.truncate(20, "......");
    let table = table.to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]-------"
            "A qui...... | ......"
            "123         | ......"
            "asd         | ......"
            "-[ RECORD 1 ]-------"
            "A qui...... | ......"
            "123         | ......"
            "asd         | ......"
        )
    );
}

test_table!(
    truncate_too_small,
    {
        let data = Matrix::new(3, 3).insert((1, 0), "a long string").to_vec();
        let mut table = ExtendedTable::from(data);
        let success = table.truncate(2, "");
        assert!(!success);
        table
    },
    "-[ RECORD 0 ]-----------"
    "N        | a long string"
    "column 0 | 0-0"
    "column 1 | 0-1"
    "column 2 | 0-2"
    "-[ RECORD 1 ]-----------"
    "N        | 1"
    "column 0 | 1-0"
    "column 1 | 1-1"
    "column 2 | 1-2"
    "-[ RECORD 2 ]-----------"
    "N        | 2"
    "column 0 | 2-0"
    "column 1 | 2-1"
    "column 2 | 2-2"
);

#[cfg(feature = "color")]
#[test]
fn display_colored() {
    let mut data = Matrix::list::<3, 3>();
    data[0][2] = "https://getfedora.org/"
        .red()
        .on_color(AnsiColors::Blue)
        .to_string();
    data[1][2] = "https://www.opensuse.org/"
        .green()
        .on_color(AnsiColors::Black)
        .to_string();
    data[2][2] = "https://endeavouros.com/".blue().underline().to_string();

    assert_expanded_display!(
        data,
        static_table!(
            "-[ RECORD 0 ]------------------------------------------------------------"
            "N        | 0"
            "column 0 | 0-0"
            "column 1 | \\u{1b}[31;44mhttps://getfedora.org/\\u{1b}[0m"
            "column 2 | 0-2"
            "-[ RECORD 1 ]------------------------------------------------------------"
            "N        | 1"
            "column 0 | 1-0"
            "column 1 | \\u{1b}[32;40mhttps://www.opensuse.org/\\u{1b}[0m"
            "column 2 | 1-2"
            "-[ RECORD 2 ]------------------------------------------------------------"
            "N        | 2"
            "column 0 | 2-0"
            "column 1 | \\u{1b}[4m\\u{1b}[34mhttps://endeavouros.com/\\u{1b}[39m\\u{1b}[0m"
            "column 2 | 2-2"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn display_with_truncate_colored() {
    let mut data = Matrix::list::<2, 3>();
    data[0][2] = "https://getfedora.org/".red().to_string();
    data[1][1] = "https://endeavouros.com/"
        .white()
        .on_color(AnsiColors::Black)
        .to_string();
    data[1][2] = "https://www.opensuse.org/".to_string();

    let mut table = ExtendedTable::new(&data);
    table.truncate(20, "");
    let table = table.to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]-------"
            "N        | 0"
            "column 0 | 0-0"
            "column 1 | \\u{1b}[31"
            "column 2 | 0-2"
            "-[ RECORD 1 ]-------"
            "N        | 1"
            "column 0 | \\u{1b}[37"
            "column 1 | https://w"
            "column 2 | 1-2"
        )
    );
}
