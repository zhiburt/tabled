use std::iter::FromIterator;

use crate::util::create_vector;

use tabled::{
    builder::Builder,
    object::{Rows, Segment},
    style::{Border, BorderText},
    Highlight, Modify, Padding, Style, Table, TableIteratorExt,
};

mod util;

#[test]
fn default_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::ascii()).to_string();

    assert_eq!(
        table,
        concat!(
            "+---+----------+----------+----------+\n",
            "| N | column 0 | column 1 | column 2 |\n",
            "+---+----------+----------+----------+\n",
            "| 0 |   0-0    |   0-1    |   0-2    |\n",
            "+---+----------+----------+----------+\n",
            "| 1 |   1-0    |   1-1    |   1-2    |\n",
            "+---+----------+----------+----------+\n",
            "| 2 |   2-0    |   2-1    |   2-2    |\n",
            "+---+----------+----------+----------+\n",
        )
    );
}

#[test]
fn psql_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::psql()).to_string();

    assert_eq!(
        table,
        concat!(
            " N | column 0 | column 1 | column 2 \n",
            "---+----------+----------+----------\n",
            " 0 |   0-0    |   0-1    |   0-2    \n",
            " 1 |   1-0    |   1-1    |   1-2    \n",
            " 2 |   2-0    |   2-1    |   2-2    \n",
        )
    );
}

#[test]
fn github_markdown_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::github_markdown()).to_string();

    assert_eq!(
        table,
        concat!(
            "| N | column 0 | column 1 | column 2 |\n",
            "|---+----------+----------+----------|\n",
            "| 0 |   0-0    |   0-1    |   0-2    |\n",
            "| 1 |   1-0    |   1-1    |   1-2    |\n",
            "| 2 |   2-0    |   2-1    |   2-2    |\n",
        )
    );
}

#[test]
fn pseudo_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌───┬──────────┬──────────┬──────────┐\n",
            "│ N │ column 0 │ column 1 │ column 2 │\n",
            "├───┼──────────┼──────────┼──────────┤\n",
            "│ 0 │   0-0    │   0-1    │   0-2    │\n",
            "├───┼──────────┼──────────┼──────────┤\n",
            "│ 1 │   1-0    │   1-1    │   1-2    │\n",
            "├───┼──────────┼──────────┼──────────┤\n",
            "│ 2 │   2-0    │   2-1    │   2-2    │\n",
            "└───┴──────────┴──────────┴──────────┘\n",
        )
    );
}

#[test]
fn rounded_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::rounded()).to_string();

    assert_eq!(
        table,
        concat!(
            "╭───┬──────────┬──────────┬──────────╮\n",
            "│ N │ column 0 │ column 1 │ column 2 │\n",
            "├───┼──────────┼──────────┼──────────┤\n",
            "│ 0 │   0-0    │   0-1    │   0-2    │\n",
            "│ 1 │   1-0    │   1-1    │   1-2    │\n",
            "│ 2 │   2-0    │   2-1    │   2-2    │\n",
            "╰───┴──────────┴──────────┴──────────╯\n",
        )
    );
}

#[test]
fn pseudo_clean_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern().horizontal_off())
        .to_string();

    assert_eq!(
        table,
        concat!(
            "┌───┬──────────┬──────────┬──────────┐\n",
            "│ N │ column 0 │ column 1 │ column 2 │\n",
            "├───┼──────────┼──────────┼──────────┤\n",
            "│ 0 │   0-0    │   0-1    │   0-2    │\n",
            "│ 1 │   1-0    │   1-1    │   1-2    │\n",
            "│ 2 │   2-0    │   2-1    │   2-2    │\n",
            "└───┴──────────┴──────────┴──────────┘\n",
        )
    );
}

#[test]
fn blank_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::blank()).to_string();

    assert_eq!(
        table,
        concat!(
            " N   column 0   column 1   column 2 \n",
            " 0     0-0        0-1        0-2    \n",
            " 1     1-0        1-1        1-2    \n",
            " 2     2-0        2-1        2-2    \n",
        )
    );
}

#[test]
fn extended_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::extended()).to_string();

    assert_eq!(
        table,
        concat!(
            "╔═══╦══════════╦══════════╦══════════╗\n",
            "║ N ║ column 0 ║ column 1 ║ column 2 ║\n",
            "╠═══╬══════════╬══════════╬══════════╣\n",
            "║ 0 ║   0-0    ║   0-1    ║   0-2    ║\n",
            "╠═══╬══════════╬══════════╬══════════╣\n",
            "║ 1 ║   1-0    ║   1-1    ║   1-2    ║\n",
            "╠═══╬══════════╬══════════╬══════════╣\n",
            "║ 2 ║   2-0    ║   2-1    ║   2-2    ║\n",
            "╚═══╩══════════╩══════════╩══════════╝\n",
        )
    );
}

#[test]
fn ascii_dots_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::dots()).to_string();

    assert_eq!(
        table,
        concat!(
            "......................................\n",
            ": N : column 0 : column 1 : column 2 :\n",
            ":...:..........:..........:..........:\n",
            ": 0 :   0-0    :   0-1    :   0-2    :\n",
            ":...:..........:..........:..........:\n",
            ": 1 :   1-0    :   1-1    :   1-2    :\n",
            ":...:..........:..........:..........:\n",
            ": 2 :   2-0    :   2-1    :   2-2    :\n",
            ":...:..........:..........:..........:\n",
        )
    );
}

#[test]
fn re_structured_text_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::re_structured_text())
        .to_string();

    assert_eq!(
        table,
        concat!(
            "=== ========== ========== ==========\n",
            " N   column 0   column 1   column 2 \n",
            "=== ========== ========== ==========\n",
            " 0     0-0        0-1        0-2    \n",
            " 1     1-0        1-1        1-2    \n",
            " 2     2-0        2-1        2-2    \n",
            "=== ========== ========== ==========\n",
        )
    );
}

#[test]
fn style_head_changes() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern().horizontal_off().header_off())
        .to_string();

    assert_eq!(
        table,
        concat!(
            "┌───┬──────────┬──────────┬──────────┐\n",
            "│ N │ column 0 │ column 1 │ column 2 │\n",
            "│ 0 │   0-0    │   0-1    │   0-2    │\n",
            "│ 1 │   1-0    │   1-1    │   1-2    │\n",
            "│ 2 │   2-0    │   2-1    │   2-2    │\n",
            "└───┴──────────┴──────────┴──────────┘\n",
        )
    );
}

#[test]
fn style_frame_changes() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern().top_off().bottom_off().horizontal_off())
        .to_string();

    assert_eq!(
        table,
        concat!(
            "│ N │ column 0 │ column 1 │ column 2 │\n",
            "├───┼──────────┼──────────┼──────────┤\n",
            "│ 0 │   0-0    │   0-1    │   0-2    │\n",
            "│ 1 │   1-0    │   1-1    │   1-2    │\n",
            "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        )
    );
}

#[test]
fn custom_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(
            Style::blank()
                .bottom('*')
                .bottom_intersection('\'')
                .vertical('\'')
                .horizontal('`')
                .header('`')
                .inner_intersection('\''),
        )
        .to_string();

    assert_eq!(
        table,
        concat!(
            " N ' column 0 ' column 1 ' column 2 \n",
            "````````````````````````````````````\n",
            " 0 '   0-0    '   0-1    '   0-2    \n",
            "```'``````````'``````````'``````````\n",
            " 1 '   1-0    '   1-1    '   1-2    \n",
            "```'``````````'``````````'``````````\n",
            " 2 '   2-0    '   2-1    '   2-2    \n",
            "***'**********'**********'**********\n",
        )
    );
}

#[test]
fn style_single_cell() {
    let data = create_vector::<0, 0>();
    let table = Table::new(&data).with(Style::ascii()).to_string();

    assert_eq!(table, concat!("+---+\n", "| N |\n", "+---+\n"));

    let table = Table::new(&data).with(Style::blank()).to_string();

    assert_eq!(table, " N \n");
}

#[test]
fn top_border_override_first_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(BorderText::first("-Table"))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "-Table---------+----------+\n",
            "| N | column 0 | column 1 |\n",
            "+---+----------+----------+\n",
            "| 0 |   0-0    |   0-1    |\n",
            "+---+----------+----------+\n",
            "| 1 |   1-0    |   1-1    |\n",
            "+---+----------+----------+\n",
        )
    );
}

#[test]
fn top_border_override_last_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(BorderText::last("-Table"))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "+---+----------+----------+\n",
            "| N | column 0 | column 1 |\n",
            "+---+----------+----------+\n",
            "| 0 |   0-0    |   0-1    |\n",
            "+---+----------+----------+\n",
            "| 1 |   1-0    |   1-1    |\n",
            "-Table---------+----------+\n",
        )
    );
}

#[test]
fn top_border_override_new_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(BorderText::new(1, "-Table"))
        .with(BorderText::new(2, "-Table"))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "+---+----------+----------+\n",
            "| N | column 0 | column 1 |\n",
            "-Table---------+----------+\n",
            "| 0 |   0-0    |   0-1    |\n",
            "-Table---------+----------+\n",
            "| 1 |   1-0    |   1-1    |\n",
            "+---+----------+----------+\n",
        )
    );
}

#[test]
fn top_border_override_new_doesnt_panic_when_index_is_invalid() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(BorderText::new(100, "-Table"))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "+---+----------+----------+\n",
            "| N | column 0 | column 1 |\n",
            "+---+----------+----------+\n",
            "| 0 |   0-0    |   0-1    |\n",
            "+---+----------+----------+\n",
            "| 1 |   1-0    |   1-1    |\n",
            "+---+----------+----------+\n",
        )
    );
}

#[test]
fn top_override_doesnt_work_with_style_with_no_top_border_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(BorderText::first("-Table"))
        .to_string();

    assert_eq!(
        table,
        concat!(
            " N | column 0 | column 1 \n",
            "---+----------+----------\n",
            " 0 |   0-0    |   0-1    \n",
            " 1 |   1-0    |   1-1    \n",
        )
    );
}

#[test]
fn top_border_override_cleared_after_restyling_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(BorderText::first("-Table"))
        .with(Style::ascii())
        .to_string();

    assert_eq!(
        table,
        concat!(
            "+---+----------+----------+\n",
            "| N | column 0 | column 1 |\n",
            "+---+----------+----------+\n",
            "| 0 |   0-0    |   0-1    |\n",
            "+---+----------+----------+\n",
            "| 1 |   1-0    |   1-1    |\n",
            "+---+----------+----------+\n",
        )
    );
}

#[test]
fn top_border_override_with_big_string_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(BorderText::first(
            "-Tableeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee1231",
        ))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "-Tableeeeeeeeeeeeeeeeeeeeee\n",
            "| N | column 0 | column 1 |\n",
            "+---+----------+----------+\n",
            "| 0 |   0-0    |   0-1    |\n",
            "+---+----------+----------+\n",
            "| 1 |   1-0    |   1-1    |\n",
            "+---+----------+----------+\n",
        )
    );
}

#[test]
fn empty_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::empty())
        .with(Modify::new(Segment::all()).with(Padding::zero()))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "Ncolumn 0column 1column 2\n",
            "0  0-0     0-1     0-2   \n",
            "1  1-0     1-1     1-2   \n",
            "2  2-0     2-1     2-2   \n",
        )
    );
}

#[test]
fn single_column_style() {
    let data = create_vector::<2, 0>();
    let table = Table::new(&data).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌───┐\n",
            "│ N │\n",
            "├───┤\n",
            "│ 0 │\n",
            "├───┤\n",
            "│ 1 │\n",
            "└───┘\n",
        )
    );

    let table = Table::new(&data).with(Style::blank()).to_string();

    assert_eq!(table, concat!(" N \n", " 0 \n", " 1 \n"));
}

#[test]
fn single_column_last_row_style() {
    let data = create_vector::<3, 0>();
    let table = Table::new(&data)
        .with(Style::re_structured_text())
        .to_string();

    assert_eq!(
        table,
        concat!("===\n", " N \n", "===\n", " 0 \n", " 1 \n", " 2 \n", "===\n")
    );
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
        Style::empty().top('-'),
        concat!(
            "---------------------------------\n",
            " N  column 0  column 1  column 2 \n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
        ),
    );
    test_style!(
        Style::empty().bottom('-'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
            "---------------------------------\n",
        ),
    );
    test_style!(
        Style::empty().left('-'),
        concat!(
            "- N  column 0  column 1  column 2 \n",
            "- 0    0-0       0-1       0-2    \n",
            "- 1    1-0       1-1       1-2    \n",
            "- 2    2-0       2-1       2-2    \n",
        ),
    );
    test_style!(
        Style::empty().right('-'),
        concat!(
            " N  column 0  column 1  column 2 -\n",
            " 0    0-0       0-1       0-2    -\n",
            " 1    1-0       1-1       1-2    -\n",
            " 2    2-0       2-1       2-2    -\n",
        ),
    );
    test_style!(
        Style::empty().horizontal('-'),
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
        Style::empty().header('-'),
        concat!(
            " N  column 0  column 1  column 2 \n",
            "---------------------------------\n",
            " 0    0-0       0-1       0-2    \n",
            " 1    1-0       1-1       1-2    \n",
            " 2    2-0       2-1       2-2    \n",
        ),
    );
    test_style!(
        Style::empty().vertical('-'),
        concat!(
            " N - column 0 - column 1 - column 2 \n",
            " 0 -   0-0    -   0-1    -   0-2    \n",
            " 1 -   1-0    -   1-1    -   1-2    \n",
            " 2 -   2-0    -   2-1    -   2-2    \n",
        ),
    );

    // Combinations

    test_style!(
        Style::empty().top('-').bottom('+'),
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
        Style::empty().top('-').left('+'),
        concat!(
            "+---------------------------------\n",
            "+ N  column 0  column 1  column 2 \n",
            "+ 0    0-0       0-1       0-2    \n",
            "+ 1    1-0       1-1       1-2    \n",
            "+ 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::empty().top('-').right('+'),
        concat!(
            "---------------------------------+\n",
            " N  column 0  column 1  column 2 +\n",
            " 0    0-0       0-1       0-2    +\n",
            " 1    1-0       1-1       1-2    +\n",
            " 2    2-0       2-1       2-2    +\n",
        )
    );
    test_style!(
        Style::empty().top('-').horizontal('+'),
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
        Style::empty().top('-').vertical('+'),
        concat!(
            "---+----------+----------+----------\n",
            " N + column 0 + column 1 + column 2 \n",
            " 0 +   0-0    +   0-1    +   0-2    \n",
            " 1 +   1-0    +   1-1    +   1-2    \n",
            " 2 +   2-0    +   2-1    +   2-2    \n",
        )
    );
    test_style!(
        Style::empty().top('-').header('+'),
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
        Style::empty().bottom('-').top('+'),
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
        Style::empty().bottom('-').left('+'),
        concat!(
            "+ N  column 0  column 1  column 2 \n",
            "+ 0    0-0       0-1       0-2    \n",
            "+ 1    1-0       1-1       1-2    \n",
            "+ 2    2-0       2-1       2-2    \n",
            "+---------------------------------\n",
        )
    );
    test_style!(
        Style::empty().bottom('-').right('+'),
        concat!(
            " N  column 0  column 1  column 2 +\n",
            " 0    0-0       0-1       0-2    +\n",
            " 1    1-0       1-1       1-2    +\n",
            " 2    2-0       2-1       2-2    +\n",
            "---------------------------------+\n",
        )
    );
    test_style!(
        Style::empty().bottom('-').vertical('+'),
        concat!(
            " N + column 0 + column 1 + column 2 \n",
            " 0 +   0-0    +   0-1    +   0-2    \n",
            " 1 +   1-0    +   1-1    +   1-2    \n",
            " 2 +   2-0    +   2-1    +   2-2    \n",
            "---+----------+----------+----------\n",
        )
    );
    test_style!(
        Style::empty().bottom('-').horizontal('+'),
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
        Style::empty().bottom('-').header('+'),
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
        Style::empty().left('-').top('+'),
        concat!(
            "++++++++++++++++++++++++++++++++++\n",
            "- N  column 0  column 1  column 2 \n",
            "- 0    0-0       0-1       0-2    \n",
            "- 1    1-0       1-1       1-2    \n",
            "- 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::empty().left('-').bottom('+'),
        concat!(
            "- N  column 0  column 1  column 2 \n",
            "- 0    0-0       0-1       0-2    \n",
            "- 1    1-0       1-1       1-2    \n",
            "- 2    2-0       2-1       2-2    \n",
            "++++++++++++++++++++++++++++++++++\n",
        )
    );
    test_style!(
        Style::empty().left('-').right('+'),
        concat!(
            "- N  column 0  column 1  column 2 +\n",
            "- 0    0-0       0-1       0-2    +\n",
            "- 1    1-0       1-1       1-2    +\n",
            "- 2    2-0       2-1       2-2    +\n",
        )
    );
    test_style!(
        Style::empty().left('-').vertical('+'),
        concat!(
            "- N + column 0 + column 1 + column 2 \n",
            "- 0 +   0-0    +   0-1    +   0-2    \n",
            "- 1 +   1-0    +   1-1    +   1-2    \n",
            "- 2 +   2-0    +   2-1    +   2-2    \n",
        )
    );
    test_style!(
        Style::empty().left('-').horizontal('+'),
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
        Style::empty().left('-').header('+'),
        concat!(
            "- N  column 0  column 1  column 2 \n",
            " +++++++++++++++++++++++++++++++++\n",
            "- 0    0-0       0-1       0-2    \n",
            "- 1    1-0       1-1       1-2    \n",
            "- 2    2-0       2-1       2-2    \n",
        )
    );

    test_style!(
        Style::empty().right('-').top('+'),
        concat!(
            "++++++++++++++++++++++++++++++++++\n",
            " N  column 0  column 1  column 2 -\n",
            " 0    0-0       0-1       0-2    -\n",
            " 1    1-0       1-1       1-2    -\n",
            " 2    2-0       2-1       2-2    -\n",
        )
    );
    test_style!(
        Style::empty().right('-').bottom('+'),
        concat!(
            " N  column 0  column 1  column 2 -\n",
            " 0    0-0       0-1       0-2    -\n",
            " 1    1-0       1-1       1-2    -\n",
            " 2    2-0       2-1       2-2    -\n",
            "++++++++++++++++++++++++++++++++++\n",
        )
    );
    test_style!(
        Style::empty().right('-').left('+'),
        concat!(
            "+ N  column 0  column 1  column 2 -\n",
            "+ 0    0-0       0-1       0-2    -\n",
            "+ 1    1-0       1-1       1-2    -\n",
            "+ 2    2-0       2-1       2-2    -\n",
        )
    );
    test_style!(
        Style::empty().right('-').vertical('+'),
        concat!(
            " N + column 0 + column 1 + column 2 -\n",
            " 0 +   0-0    +   0-1    +   0-2    -\n",
            " 1 +   1-0    +   1-1    +   1-2    -\n",
            " 2 +   2-0    +   2-1    +   2-2    -\n",
        )
    );
    test_style!(
        Style::empty().right('-').horizontal('+'),
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
        Style::empty().right('-').header('+'),
        concat!(
            " N  column 0  column 1  column 2 -\n",
            "+++++++++++++++++++++++++++++++++ \n",
            " 0    0-0       0-1       0-2    -\n",
            " 1    1-0       1-1       1-2    -\n",
            " 2    2-0       2-1       2-2    -\n",
        )
    );

    test_style!(
        Style::empty().vertical('-').top('+'),
        concat!(
            "++++++++++++++++++++++++++++++++++++\n",
            " N - column 0 - column 1 - column 2 \n",
            " 0 -   0-0    -   0-1    -   0-2    \n",
            " 1 -   1-0    -   1-1    -   1-2    \n",
            " 2 -   2-0    -   2-1    -   2-2    \n",
        )
    );
    test_style!(
        Style::empty().vertical('-').bottom('+'),
        concat!(
            " N - column 0 - column 1 - column 2 \n",
            " 0 -   0-0    -   0-1    -   0-2    \n",
            " 1 -   1-0    -   1-1    -   1-2    \n",
            " 2 -   2-0    -   2-1    -   2-2    \n",
            "++++++++++++++++++++++++++++++++++++\n",
        )
    );
    test_style!(
        Style::empty().vertical('-').left('+'),
        concat!(
            "+ N - column 0 - column 1 - column 2 \n",
            "+ 0 -   0-0    -   0-1    -   0-2    \n",
            "+ 1 -   1-0    -   1-1    -   1-2    \n",
            "+ 2 -   2-0    -   2-1    -   2-2    \n",
        )
    );
    test_style!(
        Style::empty().vertical('-').right('+'),
        concat!(
            " N - column 0 - column 1 - column 2 +\n",
            " 0 -   0-0    -   0-1    -   0-2    +\n",
            " 1 -   1-0    -   1-1    -   1-2    +\n",
            " 2 -   2-0    -   2-1    -   2-2    +\n",
        )
    );
    test_style!(
        Style::empty().vertical('-').horizontal('+'),
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
        Style::empty().vertical('-').header('+'),
        concat!(
            " N - column 0 - column 1 - column 2 \n",
            "++++++++++++++++++++++++++++++++++++\n",
            " 0 -   0-0    -   0-1    -   0-2    \n",
            " 1 -   1-0    -   1-1    -   1-2    \n",
            " 2 -   2-0    -   2-1    -   2-2    \n",
        )
    );

    test_style!(
        Style::empty().horizontal('-').top('+'),
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
        Style::empty().horizontal('-').bottom('+'),
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
        Style::empty().horizontal('-').left('+'),
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
        Style::empty().horizontal('-').right('+'),
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
        Style::empty().horizontal('-').vertical('+'),
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
        Style::empty().horizontal('-').header('+'),
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
        Style::empty().header('-').top('+'),
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
        Style::empty().header('-').bottom('+'),
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
        Style::empty().header('-').left('+'),
        concat!(
            "+ N  column 0  column 1  column 2 \n",
            " ---------------------------------\n",
            "+ 0    0-0       0-1       0-2    \n",
            "+ 1    1-0       1-1       1-2    \n",
            "+ 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::empty().header('-').right('+'),
        concat!(
            " N  column 0  column 1  column 2 +\n",
            "--------------------------------- \n",
            " 0    0-0       0-1       0-2    +\n",
            " 1    1-0       1-1       1-2    +\n",
            " 2    2-0       2-1       2-2    +\n",
        )
    );
    test_style!(
        Style::empty().header('-').vertical('+'),
        concat!(
            " N + column 0 + column 1 + column 2 \n",
            "---+----------+----------+----------\n",
            " 0 +   0-0    +   0-1    +   0-2    \n",
            " 1 +   1-0    +   1-1    +   1-2    \n",
            " 2 +   2-0    +   2-1    +   2-2    \n",
        )
    );
    test_style!(
        Style::empty().header('-').horizontal('+'),
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
        Style::empty()
            .top('-')
            .bottom('+')
            .left('|')
            .right('*')
            .horizontal('x')
            .header('x')
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

    let full_style = Style::empty()
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
        .bottom_right_corner('%')
        .inner_intersection('+')
        .header_intersection('m');
    test_style!(
        full_style.clone(),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
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
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().bottom('q'),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq\n",
        )
    );
    test_style!(
        full_style.clone().left('w'),
        concat!(
            "w---!----------!----------!----------.\n",
            "w N # column 0 # column 1 # column 2 *\n",
            "w,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$\n",
            "w 0 #   0-0    #   0-1    #   0-2    *\n",
            "wxxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "w 1 #   1-0    #   1-1    #   1-2    *\n",
            "wxxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "w 2 #   2-0    #   2-1    #   2-2    *\n",
            "w+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().right('i'),
        concat!(
            ";---!----------!----------!----------i\n",
            "| N # column 0 # column 1 # column 2 i\n",
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,i\n",
            "| 0 #   0-0    #   0-1    #   0-2    i\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxxi\n",
            "| 1 #   1-0    #   1-1    #   1-2    i\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxxi\n",
            "| 2 #   2-0    #   2-1    #   2-2    i\n",
            "?+++@++++++++++@++++++++++@++++++++++i\n",
        )
    );
    test_style!(
        full_style.clone().horizontal('q'),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "q,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,q\n",
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
            "=qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
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
    test_style!(Style::empty().top('-').top_off(), empty_table);
    test_style!(Style::empty().bottom('-').bottom_off(), empty_table);
    test_style!(Style::empty().right('-').right_off(), empty_table);
    test_style!(Style::empty().left('-').left_off(), empty_table);
    test_style!(Style::empty().horizontal('-').horizontal_off(), empty_table);
    test_style!(Style::empty().vertical('-').vertical_off(), empty_table);
    test_style!(Style::empty().header('-').header_off(), empty_table);

    test_style!(
        full_style.clone().top_off(),
        concat!(
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().bottom_off(),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
        )
    );
    test_style!(
        full_style.clone().right_off(),
        concat!(
            ";---!----------!----------!----------\n",
            "| N # column 0 # column 1 # column 2 \n",
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,\n",
            "| 0 #   0-0    #   0-1    #   0-2    \n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx\n",
            "| 1 #   1-0    #   1-1    #   1-2    \n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx\n",
            "| 2 #   2-0    #   2-1    #   2-2    \n",
            "?+++@++++++++++@++++++++++@++++++++++\n",
        )
    );
    test_style!(
        full_style.clone().left_off(),
        concat!(
            "---!----------!----------!----------.\n",
            " N # column 0 # column 1 # column 2 *\n",
            ",,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$\n",
            " 0 #   0-0    #   0-1    #   0-2    *\n",
            "xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            " 1 #   1-0    #   1-1    #   1-2    *\n",
            "xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            " 2 #   2-0    #   2-1    #   2-2    *\n",
            "+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
    test_style!(
        full_style.clone().horizontal_off(),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$\n",
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
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 0 #   0-0    #   0-1    #   0-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
}

#[test]
fn single_cell_style() {
    let table = Builder::from_iter(&[[""]])
        .build()
        .with(Style::modern())
        .to_string();

    assert_eq!(
        table,
        "┌──┐\n\
         │  │\n\
         └──┘\n"
    );
}

#[test]
fn border_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(Modify::new(Rows::single(1)).with(Border::filled('*').top('#')))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "+---+----------+----------+\n",
            "| N | column 0 | column 1 |\n",
            "*###*##########*##########*\n",
            "* 0 *   0-0    *   0-1    *\n",
            "***************************\n",
            "| 1 |   1-0    |   1-1    |\n",
            "+---+----------+----------+\n",
        )
    );

    let table = Table::new(&data)
        .with(Style::empty())
        .with(Modify::new(Rows::single(1)).with(Border::filled('*').top('#')))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "  N   column 0   column 1  \n",
            "*###*##########*##########*\n",
            "* 0 *   0-0    *   0-1    *\n",
            "***************************\n",
            "  1     1-0        1-1     \n",
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn border_colored_test() {
    use owo_colors::OwoColorize;
    use tabled::style::Symbol;

    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(
            Modify::new(Rows::single(1)).with(
                Border::filled(Symbol::ansi('*'.blue().to_string()).unwrap())
                    .top(Symbol::ansi('#'.truecolor(12, 220, 100).to_string()).unwrap()),
            ),
        )
        .to_string();

    assert_eq!(
        table,
        concat!(
            "+---+----------+----------+\n",
            "| N | column 0 | column 1 |\n",
            "\u{1b}[34m*\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[34m*\u{1b}[0m\n",
            "\u{1b}[34m*\u{1b}[0m 0 \u{1b}[34m*\u{1b}[0m   0-0    \u{1b}[34m*\u{1b}[0m   0-1    \u{1b}[34m*\u{1b}[0m\n",
            "\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\n",
            "| 1 |   1-0    |   1-1    |\n",
            "+---+----------+----------+\n",
        )
    );

    let table = Table::new(&data)
        .with(Style::empty())
        .with(
            Modify::new(Rows::single(1)).with(
                Border::filled(Symbol::ansi('*'.blue().to_string()).unwrap())
                    .top(Symbol::ansi('#'.truecolor(12, 220, 100).to_string()).unwrap()),
            ),
        )
        .to_string();

    assert_eq!(
        table,
        concat!(
            "  N   column 0   column 1  \n",
            "\u{1b}[34m*\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[38;2;12;220;100m#\u{1b}[0m\u{1b}[34m*\u{1b}[0m\n",
            "\u{1b}[34m*\u{1b}[0m 0 \u{1b}[34m*\u{1b}[0m   0-0    \u{1b}[34m*\u{1b}[0m   0-1    \u{1b}[34m*\u{1b}[0m\n",
            "\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\u{1b}[34m*\u{1b}[0m\n",
            "  1     1-0        1-1     \n",
        )
    );
}

#[test]
fn style_frame_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(Highlight::new(Rows::single(1), Style::modern().frame()))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "+---+----------+----------+\n",
            "| N | column 0 | column 1 |\n",
            "┌─────────────────────────┐\n",
            "│ 0 |   0-0    |   0-1    │\n",
            "└─────────────────────────┘\n",
            "| 1 |   1-0    |   1-1    |\n",
            "+---+----------+----------+\n",
        )
    );

    let table = Table::new(&data)
        .with(Style::blank())
        .with(Highlight::new(Rows::single(0), Style::extended().frame()))
        .with(Highlight::new(Rows::single(2), Style::extended().frame()))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "╔═════════════════════════╗\n",
            "║ N   column 0   column 1 ║\n",
            "╚═════════════════════════╝\n",
            "  0     0-0        0-1     \n",
            "╔═════════════════════════╗\n",
            "║ 1     1-0        1-1    ║\n",
            "╚═════════════════════════╝\n",
        )
    );
}

#[test]
fn single_column_horizontal_off_test() {
    let data = create_vector::<3, 0>();
    let table = Table::new(&data)
        .with(Style::ascii().header_off().horizontal_off().vertical_off())
        .to_string();

    assert_eq!(
        table,
        concat!("+---+\n", "| N |\n", "| 0 |\n", "| 1 |\n", "| 2 |\n", "+---+\n",)
    );
}

#[test]
fn single_row_test() {
    let data = create_vector::<0, 3>();
    let table = Table::new(&data).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌───┬──────────┬──────────┬──────────┐\n",
            "│ N │ column 0 │ column 1 │ column 2 │\n",
            "└───┴──────────┴──────────┴──────────┘\n",
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn style_with_color_test() {
    use owo_colors::OwoColorize;

    let style = Style::ascii()
        .left(tabled::style::Symbol::ansi('['.red().to_string()).unwrap())
        .right(tabled::style::Symbol::ansi(']'.red().to_string()).unwrap())
        .top(tabled::style::Symbol::ansi('-'.blue().to_string()).unwrap())
        .bottom(tabled::style::Symbol::ansi('-'.blue().to_string()).unwrap())
        .vertical(tabled::style::Symbol::ansi('|'.yellow().to_string()).unwrap())
        .inner_intersection(tabled::style::Symbol::ansi('+'.purple().to_string()).unwrap())
        .header_intersection(tabled::style::Symbol::ansi('+'.purple().to_string()).unwrap());

    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(style).to_string();

    assert_eq!(table, "\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[33m|\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[33m|\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[33m|\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\n\u{1b}[31m[\u{1b}[0m N \u{1b}[33m|\u{1b}[0m column 0 \u{1b}[33m|\u{1b}[0m column 1 \u{1b}[33m|\u{1b}[0m column 2 \u{1b}[31m]\u{1b}[0m\n\u{1b}[31m[\u{1b}[0m---\u{1b}[35m+\u{1b}[0m----------\u{1b}[35m+\u{1b}[0m----------\u{1b}[35m+\u{1b}[0m----------\u{1b}[31m]\u{1b}[0m\n\u{1b}[31m[\u{1b}[0m 0 \u{1b}[33m|\u{1b}[0m   0-0    \u{1b}[33m|\u{1b}[0m   0-1    \u{1b}[33m|\u{1b}[0m   0-2    \u{1b}[31m]\u{1b}[0m\n\u{1b}[31m[\u{1b}[0m---\u{1b}[35m+\u{1b}[0m----------\u{1b}[35m+\u{1b}[0m----------\u{1b}[35m+\u{1b}[0m----------\u{1b}[31m]\u{1b}[0m\n\u{1b}[31m[\u{1b}[0m 1 \u{1b}[33m|\u{1b}[0m   1-0    \u{1b}[33m|\u{1b}[0m   1-1    \u{1b}[33m|\u{1b}[0m   1-2    \u{1b}[31m]\u{1b}[0m\n\u{1b}[31m[\u{1b}[0m---\u{1b}[35m+\u{1b}[0m----------\u{1b}[35m+\u{1b}[0m----------\u{1b}[35m+\u{1b}[0m----------\u{1b}[31m]\u{1b}[0m\n\u{1b}[31m[\u{1b}[0m 2 \u{1b}[33m|\u{1b}[0m   2-0    \u{1b}[33m|\u{1b}[0m   2-1    \u{1b}[33m|\u{1b}[0m   2-2    \u{1b}[31m]\u{1b}[0m\n\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[33m|\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[33m|\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[33m|\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\u{1b}[34m-\u{1b}[0m\n");
}
