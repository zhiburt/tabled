mod util;

#[test]
fn override_top_test() {
    let mut grid = util::new_grid::<2, 2>();

    grid.override_split_line(0, "T");

    let expected = concat!(
        "T---+---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(0, " Tab");

    let expected = concat!(
        " Tab+---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(0, "Table");

    let expected = concat!(
        "Table---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(0, "Table T");

    let expected = concat!(
        "Table T-+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(0, "Table TES");

    let expected = concat!(
        "Table TES\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(0, "Table LONGER THEN LINE");

    let expected = concat!(
        "Table LON\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n",
    );

    assert_eq!(expected, grid.to_string());
}

#[test]
fn override_bottom_test() {
    let mut grid = util::new_grid::<2, 2>();

    grid.override_split_line(2, "T");

    let expected = concat!(
        "+---+---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "T---+---+\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(2, " Tab");

    let expected = concat!(
        "+---+---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        " Tab+---+\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(2, "Table");

    let expected = concat!(
        "+---+---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "Table---+\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(2, "Table T");

    let expected = concat!(
        "+---+---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "Table T-+\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(2, "Table TES");

    let expected = concat!(
        "+---+---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "Table TES\n",
    );

    assert_eq!(expected, grid.to_string());

    grid.override_split_line(2, "Table LONGER THEN LINE");

    let expected = concat!(
        "+---+---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "Table LON\n",
    );

    assert_eq!(expected, grid.to_string());
}
