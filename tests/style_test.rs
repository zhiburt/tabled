use std::iter::FromIterator;

use crate::util::{create_vector, static_table};

use tabled::{
    builder::Builder,
    object::{Cell, Rows, Segment},
    style::{Border, BorderText, StyleConfig},
    Highlight, Modify, Padding, Span, Style, Table, TableIteratorExt,
};

mod util;

#[test]
fn default_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::ascii()).to_string();

    assert_eq!(
        table,
        static_table!(
            "+---+----------+----------+----------+"
            "| N | column 0 | column 1 | column 2 |"
            "+---+----------+----------+----------+"
            "| 0 |   0-0    |   0-1    |   0-2    |"
            "+---+----------+----------+----------+"
            "| 1 |   1-0    |   1-1    |   1-2    |"
            "+---+----------+----------+----------+"
            "| 2 |   2-0    |   2-1    |   2-2    |"
            "+---+----------+----------+----------+"
        )
    );
}

#[test]
fn psql_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::psql()).to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |   2-1    |   2-2    "
        )
    );
}

#[test]
fn github_markdown_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::github_markdown()).to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 | column 1 | column 2 |"
            "|---+----------+----------+----------|"
            "| 0 |   0-0    |   0-1    |   0-2    |"
            "| 1 |   1-0    |   1-1    |   1-2    |"
            "| 2 |   2-0    |   2-1    |   2-2    |"
        )
    );
}

#[test]
fn pseudo_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::modern()).to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───┬──────────┬──────────┬──────────┐"
            "│ N │ column 0 │ column 1 │ column 2 │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 2 │   2-0    │   2-1    │   2-2    │"
            "└───┴──────────┴──────────┴──────────┘"
        )
    );
}

#[test]
fn rounded_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::rounded()).to_string();

    assert_eq!(
        table,
        static_table!(
            "╭───┬──────────┬──────────┬──────────╮"
            "│ N │ column 0 │ column 1 │ column 2 │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "│ 2 │   2-0    │   2-1    │   2-2    │"
            "╰───┴──────────┴──────────┴──────────╯"
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
        static_table!(
            "┌───┬──────────┬──────────┬──────────┐"
            "│ N │ column 0 │ column 1 │ column 2 │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "│ 2 │   2-0    │   2-1    │   2-2    │"
            "└───┴──────────┴──────────┴──────────┘"
        )
    );
}

#[test]
fn blank_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::blank()).to_string();

    assert_eq!(
        table,
        static_table!(
            " N   column 0   column 1   column 2 "
            " 0     0-0        0-1        0-2    "
            " 1     1-0        1-1        1-2    "
            " 2     2-0        2-1        2-2    "
        )
    );
}

#[test]
fn extended_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::extended()).to_string();

    assert_eq!(
        table,
        static_table!(
            "╔═══╦══════════╦══════════╦══════════╗"
            "║ N ║ column 0 ║ column 1 ║ column 2 ║"
            "╠═══╬══════════╬══════════╬══════════╣"
            "║ 0 ║   0-0    ║   0-1    ║   0-2    ║"
            "╠═══╬══════════╬══════════╬══════════╣"
            "║ 1 ║   1-0    ║   1-1    ║   1-2    ║"
            "╠═══╬══════════╬══════════╬══════════╣"
            "║ 2 ║   2-0    ║   2-1    ║   2-2    ║"
            "╚═══╩══════════╩══════════╩══════════╝"
        )
    );
}

#[test]
fn ascii_dots_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::dots()).to_string();

    assert_eq!(
        table,
        static_table!(
            "......................................"
            ": N : column 0 : column 1 : column 2 :"
            ":...:..........:..........:..........:"
            ": 0 :   0-0    :   0-1    :   0-2    :"
            ":...:..........:..........:..........:"
            ": 1 :   1-0    :   1-1    :   1-2    :"
            ":...:..........:..........:..........:"
            ": 2 :   2-0    :   2-1    :   2-2    :"
            ":...:..........:..........:..........:"
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
        static_table!(
            "=== ========== ========== =========="
            " N   column 0   column 1   column 2 "
            "=== ========== ========== =========="
            " 0     0-0        0-1        0-2    "
            " 1     1-0        1-1        1-2    "
            " 2     2-0        2-1        2-2    "
            "=== ========== ========== =========="
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
        static_table!(
            "┌───┬──────────┬──────────┬──────────┐"
            "│ N │ column 0 │ column 1 │ column 2 │"
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "│ 2 │   2-0    │   2-1    │   2-2    │"
            "└───┴──────────┴──────────┴──────────┘"
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
        static_table!(
            "│ N │ column 0 │ column 1 │ column 2 │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "│ 2 │   2-0    │   2-1    │   2-2    │"
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
        static_table!(
            " N ' column 0 ' column 1 ' column 2 "
            "````````````````````````````````````"
            " 0 '   0-0    '   0-1    '   0-2    "
            "```'``````````'``````````'``````````"
            " 1 '   1-0    '   1-1    '   1-2    "
            "```'``````````'``````````'``````````"
            " 2 '   2-0    '   2-1    '   2-2    "
            "***'**********'**********'**********"
        )
    );
}

#[test]
fn style_single_cell() {
    let data = create_vector::<0, 0>();
    let table = Table::new(&data).with(Style::ascii()).to_string();

    assert_eq!(table, static_table!("+---+" "| N |" "+---+"));

    let table = Table::new(&data).with(Style::blank()).to_string();

    assert_eq!(table, " N ");
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
        static_table!(
            "-Table---------+----------+"
            "| N | column 0 | column 1 |"
            "+---+----------+----------+"
            "| 0 |   0-0    |   0-1    |"
            "+---+----------+----------+"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
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
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "+---+----------+----------+"
            "| 0 |   0-0    |   0-1    |"
            "+---+----------+----------+"
            "| 1 |   1-0    |   1-1    |"
            "-Table---------+----------+"
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
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "-Table---------+----------+"
            "| 0 |   0-0    |   0-1    |"
            "-Table---------+----------+"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
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
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "+---+----------+----------+"
            "| 0 |   0-0    |   0-1    |"
            "+---+----------+----------+"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
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
        static_table!(
            " N | column 0 | column 1 "
            "---+----------+----------"
            " 0 |   0-0    |   0-1    "
            " 1 |   1-0    |   1-1    "
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
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "+---+----------+----------+"
            "| 0 |   0-0    |   0-1    |"
            "+---+----------+----------+"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
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
        static_table!(
            "-Tableeeeeeeeeeeeeeeeeeeeee"
            "| N | column 0 | column 1 |"
            "+---+----------+----------+"
            "| 0 |   0-0    |   0-1    |"
            "+---+----------+----------+"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
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
        static_table!(
            "Ncolumn 0column 1column 2"
            "0  0-0     0-1     0-2   "
            "1  1-0     1-1     1-2   "
            "2  2-0     2-1     2-2   "
        )
    );
}

#[test]
fn single_column_style() {
    let data = create_vector::<2, 0>();
    let table = Table::new(&data).with(Style::modern()).to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───┐"
            "│ N │"
            "├───┤"
            "│ 0 │"
            "├───┤"
            "│ 1 │"
            "└───┘"
        )
    );

    let table = Table::new(&data).with(Style::blank()).to_string();

    assert_eq!(table, static_table!(" N " " 0 " " 1 "));
}

#[test]
fn single_column_last_row_style() {
    let data = create_vector::<3, 0>();
    let table = Table::new(&data)
        .with(Style::re_structured_text())
        .to_string();

    assert_eq!(
        table,
        static_table!("===" " N " "===" " 0 " " 1 " " 2 " "===")
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
        static_table!(
            "---------------------------------"
            " N  column 0  column 1  column 2 "
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
        ),
    );
    test_style!(
        Style::empty().bottom('-'),
        static_table!(
            " N  column 0  column 1  column 2 "
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "---------------------------------"
        ),
    );
    test_style!(
        Style::empty().left('-'),
        static_table!(
            "- N  column 0  column 1  column 2 "
            "- 0    0-0       0-1       0-2    "
            "- 1    1-0       1-1       1-2    "
            "- 2    2-0       2-1       2-2    "
        ),
    );
    test_style!(
        Style::empty().right('-'),
        static_table!(
            " N  column 0  column 1  column 2 -"
            " 0    0-0       0-1       0-2    -"
            " 1    1-0       1-1       1-2    -"
            " 2    2-0       2-1       2-2    -"
        ),
    );
    test_style!(
        Style::empty().horizontal('-'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            "---------------------------------"
            " 1    1-0       1-1       1-2    "
            "---------------------------------"
            " 2    2-0       2-1       2-2    "
        ),
    );
    test_style!(
        Style::empty().header('-'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
        ),
    );
    test_style!(
        Style::empty().vertical('-'),
        static_table!(
            " N - column 0 - column 1 - column 2 "
            " 0 -   0-0    -   0-1    -   0-2    "
            " 1 -   1-0    -   1-1    -   1-2    "
            " 2 -   2-0    -   2-1    -   2-2    "
        ),
    );

    // Combinations

    test_style!(
        Style::empty().top('-').bottom('+'),
        static_table!(
            "---------------------------------"
            " N  column 0  column 1  column 2 "
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "+++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().top('-').left('+'),
        static_table!(
            "+---------------------------------"
            "+ N  column 0  column 1  column 2 "
            "+ 0    0-0       0-1       0-2    "
            "+ 1    1-0       1-1       1-2    "
            "+ 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().top('-').right('+'),
        static_table!(
            "---------------------------------+"
            " N  column 0  column 1  column 2 +"
            " 0    0-0       0-1       0-2    +"
            " 1    1-0       1-1       1-2    +"
            " 2    2-0       2-1       2-2    +"
        )
    );
    test_style!(
        Style::empty().top('-').horizontal('+'),
        static_table!(
            "---------------------------------"
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            "+++++++++++++++++++++++++++++++++"
            " 1    1-0       1-1       1-2    "
            "+++++++++++++++++++++++++++++++++"
            " 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().top('-').vertical('+'),
        static_table!(
            "---+----------+----------+----------"
            " N + column 0 + column 1 + column 2 "
            " 0 +   0-0    +   0-1    +   0-2    "
            " 1 +   1-0    +   1-1    +   1-2    "
            " 2 +   2-0    +   2-1    +   2-2    "
        )
    );
    test_style!(
        Style::empty().top('-').header('+'),
        static_table!(
            "---------------------------------"
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
        )
    );

    test_style!(
        Style::empty().bottom('-').top('+'),
        static_table!(
            "+++++++++++++++++++++++++++++++++"
            " N  column 0  column 1  column 2 "
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "---------------------------------"
        )
    );
    test_style!(
        Style::empty().bottom('-').left('+'),
        static_table!(
            "+ N  column 0  column 1  column 2 "
            "+ 0    0-0       0-1       0-2    "
            "+ 1    1-0       1-1       1-2    "
            "+ 2    2-0       2-1       2-2    "
            "+---------------------------------"
        )
    );
    test_style!(
        Style::empty().bottom('-').right('+'),
        static_table!(
            " N  column 0  column 1  column 2 +"
            " 0    0-0       0-1       0-2    +"
            " 1    1-0       1-1       1-2    +"
            " 2    2-0       2-1       2-2    +"
            "---------------------------------+"
        )
    );
    test_style!(
        Style::empty().bottom('-').vertical('+'),
        static_table!(
            " N + column 0 + column 1 + column 2 "
            " 0 +   0-0    +   0-1    +   0-2    "
            " 1 +   1-0    +   1-1    +   1-2    "
            " 2 +   2-0    +   2-1    +   2-2    "
            "---+----------+----------+----------"
        )
    );
    test_style!(
        Style::empty().bottom('-').horizontal('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            "+++++++++++++++++++++++++++++++++"
            " 1    1-0       1-1       1-2    "
            "+++++++++++++++++++++++++++++++++"
            " 2    2-0       2-1       2-2    "
            "---------------------------------"
        )
    );
    test_style!(
        Style::empty().bottom('-').header('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "---------------------------------"
        )
    );

    test_style!(
        Style::empty().left('-').top('+'),
        static_table!(
            "++++++++++++++++++++++++++++++++++"
            "- N  column 0  column 1  column 2 "
            "- 0    0-0       0-1       0-2    "
            "- 1    1-0       1-1       1-2    "
            "- 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().left('-').bottom('+'),
        static_table!(
            "- N  column 0  column 1  column 2 "
            "- 0    0-0       0-1       0-2    "
            "- 1    1-0       1-1       1-2    "
            "- 2    2-0       2-1       2-2    "
            "++++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().left('-').right('+'),
        static_table!(
            "- N  column 0  column 1  column 2 +"
            "- 0    0-0       0-1       0-2    +"
            "- 1    1-0       1-1       1-2    +"
            "- 2    2-0       2-1       2-2    +"
        )
    );
    test_style!(
        Style::empty().left('-').vertical('+'),
        static_table!(
            "- N + column 0 + column 1 + column 2 "
            "- 0 +   0-0    +   0-1    +   0-2    "
            "- 1 +   1-0    +   1-1    +   1-2    "
            "- 2 +   2-0    +   2-1    +   2-2    "
        )
    );
    test_style!(
        Style::empty().left('-').horizontal('+'),
        static_table!(
            "- N  column 0  column 1  column 2 "
            "++++++++++++++++++++++++++++++++++"
            "- 0    0-0       0-1       0-2    "
            "++++++++++++++++++++++++++++++++++"
            "- 1    1-0       1-1       1-2    "
            "++++++++++++++++++++++++++++++++++"
            "- 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().left('-').header('+'),
        static_table!(
            "- N  column 0  column 1  column 2 "
            " +++++++++++++++++++++++++++++++++"
            "- 0    0-0       0-1       0-2    "
            "- 1    1-0       1-1       1-2    "
            "- 2    2-0       2-1       2-2    "
        )
    );

    test_style!(
        Style::empty().right('-').top('+'),
        static_table!(
            "++++++++++++++++++++++++++++++++++"
            " N  column 0  column 1  column 2 -"
            " 0    0-0       0-1       0-2    -"
            " 1    1-0       1-1       1-2    -"
            " 2    2-0       2-1       2-2    -"
        )
    );
    test_style!(
        Style::empty().right('-').bottom('+'),
        static_table!(
            " N  column 0  column 1  column 2 -"
            " 0    0-0       0-1       0-2    -"
            " 1    1-0       1-1       1-2    -"
            " 2    2-0       2-1       2-2    -"
            "++++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().right('-').left('+'),
        static_table!(
            "+ N  column 0  column 1  column 2 -"
            "+ 0    0-0       0-1       0-2    -"
            "+ 1    1-0       1-1       1-2    -"
            "+ 2    2-0       2-1       2-2    -"
        )
    );
    test_style!(
        Style::empty().right('-').vertical('+'),
        static_table!(
            " N + column 0 + column 1 + column 2 -"
            " 0 +   0-0    +   0-1    +   0-2    -"
            " 1 +   1-0    +   1-1    +   1-2    -"
            " 2 +   2-0    +   2-1    +   2-2    -"
        )
    );
    test_style!(
        Style::empty().right('-').horizontal('+'),
        static_table!(
            " N  column 0  column 1  column 2 -"
            "++++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    -"
            "++++++++++++++++++++++++++++++++++"
            " 1    1-0       1-1       1-2    -"
            "++++++++++++++++++++++++++++++++++"
            " 2    2-0       2-1       2-2    -"
        )
    );
    test_style!(
        Style::empty().right('-').header('+'),
        static_table!(
            " N  column 0  column 1  column 2 -"
            "+++++++++++++++++++++++++++++++++ "
            " 0    0-0       0-1       0-2    -"
            " 1    1-0       1-1       1-2    -"
            " 2    2-0       2-1       2-2    -"
        )
    );

    test_style!(
        Style::empty().vertical('-').top('+'),
        static_table!(
            "++++++++++++++++++++++++++++++++++++"
            " N - column 0 - column 1 - column 2 "
            " 0 -   0-0    -   0-1    -   0-2    "
            " 1 -   1-0    -   1-1    -   1-2    "
            " 2 -   2-0    -   2-1    -   2-2    "
        )
    );
    test_style!(
        Style::empty().vertical('-').bottom('+'),
        static_table!(
            " N - column 0 - column 1 - column 2 "
            " 0 -   0-0    -   0-1    -   0-2    "
            " 1 -   1-0    -   1-1    -   1-2    "
            " 2 -   2-0    -   2-1    -   2-2    "
            "++++++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().vertical('-').left('+'),
        static_table!(
            "+ N - column 0 - column 1 - column 2 "
            "+ 0 -   0-0    -   0-1    -   0-2    "
            "+ 1 -   1-0    -   1-1    -   1-2    "
            "+ 2 -   2-0    -   2-1    -   2-2    "
        )
    );
    test_style!(
        Style::empty().vertical('-').right('+'),
        static_table!(
            " N - column 0 - column 1 - column 2 +"
            " 0 -   0-0    -   0-1    -   0-2    +"
            " 1 -   1-0    -   1-1    -   1-2    +"
            " 2 -   2-0    -   2-1    -   2-2    +"
        )
    );
    test_style!(
        Style::empty().vertical('-').horizontal('+'),
        static_table!(
            " N - column 0 - column 1 - column 2 "
            "++++++++++++++++++++++++++++++++++++"
            " 0 -   0-0    -   0-1    -   0-2    "
            "++++++++++++++++++++++++++++++++++++"
            " 1 -   1-0    -   1-1    -   1-2    "
            "++++++++++++++++++++++++++++++++++++"
            " 2 -   2-0    -   2-1    -   2-2    "
        )
    );
    test_style!(
        Style::empty().vertical('-').header('+'),
        static_table!(
            " N - column 0 - column 1 - column 2 "
            "++++++++++++++++++++++++++++++++++++"
            " 0 -   0-0    -   0-1    -   0-2    "
            " 1 -   1-0    -   1-1    -   1-2    "
            " 2 -   2-0    -   2-1    -   2-2    "
        )
    );

    test_style!(
        Style::empty().horizontal('-').top('+'),
        static_table!(
            "+++++++++++++++++++++++++++++++++"
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            "---------------------------------"
            " 1    1-0       1-1       1-2    "
            "---------------------------------"
            " 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().horizontal('-').bottom('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            "---------------------------------"
            " 1    1-0       1-1       1-2    "
            "---------------------------------"
            " 2    2-0       2-1       2-2    "
            "+++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().horizontal('-').left('+'),
        static_table!(
            "+ N  column 0  column 1  column 2 "
            "+---------------------------------"
            "+ 0    0-0       0-1       0-2    "
            "+---------------------------------"
            "+ 1    1-0       1-1       1-2    "
            "+---------------------------------"
            "+ 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().horizontal('-').right('+'),
        static_table!(
            " N  column 0  column 1  column 2 +"
            "---------------------------------+"
            " 0    0-0       0-1       0-2    +"
            "---------------------------------+"
            " 1    1-0       1-1       1-2    +"
            "---------------------------------+"
            " 2    2-0       2-1       2-2    +"
        )
    );
    test_style!(
        Style::empty().horizontal('-').vertical('+'),
        static_table!(
            " N + column 0 + column 1 + column 2 "
            "---+----------+----------+----------"
            " 0 +   0-0    +   0-1    +   0-2    "
            "---+----------+----------+----------"
            " 1 +   1-0    +   1-1    +   1-2    "
            "---+----------+----------+----------"
            " 2 +   2-0    +   2-1    +   2-2    "
        )
    );
    test_style!(
        Style::empty().horizontal('-').header('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            "---------------------------------"
            " 1    1-0       1-1       1-2    "
            "---------------------------------"
            " 2    2-0       2-1       2-2    "
        )
    );

    test_style!(
        Style::empty().header('-').top('+'),
        static_table!(
            "+++++++++++++++++++++++++++++++++"
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().header('-').bottom('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "+++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().header('-').left('+'),
        static_table!(
            "+ N  column 0  column 1  column 2 "
            " ---------------------------------"
            "+ 0    0-0       0-1       0-2    "
            "+ 1    1-0       1-1       1-2    "
            "+ 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().header('-').right('+'),
        static_table!(
            " N  column 0  column 1  column 2 +"
            "--------------------------------- "
            " 0    0-0       0-1       0-2    +"
            " 1    1-0       1-1       1-2    +"
            " 2    2-0       2-1       2-2    +"
        )
    );
    test_style!(
        Style::empty().header('-').vertical('+'),
        static_table!(
            " N + column 0 + column 1 + column 2 "
            "---+----------+----------+----------"
            " 0 +   0-0    +   0-1    +   0-2    "
            " 1 +   1-0    +   1-1    +   1-2    "
            " 2 +   2-0    +   2-1    +   2-2    "
        )
    );
    test_style!(
        Style::empty().header('-').horizontal('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            "+++++++++++++++++++++++++++++++++"
            " 1    1-0       1-1       1-2    "
            "+++++++++++++++++++++++++++++++++"
            " 2    2-0       2-1       2-2    "
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
        static_table!(
            "|---#----------#----------#----------*"
            "| N # column 0 # column 1 # column 2 *"
            "xxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxx"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "xxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxx"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "xxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxx"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "|+++#++++++++++#++++++++++#++++++++++*"
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
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );

    // Overwrite intersections and corners

    test_style!(
        full_style.clone().top('q'),
        static_table!(
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
            "| N # column 0 # column 1 # column 2 *"
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().bottom('q'),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
        )
    );
    test_style!(
        full_style.clone().left('w'),
        static_table!(
            "w---!----------!----------!----------."
            "w N # column 0 # column 1 # column 2 *"
            "w,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$"
            "w 0 #   0-0    #   0-1    #   0-2    *"
            "wxxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "w 1 #   1-0    #   1-1    #   1-2    *"
            "wxxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "w 2 #   2-0    #   2-1    #   2-2    *"
            "w+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().right('i'),
        static_table!(
            ";---!----------!----------!----------i"
            "| N # column 0 # column 1 # column 2 i"
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,i"
            "| 0 #   0-0    #   0-1    #   0-2    i"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxxi"
            "| 1 #   1-0    #   1-1    #   1-2    i"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxxi"
            "| 2 #   2-0    #   2-1    #   2-2    i"
            "?+++@++++++++++@++++++++++@++++++++++i"
        )
    );
    test_style!(
        full_style.clone().horizontal('q'),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            "q,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,q"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().vertical('q'),
        static_table!(
            ";---q----------q----------q----------."
            "| N q column 0 q column 1 q column 2 *"
            "=,,,q,,,,,,,,,,q,,,,,,,,,,q,,,,,,,,,,$"
            "| 0 q   0-0    q   0-1    q   0-2    *"
            "=xxxqxxxxxxxxxxqxxxxxxxxxxqxxxxxxxxxx$"
            "| 1 q   1-0    q   1-1    q   1-2    *"
            "=xxxqxxxxxxxxxxqxxxxxxxxxxqxxxxxxxxxx$"
            "| 2 q   2-0    q   2-1    q   2-2    *"
            "?+++q++++++++++q++++++++++q++++++++++%"
        )
    );
    test_style!(
        full_style.clone().header('q'),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            "=qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq$"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );

    // Turn off borders

    let empty_table = static_table!(
        " N  column 0  column 1  column 2 "
        " 0    0-0       0-1       0-2    "
        " 1    1-0       1-1       1-2    "
        " 2    2-0       2-1       2-2    "
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
        static_table!(
            "| N # column 0 # column 1 # column 2 *"
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().bottom_off(),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
        )
    );
    test_style!(
        full_style.clone().right_off(),
        static_table!(
            ";---!----------!----------!----------"
            "| N # column 0 # column 1 # column 2 "
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,"
            "| 0 #   0-0    #   0-1    #   0-2    "
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx"
            "| 1 #   1-0    #   1-1    #   1-2    "
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx"
            "| 2 #   2-0    #   2-1    #   2-2    "
            "?+++@++++++++++@++++++++++@++++++++++"
        )
    );
    test_style!(
        full_style.clone().left_off(),
        static_table!(
            "---!----------!----------!----------."
            " N # column 0 # column 1 # column 2 *"
            ",,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$"
            " 0 #   0-0    #   0-1    #   0-2    *"
            "xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            " 1 #   1-0    #   1-1    #   1-2    *"
            "xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            " 2 #   2-0    #   2-1    #   2-2    *"
            "+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().horizontal_off(),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            "=,,,m,,,,,,,,,,m,,,,,,,,,,m,,,,,,,,,,$"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().vertical_off(),
        static_table!(
            ";---------------------------------."
            "| N  column 0  column 1  column 2 *"
            "=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,$"
            "| 0    0-0       0-1       0-2    *"
            "=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx$"
            "| 1    1-0       1-1       1-2    *"
            "=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx$"
            "| 2    2-0       2-1       2-2    *"
            "?+++++++++++++++++++++++++++++++++%"
        )
    );
    test_style!(
        full_style.header_off(),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
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
         └──┘"
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
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "*###*##########*##########*"
            "* 0 *   0-0    *   0-1    *"
            "***************************"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
        )
    );

    let table = Table::new(&data)
        .with(Style::empty())
        .with(Modify::new(Rows::single(1)).with(Border::filled('*').top('#')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "  N   column 0   column 1  "
            "*###*##########*##########*"
            "* 0 *   0-0    *   0-1    *"
            "***************************"
            "  1     1-0        1-1     "
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn border_colored_test() {
    use owo_colors::OwoColorize;
    use tabled::style::{ColoredBorder, Symbol};

    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(
            Modify::new(Rows::single(1)).with(
                ColoredBorder::filled(Symbol::ansi('*'.blue().to_string()).unwrap())
                    .top(Symbol::ansi('#'.truecolor(12, 220, 100).to_string()).unwrap()),
            ),
        )
        .to_string();

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table),
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "*###*##########*##########*"
            "* 0 *   0-0    *   0-1    *"
            "***************************"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
        )
    );

    assert_eq!(
        table,
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m###\u{1b}[39m\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m##########\u{1b}[39m\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m##########\u{1b}[39m\u{1b}[34m*\u{1b}[39m"
            "\u{1b}[34m*\u{1b}[39m 0 \u{1b}[34m*\u{1b}[39m   0-0    \u{1b}[34m*\u{1b}[39m   0-1    \u{1b}[34m*\u{1b}[39m"
            "\u{1b}[34m***************************\u{1b}[39m"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
        )
    );

    let table = Table::new(&data)
        .with(Style::empty())
        .with(
            Modify::new(Rows::single(1)).with(
                ColoredBorder::filled(Symbol::ansi('*'.blue().to_string()).unwrap())
                    .top(Symbol::ansi('#'.truecolor(12, 220, 100).to_string()).unwrap()),
            ),
        )
        .to_string();

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table),
        static_table!(
            "  N   column 0   column 1  "
            "*###*##########*##########*"
            "* 0 *   0-0    *   0-1    *"
            "***************************"
            "  1     1-0        1-1     "
        )
    );

    assert_eq!(
        table,
        "  N   column 0   column 1  \n\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m###\u{1b}[39m\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m##########\u{1b}[39m\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m##########\u{1b}[39m\u{1b}[34m*\u{1b}[39m\n\u{1b}[34m*\u{1b}[39m 0 \u{1b}[34m*\u{1b}[39m   0-0    \u{1b}[34m*\u{1b}[39m   0-1    \u{1b}[34m*\u{1b}[39m\n\u{1b}[34m***************************\u{1b}[39m\n  1     1-0        1-1     ",
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
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "┌─────────────────────────┐"
            "│ 0 |   0-0    |   0-1    │"
            "└─────────────────────────┘"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
        )
    );

    let table = Table::new(&data)
        .with(Style::blank())
        .with(Highlight::new(Rows::single(0), Style::extended().frame()))
        .with(Highlight::new(Rows::single(2), Style::extended().frame()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "╔═════════════════════════╗"
            "║ N   column 0   column 1 ║"
            "╚═════════════════════════╝"
            "  0     0-0        0-1     "
            "╔═════════════════════════╗"
            "║ 1     1-0        1-1    ║"
            "╚═════════════════════════╝"
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
        static_table!("+---+" "| N |" "| 0 |" "| 1 |" "| 2 |" "+---+")
    );
}

#[test]
fn single_row_test() {
    let data = create_vector::<0, 3>();
    let table = Table::new(&data).with(Style::modern()).to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───┬──────────┬──────────┬──────────┐"
            "│ N │ column 0 │ column 1 │ column 2 │"
            "└───┴──────────┴──────────┴──────────┘"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn style_with_color_test() {
    use owo_colors::OwoColorize;
    use tabled::style::Symbol;

    let style: StyleConfig = Style::ascii().into();
    let mut style = style.colored();
    style
        .set_left(Some(Symbol::ansi('['.red().to_string()).unwrap()))
        .set_right(Some(Symbol::ansi(']'.red().to_string()).unwrap()))
        .set_top(Some(Symbol::ansi('-'.blue().to_string()).unwrap()))
        .set_bottom(Some(Symbol::ansi('-'.blue().to_string()).unwrap()))
        .set_vertical(Some(Symbol::ansi('|'.yellow().to_string()).unwrap()))
        .set_internal(Some(Symbol::ansi('+'.purple().to_string()).unwrap()));

    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(style).to_string();

    println!("{}", table);

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table),
        static_table!(
            "+---+----------+----------+----------+"
            "[ N | column 0 | column 1 | column 2 ]"
            "+---+----------+----------+----------+"
            "[ 0 |   0-0    |   0-1    |   0-2    ]"
            "+---+----------+----------+----------+"
            "[ 1 |   1-0    |   1-1    |   1-2    ]"
            "+---+----------+----------+----------+"
            "[ 2 |   2-0    |   2-1    |   2-2    ]"
            "+---+----------+----------+----------+"
        )
    );

    assert_eq!(table, "+\u{1b}[34m---\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\n\u{1b}[31m[\u{1b}[39m N \u{1b}[33m|\u{1b}[39m column 0 \u{1b}[33m|\u{1b}[39m column 1 \u{1b}[33m|\u{1b}[39m column 2 \u{1b}[31m]\u{1b}[39m\n+---\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------+\n\u{1b}[31m[\u{1b}[39m 0 \u{1b}[33m|\u{1b}[39m   0-0    \u{1b}[33m|\u{1b}[39m   0-1    \u{1b}[33m|\u{1b}[39m   0-2    \u{1b}[31m]\u{1b}[39m\n+---\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------+\n\u{1b}[31m[\u{1b}[39m 1 \u{1b}[33m|\u{1b}[39m   1-0    \u{1b}[33m|\u{1b}[39m   1-1    \u{1b}[33m|\u{1b}[39m   1-2    \u{1b}[31m]\u{1b}[39m\n+---\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------+\n\u{1b}[31m[\u{1b}[39m 2 \u{1b}[33m|\u{1b}[39m   2-0    \u{1b}[33m|\u{1b}[39m   2-1    \u{1b}[33m|\u{1b}[39m   2-2    \u{1b}[31m]\u{1b}[39m\n+\u{1b}[34m---\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+");
}

#[test]
fn empty_border_text_doesnt_panic_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data).with(BorderText::first("")).to_string();

    assert_eq!(
        table,
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "+---+----------+----------+"
            "| 0 |   0-0    |   0-1    |"
            "+---+----------+----------+"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
        )
    );
}

#[test]
fn span_correct_test() {
    let data = create_vector::<6, 4>();
    let table = Table::new(&data)
        .with(Modify::new(Cell(0, 3)).with(Span::column(2)))
        .with(Modify::new(Cell(1, 0)).with(Span::column(3)))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 3)).with(Span::column(2)))
        .with(Modify::new(Cell(3, 0)).with(Span::column(5)))
        .with(Modify::new(Cell(4, 1)).with(Span::column(4)))
        .with(Modify::new(Cell(5, 0)).with(Span::column(5)))
        .with(Modify::new(Cell(6, 0)).with(Span::column(5)))
        .with(Style::correct_spans())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+---+----------+----------+-----------+"
            "| N | column 0 | column 1 | column 2  |"
            "+---+----------+----------+-----+-----+"
            "|            0            | 0-2 | 0-3 |"
            "+--------------+----------+-----+-----+"
            "|      1       |   1-1    |    1-2    |"
            "+--------------+----------+-----------+"
            "|                  2                  |"
            "+---+---------------------------------+"
            "| 3 |               3-0               |"
            "+---+---------------------------------+"
            "|                  4                  |"
            "+-------------------------------------+"
            "|                  5                  |"
            "+-------------------------------------+"
        )
    );

    let table = Table::new(&data)
        .with(Modify::new(Cell(0, 0)).with(Span::column(5)))
        .with(Modify::new(Cell(1, 0)).with(Span::column(3)))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 3)).with(Span::column(2)))
        .with(Modify::new(Cell(3, 0)).with(Span::column(5)))
        .with(Modify::new(Cell(4, 1)).with(Span::column(4)))
        .with(Modify::new(Cell(5, 0)).with(Span::column(5)))
        .with(Modify::new(Cell(6, 0)).with(Span::column(5)))
        .with(Style::correct_spans())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+----------------------+"
            "|          N           |"
            "+----------+-----+-----+"
            "|    0     | 0-2 | 0-3 |"
            "+----+-----+-----+-----+"
            "| 1  | 1-1 |    1-2    |"
            "+----+-----+-----------+"
            "|          2           |"
            "+---+------------------+"
            "| 3 |       3-0        |"
            "+---+------------------+"
            "|          4           |"
            "+----------------------+"
            "|          5           |"
            "+----------------------+"
        )
    );
}

#[test]
fn style_settings_usage_test() {
    let mut data = create_vector::<3, 3>();
    data[0][1] = "a longer string".to_owned();

    let mut style: StyleConfig = Style::modern().into();
    style
        .set_internal(Some('x'))
        .set_bottom(Some('a'))
        .set_left(Some('b'))
        .set_right(None)
        .set_top(None)
        .set_top_split(None)
        .set_top_left(None)
        .set_top_right(None);

    let table = Table::new(&data).with(style).to_string();

    assert_eq!(
        table,
        static_table!(
            "b N │    column 0     │ column 1 │ column 2  "
            "├───x─────────────────x──────────x──────────┤"
            "b 0 │ a longer string │   0-1    │   0-2     "
            "├───x─────────────────x──────────x──────────┤"
            "b 1 │       1-0       │   1-1    │   1-2     "
            "├───x─────────────────x──────────x──────────┤"
            "b 2 │       2-0       │   2-1    │   2-2     "
            "└aaa┴aaaaaaaaaaaaaaaaa┴aaaaaaaaaa┴aaaaaaaaaa┘"
        )
    );

    let mut style: StyleConfig = Style::modern().into();
    style.set_bottom(None);

    let table = Table::new(&data).with(style).to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───┬─────────────────┬──────────┬──────────┐"
            "│ N │    column 0     │ column 1 │ column 2 │"
            "├───┼─────────────────┼──────────┼──────────┤"
            "│ 0 │ a longer string │   0-1    │   0-2    │"
            "├───┼─────────────────┼──────────┼──────────┤"
            "│ 1 │       1-0       │   1-1    │   1-2    │"
            "├───┼─────────────────┼──────────┼──────────┤"
            "│ 2 │       2-0       │   2-1    │   2-2    │"
        )
    );

    let mut style: StyleConfig = Style::modern().into();
    style.set_bottom(None);

    let table = Table::new(&data)
        .with(style)
        .with(Modify::new(Rows::last()).with(Border::default().bottom_left_corner('*')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───┬─────────────────┬──────────┬──────────┐"
            "│ N │    column 0     │ column 1 │ column 2 │"
            "├───┼─────────────────┼──────────┼──────────┤"
            "│ 0 │ a longer string │   0-1    │   0-2    │"
            "├───┼─────────────────┼──────────┼──────────┤"
            "│ 1 │       1-0       │   1-1    │   1-2    │"
            "├───┼─────────────────┼──────────┼──────────┤"
            "│ 2 │       2-0       │   2-1    │   2-2    │"
            "*   *                 *          *          ┘"
        )
    );
}

#[test]
fn test_default_border_usage() {
    macro_rules! test_border {
        ($modify:expr, $expected:expr) => {
            let mut data = create_vector::<3, 3>();
            data[0][1] = "a longer string".to_owned();

            let table = Table::new(&data)
                .with(Style::empty())
                .with($modify)
                .to_string();

            assert_eq!(table, $expected);
        };
    }

    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().bottom_left_corner('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
            "                    *                    "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().bottom_right_corner('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
            "                              *          "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().bottom('*')),
        static_table!(
            " N     column 0      column 1  column 2 "
            " 0  a longer string    0-1       0-2    "
            " 1        1-0          1-1       1-2    "
            " 2        2-0          2-1       2-2    "
            "                    **********          "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().bottom('*').bottom_left_corner('#')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
            "                    #**********          "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().bottom('*').bottom_right_corner('#')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
            "                    **********#          "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0       *   2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().top_left_corner('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            "                    *                    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().left('#').top_left_corner('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            "                    *                    "
            " 2        2-0       #   2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().left('#').bottom_left_corner('@').top_left_corner('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            "                    *                    "
            " 2        2-0       #   2-1       2-2    "
            "                    @                    "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1    *   2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().top_right_corner('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            "                              *          "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().right('#').top_right_corner('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            "                              *          "
            " 2        2-0          2-1    #   2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().right('#').top_right_corner('*').bottom_right_corner('@')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            "                              *          "
            " 2        2-0          2-1    #   2-2    "
            "                              @          "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::default().right('#').top_right_corner('*').bottom_left_corner('@')),
        static_table!(
            " N     column 0       column 1   column 2 "
            " 0  a longer string     0-1        0-2    "
            " 1        1-0           1-1        1-2    "
            "                               *          "
            " 2        2-0           2-1    #   2-2    "
            "                    @                     "
        )
    }
    test_border! {
        Modify::new(Cell(3, 2)).with(Border::filled('@')),
        static_table!(
            " N     column 0       column 1   column 2 "
            " 0  a longer string     0-1        0-2    "
            " 1        1-0           1-1        1-2    "
            "                    @@@@@@@@@@@@          "
            " 2        2-0       @   2-1    @   2-2    "
            "                    @@@@@@@@@@@@          "
        )
    }

    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().bottom_left_corner('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            "                    *                    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().bottom_right_corner('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            "                              *          "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().bottom('*')),
        static_table!(
            " N     column 0      column 1  column 2 "
            " 0  a longer string    0-1       0-2    "
            "                    **********          "
            " 1        1-0          1-1       1-2    "
            " 2        2-0          2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().bottom('*').bottom_left_corner('#')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            "                    #**********          "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().bottom('*').bottom_right_corner('#')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            "                    **********#          "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string *   0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().top_left_corner('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            "                    *                    "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().left('#').top_left_corner('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            "                    *                    "
            " 0  a longer string #   0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().left('#').bottom_left_corner('@').top_left_corner('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            "                    *                    "
            " 0  a longer string #   0-1       0-2    "
            "                    @                    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1    *   0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().top_right_corner('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              *          "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().right('#').top_right_corner('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              *          "
            " 0  a longer string    0-1    #   0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().right('#').top_right_corner('*').bottom_right_corner('@')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              *          "
            " 0  a longer string    0-1    #   0-2    "
            "                              @          "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::default().right('#').top_right_corner('*').bottom_left_corner('@')),
        static_table!(
            " N     column 0       column 1   column 2 "
            "                               *          "
            " 0  a longer string     0-1    #   0-2    "
            "                    @                     "
            " 1        1-0           1-1        1-2    "
            " 2        2-0           2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(1, 2)).with(Border::filled('@')),
        static_table!(
            " N     column 0       column 1   column 2 "
            "                    @@@@@@@@@@@@          "
            " 0  a longer string @   0-1    @   0-2    "
            "                    @@@@@@@@@@@@          "
            " 1        1-0           1-1        1-2    "
            " 2        2-0           2-1        2-2    "
        )
    }

    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().bottom_left_corner('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              *          "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().bottom_right_corner('*')),
        static_table!(
            " N     column 0      column 1  column 2  "
            "                                        *"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().bottom('*')),
        static_table!(
            " N     column 0      column 1  column 2 "
            "                              **********"
            " 0  a longer string    0-1       0-2    "
            " 1        1-0          1-1       1-2    "
            " 2        2-0          2-1       2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().bottom('*').bottom_left_corner('#')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              #**********"
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().bottom('*').bottom_right_corner('#')),
        static_table!(
            " N     column 0      column 1  column 2  "
            "                              **********#"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().left('*')),
        static_table!(
            " N     column 0      column 1 * column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().top_left_corner('*')),
        static_table!(
            "                              *          "
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().left('#').top_left_corner('*')),
        static_table!(
            "                              *          "
            " N     column 0      column 1 # column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().left('#').bottom_left_corner('@').top_left_corner('*')),
        static_table!(
            "                              *          "
            " N     column 0      column 1 # column 2 "
            "                              @          "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().right('*')),
        static_table!(
            " N     column 0      column 1  column 2 *"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().top_right_corner('*')),
        static_table!(
            "                                        *"
            " N     column 0      column 1  column 2  "
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().right('#').top_right_corner('*')),
        static_table!(
            "                                        *"
            " N     column 0      column 1  column 2 #"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().right('#').top_right_corner('*').bottom_right_corner('@')),
        static_table!(
            "                                        *"
            " N     column 0      column 1  column 2 #"
            "                                        @"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::default().right('#').top_right_corner('*').bottom_left_corner('@')),
        static_table!(
            "                                         *"
            " N     column 0      column 1   column 2 #"
            "                              @           "
            " 0  a longer string    0-1        0-2     "
            " 1        1-0          1-1        1-2     "
            " 2        2-0          2-1        2-2     "
        )
    }
    test_border! {
        Modify::new(Cell(0, 3)).with(Border::filled('@')),
        static_table!(
            "                              @@@@@@@@@@@@"
            " N     column 0      column 1 @ column 2 @"
            "                              @@@@@@@@@@@@"
            " 0  a longer string    0-1        0-2     "
            " 1        1-0          1-1        1-2     "
            " 2        2-0          2-1        2-2     "
        )
    }
}
