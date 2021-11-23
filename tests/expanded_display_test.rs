use std::borrow::Cow;

use crate::util::create_vector;
use tabled::{display::ExpandedDisplay, Tabled};

#[cfg(feature = "color")]
use owo_colors::{AnsiColors, OwoColorize};

mod util;

#[test]
fn display() {
    let data = create_vector::<3, 3>();
    let table = ExpandedDisplay::new(&data).to_string();

    let expected = concat!(
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
    );

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
#[test]
fn display_colored() {
    let mut data = create_vector::<3, 3>();
    data[0][2] = "https://getfedora.org/"
        .red()
        .on_color(AnsiColors::Blue)
        .into();
    data[1][2] = "https://www.opensuse.org/"
        .green()
        .on_color(AnsiColors::Black)
        .into();
    data[2][2] = "https://endeavouros.com/".blue().underline().into();

    let table = ExpandedDisplay::new(&data);

    let expected = concat!(
        "-[ RECORD 0 ]-----------------------\n",
        "N        | 0\n",
        "column 0 | 0-0\n",
        "column 1 | \u{1b}[44m\u{1b}[31mhttps://getfedora.org/\u{1b}[0m\u{1b}[0m\n",
        "column 2 | 0-2\n",
        "-[ RECORD 1 ]-----------------------\n",
        "N        | 1\n",
        "column 0 | 1-0\n",
        "column 1 | \u{1b}[40m\u{1b}[32mhttps://www.opensuse.org/\u{1b}[0m\u{1b}[0m\n",
        "column 2 | 1-2\n",
        "-[ RECORD 2 ]-----------------------\n",
        "N        | 2\n",
        "column 0 | 2-0\n",
        "column 1 | \u{1b}[4m\u{1b}[34mhttps://endeavouros.com/\u{1b}[0m\u{1b}[0m\n",
        "column 2 | 2-2\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn display_empty() {
    struct Type;

    impl Tabled for Type {
        fn fields(&self) -> Vec<Cow<'_, str>> {
            Vec::new()
        }

        fn headers() -> Vec<Cow<'static, str>> {
            Vec::new()
        }
    }

    let table = ExpandedDisplay::new(&[Type]).to_string();

    assert_eq!(table, "-[ RECORD 0 ]-\n");
}

#[test]
fn display_dynamic_header_template() {
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<Cow<'_, str>> {
                vec!["He".into(), "123".into(), "asd".into()]
            }

            fn headers() -> Vec<Cow<'static, str>> {
                vec!["1".into(), "2".into(), "3".into()]
            }
        }

        let expected = concat!("-[ RECORD 0 ]-\n", "1 | He\n", "2 | 123\n", "3 | asd\n",);

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<Cow<'_, str>> {
                vec!["He".into(), "123".into(), "asd".into()]
            }

            fn headers() -> Vec<Cow<'static, str>> {
                vec!["11".into(), "2222222".into(), "3".into()]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]-\n",
            "11      | He\n",
            "2222222 | 123\n",
            "3       | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<Cow<'_, str>> {
                vec!["HeheHehe".into(), "123".into(), "asd".into()]
            }

            fn headers() -> Vec<Cow<'static, str>> {
                vec!["11".into(), "2222222".into(), "3".into()]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]-----\n",
            "11      | HeheHehe\n",
            "2222222 | 123\n",
            "3       | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<Cow<'_, str>> {
                vec!["He".into(), "123".into(), "asd".into()]
            }

            fn headers() -> Vec<Cow<'static, str>> {
                vec!["11111111111".into(), "2".into(), "3".into()]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]----\n",
            "11111111111 | He\n",
            "2           | 123\n",
            "3           | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<Cow<'_, str>> {
                vec!["He".into(), "123".into(), "asd".into()]
            }

            fn headers() -> Vec<Cow<'static, str>> {
                vec!["1111111111111".into(), "2".into(), "3".into()]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]-+----\n",
            "1111111111111 | He\n",
            "2             | 123\n",
            "3             | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<Cow<'_, str>> {
                vec!["He".into(), "123".into(), "asd".into()]
            }

            fn headers() -> Vec<Cow<'static, str>> {
                vec![
                    "11111111111111111111111111111".into(),
                    "2".into(),
                    "3".into(),
                ]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]-----------------+----\n",
            "11111111111111111111111111111 | He\n",
            "2                             | 123\n",
            "3                             | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        #[derive(Clone)]
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<Cow<'_, str>> {
                vec!["22".into()]
            }

            fn headers() -> Vec<Cow<'static, str>> {
                vec!["11111111111".into()]
            }
        }

        let expected = concat!(
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
        );

        let table = ExpandedDisplay::new(std::iter::repeat(Type).take(11)).to_string();

        assert_eq!(table, expected);
    }
}

#[test]
fn display_multiline_field() {
    struct St;

    impl Tabled for St {
        fn fields(&self) -> Vec<Cow<'_, str>> {
            vec!["1".into(), "2".into(), "3".into()]
        }

        fn headers() -> Vec<Cow<'static, str>> {
            vec!["Hello\nWorld".into(), "123".into(), "asd".into()]
        }
    }

    let data = vec![St];
    let table = ExpandedDisplay::new(&data).to_string();

    let expected = concat!(
        "-[ RECORD 0 ]---\n",
        "Hello\\nWorld | 1\n",
        "123          | 2\n",
        "asd          | 3\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn display_multiline_record_value() {
    let mut data = create_vector::<2, 3>();
    data[0][0] = "Hello\nWorld".into();
    data[0][1] = "123".into();
    data[0][2] = "asd".into();

    let table = ExpandedDisplay::new(&data).to_string();

    let expected = concat!(
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
    );

    assert_eq!(table, expected);
}

#[test]
fn display_with_header_template() {
    let data = create_vector::<2, 3>();
    let table = ExpandedDisplay::new(&data)
        .header_template(|i| format!("=== Record => {}", i))
        .to_string();

    let expected = concat!(
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
    );

    assert_eq!(table, expected);
}

#[test]
fn display_with_formatter() {
    let mut data = create_vector::<2, 3>();
    data[0][1] = "123\n456".to_owned();

    let table = ExpandedDisplay::new(&data)
        .formatter(|s| format!("{}!\n\n", s))
        .to_string();

    let expected = concat!(
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
    );

    assert_eq!(table, expected);
}

#[test]
fn display_with_one_line_formatter() {
    let mut data = create_vector::<1, 3>();
    data[0][0] = "Hello\nWorld".into();
    data[0][1] = "123".into();
    data[0][2] = "asd".into();

    let table = ExpandedDisplay::new(&data)
        .formatter(|s| s.escape_debug().to_string())
        .to_string();

    let expected = concat!(
        "-[ RECORD 0 ]----------\n",
        "N        | Hello\\nWorld\n",
        "column 0 | 123\n",
        "column 1 | asd\n",
        "column 2 | 0-2\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn display_with_truncate() {
    let data = create_vector::<3, 3>();
    let table = ExpandedDisplay::new(&data).truncate(2, "").to_string();

    let expected = concat!(
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
    );

    assert_eq!(table, expected);
}

#[test]
fn display_with_truncate_with_tail() {
    let data = create_vector::<2, 3>();
    let table = ExpandedDisplay::new(&data).truncate(2, "...").to_string();

    let expected = concat!(
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
    );

    assert_eq!(table, expected);
}

#[test]
fn display_with_wrap() {
    let data = create_vector::<2, 3>();
    let table = ExpandedDisplay::new(&data).wrap(1).to_string();

    let expected = concat!(
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
    );

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
#[test]
fn display_with_wrap_colored() {
    let mut data = create_vector::<2, 3>();
    data[0][2] = "https://getfedora.org/".red().into();
    data[1][1] = "https://endeavouros.com/"
        .white()
        .on_color(AnsiColors::Black)
        .into();
    data[1][2] = "https://www.opensuse.org/".into();

    let table = ExpandedDisplay::new(&data).wrap(2).to_string();

    let expected = concat!(
        "-[ RECORD 0 ]-\n",
        "N        | 0\n",
        "column 0 | 0-\n",
        "         | 0\n",
        "column 1 | \u{1b}[31mht\u{1b}[0m\n",
        "         | \u{1b}[31mtp\u{1b}[0m\n",
        "         | \u{1b}[31ms:\u{1b}[0m\n",
        "         | \u{1b}[31m//\u{1b}[0m\n",
        "         | \u{1b}[31mge\u{1b}[0m\n",
        "         | \u{1b}[31mtf\u{1b}[0m\n",
        "         | \u{1b}[31med\u{1b}[0m\n",
        "         | \u{1b}[31mor\u{1b}[0m\n",
        "         | \u{1b}[31ma.\u{1b}[0m\n",
        "         | \u{1b}[31mor\u{1b}[0m\n",
        "         | \u{1b}[31mg/\u{1b}[0m\n",
        "column 2 | 0-\n",
        "         | 2\n",
        "-[ RECORD 1 ]-\n",
        "N        | 1\n",
        "column 0 | \u{1b}[40m\u{1b}[37mht\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37mtp\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37ms:\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37m//\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37men\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37mde\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37mav\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37mou\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37mro\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37ms.\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37mco\u{1b}[0m\u{1b}[0m\n",
        "         | \u{1b}[40m\u{1b}[37mm/\u{1b}[0m\u{1b}[0m\n",
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
    );

    assert_eq!(table, expected);
}
