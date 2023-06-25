#![cfg(feature = "std")]

use tabled::settings::{Concat, Style};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    join_vertical_0,
    Matrix::new(2, 3).insert((1, 0), "123").with(Style::psql())
        .with(Concat::vertical(Matrix::new(2, 3).to_table()))
        .to_string(),
    "  N  | column 0 | column 1 | column 2 "
    "-----+----------+----------+----------"
    " 123 |   0-0    |   0-1    |   0-2    "
    "  1  |   1-0    |   1-1    |   1-2    "
    "  N  | column 0 | column 1 | column 2 "
    "  0  |   0-0    |   0-1    |   0-2    "
    "  1  |   1-0    |   1-1    |   1-2    "
);

test_table!(
    join_vertical_1,
    Matrix::new(2, 3)
        .with(Concat::vertical(Matrix::new(2, 3).insert((1, 0), "123").with(Style::psql()))),
    "+-----+----------+----------+----------+"
    "|  N  | column 0 | column 1 | column 2 |"
    "+-----+----------+----------+----------+"
    "|  0  |   0-0    |   0-1    |   0-2    |"
    "+-----+----------+----------+----------+"
    "|  1  |   1-0    |   1-1    |   1-2    |"
    "+-----+----------+----------+----------+"
    "|  N  | column 0 | column 1 | column 2 |"
    "+-----+----------+----------+----------+"
    "| 123 |   0-0    |   0-1    |   0-2    |"
    "+-----+----------+----------+----------+"
    "|  1  |   1-0    |   1-1    |   1-2    |"
    "+-----+----------+----------+----------+"
);

test_table!(
    join_horizontal_0,
    {
        let mut table1 = Matrix::table(2, 3);
        table1.with(Style::ascii());
        let mut table2 = Matrix::table(2, 3);
        table2.with(Style::psql());
        table2.with(Concat::horizontal(table1)).to_string()
    },
    " N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------+---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    "
);

test_table!(
    join_horizontal_1,
    {
        let mut table1 = Matrix::table(2, 3);
        table1.with(Style::ascii());
        let mut table2 = Matrix::table(2, 3);
        table2.with(Style::psql());
        table1.with(Concat::horizontal(table2)).to_string()
    },
    "+---+----------+----------+----------+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+---+----------+----------+----------+"
);

test_table!(
    join_vertical_different_size,
    {
        let mut table1 = Matrix::table(2, 2);
        table1.with(Style::psql());
        let mut table2 = Matrix::table(2, 3);
        table2.with(Style::psql());
        table1.with(Concat::vertical(table2)).to_string()
    },
    " N | column 0 | column 1 |          "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |          "
    " 1 |   1-0    |   1-1    |          "
    " N | column 0 | column 1 | column 2 "
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
);

test_table!(
    join_horizontal_different_size,
    {
        let mut table1 = Matrix::table(2, 3);
        table1.with(Style::psql());
        let mut table2 = Matrix::table(3, 3);
        table2.with(Style::psql());
        table1.with(Concat::horizontal(table2)).to_string()
    },
    " N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------+---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    "
    "   |          |          |          | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    join_horizontal_with_not_default_empty_string,
    {
        let mut table1 = Matrix::table(2, 3);
        table1.with(Style::psql());
        let mut table2 = Matrix::table(3, 3);
        table2.with(Style::psql());
        table1.with(Concat::horizontal(table2).default_cell("NaN")).to_string()
    },
    "  N  | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 "
    "-----+----------+----------+----------+---+----------+----------+----------"
    "  0  |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    "
    "  1  |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    "
    " NaN |   NaN    |   NaN    |   NaN    | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    join_vertical_with_not_default_empty_string,
    {
        let mut table1 = Matrix::table(2, 2);
        table1.with(Style::psql());
        let mut table2 = Matrix::table(2, 3);
        table2.with(Style::psql());
        table1.with(Concat::vertical(table2).default_cell("NaN")).to_string()
    },
    " N | column 0 | column 1 |   NaN    "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   NaN    "
    " 1 |   1-0    |   1-1    |   NaN    "
    " N | column 0 | column 1 | column 2 "
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
);
