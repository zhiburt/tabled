use static_table::static_table;

use tabled::assert::test_table;

test_table!(
    static_table_with_alignment_right,
    static_table!(
        [[123, 1234, 12345], [1, 1, 1], [11, 22, 33]],
        ALIGNMENT = "RIGHT"
    ),
    "+-----+------+-------+"
    "| 123 | 1234 | 12345 |"
    "+-----+------+-------+"
    "|   1 |    1 |     1 |"
    "+-----+------+-------+"
    "|  11 |   22 |    33 |"
    "+-----+------+-------+"
);

test_table!(
    static_table_with_alignment_left,
    static_table!(
        [[123, 1234, 12345], [1, 1, 1], [11, 22, 33]],
        ALIGNMENT = "LEFT"
    ),
    "+-----+------+-------+"
    "| 123 | 1234 | 12345 |"
    "+-----+------+-------+"
    "| 1   | 1    | 1     |"
    "+-----+------+-------+"
    "| 11  | 22   | 33    |"
    "+-----+------+-------+"
);

test_table!(
    static_table_with_alignment_top,
    static_table!(
        [["some\nmulti-\nline\nstring", "line", "line"], [1, 1, 1]],
        ALIGNMENT = "TOP"
    ),
    "+--------+------+------+"
    "| some   | line | line |"
    "| multi- |      |      |"
    "| line   |      |      |"
    "| string |      |      |"
    "+--------+------+------+"
    "| 1      | 1    | 1    |"
    "+--------+------+------+"
);

test_table!(
    static_table_with_alignment_bottom,
    static_table!(
        [["some\nmulti-\nline\nstring", "line", "line"], [1, 1, 1]],
        ALIGNMENT = "BOTTOM"
    ),
    "+--------+------+------+"
    "| some   |      |      |"
    "| multi- |      |      |"
    "| line   |      |      |"
    "| string | line | line |"
    "+--------+------+------+"
    "| 1      | 1    | 1    |"
    "+--------+------+------+"
);

test_table!(
    static_table_with_alignment_center,
    static_table!(
        [[123, 1234, 12345], [1, 1, 1], [11, 22, 33]],
        ALIGNMENT = "CENTER"
    ),
    "+-----+------+-------+"
    "| 123 | 1234 | 12345 |"
    "+-----+------+-------+"
    "|  1  |  1   |   1   |"
    "+-----+------+-------+"
    "| 11  |  22  |  33   |"
    "+-----+------+-------+"
);

test_table!(
    static_table_with_alignment_center_vertical,
    static_table!(
        [["some\nmulti-\nline\nstring", "line", "line"], [1, 1, 1]],
        ALIGNMENT = "CENTER_VERTICAL"
    ),
    "+--------+------+------+"
    "| some   |      |      |"
    "| multi- | line | line |"
    "| line   |      |      |"
    "| string |      |      |"
    "+--------+------+------+"
    "| 1      | 1    | 1    |"
    "+--------+------+------+"
);

test_table!(
    static_table_with_alignment_combination_bottom_right,
    static_table!(
        [["some\nmulti-\nline\nstring", "line", "line"], [1, 1, 1]],
        ALIGNMENT = "CENTER_VERTICAL",
        ALIGNMENT = "RIGHT",
    ),
    "+--------+------+------+"
    "| some   |      |      |"
    "| multi- | line | line |"
    "| line   |      |      |"
    "| string |      |      |"
    "+--------+------+------+"
    "|      1 |    1 |    1 |"
    "+--------+------+------+"
);
