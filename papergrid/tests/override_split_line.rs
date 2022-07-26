mod util;

#[test]
fn override_top_test() {
    let mut grid = util::grid::<2, 2>();

    grid.override_split_line(0, "T");

    assert_eq!(
        grid.to_string(),
        concat!(
            "T---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );

    grid.override_split_line(0, " Tab");

    assert_eq!(
        grid.to_string(),
        concat!(
            " Tab+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );

    grid.override_split_line(0, "Table");

    assert_eq!(
        grid.to_string(),
        concat!(
            "Table---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );

    grid.override_split_line(0, "Table T");

    assert_eq!(
        grid.to_string(),
        concat!(
            "Table T-+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );

    grid.override_split_line(0, "Table TES");

    assert_eq!(
        grid.to_string(),
        concat!(
            "Table TES\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );

    grid.override_split_line(0, "Table LONGER THEN LINE");

    assert_eq!(
        grid.to_string(),
        concat!(
            "Table LON\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );
}

#[test]
fn override_bottom_test() {
    let mut grid = util::grid::<2, 2>();

    grid.override_split_line(2, "T");

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "T---+---+",
        )
    );

    grid.override_split_line(2, " Tab");

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            " Tab+---+",
        )
    );

    grid.override_split_line(2, "Table");

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "Table---+",
        )
    );

    grid.override_split_line(2, "Table T");

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "Table T-+",
        )
    );

    grid.override_split_line(2, "Table TES");

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "Table TES",
        )
    );

    grid.override_split_line(2, "Table LONGER THEN LINE");

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "Table LON",
        )
    );
}
