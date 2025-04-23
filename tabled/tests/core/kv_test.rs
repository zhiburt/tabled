#![cfg(all(feature = "std", feature = "assert"))]

use std::iter::FromIterator;

use tabled::{
    assert::test_table,
    builder::Builder,
    settings::{
        formatting::Charset, Height, Highlight, Modify, Padding, Settings, Shadow, Style, Width,
    },
    Table,
};

use crate::matrix::Matrix;

mod default_types {
    use super::*;

    test_table!(
        table_str_vec,
        Table::kv(vec!["hello", "world"]),
        "+------+-------+"
        "| &str | hello |"
        "+------+-------+"
        "| &str | world |"
        "+------+-------+"
    );

    test_table!(
        table_char_vec,
        Table::kv(vec!['a', 'b', 'c']),
        "+------+---+"
        "| char | a |"
        "+------+---+"
        "| char | b |"
        "+------+---+"
        "| char | c |"
        "+------+---+"
    );

    test_table!(
        table_bool_vec,
        Table::kv(vec![true, false, true]),
        "+------+-------+"
        "| bool | true  |"
        "+------+-------+"
        "| bool | false |"
        "+------+-------+"
        "| bool | true  |"
        "+------+-------+"
    );

    test_table!(
        table_usize_vec,
        Table::kv(vec![0usize, 1usize, 2usize]),
        "+-------+---+"
        "| usize | 0 |"
        "+-------+---+"
        "| usize | 1 |"
        "+-------+---+"
        "| usize | 2 |"
        "+-------+---+"
    );

    test_table!(
        table_isize_vec,
        Table::kv(vec![0isize, 1isize, 2isize]),
        "+-------+---+"
        "| isize | 0 |"
        "+-------+---+"
        "| isize | 1 |"
        "+-------+---+"
        "| isize | 2 |"
        "+-------+---+"
    );

    test_table!(
        table_u8_vec,
        Table::kv(vec![0u8, 1u8, 2u8]),
        "+----+---+"
        "| u8 | 0 |"
        "+----+---+"
        "| u8 | 1 |"
        "+----+---+"
        "| u8 | 2 |"
        "+----+---+"
    );

    test_table!(
        table_u16_vec,
        Table::kv(vec![0u16, 1u16, 2u16]),
        "+-----+---+"
        "| u16 | 0 |"
        "+-----+---+"
        "| u16 | 1 |"
        "+-----+---+"
        "| u16 | 2 |"
        "+-----+---+"
    );

    test_table!(
        table_u32_vec,
        Table::kv(vec![0u32, 1u32, 2u32]),
        "+-----+---+"
        "| u32 | 0 |"
        "+-----+---+"
        "| u32 | 1 |"
        "+-----+---+"
        "| u32 | 2 |"
        "+-----+---+"
    );

    test_table!(
        table_u64_vec,
        Table::kv(vec![0u64, 1u64, 2u64]),
        "+-----+---+"
        "| u64 | 0 |"
        "+-----+---+"
        "| u64 | 1 |"
        "+-----+---+"
        "| u64 | 2 |"
        "+-----+---+"
    );

    test_table!(
        table_u128_vec,
        Table::kv(vec![0u128, 1u128, 2u128]),
        "+------+---+"
        "| u128 | 0 |"
        "+------+---+"
        "| u128 | 1 |"
        "+------+---+"
        "| u128 | 2 |"
        "+------+---+"
    );

    test_table!(
        table_i8_vec,
        Table::kv(vec![0i8, 1i8, 2i8]),
        "+----+---+"
        "| i8 | 0 |"
        "+----+---+"
        "| i8 | 1 |"
        "+----+---+"
        "| i8 | 2 |"
        "+----+---+"
    );

    test_table!(
        table_i16_vec,
        Table::kv(vec![0i16, 1, 2]),
        "+-----+---+"
        "| i16 | 0 |"
        "+-----+---+"
        "| i16 | 1 |"
        "+-----+---+"
        "| i16 | 2 |"
        "+-----+---+"
    );

    test_table!(
        table_i32_vec,
        Table::kv(vec![0i32, 1, 2]),
        "+-----+---+"
        "| i32 | 0 |"
        "+-----+---+"
        "| i32 | 1 |"
        "+-----+---+"
        "| i32 | 2 |"
        "+-----+---+"
    );

    test_table!(
        table_i64_vec,
        Table::kv(vec![0i64, 1, 2]),
        "+-----+---+"
        "| i64 | 0 |"
        "+-----+---+"
        "| i64 | 1 |"
        "+-----+---+"
        "| i64 | 2 |"
        "+-----+---+"
    );

    test_table!(
        table_i128_vec,
        Table::kv(vec![0i128, 1, 2]),
        "+------+---+"
        "| i128 | 0 |"
        "+------+---+"
        "| i128 | 1 |"
        "+------+---+"
        "| i128 | 2 |"
        "+------+---+"
    );

    test_table!(
        table_array,
        Table::kv(vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]]),
        "+---+---+"
        "| 0 | 0 |"
        "+---+---+"
        "| 1 | 1 |"
        "+---+---+"
        "| 2 | 2 |"
        "+---+---+"
        "| 0 | 3 |"
        "+---+---+"
        "| 1 | 4 |"
        "+---+---+"
        "| 2 | 5 |"
        "+---+---+"
        "| 0 | 6 |"
        "+---+---+"
        "| 1 | 7 |"
        "+---+---+"
        "| 2 | 8 |"
        "+---+---+"
    );
}

test_table!(
    table_tuple,
    Table::kv(vec![("we are in", 2020)]),
    "+------+-----------+"
    "| &str | we are in |"
    "+------+-----------+"
    "| i32  | 2020      |"
    "+------+-----------+"
);

test_table!(
    table_single_tuple,
    Table::kv(vec![(2020,)]),
    "+-----+------+"
    "| i32 | 2020 |"
    "+-----+------+"
);

test_table!(
    table_tuple_vec,
    #[allow(unknown_lints)]
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::needless_borrows_for_generic_args)]
    Table::kv(&[(0, "Monday"), (1, "Thursday")]),
    "+------+----------+"
    "| i32  | 0        |"
    "+------+----------+"
    "| &str | Monday   |"
    "+------+----------+"
    "| i32  | 1        |"
    "+------+----------+"
    "| &str | Thursday |"
    "+------+----------+"
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
    Table::kv([["This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\r\n\r\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\r\n"]])
        .with(Charset::clean())
        .with(Style::modern()),
    r#"┌───┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐"#
    r#"│ 0 │ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                                                                                                 │"#
    r#"│   │                                                                                                                                                                                                                                                                      │"#
    r#"│   │ For convenience, we are providing full builds for Windows, Linux, and macOS. These are the "all extra features" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies │"#
    r#"│   │                                                                                                                                                                                                                                                                      │"#
    r#"└───┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘"#
);

test_table!(
    multiline_table_test_1,
    Table::kv([["This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\r\n\r\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\r\n"]])
        .with(Charset::clean())
        .with(Style::modern())
        .with(Width::wrap(100)),
    "┌──┬───────────────────────────────────────────────────────────────────────────────────────────────┐"
    "│  │ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: h │"
    "│  │ ttps://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                       │"
    "│  │                                                                                               │"
    "│  │ For convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"a │"
    "│  │ ll extra features\" builds, so be sure you have the requirements to enable all capabilities: h │"
    "│  │ ttps://github.com/nushell/book/blob/master/en/installation.md#dependencies                    │"
    "│  │                                                                                               │"
    "└──┴───────────────────────────────────────────────────────────────────────────────────────────────┘"
);

test_table!(
    multiline_table_test_2,
    Table::kv([["This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\r\n\r\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\r\n"]])
        .with(Modify::new((0, 1)).with(Charset::clean()))
        .with(Style::modern())
        .with(Width::wrap(100)),
    r#"┌──┬───────────────────────────────────────────────────────────────────────────────────────────────┐"#
    r#"│  │ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: h │"#
    r#"│  │ ttps://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                       │"#
    r#"│  │                                                                                               │"#
    r#"│  │ For convenience, we are providing full builds for Windows, Linux, and macOS. These are the "a │"#
    r#"│  │ ll extra features" builds, so be sure you have the requirements to enable all capabilities: h │"#
    r#"│  │ ttps://github.com/nushell/book/blob/master/en/installation.md#dependencies                    │"#
    r#"│  │                                                                                               │"#
    r#"└──┴───────────────────────────────────────────────────────────────────────────────────────────────┘"#
);

#[cfg(feature = "derive")]
mod derived {
    use super::*;

    use std::collections::{BTreeMap, BTreeSet};

    use tabled::{settings::style::Style, Tabled};

    #[derive(Tabled)]
    struct TestType {
        f1: u8,
        f2: &'static str,
    }

    test_table!(
        table_vector_structures,
        Table::kv([TestType { f1: 0, f2: "0" }, TestType { f1: 1, f2: "1" }]),
        "+----+---+"
        "| f1 | 0 |"
        "+----+---+"
        "| f2 | 0 |"
        "+----+---+"
        "| f1 | 1 |"
        "+----+---+"
        "| f2 | 1 |"
        "+----+---+"
    );

    test_table!(
        table_empty_vector_structures,
        Table::kv({
            let v: Vec<TestType> = Vec::new();
            v
        }),
        ""
    );

    test_table!(
        table_option,
        Table::kv(Some(TestType { f1: 0, f2: "0" })),
        "+----+---+"
        "| f1 | 0 |"
        "+----+---+"
        "| f2 | 0 |"
        "+----+---+"
    );

    test_table!(table_option_none, Table::kv(Option::<TestType>::None), "",);

    test_table!(
        table_tuple_with_structure_vec,
        Table::kv([(0, TestType { f1: 0, f2: "0str" }), (1, TestType { f1: 1, f2: "1str" })]),
        "+-----+------+"
        "| i32 | 0    |"
        "+-----+------+"
        "| f1  | 0    |"
        "+-----+------+"
        "| f2  | 0str |"
        "+-----+------+"
        "| i32 | 1    |"
        "+-----+------+"
        "| f1  | 1    |"
        "+-----+------+"
        "| f2  | 1str |"
        "+-----+------+"
    );

    test_table!(
        table_vector_structures_with_hidden_tabled,
        Table::kv({
            #[derive(Tabled)]
            struct St {
                #[allow(dead_code)]
                #[tabled(skip)]
                f1: u8,
                f2: &'static str,
            }

            vec![St { f1: 0, f2: "0" }, St { f1: 1, f2: "1" }]
        }),
        "+----+---+"
        "| f2 | 0 |"
        "+----+---+"
        "| f2 | 1 |"
        "+----+---+"
    );

    test_table!(
        table_enum,
        Table::kv({
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
        "+-----------+---+"
        "| Vowels    | + |"
        "+-----------+---+"
        "| Consonant |   |"
        "+-----------+---+"
        "| Digit     |   |"
        "+-----------+---+"
        "| Vowels    |   |"
        "+-----------+---+"
        "| Consonant | + |"
        "+-----------+---+"
        "| Digit     |   |"
        "+-----------+---+"
        "| Vowels    | + |"
        "+-----------+---+"
        "| Consonant |   |"
        "+-----------+---+"
        "| Digit     |   |"
        "+-----------+---+"
        "| Vowels    | + |"
        "+-----------+---+"
        "| Consonant |   |"
        "+-----------+---+"
        "| Digit     |   |"
        "+-----------+---+"
        "| Vowels    |   |"
        "+-----------+---+"
        "| Consonant |   |"
        "+-----------+---+"
        "| Digit     | + |"
        "+-----------+---+"
    );

    test_table!(
        table_enum_with_hidden_variant,
        Table::kv({
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
        "+-----------+---+"
        "| Vowels    | + |"
        "+-----------+---+"
        "| Consonant |   |"
        "+-----------+---+"
        "| Vowels    |   |"
        "+-----------+---+"
        "| Consonant | + |"
        "+-----------+---+"
        "| Vowels    | + |"
        "+-----------+---+"
        "| Consonant |   |"
        "+-----------+---+"
        "| Vowels    | + |"
        "+-----------+---+"
        "| Consonant |   |"
        "+-----------+---+"
        "| Vowels    |   |"
        "+-----------+---+"
        "| Consonant |   |"
        "+-----------+---+"
    );

    test_table!(
        table_btreemap,
        Table::kv({
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
        "+-----+----+"
        "| i32 | 0  |"
        "+-----+----+"
        "| b   | 1  |"
        "+-----+----+"
        "| c   | s1 |"
        "+-----+----+"
        "| i32 | 1  |"
        "+-----+----+"
        "| b   | 2  |"
        "+-----+----+"
        "| c   | s2 |"
        "+-----+----+"
        "| i32 | 3  |"
        "+-----+----+"
        "| b   | 3  |"
        "+-----+----+"
        "| c   | s3 |"
        "+-----+----+"
    );

    test_table!(
        table_emojie_utf8_style,
        Table::kv({
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
        "┌───────────────┬────────────────┐"
        "│ name          │ C 💕           │"
        "│ designed_by   │ Dennis Ritchie │"
        "│ invented_year │ 1972           │"
        "│ name          │ Rust 👍        │"
        "│ designed_by   │ Graydon Hoare  │"
        "│ invented_year │ 2010           │"
        "│ name          │ Go 🧋          │"
        "│ designed_by   │ Rob Pike       │"
        "│ invented_year │ 2009           │"
        "└───────────────┴────────────────┘"

    );

    test_table!(
        table_btreeset,
        Table::kv({
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
        "| b | 1  |"
        "+---+----+"
        "| c | s1 |"
        "+---+----+"
        "| b | 2  |"
        "+---+----+"
        "| c | s2 |"
        "+---+----+"
        "| b | 3  |"
        "+---+----+"
        "| c | s3 |"
        "+---+----+"
    );

    test_table!(
        table_emojie,
        Table::kv({
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
        "+---------------+----------------+"
        "| name          | C 💕           |"
        "+---------------+----------------+"
        "| designed_by   | Dennis Ritchie |"
        "+---------------+----------------+"
        "| invented_year | 1972           |"
        "+---------------+----------------+"
        "| name          | Rust 👍        |"
        "+---------------+----------------+"
        "| designed_by   | Graydon Hoare  |"
        "+---------------+----------------+"
        "| invented_year | 2010           |"
        "+---------------+----------------+"
        "| name          | Go 🧋          |"
        "+---------------+----------------+"
        "| designed_by   | Rob Pike       |"
        "+---------------+----------------+"
        "| invented_year | 2009           |"
        "+---------------+----------------+"
    );

    test_table!(
        tuple_combination,
        Table::kv({
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
        "+----------+-----------------+"
        "| name     | Terri Kshlerin  |"
        "+----------+-----------------+"
        "| Security |                 |"
        "+----------+-----------------+"
        "| Embedded | +               |"
        "+----------+-----------------+"
        "| Frontend |                 |"
        "+----------+-----------------+"
        "| Unknown  |                 |"
        "+----------+-----------------+"
        "| name     | Catalina Dicki  |"
        "+----------+-----------------+"
        "| Security | +               |"
        "+----------+-----------------+"
        "| Embedded |                 |"
        "+----------+-----------------+"
        "| Frontend |                 |"
        "+----------+-----------------+"
        "| Unknown  |                 |"
        "+----------+-----------------+"
        "| name     | Jennie Schmeler |"
        "+----------+-----------------+"
        "| Security |                 |"
        "+----------+-----------------+"
        "| Embedded |                 |"
        "+----------+-----------------+"
        "| Frontend | +               |"
        "+----------+-----------------+"
        "| Unknown  |                 |"
        "+----------+-----------------+"
        "| name     | Maxim Zhiburt   |"
        "+----------+-----------------+"
        "| Security |                 |"
        "+----------+-----------------+"
        "| Embedded |                 |"
        "+----------+-----------------+"
        "| Frontend |                 |"
        "+----------+-----------------+"
        "| Unknown  | +               |"
        "+----------+-----------------+"

    );

    test_table!(
        table_trait,
        Table::kv({
            #[derive(Tabled)]
            #[tabled(inline)]
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
        "+--------+-----------------+"
        "| name   | Terri Kshlerin  |"
        "+--------+-----------------+"
        "| Domain | Embedded        |"
        "+--------+-----------------+"
        "| name   | Catalina Dicki  |"
        "+--------+-----------------+"
        "| Domain | Security        |"
        "+--------+-----------------+"
        "| name   | Jennie Schmeler |"
        "+--------+-----------------+"
        "| Domain | Frontend        |"
        "+--------+-----------------+"
        "| name   | Maxim Zhiburt   |"
        "+--------+-----------------+"
        "| Domain | Unknown         |"
        "+--------+-----------------+"
    );

    test_table!(
        table_emojie_multiline,
        Table::kv({
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
        "+--------+------------------------------------+"
        "| name   | Rebase vs Merge commit in depth 👋 |"
        "+--------+------------------------------------+"
        "| author | Rose Kuphal DVM                    |"
        "+--------+------------------------------------+"
        "| text   | A multiline                        |"
        "|        |  text with 🤯 😳 🥵 🥶             |"
        "|        |  a bunch of emojies ☄️ 💥 🔥 🌪     |"
        "+--------+------------------------------------+"
        "| rating | 43                                 |"
        "+--------+------------------------------------+"
        "| name   | Keep it simple                     |"
        "+--------+------------------------------------+"
        "| author | Unknown                            |"
        "+--------+------------------------------------+"
        "| text   | 🍳                                 |"
        "+--------+------------------------------------+"
        "| rating | 100                                |"
        "+--------+------------------------------------+"
    );
}

#[cfg(feature = "ansi")]
#[test]
fn multiline_table_test2() {
    use tabled::assert::assert_table;

    let data = &[
        ["\u{1b}[37mThis is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html\n\nFor convenience, we are providing full builds for Windows, Linux, and macOS. These are the \"all extra features\" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies\n\u{1b}[0m"],
    ];

    let mut table = Table::kv(data);
    table.with(Style::modern());

    assert_table!(
        ansi_str::AnsiStr::ansi_strip(&table.to_string()),
        r#"┌───┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐"#
        r#"│ 0 │ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: https://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                                                                                                 │"#
        r#"│   │                                                                                                                                                                                                                                                                      │"#
        r#"│   │ For convenience, we are providing full builds for Windows, Linux, and macOS. These are the "all extra features" builds, so be sure you have the requirements to enable all capabilities: https://github.com/nushell/book/blob/master/en/installation.md#dependencies │"#
        r#"│   │                                                                                                                                                                                                                                                                      │"#
        r#"└───┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘"#
    );

    table.with(Width::wrap(100));

    assert_table!(
        ansi_str::AnsiStr::ansi_strip(&table.to_string()),
        r#"┌──┬───────────────────────────────────────────────────────────────────────────────────────────────┐"#
        r#"│  │ This is the 0.19 release of Nushell. If you'd like to read more about it, please check out: h │"#
        r#"│  │ ttps://www.nushell.sh/blog/2020/09/01/nushell_0_19.html                                       │"#
        r#"│  │                                                                                               │"#
        r#"│  │ For convenience, we are providing full builds for Windows, Linux, and macOS. These are the "a │"#
        r#"│  │ ll extra features" builds, so be sure you have the requirements to enable all capabilities: h │"#
        r#"│  │ ttps://github.com/nushell/book/blob/master/en/installation.md#dependencies                    │"#
        r#"│  │                                                                                               │"#
        r#"└──┴───────────────────────────────────────────────────────────────────────────────────────────────┘"#
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
    table_2x2_empty_height_list_together_with_width_list_work_0,
    {
        Builder::from_iter(vec![vec!["", ""], vec!["", ""]]).build()
            .with(Style::modern())
            .with(Padding::zero())
            .with(Height::list([1, 1]))
            .with(Width::list([1, 0]))
    },
    "┌─┬┐"
    "│ ││"
    "├─┼┤"
    "│ ││"
    "└─┴┘"
);

test_table!(
    table_2x2_empty_height_list_together_with_width_list_work_1,
    {
        Builder::from_iter(vec![vec!["", ""], vec!["", ""]]).build()
            .with(Style::modern())
            .with(Padding::zero())
            .with(Width::list([1, 0]))
            .with(Height::list([1, 0]))
    },
    "┌─┬┐"
    "│ ││"
    "├─┼┤"
    "└─┴┘"
);

test_table!(
    table_modify_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .modify((1, 0), "Hello World")
        .modify((0, 1), "Hello World 2"),
    "|      N      | Hello World 2 | column 1 | column 2 |"
    "|-------------|---------------|----------|----------|"
    "| Hello World |      0-0      |   0-1    |   0-2    |"
    "|      1      |      1-0      |   1-1    |   1-2    |"
    "|      2      |      2-0      |   2-1    |   2-2    |"
);

test_table!(
    table_tuple_settings_list_0_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .modify((1, 0), ("Hello World", "Hello World 2")),
    "|       N       | column 0 | column 1 | column 2 |"
    "|---------------|----------|----------|----------|"
    "| Hello World 2 |   0-0    |   0-1    |   0-2    |"
    "|       1       |   1-0    |   1-1    |   1-2    |"
    "|       2       |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    table_tuple_settings_list_1_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .modify((1, 0), ("Hello World", Padding::new(2, 2, 1, 1), "1")),
    "|  N  | column 0 | column 1 | column 2 |"
    "|-----|----------|----------|----------|"
    "|     |   0-0    |   0-1    |   0-2    |"
    "|  1  |          |          |          |"
    "|     |          |          |          |"
    "|  1  |   1-0    |   1-1    |   1-2    |"
    "|  2  |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    table_tuple_settings_list_2_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with((Padding::new(2, 2, 0, 0), Highlight::outline((0, 0), '*'), Shadow::new(5))),
    "*******                                            "
    "*  N  *  column 0  |  column 1  |  column 2  |▒▒▒▒▒"
    "*******------------|------------|------------|▒▒▒▒▒"
    "|  0  |    0-0     |    0-1     |    0-2     |▒▒▒▒▒"
    "|  1  |    1-0     |    1-1     |    1-2     |▒▒▒▒▒"
    "|  2  |    2-0     |    2-1     |    2-2     |▒▒▒▒▒"
    " ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
    " ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
    " ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
    " ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
    " ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
);
