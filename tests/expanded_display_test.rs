use crate::util::{create_vector, static_table};
use tabled::{display::ExpandedDisplay, Tabled};

#[cfg(feature = "color")]
use owo_colors::{AnsiColors, OwoColorize};

mod util;

macro_rules! assert_expanded_display {
    ( $data:expr, $expected:expr ) => {
        let table = ExpandedDisplay::new($data).to_string();
        assert_eq!(table, $expected);
    };
}

macro_rules! build_tabled_type {
    ( $name:ident, $length:expr, $fields:expr, $headers:expr ) => {
        #[derive(Debug, Clone, Copy)]
        struct $name;

        impl Tabled for $name {
            const LENGTH: usize = $length;

            fn fields(&self) -> Vec<String> {
                $fields.iter().map(|s| s.to_string()).collect()
            }

            fn headers() -> Vec<String> {
                $headers.iter().map(|s| s.to_string()).collect()
            }
        }
    };
}

#[test]
fn display() {
    assert_expanded_display!(
        create_vector::<3, 3>(),
        static_table!(
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
        )
    );
}

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

#[cfg(feature = "color")]
#[test]
fn display_colored() {
    let mut data = create_vector::<3, 3>();
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
            "-[ RECORD 0 ]-----------------------"
            "N        | 0"
            "column 0 | 0-0"
            "column 1 | \u{1b}[31;44mhttps://getfedora.org/\u{1b}[0m"
            "column 2 | 0-2"
            "-[ RECORD 1 ]-----------------------"
            "N        | 1"
            "column 0 | 1-0"
            "column 1 | \u{1b}[32;40mhttps://www.opensuse.org/\u{1b}[0m"
            "column 2 | 1-2"
            "-[ RECORD 2 ]-----------------------"
            "N        | 2"
            "column 0 | 2-0"
            "column 1 | \u{1b}[4m\u{1b}[34mhttps://endeavouros.com/\u{1b}[39m\u{1b}[0m"
            "column 2 | 2-2"
        )
    );
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
    let mut data = create_vector::<2, 3>();
    data[0][0] = "Hello\nWorld".to_string();
    data[0][1] = "123".to_string();
    data[0][2] = "asd".to_string();

    assert_expanded_display!(
        data,
        static_table!(
            "-[ RECORD 0 ]---"
            "N        | Hello"
            "         | World"
            "column 0 | 123"
            "column 1 | asd"
            "column 2 | 0-2"
            "-[ RECORD 1 ]---"
            "N        | 1"
            "column 0 | 1-0"
            "column 1 | 1-1"
            "column 2 | 1-2"
        )
    );
}

#[test]
fn display_with_header_template() {
    let data = create_vector::<2, 3>();
    let table = ExpandedDisplay::new(&data)
        .header_template(|i| format!("=== Record => {}", i))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "=== Record => 0"
            "N        | 0"
            "column 0 | 0-0"
            "column 1 | 0-1"
            "column 2 | 0-2"
            "=== Record => 1"
            "N        | 1"
            "column 0 | 1-0"
            "column 1 | 1-1"
            "column 2 | 1-2"
        )
    );
}

#[test]
fn display_with_formatter() {
    let mut data = create_vector::<2, 3>();
    data[0][1] = "123\n456".to_owned();

    let table = ExpandedDisplay::new(&data)
        .formatter(|s| format!("{}!\n\n", s))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]--"
            "N        | 0!"
            "         | "
            "column 0 | 123"
            "         | 456!"
            "         | "
            "column 1 | 0-1!"
            "         | "
            "column 2 | 0-2!"
            "         | "
            "-[ RECORD 1 ]--"
            "N        | 1!"
            "         | "
            "column 0 | 1-0!"
            "         | "
            "column 1 | 1-1!"
            "         | "
            "column 2 | 1-2!"
            "         | "
        )
    );
}

#[test]
fn display_with_one_line_formatter() {
    let mut data = create_vector::<1, 3>();
    data[0][0] = "Hello\nWorld".to_string();
    data[0][1] = "123".to_string();
    data[0][2] = "asd".to_string();

    let table = ExpandedDisplay::new(&data)
        .formatter(|s| s.escape_debug().to_string())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]----------"
            "N        | Hello\\nWorld"
            "column 0 | 123"
            "column 1 | asd"
            "column 2 | 0-2"
        )
    );
}

#[test]
fn display_with_truncate() {
    let data = create_vector::<3, 3>();
    let table = ExpandedDisplay::new(&data).truncate(2, "").to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]-"
            "N        | 0"
            "column 0 | 0-"
            "column 1 | 0-"
            "column 2 | 0-"
            "-[ RECORD 1 ]-"
            "N        | 1"
            "column 0 | 1-"
            "column 1 | 1-"
            "column 2 | 1-"
            "-[ RECORD 2 ]-"
            "N        | 2"
            "column 0 | 2-"
            "column 1 | 2-"
            "column 2 | 2-"
        )
    );
}

#[test]
fn display_with_truncate_with_tail() {
    let data = create_vector::<2, 3>();
    let table = ExpandedDisplay::new(&data).truncate(2, "...").to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]---"
            "N        | 0"
            "column 0 | 0-..."
            "column 1 | 0-..."
            "column 2 | 0-..."
            "-[ RECORD 1 ]---"
            "N        | 1"
            "column 0 | 1-..."
            "column 1 | 1-..."
            "column 2 | 1-..."
        )
    );
}

#[test]
fn display_with_wrap() {
    let data = create_vector::<2, 3>();
    let table = ExpandedDisplay::new(&data).wrap(1).to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]-"
            "N        | 0"
            "column 0 | 0"
            "         | -"
            "         | 0"
            "column 1 | 0"
            "         | -"
            "         | 1"
            "column 2 | 0"
            "         | -"
            "         | 2"
            "-[ RECORD 1 ]-"
            "N        | 1"
            "column 0 | 1"
            "         | -"
            "         | 0"
            "column 1 | 1"
            "         | -"
            "         | 1"
            "column 2 | 1"
            "         | -"
            "         | 2"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn display_with_wrap_colored() {
    let mut data = create_vector::<2, 3>();
    data[0][2] = "https://getfedora.org/".red().to_string();
    data[1][1] = "https://endeavouros.com/"
        .white()
        .on_color(AnsiColors::Black)
        .to_string();
    data[1][2] = "https://www.opensuse.org/".to_string();

    let table = ExpandedDisplay::new(&data).wrap(2).to_string();

    assert_eq!(
        table,
        static_table!(
            "-[ RECORD 0 ]-"
            "N        | 0"
            "column 0 | 0-"
            "         | 0"
            "column 1 | \u{1b}[31mht\u{1b}[39m"
            "         | \u{1b}[31mtp\u{1b}[39m"
            "         | \u{1b}[31ms:\u{1b}[39m"
            "         | \u{1b}[31m//\u{1b}[39m"
            "         | \u{1b}[31mge\u{1b}[39m"
            "         | \u{1b}[31mtf\u{1b}[39m"
            "         | \u{1b}[31med\u{1b}[39m"
            "         | \u{1b}[31mor\u{1b}[39m"
            "         | \u{1b}[31ma.\u{1b}[39m"
            "         | \u{1b}[31mor\u{1b}[39m"
            "         | \u{1b}[31mg/\u{1b}[39m"
            "column 2 | 0-"
            "         | 2"
            "-[ RECORD 1 ]-"
            "N        | 1"
            "column 0 | \u{1b}[37m\u{1b}[40mht\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40mtp\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40ms:\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40m//\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40men\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40mde\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40mav\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40mou\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40mro\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40ms.\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40mco\u{1b}[39m\u{1b}[49m"
            "         | \u{1b}[37m\u{1b}[40mm/\u{1b}[39m\u{1b}[49m"
            "column 1 | ht"
            "         | tp"
            "         | s:"
            "         | //"
            "         | ww"
            "         | w."
            "         | op"
            "         | en"
            "         | su"
            "         | se"
            "         | .o"
            "         | rg"
            "         | /"
            "column 2 | 1-"
            "         | 2"
        )
    );
}
