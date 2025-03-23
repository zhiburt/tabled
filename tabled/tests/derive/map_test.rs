#![cfg(all(feature = "derive", feature = "assert"))]

use tabled::{assert::test_table, Table, Tabled};

test_table!(
    test_map_path,
    {
        #[derive(Tabled)]
        struct User {
            id: usize,
            #[tabled(map = "password_string")]
            p: Pass<usize>,
        }

        struct Pass<T>([T; 4]);

        fn password_string(p: &Pass<usize>) -> String {
            p.0.iter().sum::<usize>().to_string()
        }

        let data = [
            User { id: 0, p: Pass([0, 1, 2, 3]) },
            User { id: 1, p: Pass([1, 1, 2, 3]) },
        ];

        Table::new(data)
    },
    "+----+---+"
    "| id | p |"
    "+----+---+"
    "| 0  | 6 |"
    "+----+---+"
    "| 1  | 7 |"
    "+----+---+"
);

test_table!(
    test_map_inline,
    {
        #[derive(Tabled)]
        struct User {
            id: usize,
            #[tabled(map(Pass, "pass"))]
            #[tabled(inline)]
            p: isize,
        }

        #[derive(Tabled)]
        struct Pass(String, String, String);

        fn pass(p: &isize) -> Pass {
            Pass (
                (*p > 0).then_some(String::from("+")).unwrap_or_default(),
                (*p < 0).then_some(String::from("-")).unwrap_or_default(),
                (*p == 0).then_some(String::from("=")).unwrap_or_default(),
            )
        }

        let data = [
            User { id: 0, p: 0 },
            User { id: 1, p: 1 },
            User { id: 1, p: -1 },
        ];

        Table::new(data)
    },
    "+----+---+---+---+"
    "| id | 0 | 1 | 2 |"
    "+----+---+---+---+"
    "| 0  |   |   | = |"
    "+----+---+---+---+"
    "| 1  | + |   |   |"
    "+----+---+---+---+"
    "| 1  |   | - |   |"
    "+----+---+---+---+"
);
