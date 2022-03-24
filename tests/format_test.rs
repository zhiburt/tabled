use crate::util::create_vector;
use tabled::{
    multiline,
    object::{Cell, Columns, Full, Object, Rows},
    Alignment, Format, FormatFrom, FormatWithIndex, Modify, Padding, Style, Table, Trim,
};

mod util;

#[test]
fn formatting_full_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .to_string();

    let expected = concat!(
        "+-----+------------+------------+------------+\n",
        "| [N] | [column 0] | [column 1] | [column 2] |\n",
        "+-----+------------+------------+------------+\n",
        "| [0] |   [0-0]    |   [0-1]    |   [0-2]    |\n",
        "+-----+------------+------------+------------+\n",
        "| [1] |   [1-0]    |   [1-1]    |   [1-2]    |\n",
        "+-----+------------+------------+------------+\n",
        "| [2] |   [2-0]    |   [2-1]    |   [2-2]    |\n",
        "+-----+------------+------------+------------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_head_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::first()).with(Format(|s| format!(":{}", s))))
        .to_string();

    let expected = concat!(
        "| :N | :column 0 | :column 1 | :column 2 |\n",
        "|----+-----------+-----------+-----------|\n",
        "| 0  |    0-0    |    0-1    |    0-2    |\n",
        "| 1  |    1-0    |    1-1    |    1-2    |\n",
        "| 2  |    2-0    |    2-1    |    2-2    |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_row_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Format(|s| format!("<{}>", s))))
        .to_string();

    let expected = concat!(
        "  N  | column 0 | column 1 | column 2 \n",
        "-----+----------+----------+----------\n",
        " <0> |  <0-0>   |  <0-1>   |  <0-2>   \n",
        " <1> |  <1-0>   |  <1-1>   |  <1-2>   \n",
        " <2> |  <2-0>   |  <2-1>   |  <2-2>   \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_column_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Columns::single(0)).with(Format(|s| format!("(x) {}", s))))
        .to_string();
    let expected = concat!(
        " (x) N | column 0 | column 1 | column 2 \n",
        "-------+----------+----------+----------\n",
        " (x) 0 |   0-0    |   0-1    |   0-2    \n",
        " (x) 1 |   1-0    |   1-1    |   1-2    \n",
        " (x) 2 |   2-0    |   2-1    |   2-2    \n",
    );
    assert_eq!(table, expected);
}

#[test]
fn formatting_multiline_test() {
    let mut data = create_vector::<3, 3>();
    data[1][2] = String::from("E\nnde\navou\nros");
    data[2][2] = String::from("Red\nHat");
    data[2][3] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Format(multiline(|s| format!("(x) {}", s)))))
        .to_string();

    let expected = concat!(
        " (x) N | (x) column 0 | (x) column 1 | (x) column 2 \n",
        "-------+--------------+--------------+--------------\n",
        " (x) 0 |   (x) 0-0    |   (x) 0-1    |   (x) 0-2    \n",
        " (x) 1 |   (x) 1-0    |    (x) E     |   (x) 1-2    \n",
        "       |              |   (x) nde    |              \n",
        "       |              |   (x) avou   |              \n",
        "       |              |   (x) ros    |              \n",
        " (x) 2 |   (x) 2-0    |   (x) Red    | (x) https:// \n",
        "       |              |   (x) Hat    |   (x) www    \n",
        "       |              |              |    (x) .     \n",
        "       |              |              |  (x) redhat  \n",
        "       |              |              |   (x) .com   \n",
        "       |              |              |   (x) /en    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_cell_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 0)).with(Format(|s| format!("(x) {}", s))))
        .with(Modify::new(Cell(0, 1)).with(Format(|s| format!("(x) {}", s))))
        .with(Modify::new(Cell(0, 2)).with(Format(|s| format!("(x) {}", s))))
        .to_string();

    let expected = concat!(
        " (x) N | (x) column 0 | (x) column 1 | column 2 \n",
        "-------+--------------+--------------+----------\n",
        "   0   |     0-0      |     0-1      |   0-2    \n",
        "   1   |     1-0      |     1-1      |   1-2    \n",
        "   2   |     2-0      |     2-1      |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_and_combination_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Columns::single(0).and(Rows::single(0)))
                .with(Format(|s| format!("(x) {}", s))),
        )
        .to_string();

    let expected = concat!(
        " (x) N | (x) column 0 | (x) column 1 | (x) column 2 \n",
        "-------+--------------+--------------+--------------\n",
        " (x) 0 |     0-0      |     0-1      |     0-2      \n",
        " (x) 1 |     1-0      |     1-1      |     1-2      \n",
        " (x) 2 |     2-0      |     2-1      |     2-2      \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_not_combination_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Columns::single(0).and(Rows::single(0)).not(Cell(0, 0)))
                .with(Format(|s| format!("(x) {}", s))),
        )
        .to_string();

    let expected = concat!(
        "   N   | (x) column 0 | (x) column 1 | (x) column 2 \n",
        "-------+--------------+--------------+--------------\n",
        " (x) 0 |     0-0      |     0-1      |     0-2      \n",
        " (x) 1 |     1-0      |     1-1      |     1-2      \n",
        " (x) 2 |     2-0      |     2-1      |     2-2      \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_using_lambda_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::first()).with(|s: &str| format!(":{}", s)))
        .to_string();

    let expected = concat!(
        "| :N | :column 0 | :column 1 | :column 2 |\n",
        "|----+-----------+-----------+-----------|\n",
        "| 0  |    0-0    |    0-1    |    0-2    |\n",
        "| 1  |    1-0    |    1-1    |    1-2    |\n",
        "| 2  |    2-0    |    2-1    |    2-2    |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_using_function_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::first()).with(str::to_uppercase))
        .to_string();

    let expected = concat!(
        "| N | COLUMN 0 | COLUMN 1 | COLUMN 2 |\n",
        "|---+----------+----------+----------|\n",
        "| 0 |   0-0    |   0-1    |   0-2    |\n",
        "| 1 |   1-0    |   1-1    |   1-2    |\n",
        "| 2 |   2-0    |   2-1    |   2-2    |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn format_from() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::first()).with(FormatFrom(vec![
            "Header Name 1",
            "Header Name 2",
            "Header Name 3",
        ])))
        .to_string();

    let expected = concat!(
        "| Header Name 1 | Header Name 2 | Header Name 3 | column 2 |\n",
        "|---------------+---------------+---------------+----------|\n",
        "|       0       |      0-0      |      0-1      |   0-2    |\n",
        "|       1       |      1-0      |      1-1      |   1-2    |\n",
        "|       2       |      2-0      |      2-1      |   2-2    |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn format_with_index() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(
            Modify::new(Rows::first()).with(FormatWithIndex(|a, b, c| match (b, c) {
                (0, 0) => "(0, 0)".to_string(),
                (0, 1) => "(0, 1)".to_string(),
                (0, 2) => "(0, 2)".to_string(),
                _ => a.to_string(),
            })),
        )
        .to_string();

    let expected = concat!(
        "| (0, 0) | (0, 1) | (0, 2) | column 2 |\n",
        "|--------+--------+--------+----------|\n",
        "|   0    |  0-0   |  0-1   |   0-2    |\n",
        "|   1    |  1-0   |  1-1   |   1-2    |\n",
        "|   2    |  2-0   |  2-1   |   2-2    |\n",
    );

    assert_eq!(table, expected);
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
                    .with(Format(|s| s.red().to_string())),
            )
            .with(Modify::new(Columns::new(1..2)).with(Format(|s| s.blue().to_string())))
            .to_string();

        let expected = concat!(
            " \u{1b}[31mN\u{1b}[0m | \u{1b}[34mcolumn 0\u{1b}[0m | \u{1b}[31mcolumn 1\u{1b}[0m | \u{1b}[31mcolumn 2\u{1b}[0m \n",
            "---+----------+----------+----------\n",
            " \u{1b}[31m0\u{1b}[0m |   \u{1b}[34m0-0\u{1b}[0m    |   \u{1b}[31m0-1\u{1b}[0m    |   \u{1b}[31m0-2\u{1b}[0m    \n",
            " \u{1b}[31m1\u{1b}[0m |   \u{1b}[34m1-0\u{1b}[0m    |   \u{1b}[31m1-1\u{1b}[0m    |   \u{1b}[31m1-2\u{1b}[0m    \n",
            " \u{1b}[31m2\u{1b}[0m |   \u{1b}[34m2-0\u{1b}[0m    |   \u{1b}[31m2-1\u{1b}[0m    |   \u{1b}[31m2-2\u{1b}[0m    \n",
        );

        assert_eq!(table, expected);
    }

    #[test]
    fn color_multiline_test() {
        let mut data = create_vector::<3, 3>();
        data[1][2] = String::from("E\nnde\navou\nros");
        data[2][2] = String::from("Red\nHat");
        data[2][3] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Columns::new(..1)).with(Format(multiline(|s| s.red().to_string()))))
            .with(Modify::new(Columns::new(1..2)).with(Format(multiline(|s| s.blue().to_string()))))
            .with(Modify::new(Columns::new(2..)).with(Format(multiline(|s| s.green().to_string()))))
            .to_string();

        let expected = concat!(
            " \u{1b}[31mN\u{1b}[0m | \u{1b}[34mcolumn 0\u{1b}[0m | \u{1b}[32mcolumn 1\u{1b}[0m | \u{1b}[32mcolumn 2\u{1b}[0m \n",
            "---+----------+----------+----------\n",
            " \u{1b}[31m0\u{1b}[0m |   \u{1b}[34m0-0\u{1b}[0m    |   \u{1b}[32m0-1\u{1b}[0m    |   \u{1b}[32m0-2\u{1b}[0m    \n",
            " \u{1b}[31m1\u{1b}[0m |   \u{1b}[34m1-0\u{1b}[0m    |    \u{1b}[32mE\u{1b}[0m     |   \u{1b}[32m1-2\u{1b}[0m    \n",
            "   |          |   \u{1b}[32mnde\u{1b}[0m    |          \n",
            "   |          |   \u{1b}[32mavou\u{1b}[0m   |          \n",
            "   |          |   \u{1b}[32mros\u{1b}[0m    |          \n",
            " \u{1b}[31m2\u{1b}[0m |   \u{1b}[34m2-0\u{1b}[0m    |   \u{1b}[32mRed\u{1b}[0m    | \u{1b}[32mhttps://\u{1b}[0m \n",
            "   |          |   \u{1b}[32mHat\u{1b}[0m    |   \u{1b}[32mwww\u{1b}[0m    \n",
            "   |          |          |    \u{1b}[32m.\u{1b}[0m     \n",
            "   |          |          |  \u{1b}[32mredhat\u{1b}[0m  \n",
            "   |          |          |   \u{1b}[32m.com\u{1b}[0m   \n",
            "   |          |          |   \u{1b}[32m/en\u{1b}[0m    \n",
        );

        assert_eq!(table, expected);
    }
}

#[test]
fn format_doesnt_change_padding() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .to_string();

    let expected = concat!(
        "+-------+--------------+--------------+--------------+\n",
        "|   [N] |   [column 0] |   [column 1] |   [column 2] |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn format_trim_removes_leading_spaces() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Rows::first()).with(Format(|s| format!("      {}", s))))
        .with(Modify::new(Full).with(Trim))
        .to_string();

    let expected = concat!(
        "+---+----------+----------+----------+\n",
        "| N | column 0 | column 1 | column 2 |\n",
        "+---+----------+----------+----------+\n",
        "| 0 |   0-0    |   0-1    |   0-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 1 |   1-0    |   1-1    |   1-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 2 |   2-0    |   2-1    |   2-2    |\n",
        "+---+----------+----------+----------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn format_trim_removes_trailing_spaces() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Rows::first()).with(Format(|s| format!("{}              ", s))))
        .with(Modify::new(Full).with(Trim))
        .to_string();

    let expected = concat!(
        "+---+----------+----------+----------+\n",
        "| N | column 0 | column 1 | column 2 |\n",
        "+---+----------+----------+----------+\n",
        "| 0 |   0-0    |   0-1    |   0-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 1 |   1-0    |   1-1    |   1-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 2 |   2-0    |   2-1    |   2-2    |\n",
        "+---+----------+----------+----------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn format_trim_handles_escape_characters_correctly() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Rows::first()).with(Format(|s| format!("   \n \t \r      {}        \n\n\t\t\r\r      ", s))))
        .with(Modify::new(Full).with(Trim))
        .to_string();

    let expected = concat!(
        "+---+----------+----------+----------+\n",
        "| N | column 0 | column 1 | column 2 |\n",
        "+---+----------+----------+----------+\n",
        "| 0 |   0-0    |   0-1    |   0-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 1 |   1-0    |   1-1    |   1-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 2 |   2-0    |   2-1    |   2-2    |\n",
        "+---+----------+----------+----------+\n",
    );

    assert_eq!(table, expected);
}
