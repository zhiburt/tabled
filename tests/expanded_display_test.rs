use crate::util::create_vector;
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
    let data = create_vector::<3, 3>();
    let table = ExpandedDisplay::new(&data).to_string();

    assert_eq!(
        table,
        concat!(
            "-[ RECORD 0 ]-\n",
            "N        | 0\n",
            "column 0 | 0-0\n",
            "column 1 | 0-1\n",
            "column 2 | 0-2\n",
            "-[ RECORD 1 ]-\n",
            "N        | 1\n",
            "column 0 | 1-0\n",
            "column 1 | 1-1\n",
            "column 2 | 1-2\n",
            "-[ RECORD 2 ]-\n",
            "N        | 2\n",
            "column 0 | 2-0\n",
            "column 1 | 2-1\n",
            "column 2 | 2-2\n",
        )
    );
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

    let table = ExpandedDisplay::new(&data).to_string();

    assert_eq!(
        table,
        util::static_table!(
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
        .to_owned()
            + "\n"
    );
}

#[test]
fn display_empty() {
    build_tabled_type!(EmptyType, 0, [""; 0], [""; 0]);
    assert_expanded_display!(&[EmptyType], "-[ RECORD 0 ]-\n");
}

#[test]
fn display_dynamic_header_template() {
    {
        build_tabled_type!(TestType, 3, ["He", "123", "asd"], ["1", "2", "3"]);
        assert_expanded_display!(
            &[TestType],
            concat!("-[ RECORD 0 ]-\n", "1 | He\n", "2 | 123\n", "3 | asd\n",)
        );
    }
    {
        build_tabled_type!(TestType, 3, ["He", "123", "asd"], ["11", "2222222", "3"]);
        assert_expanded_display!(
            &[TestType],
            concat!(
                "-[ RECORD 0 ]-\n",
                "11      | He\n",
                "2222222 | 123\n",
                "3       | asd\n",
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
            concat!(
                "-[ RECORD 0 ]-----\n",
                "11      | HeheHehe\n",
                "2222222 | 123\n",
                "3       | asd\n",
            )
        );
    }
    {
        build_tabled_type!(TestType, 3, ["He", "123", "asd"], ["11111111111", "2", "3"]);
        assert_expanded_display!(
            &[TestType],
            concat!(
                "-[ RECORD 0 ]----\n",
                "11111111111 | He\n",
                "2           | 123\n",
                "3           | asd\n",
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
            concat!(
                "-[ RECORD 0 ]-+----\n",
                "1111111111111 | He\n",
                "2             | 123\n",
                "3             | asd\n",
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
            concat!(
                "-[ RECORD 0 ]-----------------+----\n",
                "11111111111111111111111111111 | He\n",
                "2                             | 123\n",
                "3                             | asd\n",
            )
        );
    }
    {
        build_tabled_type!(TestType, 3, ["22"], ["11111111111"]);
        assert_expanded_display!(
            std::iter::repeat(TestType).take(11),
            concat!(
                "-[ RECORD 0 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 1 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 2 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 3 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 4 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 5 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 6 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 7 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 8 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 9 ]---\n",
                "11111111111 | 22\n",
                "-[ RECORD 10 ]--\n",
                "11111111111 | 22\n",
            )
        );
    }
}

#[test]
fn display_multiline_field() {
    build_tabled_type!(TestType, 3, ["1", "2", "3"], ["Hello\nWorld", "123", "asd"]);
    assert_expanded_display!(
        [TestType],
        concat!(
            "-[ RECORD 0 ]---\n",
            "Hello\\nWorld | 1\n",
            "123          | 2\n",
            "asd          | 3\n",
        )
    );
}

#[test]
fn display_multiline_record_value() {
    let mut data = create_vector::<2, 3>();
    data[0][0] = "Hello\nWorld".to_string();
    data[0][1] = "123".to_string();
    data[0][2] = "asd".to_string();

    let table = ExpandedDisplay::new(&data).to_string();

    assert_eq!(
        table,
        concat!(
            "-[ RECORD 0 ]---\n",
            "N        | Hello\n",
            "         | World\n",
            "column 0 | 123\n",
            "column 1 | asd\n",
            "column 2 | 0-2\n",
            "-[ RECORD 1 ]---\n",
            "N        | 1\n",
            "column 0 | 1-0\n",
            "column 1 | 1-1\n",
            "column 2 | 1-2\n",
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
        concat!(
            "=== Record => 0\n",
            "N        | 0\n",
            "column 0 | 0-0\n",
            "column 1 | 0-1\n",
            "column 2 | 0-2\n",
            "=== Record => 1\n",
            "N        | 1\n",
            "column 0 | 1-0\n",
            "column 1 | 1-1\n",
            "column 2 | 1-2\n",
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
        concat!(
            "-[ RECORD 0 ]--\n",
            "N        | 0!\n",
            "         | \n",
            "column 0 | 123\n",
            "         | 456!\n",
            "         | \n",
            "column 1 | 0-1!\n",
            "         | \n",
            "column 2 | 0-2!\n",
            "         | \n",
            "-[ RECORD 1 ]--\n",
            "N        | 1!\n",
            "         | \n",
            "column 0 | 1-0!\n",
            "         | \n",
            "column 1 | 1-1!\n",
            "         | \n",
            "column 2 | 1-2!\n",
            "         | \n",
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
        concat!(
            "-[ RECORD 0 ]----------\n",
            "N        | Hello\\nWorld\n",
            "column 0 | 123\n",
            "column 1 | asd\n",
            "column 2 | 0-2\n",
        )
    );
}

#[test]
fn display_with_truncate() {
    let data = create_vector::<3, 3>();
    let table = ExpandedDisplay::new(&data).truncate(2, "").to_string();

    assert_eq!(
        table,
        concat!(
            "-[ RECORD 0 ]-\n",
            "N        | 0\n",
            "column 0 | 0-\n",
            "column 1 | 0-\n",
            "column 2 | 0-\n",
            "-[ RECORD 1 ]-\n",
            "N        | 1\n",
            "column 0 | 1-\n",
            "column 1 | 1-\n",
            "column 2 | 1-\n",
            "-[ RECORD 2 ]-\n",
            "N        | 2\n",
            "column 0 | 2-\n",
            "column 1 | 2-\n",
            "column 2 | 2-\n",
        )
    );
}

#[test]
fn display_with_truncate_with_tail() {
    let data = create_vector::<2, 3>();
    let table = ExpandedDisplay::new(&data).truncate(2, "...").to_string();

    assert_eq!(
        table,
        concat!(
            "-[ RECORD 0 ]---\n",
            "N        | 0\n",
            "column 0 | 0-...\n",
            "column 1 | 0-...\n",
            "column 2 | 0-...\n",
            "-[ RECORD 1 ]---\n",
            "N        | 1\n",
            "column 0 | 1-...\n",
            "column 1 | 1-...\n",
            "column 2 | 1-...\n",
        )
    );
}

#[test]
fn display_with_wrap() {
    let data = create_vector::<2, 3>();
    let table = ExpandedDisplay::new(&data).wrap(1).to_string();

    assert_eq!(
        table,
        concat!(
            "-[ RECORD 0 ]-\n",
            "N        | 0\n",
            "column 0 | 0\n",
            "         | -\n",
            "         | 0\n",
            "column 1 | 0\n",
            "         | -\n",
            "         | 1\n",
            "column 2 | 0\n",
            "         | -\n",
            "         | 2\n",
            "-[ RECORD 1 ]-\n",
            "N        | 1\n",
            "column 0 | 1\n",
            "         | -\n",
            "         | 0\n",
            "column 1 | 1\n",
            "         | -\n",
            "         | 1\n",
            "column 2 | 1\n",
            "         | -\n",
            "         | 2\n",
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
        concat!(
            "-[ RECORD 0 ]-\n",
            "N        | 0\n",
            "column 0 | 0-\n",
            "         | 0\n",
            "column 1 | \u{1b}[31mht\u{1b}[39m\n",
            "         | \u{1b}[31mtp\u{1b}[39m\n",
            "         | \u{1b}[31ms:\u{1b}[39m\n",
            "         | \u{1b}[31m//\u{1b}[39m\n",
            "         | \u{1b}[31mge\u{1b}[39m\n",
            "         | \u{1b}[31mtf\u{1b}[39m\n",
            "         | \u{1b}[31med\u{1b}[39m\n",
            "         | \u{1b}[31mor\u{1b}[39m\n",
            "         | \u{1b}[31ma.\u{1b}[39m\n",
            "         | \u{1b}[31mor\u{1b}[39m\n",
            "         | \u{1b}[31mg/\u{1b}[39m\n",
            "column 2 | 0-\n",
            "         | 2\n",
            "-[ RECORD 1 ]-\n",
            "N        | 1\n",
            "column 0 | \u{1b}[37m\u{1b}[40mht\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40mtp\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40ms:\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40m//\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40men\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40mde\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40mav\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40mou\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40mro\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40ms.\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40mco\u{1b}[39m\u{1b}[49m\n",
            "         | \u{1b}[37m\u{1b}[40mm/\u{1b}[39m\u{1b}[49m\n",
            "column 1 | ht\n",
            "         | tp\n",
            "         | s:\n",
            "         | //\n",
            "         | ww\n",
            "         | w.\n",
            "         | op\n",
            "         | en\n",
            "         | su\n",
            "         | se\n",
            "         | .o\n",
            "         | rg\n",
            "         | /\n",
            "column 2 | 1-\n",
            "         | 2\n",
        )
    );
}
