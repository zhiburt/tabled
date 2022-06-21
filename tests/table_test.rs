use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
use tabled::{Style, Table, TableIteratorExt, Tabled, Width};

use crate::util::{create_vector, static_table};

mod util;

mod default_types {
    use super::*;

    #[test]
    fn table_str_vec() {
        let data = vec!["hello", "world"];
        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-------+"
                "| &str  |"
                "+-------+"
                "| hello |"
                "+-------+"
                "| world |"
                "+-------+"
            )
        );
    }

    #[test]
    fn table_char_vec() {
        let data = vec!['a', 'b', 'c'];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+------+"
                "| char |"
                "+------+"
                "|  a   |"
                "+------+"
                "|  b   |"
                "+------+"
                "|  c   |"
                "+------+"
            )
        );
    }

    #[test]
    fn table_bool_vec() {
        let data = vec![true, false, true];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-------+"
                "| bool  |"
                "+-------+"
                "| true  |"
                "+-------+"
                "| false |"
                "+-------+"
                "| true  |"
                "+-------+"
            )
        );
    }

    #[test]
    fn table_usize_vec() {
        let data = vec![0usize, 1usize, 2usize];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-------+"
                "| usize |"
                "+-------+"
                "|   0   |"
                "+-------+"
                "|   1   |"
                "+-------+"
                "|   2   |"
                "+-------+"
            )
        );
    }

    #[test]
    fn table_isize_vec() {
        let data = vec![0isize, 1isize, 2isize];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-------+"
                "| isize |"
                "+-------+"
                "|   0   |"
                "+-------+"
                "|   1   |"
                "+-------+"
                "|   2   |"
                "+-------+"
            )
        );
    }

    #[test]
    fn table_u8_vec() {
        let data = vec![0u8, 1u8, 2u8];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+----+"
                "| u8 |"
                "+----+"
                "| 0  |"
                "+----+"
                "| 1  |"
                "+----+"
                "| 2  |"
                "+----+"
            )
        );
    }

    #[test]
    fn table_u16_vec() {
        let data = vec![0u16, 1u16, 2u16];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-----+"
                "| u16 |"
                "+-----+"
                "|  0  |"
                "+-----+"
                "|  1  |"
                "+-----+"
                "|  2  |"
                "+-----+"
            )
        );
    }

    #[test]
    fn table_u32_vec() {
        let data = vec![0u32, 1u32, 2u32];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-----+"
                "| u32 |"
                "+-----+"
                "|  0  |"
                "+-----+"
                "|  1  |"
                "+-----+"
                "|  2  |"
                "+-----+"
            )
        );
    }

    #[test]
    fn table_u64_vec() {
        let data = vec![0u64, 1u64, 2u64];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-----+"
                "| u64 |"
                "+-----+"
                "|  0  |"
                "+-----+"
                "|  1  |"
                "+-----+"
                "|  2  |"
                "+-----+"
            )
        );
    }

    #[test]
    fn table_u128_vec() {
        let data = vec![0u128, 1u128, 2u128];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+------+"
                "| u128 |"
                "+------+"
                "|  0   |"
                "+------+"
                "|  1   |"
                "+------+"
                "|  2   |"
                "+------+"
            )
        );
    }

    #[test]
    fn table_i8_vec() {
        let data = vec![0i8, 1i8, 2i8];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+----+"
                "| i8 |"
                "+----+"
                "| 0  |"
                "+----+"
                "| 1  |"
                "+----+"
                "| 2  |"
                "+----+"
            )
        );
    }

    #[test]
    fn table_i16_vec() {
        let data = vec![0i16, 1i16, 2i16];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-----+"
                "| i16 |"
                "+-----+"
                "|  0  |"
                "+-----+"
                "|  1  |"
                "+-----+"
                "|  2  |"
                "+-----+"
            )
        );
    }

    #[test]
    fn table_i32_vec() {
        let data = vec![0i32, 1i32, 2i32];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-----+"
                "| i32 |"
                "+-----+"
                "|  0  |"
                "+-----+"
                "|  1  |"
                "+-----+"
                "|  2  |"
                "+-----+"
            )
        );
    }

    #[test]
    fn table_i64_vec() {
        let data = vec![0i64, 1i64, 2i64];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+-----+"
                "| i64 |"
                "+-----+"
                "|  0  |"
                "+-----+"
                "|  1  |"
                "+-----+"
                "|  2  |"
                "+-----+"
            )
        );
    }

    #[test]
    fn table_i128_vec() {
        let data = vec![0i128, 1i128, 2i128];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+------+"
                "| i128 |"
                "+------+"
                "|  0   |"
                "+------+"
                "|  1   |"
                "+------+"
                "|  2   |"
                "+------+"
            )
        );
    }

    #[test]
    fn table_array() {
        let data = vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]];

        let table = Table::new(&data).to_string();

        assert_eq!(
            table,
            static_table!(
                "+---+---+---+"
                "| 0 | 1 | 2 |"
                "+---+---+---+"
                "| 0 | 1 | 2 |"
                "+---+---+---+"
                "| 3 | 4 | 5 |"
                "+---+---+---+"
                "| 6 | 7 | 8 |"
                "+---+---+---+"
            )
        );
    }
}

#[test]
fn table_vector_structures() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let st = vec![St { f1: 0, f2: "0" }, St { f1: 1, f2: "1" }];

    let table = Table::new(st).to_string();

    assert_eq!(
        table,
        static_table!(
            "+----+----+"
            "| f1 | f2 |"
            "+----+----+"
            "| 0  | 0  |"
            "+----+----+"
            "| 1  | 1  |"
            "+----+----+"
        )
    );
}

#[test]
fn table_empty_vector_structures() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let st: Vec<St> = Vec::new();

    let table = Table::new(st).to_string();

    assert_eq!(
        table,
        static_table!(
            "+----+----+"
            "| f1 | f2 |"
            "+----+----+"
        )
    );
}

#[test]
fn table_option() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let st = Some(St { f1: 0, f2: "0" });

    let table = Table::new(st).to_string();

    assert_eq!(
        table,
        static_table!(
            "+----+----+"
            "| f1 | f2 |"
            "+----+----+"
            "| 0  | 0  |"
            "+----+----+"
        )
    );
}

#[test]
fn table_option_none() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let st: Option<St> = None;

    let table = Table::new(st).to_string();

    assert_eq!(
        table,
        static_table!(
            "+----+----+"
            "| f1 | f2 |"
            "+----+----+"
        )
    );
}

#[test]
fn table_tuple() {
    let t = ("we are in", 2020);

    let table = Table::new(&[t]).to_string();

    assert_eq!(
        table,
        static_table!(
            "+-----------+------+"
            "|   &str    | i32  |"
            "+-----------+------+"
            "| we are in | 2020 |"
            "+-----------+------+"
        )
    );
}

#[test]
fn table_single_tuple() {
    let t = (2020,);

    let table = Table::new(&[t]).to_string();

    assert_eq!(
        table,
        static_table!(
            "+------+"
            "| i32  |"
            "+------+"
            "| 2020 |"
            "+------+"
        )
    );
}

#[test]
fn table_tuple_vec() {
    let map = [(0, "Monday"), (1, "Thursday")];

    let table = Table::new(&map).to_string();

    assert_eq!(
        table,
        static_table!(
            "+-----+----------+"
            "| i32 |   &str   |"
            "+-----+----------+"
            "|  0  |  Monday  |"
            "+-----+----------+"
            "|  1  | Thursday |"
            "+-----+----------+"
        )
    );
}

#[test]
fn table_tuple_with_structure_vec() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let map = [(0, St { f1: 0, f2: "0str" }), (1, St { f1: 1, f2: "1str" })];

    let table = Table::new(&map).to_string();

    assert_eq!(
        table,
        static_table!(
            "+-----+----+------+"
            "| i32 | f1 |  f2  |"
            "+-----+----+------+"
            "|  0  | 0  | 0str |"
            "+-----+----+------+"
            "|  1  | 1  | 1str |"
            "+-----+----+------+"
        )
    );
}

#[allow(dead_code)]
#[test]
fn table_vector_structures_with_hidden_tabled() {
    #[derive(Tabled)]
    struct St {
        #[tabled(skip)]
        f1: u8,
        f2: &'static str,
    }

    let st = vec![St { f1: 0, f2: "0" }, St { f1: 1, f2: "1" }];

    let table = Table::new(&st).to_string();

    assert_eq!(
        table,
        static_table!(
            "+----+"
            "| f2 |"
            "+----+"
            "| 0  |"
            "+----+"
            "| 1  |"
            "+----+"
        )
    );
}

#[test]
fn table_enum() {
    #[allow(dead_code)]
    #[derive(Tabled)]
    enum Letters {
        Vowels { character: char, lang: u8 },
        Consonant(char),
        Digit,
    }

    let data = vec![
        Letters::Vowels {
            character: 'a',
            lang: 0,
        },
        Letters::Consonant('w'),
        Letters::Vowels {
            character: 'b',
            lang: 1,
        },
        Letters::Vowels {
            character: 'c',
            lang: 2,
        },
        Letters::Digit,
    ];

    let table = Table::new(&data).to_string();

    assert_eq!(
        table,
        static_table!(
            "+--------+-----------+-------+"
            "| Vowels | Consonant | Digit |"
            "+--------+-----------+-------+"
            "|   +    |           |       |"
            "+--------+-----------+-------+"
            "|        |     +     |       |"
            "+--------+-----------+-------+"
            "|   +    |           |       |"
            "+--------+-----------+-------+"
            "|   +    |           |       |"
            "+--------+-----------+-------+"
            "|        |           |   +   |"
            "+--------+-----------+-------+"
        )
    );
}

#[test]
fn table_enum_with_hidden_variant() {
    #[allow(dead_code)]
    #[derive(Tabled)]
    enum Letters {
        Vowels {
            character: char,
            lang: u8,
        },
        Consonant(char),
        #[tabled(skip)]
        Digit,
    }

    let data = vec![
        Letters::Vowels {
            character: 'a',
            lang: 0,
        },
        Letters::Consonant('w'),
        Letters::Vowels {
            character: 'b',
            lang: 1,
        },
        Letters::Vowels {
            character: 'c',
            lang: 2,
        },
        Letters::Digit,
    ];

    let table = Table::new(&data).to_string();

    assert_eq!(
        table,
        static_table!(
            "+--------+-----------+"
            "| Vowels | Consonant |"
            "+--------+-----------+"
            "|   +    |           |"
            "+--------+-----------+"
            "|        |     +     |"
            "+--------+-----------+"
            "|   +    |           |"
            "+--------+-----------+"
            "|   +    |           |"
            "+--------+-----------+"
            "|        |           |"
            "+--------+-----------+"
        )
    );
}

#[test]
fn table_btreemap() {
    #[derive(Tabled)]
    struct A {
        b: u8,
        c: &'static str,
    }

    let mut map = BTreeMap::new();
    map.insert(0, A { b: 1, c: "s1" });
    map.insert(1, A { b: 2, c: "s2" });
    map.insert(3, A { b: 3, c: "s3" });

    let table = Table::new(&map).to_string();

    assert_eq!(
        table,
        static_table!(
            "+-----+---+----+"
            "| i32 | b | c  |"
            "+-----+---+----+"
            "|  0  | 1 | s1 |"
            "+-----+---+----+"
            "|  1  | 2 | s2 |"
            "+-----+---+----+"
            "|  3  | 3 | s3 |"
            "+-----+---+----+"
        )
    );
}

#[test]
fn table_btreeset() {
    #[derive(Tabled, PartialEq, Eq, PartialOrd, Ord)]
    struct A {
        b: u8,
        c: &'static str,
    }

    let mut map = BTreeSet::new();
    map.insert(A { b: 1, c: "s1" });
    map.insert(A { b: 2, c: "s2" });
    map.insert(A { b: 3, c: "s3" });

    let table = Table::new(&map).to_string();

    assert_eq!(
        table,
        static_table!(
            "+---+----+"
            "| b | c  |"
            "+---+----+"
            "| 1 | s1 |"
            "+---+----+"
            "| 2 | s2 |"
            "+---+----+"
            "| 3 | s3 |"
            "+---+----+"
        )
    );
}

#[test]
fn table_emojie() {
    #[derive(Tabled)]
    struct Language {
        name: &'static str,
        designed_by: &'static str,
        invented_year: usize,
    }

    let languages = vec![
        Language {
            name: "C 💕",
            designed_by: "Dennis Ritchie",
            invented_year: 1972,
        },
        Language {
            name: "Rust 👍",
            designed_by: "Graydon Hoare",
            invented_year: 2010,
        },
        Language {
            name: "Go 🧋",
            designed_by: "Rob Pike",
            invented_year: 2009,
        },
    ];

    let table = Table::new(&languages).to_string();

    assert_eq!(
        table,
        static_table!(
            "+---------+----------------+---------------+"
            "|  name   |  designed_by   | invented_year |"
            "+---------+----------------+---------------+"
            "|  C 💕   | Dennis Ritchie |     1972      |"
            "+---------+----------------+---------------+"
            "| Rust 👍 | Graydon Hoare  |     2010      |"
            "+---------+----------------+---------------+"
            "|  Go 🧋  |    Rob Pike    |     2009      |"
            "+---------+----------------+---------------+"
        )
    );
}

#[test]
fn table_emojie_multiline() {
    #[derive(Tabled)]
    struct Article {
        name: &'static str,
        author: &'static str,
        text: &'static str,
        rating: usize,
    }

    let languages = vec![
        Article {
            name: "Rebase vs Merge commit in depth 👋",
            author: "Rose Kuphal DVM",
            text: "A multiline\n text with 🤯 😳 🥵 🥶\n a bunch of emojies ☄️ 💥 🔥 🌪",
            rating: 43,
        },
        Article {
            name: "Keep it simple",
            author: "Unknown",
            text: "🍳",
            rating: 100,
        },
    ];

    let table = Table::new(&languages).to_string();

    // Note: it looks OK in a terminal
    assert_eq!(
        table,
        static_table!(
            "+------------------------------------+-----------------+-------------------------------+--------+"
            "|                name                |     author      |             text              | rating |"
            "+------------------------------------+-----------------+-------------------------------+--------+"
            "| Rebase vs Merge commit in depth 👋 | Rose Kuphal DVM | A multiline                   |   43   |"
            "|                                    |                 |  text with 🤯 😳 🥵 🥶        |        |"
            "|                                    |                 |  a bunch of emojies ☄\u{fe0f} 💥 🔥 🌪 |        |"
            "+------------------------------------+-----------------+-------------------------------+--------+"
            "|           Keep it simple           |     Unknown     |              🍳               |  100   |"
            "+------------------------------------+-----------------+-------------------------------+--------+"
        ),
    )
}

#[test]
fn tuple_combination() {
    #[derive(Tabled)]
    enum Domain {
        Security,
        Embeded,
        Frontend,
        Unknown,
    }

    #[derive(Tabled)]
    struct Developer(#[tabled(rename = "name")] &'static str);

    let data = vec![
        (Developer("Terri Kshlerin"), Domain::Embeded),
        (Developer("Catalina Dicki"), Domain::Security),
        (Developer("Jennie Schmeler"), Domain::Frontend),
        (Developer("Maxim Zhiburt"), Domain::Unknown),
    ];

    let table = Table::new(data).with(Style::psql()).to_string();

    assert_eq!(
        table,
        static_table!(
            "      name       | Security | Embeded | Frontend | Unknown "
            "-----------------+----------+---------+----------+---------"
            " Terri Kshlerin  |          |    +    |          |         "
            " Catalina Dicki  |    +     |         |          |         "
            " Jennie Schmeler |          |         |    +     |         "
            "  Maxim Zhiburt  |          |         |          |    +    "
        )
    );
}

#[test]
fn table_trait() {
    #[derive(Tabled)]
    enum Domain {
        Security,
        Embeded,
        Frontend,
        Unknown,
    }

    #[derive(Tabled)]
    struct Developer(#[tabled(rename = "name")] &'static str);

    let data = vec![
        (Developer("Terri Kshlerin"), Domain::Embeded),
        (Developer("Catalina Dicki"), Domain::Security),
        (Developer("Jennie Schmeler"), Domain::Frontend),
        (Developer("Maxim Zhiburt"), Domain::Unknown),
    ];

    let table = (&data).table().with(Style::psql()).to_string();

    assert_eq!(
        table,
        static_table!(
            "      name       | Security | Embeded | Frontend | Unknown "
            "-----------------+----------+---------+----------+---------"
            " Terri Kshlerin  |          |    +    |          |         "
            " Catalina Dicki  |    +     |         |          |         "
            " Jennie Schmeler |          |         |    +     |         "
            "  Maxim Zhiburt  |          |         |          |    +    "
        )
    );
}

#[test]
fn build_table_from_iterator() {
    let data = create_vector::<3, 3>();
    let table = Table::from_iter(data).with(Style::psql()).to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |   2-1    |   2-2    "
        )
    );
}

#[test]
fn table_emojie_utf8_style() {
    #[derive(Tabled)]
    struct Language {
        name: &'static str,
        designed_by: &'static str,
        invented_year: usize,
    }

    let languages = vec![
        Language {
            name: "C 💕",
            designed_by: "Dennis Ritchie",
            invented_year: 1972,
        },
        Language {
            name: "Rust 👍",
            designed_by: "Graydon Hoare",
            invented_year: 2010,
        },
        Language {
            name: "Go 🧋",
            designed_by: "Rob Pike",
            invented_year: 2009,
        },
    ];

    let table = Table::new(&languages)
        .with(tabled::Style::modern().header_off().horizontal_off())
        .to_string();

    // Note: It doesn't look good in VS Code
    assert_eq!(
        table,
        static_table!(
            "┌─────────┬────────────────┬───────────────┐"
            "│  name   │  designed_by   │ invented_year │"
            "│  C 💕   │ Dennis Ritchie │     1972      │"
            "│ Rust 👍 │ Graydon Hoare  │     2010      │"
            "│  Go 🧋  │    Rob Pike    │     2009      │"
            "└─────────┴────────────────┴───────────────┘"
        ),
    );
}

#[cfg(feature = "color")]
#[test]
fn multiline_table_test2() {
    let data = [
        ["\u{1b}[37mThis is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\n\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\n\u{1b}[0m"],
    ];

    let table = Table::new(&data).with(tabled::Style::modern());

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table.to_string()),
        static_table!(
            r#"┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐"#
            r#"│                                                                                                                                  0                                                                                                                                   │"#
            r#"├──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤"#
            r#"│ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                                                                                                 │"#
            r#"│                                                                                                                                                                                                                                                                      │"#
            r#"│ For convenience, we are providing full builds for Windows, Linux, and macOS. These are the "all extra features" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies │"#
            r#"│                                                                                                                                                                                                                                                                      │"#
            r#"└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘"#
        ),
    );

    let table = table.with(Width::wrap(100));

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table.to_string()),
        static_table!(
            "┌──────────────────────────────────────────────────────────────────────────────────────────────────┐"
            "│                                                0                                                 │"
            "├──────────────────────────────────────────────────────────────────────────────────────────────────┤"
            "│ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: http │"
            "│ s://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                             │"
            "│                                                                                                  │"
            "│ For convenience, we are providing full build                                                     │"
            "│ s for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have  │"
            "│ the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/inst │"
            "│ allation.md#dependencies                                                                         │"
            "│                                                                                                  │"
            "└──────────────────────────────────────────────────────────────────────────────────────────────────┘"
        ),
    );
}

#[test]
fn multiline_table_test3() {
    let data = [
        ["This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\r\n\r\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\r\n"],
    ];

    let table = Table::new(&data).with(tabled::Style::modern());

    println!("{}", table);

    assert_eq!(
        table.to_string(),
        static_table!(
            r#"┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐"#
            r#"│                                                                                                                                  0                                                                                                                                   │"#
            r#"├──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤"#
            r#"│ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                                                                                                 │"#
            r#"│                                                                                                                                                                                                                                                                      │"#
            r#"│ For convenience, we are providing full builds for Windows, Linux, and macOS. These are the "all extra features" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies │"#
            r#"│                                                                                                                                                                                                                                                                      │"#
            r#"└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘"#
        ),
    );

    let table = table.with(Width::wrap(100));

    assert_eq!(
        table.to_string(),
        static_table!(
            "┌──────────────────────────────────────────────────────────────────────────────────────────────────┐"
            "│                                                0                                                 │"
            "├──────────────────────────────────────────────────────────────────────────────────────────────────┤"
            "│ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: http │"
            "│ s://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                             │"
            "│                                                                                                  │"
            "│ For convenience, we are providing full build                                                     │"
            "│ s for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have  │"
            "│ the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/inst │"
            "│ allation.md#dependencies                                                                         │"
            "│                                                                                                  │"
            "└──────────────────────────────────────────────────────────────────────────────────────────────────┘"
        ),
    );
}
