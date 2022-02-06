use crate::util::create_vector;
use tabled::style::{Line, TopBorderText};
use tabled::{Full, Indent, Modify, Style, Table, TableIteratorExt};

mod util;

#[test]
fn default_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::ASCII).to_string();

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
fn psql_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::PSQL).to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn github_markdown_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::GITHUB_MARKDOWN).to_string();

    let expected = concat!(
        "| N | column 0 | column 1 | column 2 |\n",
        "|---+----------+----------+----------|\n",
        "| 0 |   0-0    |   0-1    |   0-2    |\n",
        "| 1 |   1-0    |   1-1    |   1-2    |\n",
        "| 2 |   2-0    |   2-1    |   2-2    |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn pseudo_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::PSEUDO).to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn pseudo_clean_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::PSEUDO_CLEAN).to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn blank_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::BLANK).to_string();

    let expected = concat!(
        " N   column 0   column 1   column 2 \n",
        " 0     0-0        0-1        0-2    \n",
        " 1     1-0        1-1        1-2    \n",
        " 2     2-0        2-1        2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extended_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::EXTENDED).to_string();

    let expected = concat!(
        "╔═══╦══════════╦══════════╦══════════╗\n",
        "║ N ║ column 0 ║ column 1 ║ column 2 ║\n",
        "╠═══╬══════════╬══════════╬══════════╣\n",
        "║ 0 ║   0-0    ║   0-1    ║   0-2    ║\n",
        "╠═══╬══════════╬══════════╬══════════╣\n",
        "║ 1 ║   1-0    ║   1-1    ║   1-2    ║\n",
        "╠═══╬══════════╬══════════╬══════════╣\n",
        "║ 2 ║   2-0    ║   2-1    ║   2-2    ║\n",
        "╚═══╩══════════╩══════════╩══════════╝\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn ascii_dots_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::ASCII_DOTS).to_string();

    let expected = concat!(
        "......................................\n",
        ": N : column 0 : column 1 : column 2 :\n",
        ":...:..........:..........:..........:\n",
        ": 0 :   0-0    :   0-1    :   0-2    :\n",
        ": 1 :   1-0    :   1-1    :   1-2    :\n",
        ": 2 :   2-0    :   2-1    :   2-2    :\n",
        ":...:..........:..........:..........:\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn re_structured_text_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::RE_STRUCTURED_TEXT)
        .to_string();

    let expected = concat!(
        "=== ========== ========== ==========\n",
        " N   column 0   column 1   column 2 \n",
        "=== ========== ========== ==========\n",
        " 0     0-0        0-1        0-2    \n",
        " 1     1-0        1-1        1-2    \n",
        " 2     2-0        2-1        2-2    \n",
        "=== ========== ========== ==========\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_head_changes() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSEUDO_CLEAN.header(None))
        .to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_frame_changes() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSEUDO_CLEAN.frame_bottom(None).frame_top(None))
        .to_string();

    let expected = concat!(
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn old_custom_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(
            Style::BLANK
                .frame_bottom(Some(Line::short('*', '\'')))
                .split(Some(Line::short('`', '\'')))
                .inner('\''),
        )
        .to_string();

    let expected = concat!(
        " N ' column 0 ' column 1 ' column 2 \n",
        "```'``````````'``````````'``````````\n",
        " 0 '   0-0    '   0-1    '   0-2    \n",
        "```'``````````'``````````'``````````\n",
        " 1 '   1-0    '   1-1    '   1-2    \n",
        "```'``````````'``````````'``````````\n",
        " 2 '   2-0    '   2-1    '   2-2    \n",
        "***'**********'**********'**********\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_single_cell() {
    let data = create_vector::<0, 0>();
    let table = Table::new(&data).with(Style::ASCII).to_string();

    let expected = concat!("+---+\n", "| N |\n", "+---+\n",);

    assert_eq!(table, expected);

    let table = Table::new(&data).with(Style::BLANK).to_string();

    let expected = " N \n";

    assert_eq!(table, expected);
}

#[test]
fn top_border_override_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ASCII)
        .with(TopBorderText::new("-Table"))
        .to_string();

    let expected = concat!(
        "-Table---------+----------+\n",
        "| N | column 0 | column 1 |\n",
        "+---+----------+----------+\n",
        "| 0 |   0-0    |   0-1    |\n",
        "+---+----------+----------+\n",
        "| 1 |   1-0    |   1-1    |\n",
        "+---+----------+----------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn top_override_doesnt_work_with_style_with_no_top_border_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(TopBorderText::new("-Table"))
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 \n",
        "---+----------+----------\n",
        " 0 |   0-0    |   0-1    \n",
        " 1 |   1-0    |   1-1    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn top_border_override_cleared_after_restyling_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ASCII)
        .with(TopBorderText::new("-Table"))
        .with(Style::ASCII)
        .to_string();

    let expected = concat!(
        "+---+----------+----------+\n",
        "| N | column 0 | column 1 |\n",
        "+---+----------+----------+\n",
        "| 0 |   0-0    |   0-1    |\n",
        "+---+----------+----------+\n",
        "| 1 |   1-0    |   1-1    |\n",
        "+---+----------+----------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn empty_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::EMPTY)
        .with(Modify::new(Full).with(Indent::new(0, 0, 0, 0)))
        .to_string();

    let expected = concat!(
        "Ncolumn 0column 1column 2\n",
        "0  0-0     0-1     0-2   \n",
        "1  1-0     1-1     1-2   \n",
        "2  2-0     2-1     2-2   \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn custom_style_test() {
    macro_rules! test_style {
        ($style:expr, $expected:expr $(,)*) => {
            let data = create_vector::<3, 3>();
            let table = data.table().with($style).to_string();
            assert_eq!(table, $expected);
        };
    }

    // Single

    test_style!(
        Style::EMPTY.top('-'),
        concat!(
            "---------------------------------\n",
            " N  column 0  column 1  column 2 \n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
        ),
    );
    test_style!(
        Style::EMPTY.bottom('-'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
            "---------------------------------\n",
        ),
    );
    test_style!(
        Style::EMPTY.left('-'),
        concat!(
            "- N  column 0  column 1  column 2 \n",
            "- 0    0-0       0-1       0-2    \n",
            "- 1    1-0       1-1       1-2    \n",
            "- 2    2-0       2-1       2-2    \n",
        ),
    );
    test_style!(
        Style::EMPTY.right('-'),
        concat!(
            " N  column 0  column 1  column 2 -\n",
            " 0    0-0       0-1       0-2    -\n",
            " 1    1-0       1-1       1-2    -\n",
            " 2    2-0       2-1       2-2    -\n",
        ),
    );
    test_style!(
        Style::EMPTY.horizontal('-'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            "---------------------------------\n",
            " 0    0-0       0-1       0-2    \n",
            "---------------------------------\n",
            " 1    1-0       1-1       1-2    \n",
            "---------------------------------\n",
            " 2    2-0       2-1       2-2    \n",
        ),
    );
    test_style!(
        Style::EMPTY.header('-'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            "---------------------------------\n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
        ),
    );
    test_style!(
        Style::EMPTY.vertical('-'),
        concat!(
            " N - column 0 - column 1 - column 2 \n",
            " 0 -   0-0    -   0-1    -   0-2    \n",
            " 1 -   1-0    -   1-1    -   1-2    \n",
            " 2 -   2-0    -   2-1    -   2-2    \n",
        ),
    );

    // Combinations

    test_style!(
        Style::EMPTY.top('-').bottom('+'),
        concat!(
            "---------------------------------\n",
            " N  column 0  column 1  column 2 \n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
            "+++++++++++++++++++++++++++++++++\n",
        )
    );
    test_style!(
        Style::EMPTY.top('-').left('+'),
        concat!(
            "+---------------------------------\n",
            "+ N  column 0  column 1  column 2 \n",
            "+ 0    0-0       0-1       0-2    \n",
            "+ 1    1-0       1-1       1-2    \n",
            "+ 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.top('-').right('+'),
        concat!(
            "---------------------------------+\n",
            " N  column 0  column 1  column 2 +\n",
            " 0    0-0       0-1       0-2    +\n",
            " 1    1-0       1-1       1-2    +\n",
            " 2    2-0       2-1       2-2    +\n",
        )
    );
    test_style!(
        Style::EMPTY.top('-').horizontal('+'),
        concat!(
            "---------------------------------\n",
            " N  column 0  column 1  column 2 \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 0    0-0       0-1       0-2    \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 1    1-0       1-1       1-2    \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.top('-').vertical('+'),
        concat!(
            "---+----------+----------+----------\n",
            " N + column 0 + column 1 + column 2 \n",
            " 0 +   0-0    +   0-1    +   0-2    \n",
            " 1 +   1-0    +   1-1    +   1-2    \n",
            " 2 +   2-0    +   2-1    +   2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.top('-').header('+'),
        concat!(
            "---------------------------------\n",
            " N  column 0  column 1  column 2 \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
        )
    );

    test_style!(
        Style::EMPTY.bottom('-').top('+'),
        concat!(
            "+++++++++++++++++++++++++++++++++\n",
            " N  column 0  column 1  column 2 \n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
            "---------------------------------\n",
        )
    );
    test_style!(
        Style::EMPTY.bottom('-').left('+'),
        concat!(
            "+ N  column 0  column 1  column 2 \n",
            "+ 0    0-0       0-1       0-2    \n",
            "+ 1    1-0       1-1       1-2    \n",
            "+ 2    2-0       2-1       2-2    \n",
            "+---------------------------------\n",
        )
    );
    test_style!(
        Style::EMPTY.bottom('-').right('+'),
        concat!(
            " N  column 0  column 1  column 2 +\n",
            " 0    0-0       0-1       0-2    +\n",
            " 1    1-0       1-1       1-2    +\n",
            " 2    2-0       2-1       2-2    +\n",
            "---------------------------------+\n",
        )
    );
    test_style!(
        Style::EMPTY.bottom('-').vertical('+'),
        concat!(
            " N + column 0 + column 1 + column 2 \n",
            " 0 +   0-0    +   0-1    +   0-2    \n",
            " 1 +   1-0    +   1-1    +   1-2    \n",
            " 2 +   2-0    +   2-1    +   2-2    \n",
            "---+----------+----------+----------\n",
        )
    );
    test_style!(
        Style::EMPTY.bottom('-').horizontal('+'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 0    0-0       0-1       0-2    \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 1    1-0       1-1       1-2    \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 2    2-0       2-1       2-2    \n",
            "---------------------------------\n",
        )
    );
    test_style!(
        Style::EMPTY.bottom('-').header('+'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
            "---------------------------------\n",
        )
    );

    test_style!(
        Style::EMPTY.left('-').top('+'),
        concat!(
            "++++++++++++++++++++++++++++++++++\n",
            "- N  column 0  column 1  column 2 \n",
            "- 0    0-0       0-1       0-2    \n",
            "- 1    1-0       1-1       1-2    \n",
            "- 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.left('-').bottom('+'),
        concat!(
            "- N  column 0  column 1  column 2 \n",
            "- 0    0-0       0-1       0-2    \n",
            "- 1    1-0       1-1       1-2    \n",
            "- 2    2-0       2-1       2-2    \n",
            "++++++++++++++++++++++++++++++++++\n",
        )
    );
    test_style!(
        Style::EMPTY.left('-').right('+'),
        concat!(
            "- N  column 0  column 1  column 2 +\n",
            "- 0    0-0       0-1       0-2    +\n",
            "- 1    1-0       1-1       1-2    +\n",
            "- 2    2-0       2-1       2-2    +\n",
        )
    );
    test_style!(
        Style::EMPTY.left('-').vertical('+'),
        concat!(
            "- N + column 0 + column 1 + column 2 \n",
            "- 0 +   0-0    +   0-1    +   0-2    \n",
            "- 1 +   1-0    +   1-1    +   1-2    \n",
            "- 2 +   2-0    +   2-1    +   2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.left('-').horizontal('+'),
        concat!(
            "- N  column 0  column 1  column 2 \n",
            "++++++++++++++++++++++++++++++++++\n",
            "- 0    0-0       0-1       0-2    \n",
            "++++++++++++++++++++++++++++++++++\n",
            "- 1    1-0       1-1       1-2    \n",
            "++++++++++++++++++++++++++++++++++\n",
            "- 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.left('-').header('+'),
        concat!(
            "- N  column 0  column 1  column 2 \n",
            "++++++++++++++++++++++++++++++++++\n",
            "- 0    0-0       0-1       0-2    \n",
            "- 1    1-0       1-1       1-2    \n",
            "- 2    2-0       2-1       2-2    \n",
        )
    );

    test_style!(
        Style::EMPTY.right('-').top('+'),
        concat!(
            "++++++++++++++++++++++++++++++++++\n",
            " N  column 0  column 1  column 2 -\n",
            " 0    0-0       0-1       0-2    -\n",
            " 1    1-0       1-1       1-2    -\n",
            " 2    2-0       2-1       2-2    -\n",
        )
    );
    test_style!(
        Style::EMPTY.right('-').bottom('+'),
        concat!(
            " N  column 0  column 1  column 2 -\n",
            " 0    0-0       0-1       0-2    -\n",
            " 1    1-0       1-1       1-2    -\n",
            " 2    2-0       2-1       2-2    -\n",
            "++++++++++++++++++++++++++++++++++\n",
        )
    );
    test_style!(
        Style::EMPTY.right('-').left('+'),
        concat!(
            "+ N  column 0  column 1  column 2 -\n",
            "+ 0    0-0       0-1       0-2    -\n",
            "+ 1    1-0       1-1       1-2    -\n",
            "+ 2    2-0       2-1       2-2    -\n",
        )
    );
    test_style!(
        Style::EMPTY.right('-').vertical('+'),
        concat!(
            " N + column 0 + column 1 + column 2 -\n",
            " 0 +   0-0    +   0-1    +   0-2    -\n",
            " 1 +   1-0    +   1-1    +   1-2    -\n",
            " 2 +   2-0    +   2-1    +   2-2    -\n",
        )
    );
    test_style!(
        Style::EMPTY.right('-').horizontal('+'),
        concat!(
            " N  column 0  column 1  column 2 -\n",
            "++++++++++++++++++++++++++++++++++\n",
            " 0    0-0       0-1       0-2    -\n",
            "++++++++++++++++++++++++++++++++++\n",
            " 1    1-0       1-1       1-2    -\n",
            "++++++++++++++++++++++++++++++++++\n",
            " 2    2-0       2-1       2-2    -\n",
        )
    );
    test_style!(
        Style::EMPTY.right('-').header('+'),
        concat!(
            " N  column 0  column 1  column 2 -\n",
            "++++++++++++++++++++++++++++++++++\n",
            " 0    0-0       0-1       0-2    -\n",
            " 1    1-0       1-1       1-2    -\n",
            " 2    2-0       2-1       2-2    -\n",
        )
    );

    test_style!(
        Style::EMPTY.vertical('-').top('+'),
        concat!(
            "++++++++++++++++++++++++++++++++++++\n",
            " N - column 0 - column 1 - column 2 \n",
            " 0 -   0-0    -   0-1    -   0-2    \n",
            " 1 -   1-0    -   1-1    -   1-2    \n",
            " 2 -   2-0    -   2-1    -   2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.vertical('-').bottom('+'),
        concat!(
            " N - column 0 - column 1 - column 2 \n",
            " 0 -   0-0    -   0-1    -   0-2    \n",
            " 1 -   1-0    -   1-1    -   1-2    \n",
            " 2 -   2-0    -   2-1    -   2-2    \n",
            "++++++++++++++++++++++++++++++++++++\n",
        )
    );
    test_style!(
        Style::EMPTY.vertical('-').left('+'),
        concat!(
            "+ N - column 0 - column 1 - column 2 \n",
            "+ 0 -   0-0    -   0-1    -   0-2    \n",
            "+ 1 -   1-0    -   1-1    -   1-2    \n",
            "+ 2 -   2-0    -   2-1    -   2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.vertical('-').right('+'),
        concat!(
            " N - column 0 - column 1 - column 2 +\n",
            " 0 -   0-0    -   0-1    -   0-2    +\n",
            " 1 -   1-0    -   1-1    -   1-2    +\n",
            " 2 -   2-0    -   2-1    -   2-2    +\n",
        )
    );
    test_style!(
        Style::EMPTY.vertical('-').horizontal('+'),
        concat!(
            " N - column 0 - column 1 - column 2 \n",
            "++++++++++++++++++++++++++++++++++++\n",
            " 0 -   0-0    -   0-1    -   0-2    \n",
            "++++++++++++++++++++++++++++++++++++\n",
            " 1 -   1-0    -   1-1    -   1-2    \n",
            "++++++++++++++++++++++++++++++++++++\n",
            " 2 -   2-0    -   2-1    -   2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.vertical('-').header('+'),
        concat!(
            " N - column 0 - column 1 - column 2 \n",
            "++++++++++++++++++++++++++++++++++++\n",
            " 0 -   0-0    -   0-1    -   0-2    \n",
            " 1 -   1-0    -   1-1    -   1-2    \n",
            " 2 -   2-0    -   2-1    -   2-2    \n",
        )
    );

    test_style!(
        Style::EMPTY.horizontal('-').top('+'),
        concat!(
            "+++++++++++++++++++++++++++++++++\n",
            " N  column 0  column 1  column 2 \n",
            "---------------------------------\n",
            " 0    0-0       0-1       0-2    \n",
            "---------------------------------\n",
            " 1    1-0       1-1       1-2    \n",
            "---------------------------------\n",
            " 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.horizontal('-').bottom('+'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            "---------------------------------\n",
            " 0    0-0       0-1       0-2    \n",
            "---------------------------------\n",
            " 1    1-0       1-1       1-2    \n",
            "---------------------------------\n",
            " 2    2-0       2-1       2-2    \n",
            "+++++++++++++++++++++++++++++++++\n",
        )
    );
    test_style!(
        Style::EMPTY.horizontal('-').left('+'),
        concat!(
            "+ N  column 0  column 1  column 2 \n",
            "+---------------------------------\n",
            "+ 0    0-0       0-1       0-2    \n",
            "+---------------------------------\n",
            "+ 1    1-0       1-1       1-2    \n",
            "+---------------------------------\n",
            "+ 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.horizontal('-').right('+'),
        concat!(
            " N  column 0  column 1  column 2 +\n",
            "---------------------------------+\n",
            " 0    0-0       0-1       0-2    +\n",
            "---------------------------------+\n",
            " 1    1-0       1-1       1-2    +\n",
            "---------------------------------+\n",
            " 2    2-0       2-1       2-2    +\n",
        )
    );
    test_style!(
        Style::EMPTY.horizontal('-').vertical('+'),
        concat!(
            " N + column 0 + column 1 + column 2 \n",
            "---+----------+----------+----------\n",
            " 0 +   0-0    +   0-1    +   0-2    \n",
            "---+----------+----------+----------\n",
            " 1 +   1-0    +   1-1    +   1-2    \n",
            "---+----------+----------+----------\n",
            " 2 +   2-0    +   2-1    +   2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.horizontal('-').header('+'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 0    0-0       0-1       0-2    \n",
            "---------------------------------\n",
            " 1    1-0       1-1       1-2    \n",
            "---------------------------------\n",
            " 2    2-0       2-1       2-2    \n",
        )
    );

    test_style!(
        Style::EMPTY.header('-').top('+'),
        concat!(
            "+++++++++++++++++++++++++++++++++\n",
            " N  column 0  column 1  column 2 \n",
            "---------------------------------\n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.header('-').bottom('+'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            "---------------------------------\n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
            "+++++++++++++++++++++++++++++++++\n",
        )
    );
    test_style!(
        Style::EMPTY.header('-').left('+'),
        concat!(
            "+ N  column 0  column 1  column 2 \n",
            "+---------------------------------\n",
            "+ 0    0-0       0-1       0-2    \n",
            "+ 1    1-0       1-1       1-2    \n",
            "+ 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.header('-').right('+'),
        concat!(
            " N  column 0  column 1  column 2 +\n",
            "---------------------------------+\n",
            " 0    0-0       0-1       0-2    +\n",
            " 1    1-0       1-1       1-2    +\n",
            " 2    2-0       2-1       2-2    +\n",
        )
    );
    test_style!(
        Style::EMPTY.header('-').vertical('+'),
        concat!(
            " N + column 0 + column 1 + column 2 \n",
            "---+----------+----------+----------\n",
            " 0 +   0-0    +   0-1    +   0-2    \n",
            " 1 +   1-0    +   1-1    +   1-2    \n",
            " 2 +   2-0    +   2-1    +   2-2    \n",
        )
    );
    test_style!(
        Style::EMPTY.header('-').horizontal('+'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            "---------------------------------\n",
            " 0    0-0       0-1       0-2    \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 1    1-0       1-1       1-2    \n",
            "+++++++++++++++++++++++++++++++++\n",
            " 2    2-0       2-1       2-2    \n",
        )
    );

    // Full

    test_style!(
        Style::EMPTY
            .top('-')
            .bottom('+')
            .left('|')
            .right('*')
            .horizontal('x')
            .vertical('#'),
        concat!(
            "|---#----------#----------#----------*\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "xxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxx\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "xxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxx\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "xxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxx\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "|+++#++++++++++#++++++++++#++++++++++*\n",
        ),
    );

    let full_style = Style::EMPTY
        .top('-')
        .bottom('+')
        .left('|')
        .right('*')
        .horizontal('x')
        .header(',')
        .vertical('#')
        .bottom_intersection('@')
        .top_intersection('!')
        .left_intersection('=')
        .right_intersection('$')
        .top_left_corner(';')
        .bottom_left_corner('?')
        .top_right_corner('.')
        .bottom_right_corner('%');
    test_style!(
        full_style.clone(),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );

    // Overwrite intersections and corners

    test_style!(
        full_style.clone().top('q'),
        concat!(
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().bottom('q'),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq\n",
        )
    );
    test_style!(
        full_style.clone().left('w'),
        concat!(
            "w---!----------!----------!----------.\n",
            "w N # column 0 # column 1 # column 2 *\n",
            "w,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,$\n",
            "w 0 #   0-0    #   0-1    #   0-2    *\n",
            "wxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "w 1 #   1-0    #   1-1    #   1-2    *\n",
            "wxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "w 2 #   2-0    #   2-1    #   2-2    *\n",
            "w+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().right('i'),
        concat!(
            ";---!----------!----------!----------i\n",
            "| N # column 0 # column 1 # column 2 i\n",
            "=,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,i\n",
            "| 0 #   0-0    #   0-1    #   0-2    i\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxi\n",
            "| 1 #   1-0    #   1-1    #   1-2    i\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxi\n",
            "| 2 #   2-0    #   2-1    #   2-2    i\n",
            "?+++@++++++++++@++++++++++@++++++++++i\n",
        )
    );
    test_style!(
        full_style.clone().horizontal('q'),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().vertical('q'),
        concat!(
            ";---q----------q----------q----------.\n",
            "| N q column 0 q column 1 q column 2 *\n",
            "=,,,q,,,,,,,,,,q,,,,,,,,,,q,,,,,,,,,,$\n",
            "| 0 q   0-0    q   0-1    q   0-2    *\n",
            "=xxxqxxxxxxxxxxqxxxxxxxxxxqxxxxxxxxxx$\n",
            "| 1 q   1-0    q   1-1    q   1-2    *\n",
            "=xxxqxxxxxxxxxxqxxxxxxxxxxqxxxxxxxxxx$\n",
            "| 2 q   2-0    q   2-1    q   2-2    *\n",
            "?+++q++++++++++q++++++++++q++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().header('q'),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );

    // Turn off borders

    let empty_table = concat!(
        " N  column 0  column 1  column 2 \n",
        " 0    0-0       0-1       0-2    \n",
        " 1    1-0       1-1       1-2    \n",
        " 2    2-0       2-1       2-2    \n",
    );
    test_style!(Style::EMPTY.top('-').top_off(), empty_table);
    test_style!(Style::EMPTY.bottom('-').bottom_off(), empty_table);
    test_style!(Style::EMPTY.right('-').right_off(), empty_table);
    test_style!(Style::EMPTY.left('-').left_off(), empty_table);
    test_style!(Style::EMPTY.horizontal('-').horizontal_off(), empty_table);
    test_style!(Style::EMPTY.vertical('-').vertical_off(), empty_table);
    test_style!(Style::EMPTY.header('-').header_off(), empty_table);

    test_style!(
        full_style.clone().top_off(),
        concat!(
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().bottom_off(),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
        )
    );
    test_style!(
        full_style.clone().right_off(),
        concat!(
            ";---!----------!----------!----------\n",
            "| N # column 0 # column 1 # column 2 \n",
            "=,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,\n",
            "| 0 #   0-0    #   0-1    #   0-2    \n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx\n",
            "| 1 #   1-0    #   1-1    #   1-2    \n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx\n",
            "| 2 #   2-0    #   2-1    #   2-2    \n",
            "?+++@++++++++++@++++++++++@++++++++++\n",
        )
    );
    test_style!(
        full_style.clone().left_off(),
        concat!(
            "---!----------!----------!----------.\n",
            " N # column 0 # column 1 # column 2 *\n",
            ",,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,$\n",
            " 0 #   0-0    #   0-1    #   0-2    *\n",
            "xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            " 1 #   1-0    #   1-1    #   1-2    *\n",
            "xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            " 2 #   2-0    #   2-1    #   2-2    *\n",
            "+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().horizontal_off(),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().vertical_off(),
        concat!(
            ";---------------------------------.\n",
            "| N  column 0  column 1  column 2 *\n",
            "=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,$\n",
            "| 0    0-0       0-1       0-2    *\n",
            "=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx$\n",
            "| 1    1-0       1-1       1-2    *\n",
            "=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx$\n",
            "| 2    2-0       2-1       2-2    *\n",
            "?+++++++++++++++++++++++++++++++++%\n",
        )
    );
    test_style!(
        full_style.header_off(),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
}
