#![cfg(feature = "std")]

use tabled::{settings::merge::Merge, Table};

use testing_table::test_table;

test_table!(
    merge_horizontal,
    Table::new([[0, 1, 1], [1, 1, 2], [1, 1, 1]]).with(Merge::horizontal()),
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+---+---+---+"
    "| 0 | 1     |"
    "+---+---+---+"
    "| 1     | 2 |"
    "+---+---+---+"
    "| 1         |"
    "+---+---+---+"
);

test_table!(
    merge_horizontal_with_no_duplicates,
    Table::new([[0, 1, 2], [0, 1, 2], [0, 1, 2]]).with(Merge::horizontal()),
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+---+---+---+"
);

test_table!(
    merge_horizontal_empty,
    Table::new([[0usize; 0]]).with(Merge::horizontal()),
    ""
);

test_table!(
    merge_vertical_0,
    Table::new([[0, 3, 5], [0, 3, 3], [0, 2, 3]]).with(Merge::vertical()),
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+   +---+---+"
    "|   | 3 | 5 |"
    "+   +   +---+"
    "+   +---+ 3 +"
    "|   | 2 |   |"
    "+---+---+---+"
);

test_table!(
    merge_vertical_1,
    Table::new([[0, 3, 2], [0, 3, 3], [0, 2, 3]]).with(Merge::vertical()),
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+   +---+   +"
    "+   + 3 +---+"
    "+   +---+ 3 +"
    "|   | 2 |   |"
    "+---+---+---+"
);

test_table!(
    merge_vertical_with_no_duplicates,
    Table::new([[5; 3], [15; 3], [115; 3]]).with(Merge::vertical()),
    "+-----+-----+-----+"
    "| 0   | 1   | 2   |"
    "+-----+-----+-----+"
    "| 5   | 5   | 5   |"
    "+-----+-----+-----+"
    "| 15  | 15  | 15  |"
    "+-----+-----+-----+"
    "| 115 | 115 | 115 |"
    "+-----+-----+-----+"
);

test_table!(
    merge_vertical_empty,
    Table::new([[0usize; 0]]).with(Merge::vertical()),
    ""
);

test_table!(
    merge_horizontal_and_vertical_0,
    Table::new([[3, 3, 5], [3, 7, 8], [9, 10, 11]]).with(Merge::horizontal()).with(Merge::vertical()),
    "+---+----+----+"
    "| 0 | 1  | 2  |"
    "+---+----+----+"
    "| 3      | 5  |"
    "+---+----+----+"
    "| 3 | 7  | 8  |"
    "+---+----+----+"
    "| 9 | 10 | 11 |"
    "+---+----+----+"
);

test_table!(
    merge_horizontal_and_vertical_1,
    Table::new([[0, 1, 1], [1, 1, 2], [1, 1, 1]]).with(Merge::horizontal()).with(Merge::vertical()),
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+   +---+---+"
    "|   | 1     |"
    "+---+---+---+"
    "| 1     | 2 |"
    "+---+---+---+"
    "| 1         |"
    "+---+---+---+"
);

test_table!(
    merge_horizontal_and_vertical_2,
    Table::new([[3, 4, 5], [3, 3, 8], [3, 10, 11]]).with(Merge::horizontal()).with(Merge::vertical()),
    "+---+----+----+"
    "| 0 | 1  | 2  |"
    "+---+----+----+"
    "| 3 | 4  | 5  |"
    "+---+----+----+"
    "| 3      | 8  |"
    "+---+----+----+"
    "| 3 | 10 | 11 |"
    "+---+----+----+"
);

test_table!(
    merge_horizontal_and_vertical_3,
    Table::new([[3, 4, 5], [3, 3, 8], [3, 10, 11]]).with(Merge::vertical()).with(Merge::horizontal()),
    "+---+----+----+"
    "| 0 | 1  | 2  |"
    "+---+----+----+"
    "| 3 | 4  | 5  |"
    "+   +----+----+"
    "|   | 3  | 8  |"
    "+   +----+----+"
    "|   | 10 | 11 |"
    "+---+----+----+"
);

test_table!(
    merge_horizontal_and_vertical_4,
    Table::new([[3, 4, 5], [4, 4, 8], [3, 4, 11]]).with(Merge::vertical()).with(Merge::horizontal()),
    "+---+---+----+"
    "| 0 | 1 | 2  |"
    "+---+---+----+"
    "| 3 | 4 | 5  |"
    "+---+   +----+"
    "| 4 |   | 8  |"
    "+---+   +----+"
    "| 3 |   | 11 |"
    "+---+---+----+"
);

test_table!(
    merge_horizontal_and_vertical_5,
    Table::new([[3, 4, 4], [4, 4, 8], [3, 4, 11]]).with(Merge::vertical()).with(Merge::horizontal()),
    "+---+---+----+"
    "| 0 | 1 | 2  |"
    "+---+---+----+"
    "| 3 | 4 | 4  |"
    "+---+   +----+"
    "| 4 |   | 8  |"
    "+---+   +----+"
    "| 3 |   | 11 |"
    "+---+---+----+"
);

test_table!(
    merge_horizontal_and_vertical_6,
    Table::new([[4, 4, 4], [5, 4, 8], [3, 4, 11]]).with(Merge::vertical()).with(Merge::horizontal()),
    "+---+---+----+"
    "| 0 | 1 | 2  |"
    "+---+---+----+"
    "| 4 | 4 | 4  |"
    "+---+   +----+"
    "| 5 |   | 8  |"
    "+---+   +----+"
    "| 3 |   | 11 |"
    "+---+---+----+"
);

test_table!(
    merge_horizontal_and_vertical_7,
    Table::new([[0, 0, 0], [0, 0, 1], [2, 0, 0]]).with(Merge::horizontal()).with(Merge::vertical()),
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+---+---+---+"
    "| 0         |"
    "+---+---+---+"
    "| 0     | 1 |"
    "+---+---+---+"
    "| 2 | 0     |"
    "+---+---+---+"
);
