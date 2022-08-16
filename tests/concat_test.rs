use tabled::{Concat, Style};

use crate::util::{create_table, init_table, test_table};

mod util;

test_table!(
    join_vertical_0,
    {
        let table1 = init_table::<2, 3, _, _>([((0, 0), "123")]).with(Style::psql());
        let table2 = create_table::<2, 3>();
        table1.with(Concat::vertical(table2))
    },
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
    {
        let table1 = init_table::<2, 3, _, _>([((0, 0), "123")]).with(Style::psql());
        let table2 = create_table::<2, 3>();
        table2.with(Concat::vertical(table1))
    },
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
        let table1 = create_table::<2, 3>().with(Style::ascii());
        let table2 = create_table::<2, 3>().with(Style::psql());
        table2.with(Concat::horizontal(table1))
    },
    " N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------+---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    "
);

test_table!(
    join_horizontal_1,
    {
        let table1 = create_table::<2, 3>().with(Style::ascii());
        let table2 = create_table::<2, 3>().with(Style::psql());
        table1.with(Concat::horizontal(table2))
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
        let table1 = create_table::<2, 2>().with(Style::psql());
        let table2 = create_table::<2, 3>().with(Style::psql());
        table1.with(Concat::vertical(table2))
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
        let table1 = create_table::<2, 3>().with(Style::psql());
        let table2 = create_table::<3, 3>().with(Style::psql());
        table1.with(Concat::horizontal(table2))
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
        let table1 = create_table::<2, 3>().with(Style::psql());
        let table2 = create_table::<3, 3>().with(Style::psql());
        table1.with(Concat::horizontal(table2).default_cell("NaN"))
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
        let table1 = create_table::<2, 2>().with(Style::psql());
        let table2 = create_table::<2, 3>().with(Style::psql());
        table1.with(Concat::vertical(table2).default_cell("NaN"))
    },
    " N | column 0 | column 1 |   NaN    "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   NaN    "
    " 1 |   1-0    |   1-1    |   NaN    "
    " N | column 0 | column 1 | column 2 "
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
);
