#![cfg(feature = "std")]

use tabled::tables::IterTable;

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    iter_table,
    IterTable::new(Matrix::with_no_frame(3, 3).to_vec()),
    "+-----+-----+-----+"
    "| 0-0 | 0-1 | 0-2 |"
    "+-----+-----+-----+"
    "| 1-0 | 1-1 | 1-2 |"
    "+-----+-----+-----+"
    "| 2-0 | 2-1 | 2-2 |"
    "+-----+-----+-----+"
);

test_table!(
    iter_table_cols,
    IterTable::new(Matrix::with_no_frame(3, 3).to_vec()).columns(3),
    "+-----+-----+-----+"
    "| 0-0 | 0-1 | 0-2 |"
    "+-----+-----+-----+"
    "| 1-0 | 1-1 | 1-2 |"
    "+-----+-----+-----+"
    "| 2-0 | 2-1 | 2-2 |"
    "+-----+-----+-----+"
);

test_table!(
    iter_table_cols_less,
    IterTable::new(Matrix::with_no_frame(3, 3).to_vec()).columns(2),
    "+-----+-----+"
    "| 0-0 | 0-1 |"
    "+-----+-----+"
    "| 1-0 | 1-1 |"
    "+-----+-----+"
    "| 2-0 | 2-1 |"
    "+-----+-----+"
);

test_table!(
    iter_table_cols_zero,
    IterTable::new(Matrix::with_no_frame(3, 3).to_vec()).columns(0),
    ""
);

test_table!(
    iter_table_iterator,
    {
        let mut buf = String::new();
        IterTable::new((0..3).map(|i: usize| (0..5).map(move |j: usize| format!("{i},{j}")))).fmt(&mut buf).unwrap();
        buf
    },
    "+-----+-----+-----+-----+-----+"
    "| 0,0 | 0,1 | 0,2 | 0,3 | 0,4 |"
    "+-----+-----+-----+-----+-----+"
    "| 1,0 | 1,1 | 1,2 | 1,3 | 1,4 |"
    "+-----+-----+-----+-----+-----+"
    "| 2,0 | 2,1 | 2,2 | 2,3 | 2,4 |"
    "+-----+-----+-----+-----+-----+"
);

test_table!(
    iter_table_width,
    IterTable::new(Matrix::with_no_frame(3, 3).to_vec()).width(2),
    "+----+----+----+"
    "| 0- | 0- | 0- |"
    "+----+----+----+"
    "| 1- | 1- | 1- |"
    "+----+----+----+"
    "| 2- | 2- | 2- |"
    "+----+----+----+"
);

test_table!(
    iter_table_height_does_not_work,
    IterTable::new(Matrix::with_no_frame(3, 3).to_vec()).height(5),
    "+-----+-----+-----+"
    "| 0-0 | 0-1 | 0-2 |"
    "|     |     |     |"
    "|     |     |     |"
    "|     |     |     |"
    "|     |     |     |"
    "+-----+-----+-----+"
    "| 1-0 | 1-1 | 1-2 |"
    "|     |     |     |"
    "|     |     |     |"
    "|     |     |     |"
    "|     |     |     |"
    "+-----+-----+-----+"
    "| 2-0 | 2-1 | 2-2 |"
    "|     |     |     |"
    "|     |     |     |"
    "|     |     |     |"
    "|     |     |     |"
    "+-----+-----+-----+"
);

test_table!(
    iter_table_sniff_0,
    IterTable::new(Matrix::with_no_frame(3, 3).to_vec()).sniff(0),
    ""
);

test_table!(
    iter_table_multiline,
    IterTable::new(
        vec![
            vec!["0", "1", "2", "3"],
            vec!["0\n1\n2\n3\n4", "0\n1\n2\n\n\n3\n4", "0\n1\n2\n3\n4\n\n\n", "0\n1\n2\n\n\n3\n4\n"]
        ]
    ),
    "+---+---+---+---+"
    "| 0 | 1 | 2 | 3 |"
    "+---+---+---+---+"
    "| 0 | 0 | 0 | 0 |"
    "| 1 | 1 | 1 | 1 |"
    "| 2 | 2 | 2 | 2 |"
    "| 3 |   | 3 |   |"
    "| 4 |   | 4 |   |"
    "|   | 3 |   | 3 |"
    "|   | 4 |   | 4 |"
    "|   |   |   |   |"
    "+---+---+---+---+"
);

test_table!(
    iter_table_multiline_sniff_1,
    IterTable::new(
        vec![
            vec!["0", "1", "2", "3"],
            vec!["0\n1\n2\n3\n4", "0\n1\n2\n\n\n3\n4", "0\n1\n2\n3\n4\n\n\n", "0\n1\n2\n\n\n3\n4\n"]
        ]
    )
    .sniff(1),
    "+---+---+---+---+\n| 0 | 1 | 2 | 3 |\n+---+---+---+---+\n| 0\n1\n2\n3\n4 | 0\n1\n2\n\n\n3\n4 | 0\n1\n2\n3\n4\n\n\n | 0\n1\n2\n\n\n3\n4\n |\n+---+---+---+---+"
);

test_table!(
    iter_table_multiline_sniff_2,
    IterTable::new(
        vec![
            vec!["0", "1", "2", "3"],
            vec!["0\n1\n2\n3\n4", "0\n1\n2\n\n\n3\n4", "0\n1\n2\n3\n4\n\n\n", "0\n1\n2\n\n\n3\n4\n"],
            vec!["0\n1\n2\n3\n4", "0\n1\n2\n\n\n3\n4", "0\n1\n2\n3\n4\n\n\n", "0\n1\n2\n\n\n3\n4\n"],
        ]
    )
    .sniff(2),
    "+---+---+---+---+\n| 0 | 1 | 2 | 3 |\n+---+---+---+---+\n| 0 | 0 | 0 | 0 |\n| 1 | 1 | 1 | 1 |\n| 2 | 2 | 2 | 2 |\n| 3 |   | 3 |   |\n| 4 |   | 4 |   |\n|   | 3 |   | 3 |\n|   | 4 |   | 4 |\n|   |   |   |   |\n+---+---+---+---+\n| 0\n1\n2\n3\n4 | 0\n1\n2\n\n\n3\n4 | 0\n1\n2\n3\n4\n\n\n | 0\n1\n2\n\n\n3\n4\n |\n+---+---+---+---+"
);

test_table!(
    iter_table_multiline_height_work,
    IterTable::new(
        vec![
            vec!["0", "1", "2", "3"],
            vec!["0\n1\n2\n3\n4", "0\n1\n2\n\n\n3\n4", "0\n1\n2\n3\n4\n\n\n", "0\n1\n2\n\n\n3\n4\n"]
        ]
    )
    .height(3)
    ,
    "+---+---+---+---+"
    "| 0 | 1 | 2 | 3 |"
    "|   |   |   |   |"
    "|   |   |   |   |"
    "+---+---+---+---+"
    "| 0 | 0 | 0 | 0 |"
    "| 1 | 1 | 1 | 1 |"
    "| 2 | 2 | 2 | 2 |"
    "+---+---+---+---+"
);

test_table!(
    iter_table_sniff_cut,
    IterTable::new(
        vec![
            vec!["12", "12", "22", "32"],
            vec!["0", "0", "0", "0"],
            vec!["023", "123", "223", "323"],
        ]
    )
    .sniff(2)
    ,
    "+----+----+----+----+"
    "| 12 | 12 | 22 | 32 |"
    "+----+----+----+----+"
    "| 0  | 0  | 0  | 0  |"
    "+----+----+----+----+"
    "| 02 | 12 | 22 | 32 |"
    "+----+----+----+----+"
);

test_table!(
    iter_table_sniff,
    IterTable::new(
        vec![
            vec!["023", "123", "223", "323"],
            vec!["12", "12", "22", "32"],
            vec!["0", "0", "0", "0"],
        ]
    )
    .sniff(2)
    ,
    "+-----+-----+-----+-----+"
    "| 023 | 123 | 223 | 323 |"
    "+-----+-----+-----+-----+"
    "| 12  | 12  | 22  | 32  |"
    "+-----+-----+-----+-----+"
    "| 0   | 0   | 0   | 0   |"
    "+-----+-----+-----+-----+"
);
