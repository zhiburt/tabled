use crate::util::create_vector;
use tabled::style::TopBorderText;
use tabled::{object::Full, Modify, Padding, Style, Table, TableIteratorExt};

mod util;

#[test]
fn default_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::ascii()).to_string();

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
    let table = Table::new(&data).with(Style::psql()).to_string();

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
    let table = Table::new(&data).with(Style::github_markdown()).to_string();

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
    let table = Table::new(&data).with(Style::modern()).to_string();

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
    let table = Table::new(&data)
        .with(Style::modern().horizontal_off())
        .to_string();

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
    let table = Table::new(&data).with(Style::blank()).to_string();

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
    let table = Table::new(&data).with(Style::extended()).to_string();

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
    let table = Table::new(&data).with(Style::dots()).to_string();

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
        .with(Style::re_structured_text())
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
        .with(Style::modern().horizontal_off().header_off())
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
        .with(Style::modern().top_off().bottom_off().horizontal_off())
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

    let expected = concat!(
        " N ' column 0 ' column 1 ' column 2 \n",
        "````````````````````````````````````\n",
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
    let table = Table::new(&data).with(Style::ascii()).to_string();

    let expected = concat!("+---+\n", "| N |\n", "+---+\n",);

    assert_eq!(table, expected);

    let table = Table::new(&data).with(Style::blank()).to_string();

    let expected = " N \n";

    assert_eq!(table, expected);
}

#[test]
fn top_border_override_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
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
        .with(Style::psql())
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
        .with(Style::ascii())
        .with(TopBorderText::new("-Table"))
        .with(Style::ascii())
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
        .with(Style::empty())
        .with(Modify::new(Full).with(Padding::zero()))
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
fn single_column_style() {
    let data = create_vector::<2, 0>();
    let table = Table::new(&data).with(Style::modern()).to_string();

    let expected = concat!(
        "┌───┐\n",
        "│ N │\n",
        "├───┤\n",
        "│ 0 │\n",
        "├───┤\n",
        "│ 1 │\n",
        "└───┘\n",
    );

    assert_eq!(table, expected);

    let table = Table::new(&data).with(Style::blank()).to_string();

    let expected = concat!(" N \n", " 0 \n", " 1 \n",);

    assert_eq!(table, expected);
}

#[test]
fn single_column_last_row_style() {
    let data = create_vector::<3, 0>();
    let table = Table::new(&data)
        .with(Style::re_structured_text())
        .to_string();

    let expected = concat!("===\n", " N \n", "===\n", " 0 \n", " 1 \n", " 2 \n", "===\n",);

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
            "++++++++++++++++++++++++++++++++++\n",
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
            "++++++++++++++++++++++++++++++++++\n",
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
            "+---------------------------------\n",
            "+ 0    0-0       0-1       0-2    \n",
            "+ 1    1-0       1-1       1-2    \n",
            "+ 2    2-0       2-1       2-2    \n",
        )
    );
    test_style!(
        Style::empty().header('-').right('+'),
        concat!(
            " N  column 0  column 1  column 2 +\n",
            "---------------------------------+\n",
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
        .left_header_intersection('o')
        .right_header_intersection('w')
        .header_intersection('m');
    test_style!(
        full_style.clone(),
        concat!(
            ";---!----------!----------!----------.\n",
            "| N # column 0 # column 1 # column 2 *\n",
            "o,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,w\n",
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
            "o,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,w\n",
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
            "o,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,w\n",
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
            "w,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,w\n",
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
            "o,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,i\n",
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
            "o,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,w\n",
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
            "o,,,q,,,,,,,,,,q,,,,,,,,,,q,,,,,,,,,,w\n",
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
            "o,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,w\n",
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
            "o,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,w\n",
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
            "o,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,\n",
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
            ",,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,w\n",
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
            "o,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,w\n",
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
            "o,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,w\n",
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
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 1 #   1-0    #   1-1    #   1-2    *\n",
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$\n",
            "| 2 #   2-0    #   2-1    #   2-2    *\n",
            "?+++@++++++++++@++++++++++@++++++++++%\n",
        )
    );
}
