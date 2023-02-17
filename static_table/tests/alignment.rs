use static_table::static_table;

#[test]
fn static_table_with_alignment_right() {
    let table = static_table!(
        [[123, 1234, 12345], [1, 1, 1], [11, 22, 33]],
        ALIGNMENT = "RIGHT"
    );
    let expected = concat!(
        "+-----+------+-------+\n",
        "| 123 | 1234 | 12345 |\n",
        "+-----+------+-------+\n",
        "|   1 |    1 |     1 |\n",
        "+-----+------+-------+\n",
        "|  11 |   22 |    33 |\n",
        "+-----+------+-------+",
    );
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_alignment_left() {
    let table = static_table!(
        [[123, 1234, 12345], [1, 1, 1], [11, 22, 33]],
        ALIGNMENT = "LEFT"
    );
    let expected = concat!(
        "+-----+------+-------+\n",
        "| 123 | 1234 | 12345 |\n",
        "+-----+------+-------+\n",
        "| 1   | 1    | 1     |\n",
        "+-----+------+-------+\n",
        "| 11  | 22   | 33    |\n",
        "+-----+------+-------+",
    );
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_alignment_top() {
    let table = static_table!(
        [["some\nmulti-\nline\nstring", "line", "line"], [1, 1, 1]],
        ALIGNMENT = "TOP"
    );
    let expected = concat!(
        "+--------+------+------+\n",
        "| some   | line | line |\n",
        "| multi- |      |      |\n",
        "| line   |      |      |\n",
        "| string |      |      |\n",
        "+--------+------+------+\n",
        "| 1      | 1    | 1    |\n",
        "+--------+------+------+",
    );
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_alignment_bottom() {
    let table = static_table!(
        [["some\nmulti-\nline\nstring", "line", "line"], [1, 1, 1]],
        ALIGNMENT = "BOTTOM"
    );
    let expected = concat!(
        "+--------+------+------+\n",
        "| some   |      |      |\n",
        "| multi- |      |      |\n",
        "| line   |      |      |\n",
        "| string | line | line |\n",
        "+--------+------+------+\n",
        "| 1      | 1    | 1    |\n",
        "+--------+------+------+",
    );
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_alignment_center() {
    let table = static_table!(
        [[123, 1234, 12345], [1, 1, 1], [11, 22, 33]],
        ALIGNMENT = "CENTER"
    );
    let expected = concat!(
        "+-----+------+-------+\n",
        "| 123 | 1234 | 12345 |\n",
        "+-----+------+-------+\n",
        "|  1  |  1   |   1   |\n",
        "+-----+------+-------+\n",
        "| 11  |  22  |  33   |\n",
        "+-----+------+-------+",
    );
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_alignment_center_vertical() {
    let table = static_table!(
        [["some\nmulti-\nline\nstring", "line", "line"], [1, 1, 1]],
        ALIGNMENT = "CENTER_VERTICAL"
    );
    let expected = concat!(
        "+--------+------+------+\n",
        "| some   |      |      |\n",
        "| multi- | line | line |\n",
        "| line   |      |      |\n",
        "| string |      |      |\n",
        "+--------+------+------+\n",
        "| 1      | 1    | 1    |\n",
        "+--------+------+------+",
    );
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_alignment_combination_bottom_right() {
    let table = static_table!(
        [["some\nmulti-\nline\nstring", "line", "line"], [1, 1, 1]],
        ALIGNMENT = "CENTER_VERTICAL",
        ALIGNMENT = "RIGHT",
    );
    let expected = concat!(
        "+--------+------+------+\n",
        "| some   |      |      |\n",
        "| multi- | line | line |\n",
        "| line   |      |      |\n",
        "| string |      |      |\n",
        "+--------+------+------+\n",
        "|      1 |    1 |    1 |\n",
        "+--------+------+------+",
    );
    assert_eq!(table, expected);
}
