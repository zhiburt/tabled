#![cfg(feature = "std")]

use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    settings::{formatting::Charset, Height, Modify, Padding, Settings, Style, Width},
    Table,
};

use crate::matrix::Matrix;
use testing_table::test_table;

mod default_types {
    use super::*;

    test_table!(
        table_str_vec,
        Table::new(vec!["hello", "world"]),
        "+-------+"
        "| &str  |"
        "+-------+"
        "| hello |"
        "+-------+"
        "| world |"
        "+-------+"
    );

    test_table!(
        table_char_vec,
        Table::new(vec!['a', 'b', 'c']),
        "+------+"
        "| char |"
        "+------+"
        "| a    |"
        "+------+"
        "| b    |"
        "+------+"
        "| c    |"
        "+------+"
    );

    test_table!(
        table_bool_vec,
        Table::new(vec![true, false, true]),
        "+-------+"
        "| bool  |"
        "+-------+"
        "| true  |"
        "+-------+"
        "| false |"
        "+-------+"
        "| true  |"
        "+-------+"
    );

    test_table!(
        table_usize_vec,
        Table::new(vec![0usize, 1usize, 2usize]),
        "+-------+"
        "| usize |"
        "+-------+"
        "| 0     |"
        "+-------+"
        "| 1     |"
        "+-------+"
        "| 2     |"
        "+-------+"
    );

    test_table!(
        table_isize_vec,
        Table::new(vec![0isize, 1isize, 2isize]),
        "+-------+"
        "| isize |"
        "+-------+"
        "| 0     |"
        "+-------+"
        "| 1     |"
        "+-------+"
        "| 2     |"
        "+-------+"
    );

    test_table!(
        table_u8_vec,
        Table::new(vec![0u8, 1u8, 2u8]),
        "+----+"
        "| u8 |"
        "+----+"
        "| 0  |"
        "+----+"
        "| 1  |"
        "+----+"
        "| 2  |"
        "+----+"
    );

    test_table!(
        table_u16_vec,
        Table::new(vec![0u16, 1u16, 2u16]),
        "+-----+"
        "| u16 |"
        "+-----+"
        "| 0   |"
        "+-----+"
        "| 1   |"
        "+-----+"
        "| 2   |"
        "+-----+"
    );

    test_table!(
        table_u32_vec,
        Table::new(vec![0u32, 1u32, 2u32]),
        "+-----+"
        "| u32 |"
        "+-----+"
        "| 0   |"
        "+-----+"
        "| 1   |"
        "+-----+"
        "| 2   |"
        "+-----+"
    );

    test_table!(
        table_u64_vec,
        Table::new(vec![0u64, 1u64, 2u64]),
        "+-----+"
        "| u64 |"
        "+-----+"
        "| 0   |"
        "+-----+"
        "| 1   |"
        "+-----+"
        "| 2   |"
        "+-----+"
    );

    test_table!(
        table_u128_vec,
        Table::new(vec![0u128, 1u128, 2u128]),
        "+------+"
        "| u128 |"
        "+------+"
        "| 0    |"
        "+------+"
        "| 1    |"
        "+------+"
        "| 2    |"
        "+------+"
    );

    test_table!(
        table_i8_vec,
        Table::new(vec![0i8, 1i8, 2i8]),
        "+----+"
        "| i8 |"
        "+----+"
        "| 0  |"
        "+----+"
        "| 1  |"
        "+----+"
        "| 2  |"
        "+----+"
    );

    test_table!(
        table_i16_vec,
        Table::new(vec![0i16, 1, 2]),
        "+-----+"
        "| i16 |"
        "+-----+"
        "| 0   |"
        "+-----+"
        "| 1   |"
        "+-----+"
        "| 2   |"
        "+-----+"
    );

    test_table!(
        table_i32_vec,
        Table::new(vec![0i32, 1, 2]),
        "+-----+"
        "| i32 |"
        "+-----+"
        "| 0   |"
        "+-----+"
        "| 1   |"
        "+-----+"
        "| 2   |"
        "+-----+"
    );

    test_table!(
        table_i64_vec,
        Table::new(vec![0i64, 1, 2]),
        "+-----+"
        "| i64 |"
        "+-----+"
        "| 0   |"
        "+-----+"
        "| 1   |"
        "+-----+"
        "| 2   |"
        "+-----+"
    );

    test_table!(
        table_i128_vec,
        Table::new(vec![0i128, 1, 2]),
        "+------+"
        "| i128 |"
        "+------+"
        "| 0    |"
        "+------+"
        "| 1    |"
        "+------+"
        "| 2    |"
        "+------+"
    );

    test_table!(
        table_array,
        Table::new(vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]]),
        "+---+---+---+"
        "| 0 | 1 | 2 |"
        "+---+---+---+"
        "| 0 | 1 | 2 |"
        "+---+---+---+"
        "| 3 | 4 | 5 |"
        "+---+---+---+"
        "| 6 | 7 | 8 |"
        "+---+---+---+"
    );
}

test_table!(
    table_tuple,
    Table::new(vec![("we are in", 2020)]),
    "+-----------+------+"
    "| &str      | i32  |"
    "+-----------+------+"
    "| we are in | 2020 |"
    "+-----------+------+"
);

test_table!(
    table_single_tuple,
    Table::new(vec![(2020,)]),
    "+------+"
    "| i32  |"
    "+------+"
    "| 2020 |"
    "+------+"
);

test_table!(
    table_tuple_vec,
    #[allow(clippy::needless_borrow)]
    Table::new(&[(0, "Monday"), (1, "Thursday")]),
    "+-----+----------+"
    "| i32 | &str     |"
    "+-----+----------+"
    "| 0   | Monday   |"
    "+-----+----------+"
    "| 1   | Thursday |"
    "+-----+----------+"
);

test_table!(
    build_table_from_iterator,
    Matrix::new(3, 3).with(Style::psql()),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    multiline_table_test_0,
    Table::new([["This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\r\n\r\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\r\n"]])
        .with(Charset::clean())
        .with(Style::modern()),
    r#"┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐"#
    r#"│ 0                                                                                                                                                                                                                                                                    │"#
    r#"├──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤"#
    r#"│ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                                                                                                 │"#
    r#"│                                                                                                                                                                                                                                                                      │"#
    r#"│ For convenience, we are providing full builds for Windows, Linux, and macOS. These are the "all extra features" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies │"#
    r#"│                                                                                                                                                                                                                                                                      │"#
    r#"└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘"#
);

test_table!(
    multiline_table_test_1,
    Table::new([["This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\r\n\r\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\r\n"]])
        .with(Charset::clean())
        .with(Style::modern())
        .with(Width::wrap(100)),
    "┌──────────────────────────────────────────────────────────────────────────────────────────────────┐"
    "│ 0                                                                                                │"
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
);

test_table!(
    multiline_table_test_2,
    Table::new([["This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\r\n\r\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\r\n"]])
        .with(Modify::new((1, 0)).with(Charset::clean()))
        .with(Style::modern())
        .with(Width::wrap(100)),
    "┌──────────────────────────────────────────────────────────────────────────────────────────────────┐"
    "│ 0                                                                                                │"
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
);

#[cfg(feature = "derive")]
mod derived {
    use super::*;

    use std::collections::{BTreeMap, BTreeSet};

    use tabled::Tabled;

    #[derive(Tabled)]
    struct TestType {
        f1: u8,
        f2: &'static str,
    }

    test_table!(
        table_vector_structures,
        Table::new([TestType { f1: 0, f2: "0" }, TestType { f1: 1, f2: "1" }]),
        "+----+----+"
        "| f1 | f2 |"
        "+----+----+"
        "| 0  | 0  |"
        "+----+----+"
        "| 1  | 1  |"
        "+----+----+"
    );

    test_table!(
        table_empty_vector_structures,
        Table::new({let v: Vec<TestType> = Vec::new(); v}),
        "+----+----+"
        "| f1 | f2 |"
        "+----+----+"
    );

    test_table!(
        table_option,
        Table::new(Some(TestType { f1: 0, f2: "0" })),
        "+----+----+"
        "| f1 | f2 |"
        "+----+----+"
        "| 0  | 0  |"
        "+----+----+"
    );

    test_table!(
        table_option_none,
        Table::new(Option::<TestType>::None),
        "+----+----+"
        "| f1 | f2 |"
        "+----+----+"
    );

    test_table!(
        table_tuple_with_structure_vec,
        Table::new([(0, TestType { f1: 0, f2: "0str" }), (1, TestType { f1: 1, f2: "1str" })]),
        "+-----+----+------+"
        "| i32 | f1 | f2   |"
        "+-----+----+------+"
        "| 0   | 0  | 0str |"
        "+-----+----+------+"
        "| 1   | 1  | 1str |"
        "+-----+----+------+"
    );

    test_table!(
        table_vector_structures_with_hidden_tabled,
        Table::new({
            #[derive(Tabled)]
            struct St {
                #[allow(dead_code)]
                #[tabled(skip)]
                f1: u8,
                f2: &'static str,
            }

            vec![St { f1: 0, f2: "0" }, St { f1: 1, f2: "1" }]
        }),
        "+----+"
        "| f2 |"
        "+----+"
        "| 0  |"
        "+----+"
        "| 1  |"
        "+----+"
    );

    test_table!(
        table_enum,
        Table::new({
            #[derive(Tabled)]
            enum Letters {
                Vowels { character: char, lang: u8 },
                Consonant(char),
                Digit,
            }

            vec![
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
            ]
        }),
        "+--------+-----------+-------+"
        "| Vowels | Consonant | Digit |"
        "+--------+-----------+-------+"
        "| +      |           |       |"
        "+--------+-----------+-------+"
        "|        | +         |       |"
        "+--------+-----------+-------+"
        "| +      |           |       |"
        "+--------+-----------+-------+"
        "| +      |           |       |"
        "+--------+-----------+-------+"
        "|        |           | +     |"
        "+--------+-----------+-------+"
    );

    test_table!(
        table_enum_with_hidden_variant,
        Table::new({
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

            vec![
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
            ]
        }),
        "+--------+-----------+"
        "| Vowels | Consonant |"
        "+--------+-----------+"
        "| +      |           |"
        "+--------+-----------+"
        "|        | +         |"
        "+--------+-----------+"
        "| +      |           |"
        "+--------+-----------+"
        "| +      |           |"
        "+--------+-----------+"
        "|        |           |"
        "+--------+-----------+"
    );

    test_table!(
        table_btreemap,
        Table::new({
            #[derive(Tabled)]
            struct A {
                b: u8,
                c: &'static str,
            }

            let mut map = BTreeMap::new();
            map.insert(0, A { b: 1, c: "s1" });
            map.insert(1, A { b: 2, c: "s2" });
            map.insert(3, A { b: 3, c: "s3" });

            map
        }),
        "+-----+---+----+"
        "| i32 | b | c  |"
        "+-----+---+----+"
        "| 0   | 1 | s1 |"
        "+-----+---+----+"
        "| 1   | 2 | s2 |"
        "+-----+---+----+"
        "| 3   | 3 | s3 |"
        "+-----+---+----+"
    );

    test_table!(
        table_emojie_utf8_style,
        Table::new({
            #[derive(Tabled)]
            struct Language {
                name: &'static str,
                designed_by: &'static str,
                invented_year: usize,
            }

            vec![
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
            ]
        }).with(Style::modern().remove_horizontal()),
        // Note: It doesn't look good in VS Code
        "┌─────────┬────────────────┬───────────────┐"
        "│ name    │ designed_by    │ invented_year │"
        "│ C 💕    │ Dennis Ritchie │ 1972          │"
        "│ Rust 👍 │ Graydon Hoare  │ 2010          │"
        "│ Go 🧋   │ Rob Pike       │ 2009          │"
        "└─────────┴────────────────┴───────────────┘"
    );

    test_table!(
        table_btreeset,
        Table::new({
            #[derive(Tabled, PartialEq, Eq, PartialOrd, Ord)]
            struct A {
                b: u8,
                c: &'static str,
            }

            let mut map = BTreeSet::new();
            map.insert(A { b: 1, c: "s1" });
            map.insert(A { b: 2, c: "s2" });
            map.insert(A { b: 3, c: "s3" });

            map
        }),
        "+---+----+"
        "| b | c  |"
        "+---+----+"
        "| 1 | s1 |"
        "+---+----+"
        "| 2 | s2 |"
        "+---+----+"
        "| 3 | s3 |"
        "+---+----+"
    );

    test_table!(
        table_emojie,
        Table::new({
            #[derive(Tabled)]
            struct Language {
                name: &'static str,
                designed_by: &'static str,
                invented_year: usize,
            }

            vec![
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
            ]
        }),
        "+---------+----------------+---------------+"
        "| name    | designed_by    | invented_year |"
        "+---------+----------------+---------------+"
        "| C 💕    | Dennis Ritchie | 1972          |"
        "+---------+----------------+---------------+"
        "| Rust 👍 | Graydon Hoare  | 2010          |"
        "+---------+----------------+---------------+"
        "| Go 🧋   | Rob Pike       | 2009          |"
        "+---------+----------------+---------------+"
    );

    test_table!(
        tuple_combination,
        Table::new({
            #[derive(Tabled)]
            enum Domain {
                Security,
                Embedded,
                Frontend,
                Unknown,
            }

            #[derive(Tabled)]
            struct Developer(#[tabled(rename = "name")] &'static str);

            vec![
                (Developer("Terri Kshlerin"), Domain::Embedded),
                (Developer("Catalina Dicki"), Domain::Security),
                (Developer("Jennie Schmeler"), Domain::Frontend),
                (Developer("Maxim Zhiburt"), Domain::Unknown),
            ]
        }),
        "+-----------------+----------+----------+----------+---------+"
        "| name            | Security | Embedded | Frontend | Unknown |"
        "+-----------------+----------+----------+----------+---------+"
        "| Terri Kshlerin  |          | +        |          |         |"
        "+-----------------+----------+----------+----------+---------+"
        "| Catalina Dicki  | +        |          |          |         |"
        "+-----------------+----------+----------+----------+---------+"
        "| Jennie Schmeler |          |          | +        |         |"
        "+-----------------+----------+----------+----------+---------+"
        "| Maxim Zhiburt   |          |          |          | +       |"
        "+-----------------+----------+----------+----------+---------+"

    );

    test_table!(
        table_trait,
        Table::new({
            #[derive(Tabled)]
            enum Domain {
                Security,
                Embedded,
                Frontend,
                Unknown,
            }

            #[derive(Tabled)]
            struct Developer(#[tabled(rename = "name")] &'static str);

            vec![
                (Developer("Terri Kshlerin"), Domain::Embedded),
                (Developer("Catalina Dicki"), Domain::Security),
                (Developer("Jennie Schmeler"), Domain::Frontend),
                (Developer("Maxim Zhiburt"), Domain::Unknown),
            ]
        }),
        "+-----------------+----------+----------+----------+---------+"
        "| name            | Security | Embedded | Frontend | Unknown |"
        "+-----------------+----------+----------+----------+---------+"
        "| Terri Kshlerin  |          | +        |          |         |"
        "+-----------------+----------+----------+----------+---------+"
        "| Catalina Dicki  | +        |          |          |         |"
        "+-----------------+----------+----------+----------+---------+"
        "| Jennie Schmeler |          |          | +        |         |"
        "+-----------------+----------+----------+----------+---------+"
        "| Maxim Zhiburt   |          |          |          | +       |"
        "+-----------------+----------+----------+----------+---------+"
    );

    test_table!(
        table_emojie_multiline,
        Table::new({
            #[derive(Tabled)]
            struct Article {
                name: &'static str,
                author: &'static str,
                text: &'static str,
                rating: usize,
            }

            vec![
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
            ]
        }),
        "+------------------------------------+-----------------+-------------------------------+--------+"
        "| name                               | author          | text                          | rating |"
        "+------------------------------------+-----------------+-------------------------------+--------+"
        "| Rebase vs Merge commit in depth 👋 | Rose Kuphal DVM | A multiline                   | 43     |"
        "|                                    |                 |  text with 🤯 😳 🥵 🥶        |        |"
        "|                                    |                 |  a bunch of emojies ☄\u{fe0f} 💥 🔥 🌪 |        |"
        "+------------------------------------+-----------------+-------------------------------+--------+"
        "| Keep it simple                     | Unknown         | 🍳                            | 100    |"
        "+------------------------------------+-----------------+-------------------------------+--------+"
    );
}

#[cfg(feature = "color")]
#[test]
fn multiline_table_test2() {
    use testing_table::assert_table;

    let data = &[
        ["\u{1b}[37mThis is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\n\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\n\u{1b}[0m"],
    ];

    let mut table = Table::new(data);
    table.with(Style::modern());

    assert_table!(
        ansi_str::AnsiStr::ansi_strip(&table.to_string()),
        r#"┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐"#
        r#"│ 0                                                                                                                                                                                                                                                                    │"#
        r#"├──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤"#
        r#"│ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                                                                                                 │"#
        r#"│                                                                                                                                                                                                                                                                      │"#
        r#"│ For convenience, we are providing full builds for Windows, Linux, and macOS. These are the "all extra features" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies │"#
        r#"│                                                                                                                                                                                                                                                                      │"#
        r#"└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘"#
    );

    table.with(Width::wrap(100));

    assert_table!(
        ansi_str::AnsiStr::ansi_strip(&table.to_string()),
        "┌──────────────────────────────────────────────────────────────────────────────────────────────────┐"
        "│ 0                                                                                                │"
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
    );
}

test_table!(
    table_1x1_empty,
    {
        Builder::from_iter(vec![vec![""]]).build()
            .with(Style::modern())
            .with(Settings::new(Height::limit(0), Width::increase(10)))
    },
    "┌────────┐"
    "└────────┘"
);

test_table!(
    table_2x2_empty,
    {
        Builder::from_iter(vec![vec![" ", ""], vec![" ", ""]]).build()
            .with(Style::modern())
            .with(Padding::zero())
            .with(Height::list([1, 0]))
    },
    "┌─┬┐"
    "│ ││"
    "├─┼┤"
    "└─┴┘"
);

test_table!(
    table_2x2_empty_height_list_together_with_width_list_dont_work_0,
    {
        Builder::from_iter(vec![vec!["", ""], vec!["", ""]]).build()
            .with(Style::modern())
            .with(Padding::zero())
            .with(Height::list([1, 0]))
            .with(Width::list([1, 0]))
    },
    "┌─┬┐"
    "│ ││"
    "├─┼┤"
    "│ ││"
    "└─┴┘"
);

test_table!(
    table_2x2_empty_height_list_together_with_width_list_dont_work_1,
    {
        Builder::from_iter(vec![vec!["", ""], vec!["", ""]]).build()
            .with(Style::modern())
            .with(Padding::zero())
            .with(Width::list([1, 0]))
            .with(Height::list([1, 0]))
    },
    "┌┬┐"
    "│││"
    "├┼┤"
    "└┴┘"
);
