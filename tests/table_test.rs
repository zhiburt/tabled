use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
use tabled::{Style, Table, TableIteratorExt, Tabled};

use crate::util::create_vector;

mod util;

mod default_types {
    use super::*;

    #[test]
    fn table_str_vec() {
        let data = vec!["hello", "world"];
        let expected = "+-------+\n\
                             | &str  |\n\
                             +-------+\n\
                             | hello |\n\
                             +-------+\n\
                             | world |\n\
                             +-------+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_char_vec() {
        let data = vec!['a', 'b', 'c'];
        let expected = "+------+\n\
                             | char |\n\
                             +------+\n\
                             |  a   |\n\
                             +------+\n\
                             |  b   |\n\
                             +------+\n\
                             |  c   |\n\
                             +------+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_bool_vec() {
        let data = vec![true, false, true];
        let expected = "+-------+\n\
                             | bool  |\n\
                             +-------+\n\
                             | true  |\n\
                             +-------+\n\
                             | false |\n\
                             +-------+\n\
                             | true  |\n\
                             +-------+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_usize_vec() {
        let data = vec![0usize, 1usize, 2usize];
        let expected = "+-------+\n\
                             | usize |\n\
                             +-------+\n\
                             |   0   |\n\
                             +-------+\n\
                             |   1   |\n\
                             +-------+\n\
                             |   2   |\n\
                             +-------+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_isize_vec() {
        let data = vec![0isize, 1isize, 2isize];
        let expected = "+-------+\n\
                             | isize |\n\
                             +-------+\n\
                             |   0   |\n\
                             +-------+\n\
                             |   1   |\n\
                             +-------+\n\
                             |   2   |\n\
                             +-------+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_u8_vec() {
        let data = vec![0u8, 1u8, 2u8];
        let expected = "+----+\n\
                             | u8 |\n\
                             +----+\n\
                             | 0  |\n\
                             +----+\n\
                             | 1  |\n\
                             +----+\n\
                             | 2  |\n\
                             +----+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_u16_vec() {
        let data = vec![0u16, 1u16, 2u16];
        let expected = "+-----+\n\
                             | u16 |\n\
                             +-----+\n\
                             |  0  |\n\
                             +-----+\n\
                             |  1  |\n\
                             +-----+\n\
                             |  2  |\n\
                             +-----+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_u32_vec() {
        let data = vec![0u32, 1u32, 2u32];
        let expected = "+-----+\n\
                             | u32 |\n\
                             +-----+\n\
                             |  0  |\n\
                             +-----+\n\
                             |  1  |\n\
                             +-----+\n\
                             |  2  |\n\
                             +-----+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_u64_vec() {
        let data = vec![0u64, 1u64, 2u64];
        let expected = "+-----+\n\
                             | u64 |\n\
                             +-----+\n\
                             |  0  |\n\
                             +-----+\n\
                             |  1  |\n\
                             +-----+\n\
                             |  2  |\n\
                             +-----+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_u128_vec() {
        let data = vec![0u128, 1u128, 2u128];
        let expected = "+------+\n\
                             | u128 |\n\
                             +------+\n\
                             |  0   |\n\
                             +------+\n\
                             |  1   |\n\
                             +------+\n\
                             |  2   |\n\
                             +------+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_i8_vec() {
        let data = vec![0i8, 1i8, 2i8];
        let expected = "+----+\n\
                             | i8 |\n\
                             +----+\n\
                             | 0  |\n\
                             +----+\n\
                             | 1  |\n\
                             +----+\n\
                             | 2  |\n\
                             +----+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_i16_vec() {
        let data = vec![0i16, 1i16, 2i16];
        let expected = "+-----+\n\
                             | i16 |\n\
                             +-----+\n\
                             |  0  |\n\
                             +-----+\n\
                             |  1  |\n\
                             +-----+\n\
                             |  2  |\n\
                             +-----+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_i32_vec() {
        let data = vec![0i32, 1i32, 2i32];
        let expected = "+-----+\n\
                             | i32 |\n\
                             +-----+\n\
                             |  0  |\n\
                             +-----+\n\
                             |  1  |\n\
                             +-----+\n\
                             |  2  |\n\
                             +-----+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_i64_vec() {
        let data = vec![0i64, 1i64, 2i64];
        let expected = "+-----+\n\
                             | i64 |\n\
                             +-----+\n\
                             |  0  |\n\
                             +-----+\n\
                             |  1  |\n\
                             +-----+\n\
                             |  2  |\n\
                             +-----+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_i128_vec() {
        let data = vec![0i128, 1i128, 2i128];
        let expected = "+------+\n\
                             | i128 |\n\
                             +------+\n\
                             |  0   |\n\
                             +------+\n\
                             |  1   |\n\
                             +------+\n\
                             |  2   |\n\
                             +------+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
    }

    #[test]
    fn table_array() {
        let data = vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]];
        let expected = "+---+---+---+\n\
                             | 0 | 1 | 2 |\n\
                             +---+---+---+\n\
                             | 0 | 1 | 2 |\n\
                             +---+---+---+\n\
                             | 3 | 4 | 5 |\n\
                             +---+---+---+\n\
                             | 6 | 7 | 8 |\n\
                             +---+---+---+\n";

        let table = Table::new(&data).to_string();

        assert_eq!(table, expected);
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
    let expected = "+----+----+\n\
                         | f1 | f2 |\n\
                         +----+----+\n\
                         | 0  | 0  |\n\
                         +----+----+\n\
                         | 1  | 1  |\n\
                         +----+----+\n";

    let table = Table::new(st).to_string();

    assert_eq!(table, expected);
}

#[test]
fn table_empty_vector_structures() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let st: Vec<St> = Vec::new();
    let expected = "+----+----+\n\
                         | f1 | f2 |\n\
                         +----+----+\n";

    let table = Table::new(st).to_string();

    assert_eq!(table, expected);
}

#[test]
fn table_option() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let st = Some(St { f1: 0, f2: "0" });
    let expected = "+----+----+\n\
                         | f1 | f2 |\n\
                         +----+----+\n\
                         | 0  | 0  |\n\
                         +----+----+\n";

    let table = Table::new(st).to_string();

    assert_eq!(table, expected);
}

#[test]
fn table_option_none() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let st: Option<St> = None;
    let expected = "+----+----+\n\
                         | f1 | f2 |\n\
                         +----+----+\n";

    let table = Table::new(st).to_string();

    assert_eq!(table, expected);
}

#[test]
fn table_tuple() {
    let t = ("we are in", 2020);
    let expected = "+-----------+------+\n\
                         |   &str    | i32  |\n\
                         +-----------+------+\n\
                         | we are in | 2020 |\n\
                         +-----------+------+\n";

    let table = Table::new(&[t]).to_string();

    assert_eq!(table, expected);
}

#[test]
fn table_single_tuple() {
    let t = (2020,);
    let expected = "+------+\n\
                         | i32  |\n\
                         +------+\n\
                         | 2020 |\n\
                         +------+\n";

    let table = Table::new(&[t]).to_string();

    assert_eq!(table, expected);
}

#[test]
fn table_tuple_vec() {
    let map = [(0, "Monday"), (1, "Thursday")];
    let expected = "+-----+----------+\n\
                         | i32 |   &str   |\n\
                         +-----+----------+\n\
                         |  0  |  Monday  |\n\
                         +-----+----------+\n\
                         |  1  | Thursday |\n\
                         +-----+----------+\n";

    let table = Table::new(&map).to_string();

    assert_eq!(table, expected);
}

#[test]
fn table_tuple_with_structure_vec() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let map = [(0, St { f1: 0, f2: "0str" }), (1, St { f1: 1, f2: "1str" })];
    let expected = "+-----+----+------+\n\
                         | i32 | f1 |  f2  |\n\
                         +-----+----+------+\n\
                         |  0  | 0  | 0str |\n\
                         +-----+----+------+\n\
                         |  1  | 1  | 1str |\n\
                         +-----+----+------+\n";

    let table = Table::new(&map).to_string();

    assert_eq!(table, expected);
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
    let expected = "+----+\n\
                         | f2 |\n\
                         +----+\n\
                         | 0  |\n\
                         +----+\n\
                         | 1  |\n\
                         +----+\n";

    let table = Table::new(&st).to_string();

    assert_eq!(table, expected);
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

    let expected = "+--------+-----------+-------+\n\
                         | Vowels | Consonant | Digit |\n\
                         +--------+-----------+-------+\n\
                         |   +    |           |       |\n\
                         +--------+-----------+-------+\n\
                         |        |     +     |       |\n\
                         +--------+-----------+-------+\n\
                         |   +    |           |       |\n\
                         +--------+-----------+-------+\n\
                         |   +    |           |       |\n\
                         +--------+-----------+-------+\n\
                         |        |           |   +   |\n\
                         +--------+-----------+-------+\n";

    let table = Table::new(&data).to_string();

    assert_eq!(table, expected);
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

    let expected = "+--------+-----------+\n\
                         | Vowels | Consonant |\n\
                         +--------+-----------+\n\
                         |   +    |           |\n\
                         +--------+-----------+\n\
                         |        |     +     |\n\
                         +--------+-----------+\n\
                         |   +    |           |\n\
                         +--------+-----------+\n\
                         |   +    |           |\n\
                         +--------+-----------+\n\
                         |        |           |\n\
                         +--------+-----------+\n";

    let table = Table::new(&data).to_string();

    assert_eq!(table, expected);
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

    let expected = "+-----+---+----+\n\
                         | i32 | b | c  |\n\
                         +-----+---+----+\n\
                         |  0  | 1 | s1 |\n\
                         +-----+---+----+\n\
                         |  1  | 2 | s2 |\n\
                         +-----+---+----+\n\
                         |  3  | 3 | s3 |\n\
                         +-----+---+----+\n";

    let table = Table::new(&map).to_string();

    assert_eq!(table, expected);
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

    let expected = "+---+----+\n\
                         | b | c  |\n\
                         +---+----+\n\
                         | 1 | s1 |\n\
                         +---+----+\n\
                         | 2 | s2 |\n\
                         +---+----+\n\
                         | 3 | s3 |\n\
                         +---+----+\n";

    let table = Table::new(&map).to_string();

    assert_eq!(table, expected);
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

    let expected = "+---------+----------------+---------------+\n\
                         |  name   |  designed_by   | invented_year |\n\
                         +---------+----------------+---------------+\n\
                         |  C 💕   | Dennis Ritchie |     1972      |\n\
                         +---------+----------------+---------------+\n\
                         | Rust 👍 | Graydon Hoare  |     2010      |\n\
                         +---------+----------------+---------------+\n\
                         |  Go 🧋  |    Rob Pike    |     2009      |\n\
                         +---------+----------------+---------------+\n";

    let table = Table::new(&languages).to_string();

    assert_eq!(table, expected);
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

    // Note: it looks OK in a terminal
    let expected =
    "+------------------------------------+-----------------+-------------------------------+--------+\n\
     |                name                |     author      |             text              | rating |\n\
     +------------------------------------+-----------------+-------------------------------+--------+\n\
     | Rebase vs Merge commit in depth 👋 | Rose Kuphal DVM | A multiline                   |   43   |\n\
     |                                    |                 |  text with 🤯 😳 🥵 🥶        |        |\n\
     |                                    |                 |  a bunch of emojies ☄\u{fe0f} 💥 🔥 🌪 |        |\n\
     +------------------------------------+-----------------+-------------------------------+--------+\n\
     |           Keep it simple           |     Unknown     |              🍳               |  100   |\n\
     +------------------------------------+-----------------+-------------------------------+--------+\n";

    let table = Table::new(&languages).to_string();

    assert_eq!(table, expected);
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
        concat!(
            "      name       | Security | Embeded | Frontend | Unknown \n",
            "-----------------+----------+---------+----------+---------\n",
            " Terri Kshlerin  |          |    +    |          |         \n",
            " Catalina Dicki  |    +     |         |          |         \n",
            " Jennie Schmeler |          |         |    +     |         \n",
            "  Maxim Zhiburt  |          |         |          |    +    \n"
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
        concat!(
            "      name       | Security | Embeded | Frontend | Unknown \n",
            "-----------------+----------+---------+----------+---------\n",
            " Terri Kshlerin  |          |    +    |          |         \n",
            " Catalina Dicki  |    +     |         |          |         \n",
            " Jennie Schmeler |          |         |    +     |         \n",
            "  Maxim Zhiburt  |          |         |          |    +    \n"
        )
    );
}

#[test]
fn build_table_from_iterator() {
    let data = create_vector::<3, 3>();
    let table = Table::from_iter(data).with(Style::psql()).to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
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

    // Note: It doesn't look good in VS Code
    let expected = "┌─────────┬────────────────┬───────────────┐\n\
                         │  name   │  designed_by   │ invented_year │\n\
                         │  C 💕   │ Dennis Ritchie │     1972      │\n\
                         │ Rust 👍 │ Graydon Hoare  │     2010      │\n\
                         │  Go 🧋  │    Rob Pike    │     2009      │\n\
                         └─────────┴────────────────┴───────────────┘\n";

    let table = Table::new(&languages)
        .with(tabled::Style::modern().header_off().horizontal_off())
        .to_string();

    assert_eq!(table, expected);
}
