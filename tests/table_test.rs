// Copyright (c) 2021 Maxim Zhiburt
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

use std::collections::{BTreeMap, BTreeSet};
use tabled::{table, Tabled};

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

    let table = table!(&st);

    assert_eq!(expected, table);
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

    let table = table!(&st);

    assert_eq!(expected, table);
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

    let table = table!(&st);
    assert_eq!(expected, table);
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

    let table = table!(&st);
    assert_eq!(expected, table);
}

#[test]
fn table_tuple() {
    let t = ("we are in", 2020);
    let expected = "+-----------+------+\n\
                         |   &str    | i32  |\n\
                         +-----------+------+\n\
                         | we are in | 2020 |\n\
                         +-----------+------+\n";

    let table = table!(&[t]);
    assert_eq!(expected, table);
}

#[test]
fn table_single_tuple() {
    let t = (2020,);
    let expected = "+------+\n\
                         | i32  |\n\
                         +------+\n\
                         | 2020 |\n\
                         +------+\n";

    let table = table!(&[t]);
    assert_eq!(expected, table);
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

    let table = table!(&map);
    assert_eq!(expected, table);
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

    let table = table!(&map);
    assert_eq!(expected, table);
}

#[test]
fn table_vector_structures_with_hidden_field() {
    #[derive(Tabled)]
    struct St {
        #[header(hidden = true)]
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

    let table = table!(&st);

    assert_eq!(expected, table);
}

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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&data);
        assert_eq!(expected, table);
    }

    #[test]
    fn table_enum() {
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

        let table = table!(&data);
        assert_eq!(expected, table);
    }

    #[test]
    fn table_enum_with_hidden_variant() {
        #[derive(Tabled)]
        enum Letters {
            Vowels {
                character: char,
                lang: u8,
            },
            Consonant(char),
            #[header(hidden)]
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

        let table = table!(&data);
        assert_eq!(expected, table);
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

        let table = table!(&map);
        assert_eq!(expected, table);
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

        let table = table!(&map);
        assert_eq!(expected, table);
    }
}
