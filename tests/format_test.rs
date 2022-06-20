use tabled::{
    object::{Cell, Columns, Object, Rows, Segment},
    Alignment, Format, Modify, Padding, Style, Table,
};

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn formatting_full_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-----+------------+------------+------------+"
            "| [N] | [column 0] | [column 1] | [column 2] |"
            "+-----+------------+------------+------------+"
            "| [0] |   [0-0]    |   [0-1]    |   [0-2]    |"
            "+-----+------------+------------+------------+"
            "| [1] |   [1-0]    |   [1-1]    |   [1-2]    |"
            "+-----+------------+------------+------------+"
            "| [2] |   [2-0]    |   [2-1]    |   [2-2]    |"
            "+-----+------------+------------+------------+"
        )
    );
}

#[test]
fn formatting_head_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::first()).with(Format::new(|s| format!(":{}", s))))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| :N | :column 0 | :column 1 | :column 2 |"
            "|----+-----------+-----------+-----------|"
            "| 0  |    0-0    |    0-1    |    0-2    |"
            "| 1  |    1-0    |    1-1    |    1-2    |"
            "| 2  |    2-0    |    2-1    |    2-2    |"
        )
    );
}

#[test]
fn formatting_row_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Format::new(|s| format!("<{}>", s))))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "  N  | column 0 | column 1 | column 2 "
            "-----+----------+----------+----------"
            " <0> |  <0-0>   |  <0-1>   |  <0-2>   "
            " <1> |  <1-0>   |  <1-1>   |  <1-2>   "
            " <2> |  <2-0>   |  <2-1>   |  <2-2>   "
        )
    );
}

#[test]
fn formatting_column_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Columns::single(0)).with(Format::new(|s| format!("(x) {}", s))))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " (x) N | column 0 | column 1 | column 2 "
            "-------+----------+----------+----------"
            " (x) 0 |   0-0    |   0-1    |   0-2    "
            " (x) 1 |   1-0    |   1-1    |   1-2    "
            " (x) 2 |   2-0    |   2-1    |   2-2    "
        )
    );
}

#[test]
fn formatting_multiline_test() {
    let mut data = create_vector::<3, 3>();
    data[1][2] = String::from("E\nnde\navou\nros");
    data[2][2] = String::from("Red\nHat");
    data[2][3] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Format::multiline(|s| format!("(x) {}", s))))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " (x) N | (x) column 0 | (x) column 1 | (x) column 2 "
            "-------+--------------+--------------+--------------"
            " (x) 0 |   (x) 0-0    |   (x) 0-1    |   (x) 0-2    "
            " (x) 1 |   (x) 1-0    |   (x) E      |   (x) 1-2    "
            "       |              |   (x) nde    |              "
            "       |              |   (x) avou   |              "
            "       |              |   (x) ros    |              "
            " (x) 2 |   (x) 2-0    |   (x) Red    | (x) https:// "
            "       |              |   (x) Hat    | (x) www      "
            "       |              |              | (x) .        "
            "       |              |              | (x) redhat   "
            "       |              |              | (x) .com     "
            "       |              |              | (x) /en      "
        )
    );
}

#[test]
fn formatting_cell_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 0)).with(Format::new(|s| format!("(x) {}", s))))
        .with(Modify::new(Cell(0, 1)).with(Format::new(|s| format!("(x) {}", s))))
        .with(Modify::new(Cell(0, 2)).with(Format::new(|s| format!("(x) {}", s))))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " (x) N | (x) column 0 | (x) column 1 | column 2 "
            "-------+--------------+--------------+----------"
            "   0   |     0-0      |     0-1      |   0-2    "
            "   1   |     1-0      |     1-1      |   1-2    "
            "   2   |     2-0      |     2-1      |   2-2    "
        )
    );
}

#[test]
fn formatting_and_combination_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Columns::single(0).and(Rows::single(0)))
                .with(Format::new(|s| format!("(x) {}", s))),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " (x) N | (x) column 0 | (x) column 1 | (x) column 2 "
            "-------+--------------+--------------+--------------"
            " (x) 0 |     0-0      |     0-1      |     0-2      "
            " (x) 1 |     1-0      |     1-1      |     1-2      "
            " (x) 2 |     2-0      |     2-1      |     2-2      "
        )
    );
}

#[test]
fn formatting_not_combination_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Columns::single(0).and(Rows::single(0)).not(Cell(0, 0)))
                .with(Format::new(|s| format!("(x) {}", s))),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "   N   | (x) column 0 | (x) column 1 | (x) column 2 "
            "-------+--------------+--------------+--------------"
            " (x) 0 |     0-0      |     0-1      |     0-2      "
            " (x) 1 |     1-0      |     1-1      |     1-2      "
            " (x) 2 |     2-0      |     2-1      |     2-2      "
        )
    );
}

#[test]
fn formatting_using_lambda_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::first()).with(|s: &str| format!(":{}", s)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| :N | :column 0 | :column 1 | :column 2 |"
            "|----+-----------+-----------+-----------|"
            "| 0  |    0-0    |    0-1    |    0-2    |"
            "| 1  |    1-0    |    1-1    |    1-2    |"
            "| 2  |    2-0    |    2-1    |    2-2    |"
        )
    );
}

#[test]
fn formatting_using_function_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::first()).with(str::to_uppercase))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | COLUMN 0 | COLUMN 1 | COLUMN 2 |"
            "|---+----------+----------+----------|"
            "| 0 |   0-0    |   0-1    |   0-2    |"
            "| 1 |   1-0    |   1-1    |   1-2    |"
            "| 2 |   2-0    |   2-1    |   2-2    |"
        )
    );
}

#[test]
fn format_with_index() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(
            Modify::new(Rows::first()).with(Format::with_index(|a, (b, c)| match (b, c) {
                (0, 0) => "(0, 0)".to_string(),
                (0, 1) => "(0, 1)".to_string(),
                (0, 2) => "(0, 2)".to_string(),
                _ => a.to_string(),
            })),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| (0, 0) | (0, 1) | (0, 2) | column 2 |"
            "|--------+--------+--------+----------|"
            "|   0    |  0-0   |  0-1   |   0-2    |"
            "|   1    |  1-0   |  1-1   |   1-2    |"
            "|   2    |  2-0   |  2-1   |   2-2    |"
        )
    );
}

#[cfg(feature = "color")]
mod color {

    use super::*;
    use owo_colors::OwoColorize;

    #[test]
    fn color_column_test() {
        let data = create_vector::<3, 3>();
        let table = Table::new(&data)
            .with(Style::psql())
            .with(
                Modify::new(Columns::new(..1).and(Columns::new(2..)))
                    .with(Format::new(|s| s.red().to_string())),
            )
            .with(Modify::new(Columns::new(1..2)).with(Format::new(|s| s.blue().to_string())))
            .to_string();

        assert_eq!(
            table,
            static_table!(
                " \u{1b}[31mN\u{1b}[39m | \u{1b}[34mcolumn 0\u{1b}[39m | \u{1b}[31mcolumn 1\u{1b}[39m | \u{1b}[31mcolumn 2\u{1b}[39m "
                "---+----------+----------+----------"
                " \u{1b}[31m0\u{1b}[39m |   \u{1b}[34m0-0\u{1b}[39m    |   \u{1b}[31m0-1\u{1b}[39m    |   \u{1b}[31m0-2\u{1b}[39m    "
                " \u{1b}[31m1\u{1b}[39m |   \u{1b}[34m1-0\u{1b}[39m    |   \u{1b}[31m1-1\u{1b}[39m    |   \u{1b}[31m1-2\u{1b}[39m    "
                " \u{1b}[31m2\u{1b}[39m |   \u{1b}[34m2-0\u{1b}[39m    |   \u{1b}[31m2-1\u{1b}[39m    |   \u{1b}[31m2-2\u{1b}[39m    "
            )
        );
    }

    #[test]
    fn color_multiline_test() {
        let mut data = create_vector::<3, 3>();
        data[1][2] = String::from("E\nnde\navou\nros");
        data[2][2] = String::from("Red\nHat");
        data[2][3] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Columns::new(..1)).with(Format::multiline(|s| s.red().to_string())))
            .with(Modify::new(Columns::new(1..2)).with(Format::multiline(|s| s.blue().to_string())))
            .with(Modify::new(Columns::new(2..)).with(Format::multiline(|s| s.green().to_string())))
            .to_string();

        assert_eq!(
            table,
            static_table!(
                " \u{1b}[31mN\u{1b}[39m | \u{1b}[34mcolumn 0\u{1b}[39m | \u{1b}[32mcolumn 1\u{1b}[39m | \u{1b}[32mcolumn 2\u{1b}[39m "
                "---+----------+----------+----------"
                " \u{1b}[31m0\u{1b}[39m |   \u{1b}[34m0-0\u{1b}[39m    |   \u{1b}[32m0-1\u{1b}[39m    |   \u{1b}[32m0-2\u{1b}[39m    "
                " \u{1b}[31m1\u{1b}[39m |   \u{1b}[34m1-0\u{1b}[39m    |   \u{1b}[32mE\u{1b}[39m      |   \u{1b}[32m1-2\u{1b}[39m    "
                "   |          |   \u{1b}[32m\u{1b}[39m\u{1b}[32mnde\u{1b}[39m    |          "
                "   |          |   \u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32mavou\u{1b}[39m   |          "
                "   |          |   \u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32mros\u{1b}[39m    |          "
                " \u{1b}[31m2\u{1b}[39m |   \u{1b}[34m2-0\u{1b}[39m    |   \u{1b}[32mRed\u{1b}[39m    | \u{1b}[32mhttps://\u{1b}[39m "
                "   |          |   \u{1b}[32m\u{1b}[39m\u{1b}[32mHat\u{1b}[39m    | \u{1b}[32m\u{1b}[39m\u{1b}[32mwww\u{1b}[39m      "
                "   |          |          | \u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m.\u{1b}[39m        "
                "   |          |          | \u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32mredhat\u{1b}[39m   "
                "   |          |          | \u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m.com\u{1b}[39m     "
                "   |          |          | \u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m\u{1b}[39m\u{1b}[32m/en\u{1b}[39m      "
            )
        );
    }
}

#[test]
fn format_doesnt_change_padding() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-------+--------------+--------------+--------------+"
            "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
            "+-------+--------------+--------------+--------------+"
            "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
            "+-------+--------------+--------------+--------------+"
            "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
            "+-------+--------------+--------------+--------------+"
            "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |"
            "+-------+--------------+--------------+--------------+"
        )
    );
}
