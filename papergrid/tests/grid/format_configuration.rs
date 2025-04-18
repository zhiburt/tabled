#![cfg(feature = "std")]

use papergrid::config::{AlignmentHorizontal, AlignmentVertical, Entity, Formatting, Position};

use crate::util::grid;
use testing_table::static_table;

#[test]
fn formatting_test() {
    let tests = [
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Top,
            Formatting::new(false, false, true),
            static_table!(
                "+-------------+----------+"
                "|A long string|          |"
                "|             |          |"
                "|             |          |"
                "|             |A         |"
                "|             |    string|"
                "|             |with      |"
                "|             | new      |"
                "|             |line      |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|1-0          |1-1       |"
                "+-------------+----------+"
                "|A one more   |...       |"
                "|    string   |          |"
                "|with         |          |"
                "| new         |          |"
                "|line         |          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Top,
            Formatting::new(true, false, true),
            static_table!(
                "+-------------+----------+"
                "|A long string|          |"
                "|             |          |"
                "|             |          |"
                "|             |A         |"
                "|             |string    |"
                "|             |with      |"
                "|             |new       |"
                "|             |line      |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|1-0          |1-1       |"
                "+-------------+----------+"
                "|A one more   |...       |"
                "|string       |          |"
                "|with         |          |"
                "|new          |          |"
                "|line         |          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Top,
            Formatting::new(true, true, true),
            static_table!(
                "+-------------+----------+"
                "|A long string|A         |"
                "|             |string    |"
                "|             |with      |"
                "|             |new       |"
                "|             |line      |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|1-0          |1-1       |"
                "+-------------+----------+"
                "|A one more   |...       |"
                "|string       |          |"
                "|with         |          |"
                "|new          |          |"
                "|line         |          |"
                "+-------------+----------+"
            ),
        ),
        //
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Top,
            Formatting::new(false, false, true),
            static_table!(
                "+-------------+----------+"
                "|A long string|          |"
                "|             |          |"
                "|             |          |"
                "|             |    A     |"
                "|             |    string|"
                "|             |   with   |"
                "|             |    new   |"
                "|             |   line   |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|     1-0     |   1-1    |"
                "+-------------+----------+"
                "| A one more  |   ...    |"
                "|     string  |          |"
                "|    with     |          |"
                "|     new     |          |"
                "|    line     |          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Top,
            Formatting::new(true, false, true),
            static_table!(
                "+-------------+----------+"
                "|A long string|          |"
                "|             |          |"
                "|             |          |"
                "|             |    A     |"
                "|             |  string  |"
                "|             |   with   |"
                "|             |   new    |"
                "|             |   line   |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|     1-0     |   1-1    |"
                "+-------------+----------+"
                "| A one more  |   ...    |"
                "|   string    |          |"
                "|    with     |          |"
                "|     new     |          |"
                "|    line     |          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Top,
            Formatting::new(true, true, true),
            static_table!(
                "+-------------+----------+"
                "|A long string|    A     |"
                "|             |  string  |"
                "|             |   with   |"
                "|             |   new    |"
                "|             |   line   |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|     1-0     |   1-1    |"
                "+-------------+----------+"
                "| A one more  |   ...    |"
                "|   string    |          |"
                "|    with     |          |"
                "|     new     |          |"
                "|    line     |          |"
                "+-------------+----------+"
            ),
        ),
        //
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Top,
            Formatting::new(false, false, true),
            static_table!(
                "+-------------+----------+"
                "|A long string|          |"
                "|             |          |"
                "|             |          |"
                "|             |         A|"
                "|             |    string|"
                "|             |      with|"
                "|             |       new|"
                "|             |      line|"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|          1-0|       1-1|"
                "+-------------+----------+"
                "|   A one more|       ...|"
                "|       string|          |"
                "|         with|          |"
                "|          new|          |"
                "|         line|          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Top,
            Formatting::new(true, false, true),
            static_table!(
                "+-------------+----------+"
                "|A long string|          |"
                "|             |          |"
                "|             |          |"
                "|             |         A|"
                "|             |    string|"
                "|             |      with|"
                "|             |       new|"
                "|             |      line|"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|          1-0|       1-1|"
                "+-------------+----------+"
                "|   A one more|       ...|"
                "|       string|          |"
                "|         with|          |"
                "|          new|          |"
                "|         line|          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Top,
            Formatting::new(true, true, true),
            static_table!(
                "+-------------+----------+"
                "|A long string|         A|"
                "|             |    string|"
                "|             |      with|"
                "|             |       new|"
                "|             |      line|"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|          1-0|       1-1|"
                "+-------------+----------+"
                "|   A one more|       ...|"
                "|       string|          |"
                "|         with|          |"
                "|          new|          |"
                "|         line|          |"
                "+-------------+----------+"
            ),
        ),
        // asd
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Center,
            Formatting::new(false, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |A         |"
                "|             |    string|"
                "|A long string|with      |"
                "|             | new      |"
                "|             |line      |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|1-0          |1-1       |"
                "+-------------+----------+"
                "|A one more   |          |"
                "|    string   |          |"
                "|with         |...       |"
                "| new         |          |"
                "|line         |          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Center,
            Formatting::new(true, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |A         |"
                "|             |string    |"
                "|A long string|with      |"
                "|             |new       |"
                "|             |line      |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|1-0          |1-1       |"
                "+-------------+----------+"
                "|A one more   |          |"
                "|string       |          |"
                "|with         |...       |"
                "|new          |          |"
                "|line         |          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Center,
            Formatting::new(true, true, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |A         |"
                "|             |string    |"
                "|A long string|with      |"
                "|             |new       |"
                "|             |line      |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|1-0          |1-1       |"
                "+-------------+----------+"
                "|A one more   |          |"
                "|string       |          |"
                "|with         |...       |"
                "|new          |          |"
                "|line         |          |"
                "+-------------+----------+"
            ),
        ),
        //
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Center,
            Formatting::new(false, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |    A     |"
                "|             |    string|"
                "|A long string|   with   |"
                "|             |    new   |"
                "|             |   line   |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|     1-0     |   1-1    |"
                "+-------------+----------+"
                "| A one more  |          |"
                "|     string  |          |"
                "|    with     |   ...    |"
                "|     new     |          |"
                "|    line     |          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Center,
            Formatting::new(true, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |    A     |"
                "|             |  string  |"
                "|A long string|   with   |"
                "|             |   new    |"
                "|             |   line   |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|     1-0     |   1-1    |"
                "+-------------+----------+"
                "| A one more  |          |"
                "|   string    |          |"
                "|    with     |   ...    |"
                "|     new     |          |"
                "|    line     |          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Center,
            Formatting::new(true, true, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |    A     |"
                "|             |  string  |"
                "|A long string|   with   |"
                "|             |   new    |"
                "|             |   line   |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|     1-0     |   1-1    |"
                "+-------------+----------+"
                "| A one more  |          |"
                "|   string    |          |"
                "|    with     |   ...    |"
                "|     new     |          |"
                "|    line     |          |"
                "+-------------+----------+"
            ),
        ),
        //
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Center,
            Formatting::new(false, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |         A|"
                "|             |    string|"
                "|A long string|      with|"
                "|             |       new|"
                "|             |      line|"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|          1-0|       1-1|"
                "+-------------+----------+"
                "|   A one more|          |"
                "|       string|          |"
                "|         with|       ...|"
                "|          new|          |"
                "|         line|          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Center,
            Formatting::new(true, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |         A|"
                "|             |    string|"
                "|A long string|      with|"
                "|             |       new|"
                "|             |      line|"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|          1-0|       1-1|"
                "+-------------+----------+"
                "|   A one more|          |"
                "|       string|          |"
                "|         with|       ...|"
                "|          new|          |"
                "|         line|          |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Center,
            Formatting::new(true, true, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |         A|"
                "|             |    string|"
                "|A long string|      with|"
                "|             |       new|"
                "|             |      line|"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "+-------------+----------+"
                "|          1-0|       1-1|"
                "+-------------+----------+"
                "|   A one more|          |"
                "|       string|          |"
                "|         with|       ...|"
                "|          new|          |"
                "|         line|          |"
                "+-------------+----------+"
            ),
        ),
        //
        // asd
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Bottom,
            Formatting::new(false, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |A         |"
                "|             |    string|"
                "|             |with      |"
                "|             | new      |"
                "|             |line      |"
                "|             |          |"
                "|             |          |"
                "|A long string|          |"
                "+-------------+----------+"
                "|1-0          |1-1       |"
                "+-------------+----------+"
                "|A one more   |          |"
                "|    string   |          |"
                "|with         |          |"
                "| new         |          |"
                "|line         |...       |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Bottom,
            Formatting::new(true, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |A         |"
                "|             |string    |"
                "|             |with      |"
                "|             |new       |"
                "|             |line      |"
                "|             |          |"
                "|             |          |"
                "|A long string|          |"
                "+-------------+----------+"
                "|1-0          |1-1       |"
                "+-------------+----------+"
                "|A one more   |          |"
                "|string       |          |"
                "|with         |          |"
                "|new          |          |"
                "|line         |...       |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Bottom,
            Formatting::new(true, true, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |A         |"
                "|             |string    |"
                "|             |with      |"
                "|             |new       |"
                "|A long string|line      |"
                "+-------------+----------+"
                "|1-0          |1-1       |"
                "+-------------+----------+"
                "|A one more   |          |"
                "|string       |          |"
                "|with         |          |"
                "|new          |          |"
                "|line         |...       |"
                "+-------------+----------+"
            ),
        ),
        //
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Bottom,
            Formatting::new(false, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |    A     |"
                "|             |    string|"
                "|             |   with   |"
                "|             |    new   |"
                "|             |   line   |"
                "|             |          |"
                "|             |          |"
                "|A long string|          |"
                "+-------------+----------+"
                "|     1-0     |   1-1    |"
                "+-------------+----------+"
                "| A one more  |          |"
                "|     string  |          |"
                "|    with     |          |"
                "|     new     |          |"
                "|    line     |   ...    |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Bottom,
            Formatting::new(true, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |    A     |"
                "|             |  string  |"
                "|             |   with   |"
                "|             |   new    |"
                "|             |   line   |"
                "|             |          |"
                "|             |          |"
                "|A long string|          |"
                "+-------------+----------+"
                "|     1-0     |   1-1    |"
                "+-------------+----------+"
                "| A one more  |          |"
                "|   string    |          |"
                "|    with     |          |"
                "|     new     |          |"
                "|    line     |   ...    |"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Bottom,
            Formatting::new(true, true, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |    A     |"
                "|             |  string  |"
                "|             |   with   |"
                "|             |   new    |"
                "|A long string|   line   |"
                "+-------------+----------+"
                "|     1-0     |   1-1    |"
                "+-------------+----------+"
                "| A one more  |          |"
                "|   string    |          |"
                "|    with     |          |"
                "|     new     |          |"
                "|    line     |   ...    |"
                "+-------------+----------+"
            ),
        ),
        //
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Bottom,
            Formatting::new(false, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |         A|"
                "|             |    string|"
                "|             |      with|"
                "|             |       new|"
                "|             |      line|"
                "|             |          |"
                "|             |          |"
                "|A long string|          |"
                "+-------------+----------+"
                "|          1-0|       1-1|"
                "+-------------+----------+"
                "|   A one more|          |"
                "|       string|          |"
                "|         with|          |"
                "|          new|          |"
                "|         line|       ...|"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Bottom,
            Formatting::new(true, false, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |         A|"
                "|             |    string|"
                "|             |      with|"
                "|             |       new|"
                "|             |      line|"
                "|             |          |"
                "|             |          |"
                "|A long string|          |"
                "+-------------+----------+"
                "|          1-0|       1-1|"
                "+-------------+----------+"
                "|   A one more|          |"
                "|       string|          |"
                "|         with|          |"
                "|          new|          |"
                "|         line|       ...|"
                "+-------------+----------+"
            ),
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Bottom,
            Formatting::new(true, true, true),
            static_table!(
                "+-------------+----------+"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |          |"
                "|             |         A|"
                "|             |    string|"
                "|             |      with|"
                "|             |       new|"
                "|A long string|      line|"
                "+-------------+----------+"
                "|          1-0|       1-1|"
                "+-------------+----------+"
                "|   A one more|          |"
                "|       string|          |"
                "|         with|          |"
                "|          new|          |"
                "|         line|       ...|"
                "+-------------+----------+"
            ),
        ),
    ];

    let grid = grid(3, 2)
        .change_cell(Position::new(0, 0), "A long string")
        .change_cell(
            Position::new(0, 1),
            "\n\n\nA\n    string\nwith\n new\nline\n\n\n",
        )
        .change_cell(
            Position::new(2, 0),
            "A one more\n    string\nwith\n new\nline",
        )
        .change_cell(Position::new(2, 1), "...");

    for (i, test) in tests.iter().enumerate() {
        let table = grid
            .clone()
            .config(|cfg| {
                cfg.set_alignment_horizontal(Entity::Global, test.0);
                cfg.set_alignment_vertical(Entity::Global, test.1);
                cfg.set_line_alignment(Entity::Global, test.2.allow_lines_alignment);
                cfg.set_trim_horizontal(Entity::Global, test.2.horizontal_trim);
                cfg.set_trim_vertical(Entity::Global, test.2.vertical_trim);
            })
            .clone()
            .build();

        let expected = test.3;
        assert_eq!(table, expected, "test case #{i:?} failed");
    }
}

#[test]
fn formatting_empty_test() {
    for (rows, cols) in [(0, 0), (0, 4), (4, 0)] {
        let formatting = Formatting::new(true, true, true);
        assert_eq!(
            grid(rows, cols)
                .config(|cfg| {
                    cfg.set_line_alignment(Entity::Global, formatting.allow_lines_alignment);
                    cfg.set_trim_horizontal(Entity::Global, formatting.horizontal_trim);
                    cfg.set_trim_vertical(Entity::Global, formatting.vertical_trim);
                })
                .build(),
            ""
        );
    }
}

#[test]
fn formatting_1x1_test() {
    let json = r#"
{
    "id": "0001",
    "batters": {
        "batter": [
            { "id": "1002", "type": "Chocolate" },
        ]
    },
    "topping": [
        { "id": "5003", "type": "Chocolate" },
        { "id": "5004", "type": "Maple" }
    ]
}"#;

    let grid = grid(1, 1).data([[json]]);

    assert_eq!(
        grid.clone()
            .config(
                |cfg| cfg.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Left)
            )
            .build(),
        static_table!(
            r#"+--------------------------------------------------+"#
            r#"|                                                  |"#
            r#"|{                                                 |"#
            r#"|    "id": "0001",                                 |"#
            r#"|    "batters": {                                  |"#
            r#"|        "batter": [                               |"#
            r#"|            { "id": "1002", "type": "Chocolate" },|"#
            r#"|        ]                                         |"#
            r#"|    },                                            |"#
            r#"|    "topping": [                                  |"#
            r#"|        { "id": "5003", "type": "Chocolate" },    |"#
            r#"|        { "id": "5004", "type": "Maple" }         |"#
            r#"|    ]                                             |"#
            r#"|}                                                 |"#
            r#"+--------------------------------------------------+"#
        ),
    );

    assert_eq!(
        grid.clone()
            .config(|cfg| cfg.set_line_alignment(Entity::Global, true))
            .build(),
        static_table!(
            r#"+--------------------------------------------------+"#
            r#"|                                                  |"#
            r#"|{                                                 |"#
            r#"|    "id": "0001",                                 |"#
            r#"|    "batters": {                                  |"#
            r#"|        "batter": [                               |"#
            r#"|            { "id": "1002", "type": "Chocolate" },|"#
            r#"|        ]                                         |"#
            r#"|    },                                            |"#
            r#"|    "topping": [                                  |"#
            r#"|        { "id": "5003", "type": "Chocolate" },    |"#
            r#"|        { "id": "5004", "type": "Maple" }         |"#
            r#"|    ]                                             |"#
            r#"|}                                                 |"#
            r#"+--------------------------------------------------+"#
        ),
    );

    assert_eq!(
        grid.clone()
            .config(|cfg| cfg.set_line_alignment(Entity::Global, true))
            .config(|cfg| cfg.set_trim_horizontal(Entity::Global, true))
            .build(),
        static_table!(
            r#"+--------------------------------------------------+"#
            r#"|                                                  |"#
            r#"|{                                                 |"#
            r#"|"id": "0001",                                     |"#
            r#"|"batters": {                                      |"#
            r#"|"batter": [                                       |"#
            r#"|{ "id": "1002", "type": "Chocolate" },            |"#
            r#"|]                                                 |"#
            r#"|},                                                |"#
            r#"|"topping": [                                      |"#
            r#"|{ "id": "5003", "type": "Chocolate" },            |"#
            r#"|{ "id": "5004", "type": "Maple" }                 |"#
            r#"|]                                                 |"#
            r#"|}                                                 |"#
            r#"+--------------------------------------------------+"#
        ),
    );

    assert_eq!(
        grid.config(|cfg| cfg.set_line_alignment(Entity::Global, true))
            .config(|cfg| cfg.set_trim_horizontal(Entity::Global, true))
            .config(|cfg| cfg.set_trim_vertical(Entity::Global, true))
            .build(),
        static_table!(
            r#"+--------------------------------------------------+"#
            r#"|{                                                 |"#
            r#"|"id": "0001",                                     |"#
            r#"|"batters": {                                      |"#
            r#"|"batter": [                                       |"#
            r#"|{ "id": "1002", "type": "Chocolate" },            |"#
            r#"|]                                                 |"#
            r#"|},                                                |"#
            r#"|"topping": [                                      |"#
            r#"|{ "id": "5003", "type": "Chocolate" },            |"#
            r#"|{ "id": "5004", "type": "Maple" }                 |"#
            r#"|]                                                 |"#
            r#"|}                                                 |"#
            r#"|                                                  |"#
            r#"+--------------------------------------------------+"#
        ),
    );
}

#[test]
fn tabs_arent_handled() {
    let json = "{
\t\t \"id\": \"1\",
\t\t \"name\": \"Hello World\",
\t\t \"list\": [
\t\t\t\t [1, 2, 3],
\t\t\t\t [4, 5, 6],
\t\t ]
}";

    let grid = grid(1, 1).data([[json]]);

    println!("{}", grid.clone().build());

    assert_eq!(
        grid.build(),
        static_table!(
            "+-------------------------+"
            "|{                        |"
            "|\t\t \"id\": \"1\",            |"
            "|\t\t \"name\": \"Hello World\",|"
            "|\t\t \"list\": [             |"
            "|\t\t\t\t [1, 2, 3],          |"
            "|\t\t\t\t [4, 5, 6],          |"
            "|\t\t ]                     |"
            "|}                        |"
            "+-------------------------+"
        ),
    );
}
