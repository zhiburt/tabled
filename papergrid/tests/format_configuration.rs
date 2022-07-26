use papergrid::{AlignmentHorizontal, AlignmentVertical, Entity, Formatting};

mod util;

#[test]
fn formatting_test() {
    let tests = [
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Top,
            Formatting::new(false, false, true),
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |    string|\n\
             |             |with      |\n\
             |             | new      |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |...       |\n\
             |    string   |          |\n\
             |with         |          |\n\
             | new         |          |\n\
             |line         |          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Top,
            Formatting::new(true, false, true),
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |             |with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |...       |\n\
             |string       |          |\n\
             |with         |          |\n\
             |new          |          |\n\
             |line         |          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Top,
            Formatting::new(true, true, true),
            "+-------------+----------+\n\
             |A long string|A         |\n\
             |             |string    |\n\
             |             |with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |...       |\n\
             |string       |          |\n\
             |with         |          |\n\
             |new          |          |\n\
             |line         |          |\n\
             +-------------+----------+",
        ),
        //
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Top,
            Formatting::new(false, false, true),
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |    string|\n\
             |             |   with   |\n\
             |             |    new   |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |   ...    |\n\
             |     string  |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Top,
            Formatting::new(true, false, true),
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |             |   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |   ...    |\n\
             |   string    |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Top,
            Formatting::new(true, true, true),
            "+-------------+----------+\n\
             |A long string|    A     |\n\
             |             |  string  |\n\
             |             |   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |   ...    |\n\
             |   string    |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+",
        ),
        //
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Top,
            Formatting::new(false, false, true),
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|       ...|\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Top,
            Formatting::new(true, false, true),
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|       ...|\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Top,
            Formatting::new(true, true, true),
            "+-------------+----------+\n\
             |A long string|         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|       ...|\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+",
        ),
        // asd
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Center,
            Formatting::new(false, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |    string|\n\
             |A long string|with      |\n\
             |             | new      |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |    string   |          |\n\
             |with         |...       |\n\
             | new         |          |\n\
             |line         |          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Center,
            Formatting::new(true, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |A long string|with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |string       |          |\n\
             |with         |...       |\n\
             |new          |          |\n\
             |line         |          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Center,
            Formatting::new(true, true, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |A long string|with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |string       |          |\n\
             |with         |...       |\n\
             |new          |          |\n\
             |line         |          |\n\
             +-------------+----------+",
        ),
        //
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Center,
            Formatting::new(false, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |    string|\n\
             |A long string|   with   |\n\
             |             |    new   |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |     string  |          |\n\
             |    with     |   ...    |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Center,
            Formatting::new(true, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |A long string|   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |   string    |          |\n\
             |    with     |   ...    |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Center,
            Formatting::new(true, true, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |A long string|   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |   string    |          |\n\
             |    with     |   ...    |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+",
        ),
        //
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Center,
            Formatting::new(false, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |A long string|      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|       ...|\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Center,
            Formatting::new(true, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |A long string|      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|       ...|\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Center,
            Formatting::new(true, true, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |A long string|      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|       ...|\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+",
        ),
        //
        // asd
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Bottom,
            Formatting::new(false, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |    string|\n\
             |             |with      |\n\
             |             | new      |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |    string   |          |\n\
             |with         |          |\n\
             | new         |          |\n\
             |line         |...       |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Bottom,
            Formatting::new(true, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |             |with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |string       |          |\n\
             |with         |          |\n\
             |new          |          |\n\
             |line         |...       |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Bottom,
            Formatting::new(true, true, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |             |with      |\n\
             |             |new       |\n\
             |A long string|line      |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |string       |          |\n\
             |with         |          |\n\
             |new          |          |\n\
             |line         |...       |\n\
             +-------------+----------+",
        ),
        //
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Bottom,
            Formatting::new(false, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |    string|\n\
             |             |   with   |\n\
             |             |    new   |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |     string  |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |   ...    |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Bottom,
            Formatting::new(true, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |             |   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |   string    |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |   ...    |\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Bottom,
            Formatting::new(true, true, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |             |   with   |\n\
             |             |   new    |\n\
             |A long string|   line   |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |   string    |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |   ...    |\n\
             +-------------+----------+",
        ),
        //
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Bottom,
            Formatting::new(false, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|       ...|\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Bottom,
            Formatting::new(true, false, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|       ...|\n\
             +-------------+----------+",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Bottom,
            Formatting::new(true, true, true),
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |A long string|      line|\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|       ...|\n\
             +-------------+----------+",
        ),
    ];

    let mut grid = util::grid_with_data::<3, 2>(&[
        ((0, 0), "A long string"),
        ((0, 1), "\n\n\nA\n    string\nwith\n new\nline\n\n\n"),
        ((2, 0), "A one more\n    string\nwith\n new\nline"),
        ((2, 1), "..."),
    ]);

    for (i, test) in tests.iter().enumerate() {
        let expected = test.3;

        grid.set_alignment_horizontal(Entity::Global, test.0);
        grid.set_alignment_vertical(Entity::Global, test.1);
        grid.set_formatting(Entity::Global, test.2);

        assert_eq!(grid.to_string(), expected, "test case #{:?} failed", i,);
    }
}

#[test]
fn formatting_empty_test() {
    let formatting = Formatting::new(true, true, true);

    let mut grid = util::grid::<0, 0>();
    grid.set_formatting(Entity::Global, formatting);

    assert_eq!(grid.to_string(), "");

    let mut grid = util::grid::<4, 0>();
    grid.set_formatting(Entity::Global, formatting);

    assert_eq!(grid.to_string(), "");

    let mut grid = util::grid::<0, 4>();
    grid.set_formatting(Entity::Global, formatting);

    assert_eq!(grid.to_string(), "");
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

    let mut grid = util::grid_with_data::<1, 1>(&[((0, 0), json)]);
    grid.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Left);

    assert_eq!(
        grid.to_string(),
        vec![
            r#"+--------------------------------------------------+"#,
            r#"|                                                  |"#,
            r#"|{                                                 |"#,
            r#"|    "id": "0001",                                 |"#,
            r#"|    "batters": {                                  |"#,
            r#"|        "batter": [                               |"#,
            r#"|            { "id": "1002", "type": "Chocolate" },|"#,
            r#"|        ]                                         |"#,
            r#"|    },                                            |"#,
            r#"|    "topping": [                                  |"#,
            r#"|        { "id": "5003", "type": "Chocolate" },    |"#,
            r#"|        { "id": "5004", "type": "Maple" }         |"#,
            r#"|    ]                                             |"#,
            r#"|}                                                 |"#,
            r#"+--------------------------------------------------+"#,
        ]
        .join("\n")
    );

    grid.set_formatting(Entity::Global, Formatting::new(false, false, true));

    assert_eq!(
        grid.to_string(),
        vec![
            r#"+--------------------------------------------------+"#,
            r#"|                                                  |"#,
            r#"|{                                                 |"#,
            r#"|    "id": "0001",                                 |"#,
            r#"|    "batters": {                                  |"#,
            r#"|        "batter": [                               |"#,
            r#"|            { "id": "1002", "type": "Chocolate" },|"#,
            r#"|        ]                                         |"#,
            r#"|    },                                            |"#,
            r#"|    "topping": [                                  |"#,
            r#"|        { "id": "5003", "type": "Chocolate" },    |"#,
            r#"|        { "id": "5004", "type": "Maple" }         |"#,
            r#"|    ]                                             |"#,
            r#"|}                                                 |"#,
            r#"+--------------------------------------------------+"#,
        ]
        .join("\n")
    );

    grid.set_formatting(Entity::Global, Formatting::new(true, false, true));

    assert_eq!(
        grid.to_string(),
        vec![
            r#"+--------------------------------------------------+"#,
            r#"|                                                  |"#,
            r#"|{                                                 |"#,
            r#"|"id": "0001",                                     |"#,
            r#"|"batters": {                                      |"#,
            r#"|"batter": [                                       |"#,
            r#"|{ "id": "1002", "type": "Chocolate" },            |"#,
            r#"|]                                                 |"#,
            r#"|},                                                |"#,
            r#"|"topping": [                                      |"#,
            r#"|{ "id": "5003", "type": "Chocolate" },            |"#,
            r#"|{ "id": "5004", "type": "Maple" }                 |"#,
            r#"|]                                                 |"#,
            r#"|}                                                 |"#,
            r#"+--------------------------------------------------+"#,
        ]
        .join("\n")
    );

    grid.set_formatting(Entity::Global, Formatting::new(true, true, true));

    assert_eq!(
        grid.to_string(),
        vec![
            r#"+--------------------------------------------------+"#,
            r#"|{                                                 |"#,
            r#"|"id": "0001",                                     |"#,
            r#"|"batters": {                                      |"#,
            r#"|"batter": [                                       |"#,
            r#"|{ "id": "1002", "type": "Chocolate" },            |"#,
            r#"|]                                                 |"#,
            r#"|},                                                |"#,
            r#"|"topping": [                                      |"#,
            r#"|{ "id": "5003", "type": "Chocolate" },            |"#,
            r#"|{ "id": "5004", "type": "Maple" }                 |"#,
            r#"|]                                                 |"#,
            r#"|}                                                 |"#,
            r#"|                                                  |"#,
            r#"+--------------------------------------------------+"#,
        ]
        .join("\n")
    );
}

#[test]
fn tab_size_test() {
    let json = "{
\t\t \"id\": \"1\",
\t\t \"name\": \"Hello World\",
\t\t \"list\": [
\t\t\t\t [1, 2, 3],
\t\t\t\t [4, 5, 6],
\t\t ]
}";

    let mut grid = util::grid_with_data::<1, 1>(&[((0, 0), json)]);

    assert_eq!(
        grid.to_string(),
        "+-------------------------------+\n\
         |{                              |\n\
         |         \"id\": \"1\",            |\n\
         |         \"name\": \"Hello World\",|\n\
         |         \"list\": [             |\n\
         |                 [1, 2, 3],    |\n\
         |                 [4, 5, 6],    |\n\
         |         ]                     |\n\
         |}                              |\n\
         +-------------------------------+",
    );

    grid.set_tab_width(1);

    assert_eq!(
        grid.to_string(),
        "+-------------------------+\n\
         |{                        |\n\
         |   \"id\": \"1\",            |\n\
         |   \"name\": \"Hello World\",|\n\
         |   \"list\": [             |\n\
         |     [1, 2, 3],          |\n\
         |     [4, 5, 6],          |\n\
         |   ]                     |\n\
         |}                        |\n\
         +-------------------------+"
    );

    grid.set_tab_width(0);

    assert_eq!(
        grid.to_string(),
        "+-----------------------+\n\
         |{                      |\n\
         | \"id\": \"1\",            |\n\
         | \"name\": \"Hello World\",|\n\
         | \"list\": [             |\n\
         | [1, 2, 3],            |\n\
         | [4, 5, 6],            |\n\
         | ]                     |\n\
         |}                      |\n\
         +-----------------------+"
    );
}
