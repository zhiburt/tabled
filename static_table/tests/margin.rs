use static_table::static_table;

use testing_table::test_table;

test_table!(
    static_table_with_margin,
    static_table!(
        [[1, 2, 123], [1, 2, 123], [1, 2, 123]],
        MARGIN = "1, 2, 3, 4"
    ),
    "                  "
    "                  "
    "                  "
    " +---+---+-----+  "
    " | 1 | 2 | 123 |  "
    " +---+---+-----+  "
    " | 1 | 2 | 123 |  "
    " +---+---+-----+  "
    " | 1 | 2 | 123 |  "
    " +---+---+-----+  "
    "                  "
    "                  "
    "                  "
    "                  "
);
