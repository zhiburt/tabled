use crate::util::create_vector;
use tabled::{Border, Highlight, Style, Table};

mod util;

#[test]
fn style_highlingt_cell() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::pseudo())
        .with(Highlight::cell(
            0,
            0,
            Border::full('+', '+', '+', '+', '+', '+', '+', '+'),
        ))
        .with(Highlight::cell(
            1,
            1,
            Border::full('*', '*', '*', '*', '*', '*', '*', '*'),
        ))
        .to_string();

    let expected = concat!(
        "+++++──────────┬──────────┬──────────┐\n",
        "+ N + column 0 │ column 1 │ column 2 │\n",
        "++++************──────────┼──────────┤\n",
        "│ 0 *   0-0    *   0-1    │   0-2    │\n",
        "├───************──────────┼──────────┤\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_highlingt_row() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::pseudo())
        .with(Highlight::row(
            0,
            Border::full('+', '+', '+', '+', '+', '+', '+', '+'),
        ))
        .with(Highlight::row(
            3,
            Border::full('*', '*', '*', '*', '*', '*', '*', '*'),
        ))
        .to_string();

    let expected = concat!(
        "++++++++++++++++++++++++++++++++++++++\n",
        "+ N │ column 0 │ column 1 │ column 2 +\n",
        "++++++++++++++++++++++++++++++++++++++\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "**************************************\n",
        "* 2 │   2-0    │   2-1    │   2-2    *\n",
        "**************************************\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_highlingt_column() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::pseudo())
        .with(Highlight::column(
            0,
            Border::full('+', '+', '+', '+', '+', '+', '+', '+'),
        ))
        .with(Highlight::column(
            2,
            Border::full('*', '*', '*', '*', '*', '*', '*', '*'),
        ))
        .to_string();

    let expected = concat!(
        "+++++──────────************──────────┐\n",
        "+ N + column 0 * column 1 * column 2 │\n",
        "+───+──────────*──────────*──────────┤\n",
        "+ 0 +   0-0    *   0-1    *   0-2    │\n",
        "+───+──────────*──────────*──────────┤\n",
        "+ 1 +   1-0    *   1-1    *   1-2    │\n",
        "+───+──────────*──────────*──────────┤\n",
        "+ 2 +   2-0    *   2-1    *   2-2    │\n",
        "+++++──────────************──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_highlingt_row_range() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::pseudo())
        .with(Highlight::row_range(
            1,
            3,
            Border::full('+', '+', '+', '+', '+', '+', '+', '+'),
        ))
        .to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "++++++++++++++++++++++++++++++++++++++\n",
        "+ 0 │   0-0    │   0-1    │   0-2    +\n",
        "+───┼──────────┼──────────┼──────────+\n",
        "+ 1 │   1-0    │   1-1    │   1-2    +\n",
        "++++++++++++++++++++++++++++++++++++++\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_highlingt_column_range() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::pseudo())
        .with(Highlight::column_range(
            0,
            2,
            Border::full('+', '+', '+', '+', '+', '+', '+', '+'),
        ))
        .to_string();

    let expected = concat!(
        "++++++++++++++++──────────┬──────────┐\n",
        "+ N │ column 0 + column 1 │ column 2 │\n",
        "+───┼──────────+──────────┼──────────┤\n",
        "+ 0 │   0-0    +   0-1    │   0-2    │\n",
        "+───┼──────────+──────────┼──────────┤\n",
        "+ 1 │   1-0    +   1-1    │   1-2    │\n",
        "+───┼──────────+──────────┼──────────┤\n",
        "+ 2 │   2-0    +   2-1    │   2-2    │\n",
        "++++++++++++++++──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}
