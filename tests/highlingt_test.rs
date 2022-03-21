use crate::util::create_vector;
use tabled::{
    style::Border, Cell, Columns, Frame, Full, Highlight, Object, Rows, Segment, Style, Table, Tabled,
};

mod util;

#[test]
#[should_panic]
fn highlingt_object_exceeds_bounderies() {
    let data = create_vector::<3, 3>();
    let _table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Cell(1000, 0), Border::single('+')))
        .to_string();
}

#[test]
fn highlingt_empty_table() {
    #[derive(Tabled)]
    struct EmptyStruct;

    let data:[EmptyStruct; 0] = [];
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Full, Border::single('+')))
        .to_string();
    
    assert_eq!(table, "");
}

#[test]
fn highlingt_cell() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Cell(0, 0), Border::single('+')))
        .with(Highlight::new(Cell(1, 1), Border::single('*')))
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
fn highlingt_row() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Rows::single(0), Border::single('+')))
        .with(Highlight::new(Rows::single(3), Border::single('*')))
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
fn highlingt_column() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Columns::single(0), Border::single('+')))
        .with(Highlight::new(Columns::single(2), Border::single('*')))
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
fn highlingt_row_range() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Rows::new(1..3), Border::single('+')))
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
fn highlingt_column_range() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Columns::new(..2), Border::single('+')))
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

#[test]
fn highlingt_frame() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(
            Frame,
            Border::single('+')
                .top_left_corner('*')
                .top_right_corner('#')
                .bottom_left_corner('@')
                .bottom_right_corner('.'),
        ))
        .to_string();

    let expected = concat!(
        "*++++++++++++++++++++++++++++++++++++#\n",
        "+ N │ column 0 │ column 1 │ column 2 +\n",
        "+───*+++++++++++++++++++++#──────────+\n",
        "+ 0 +   0-0    │   0-1    +   0-2    +\n",
        "+───+──────────┼──────────+──────────+\n",
        "+ 1 +   1-0    │   1-1    +   1-2    +\n",
        "+───@+++++++++++++++++++++.──────────+\n",
        "+ 2 │   2-0    │   2-1    │   2-2    +\n",
        "@++++++++++++++++++++++++++++++++++++.\n",
    );

    println!("{}", table);

    assert_eq!(table, expected);
}

#[test]
fn highlingt_full() {
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(
            Full,
            Border::single('+')
                .top_left_corner('*')
                .top_right_corner('#')
                .bottom_left_corner('@')
                .bottom_right_corner('.'),
        ))
        .to_string();

    let expected = concat!(
        "*++++++++++++++++++++++++++++++++++++#\n",
        "+ N │ column 0 │ column 1 │ column 2 +\n",
        "+───┼──────────┼──────────┼──────────+\n",
        "+ 0 │   0-0    │   0-1    │   0-2    +\n",
        "+───┼──────────┼──────────┼──────────+\n",
        "+ 1 │   1-0    │   1-1    │   1-2    +\n",
        "+───┼──────────┼──────────┼──────────+\n",
        "+ 2 │   2-0    │   2-1    │   2-2    +\n",
        "@++++++++++++++++++++++++++++++++++++.\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn highlingt_single_column() {
    let data = create_vector::<3, 0>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(
            Cell(0, 0),
            Border::default().left('*').top('x'),
        ))
        .with(Highlight::new(Rows::new(1..3), Border::default().left('n')))
        .to_string();

    let expected = concat!(
        "┌xxx┐\n",
        "* N │\n",
        "├───┤\n",
        "n 0 │\n",
        "n───┤\n",
        "n 1 │\n",
        "├───┤\n",
        "│ 2 │\n",
        "└───┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn highlingt_complex_figures() {
    macro_rules! test_highlight {
        ($object:expr, $expected:expr,) => {
            let data = create_vector::<3, 3>();
            let border = Border::single('+')
                .top_left_corner('*')
                .top_right_corner('#')
                .bottom_left_corner('@')
                .bottom_right_corner('.');

            let table = Table::new(&data)
                .with(Style::modern())
                .with(Highlight::new($object, border))
                .to_string();

            eprintln!("{}", table);

            assert_eq!(table, $expected);
        };
    }

    test_highlight!(
        Full.not(Segment::new(2.., 1..3)),
        concat!(
            "*++++++++++++++++++++++++++++++++++++#\n",
            "+ N │ column 0 │ column 1 │ column 2 +\n",
            "+───┼──────────┼──────────┼──────────+\n",
            "+ 0 │   0-0    │   0-1    │   0-2    +\n",
            "+───*+++++++++++++++++++++#──────────+\n",
            "+ 1 +   1-0    │   1-1    +   1-2    +\n",
            "+───+──────────┼──────────+──────────+\n",
            "+ 2 +   2-0    │   2-1    +   2-2    +\n",
            "@+++.──────────┴──────────@++++++++++.\n",
        ),
    );

    test_highlight!(
        Full.not(Segment::new(0..1, 1..3)).not(Columns::single(0)),
        concat!(
            "┌───┬──────────┬──────────*++++++++++#\n",
            "│ N │ column 0 │ column 1 + column 2 +\n",
            "├───*+++++++++++++++++++++.──────────+\n",
            "│ 0 +   0-0    │   0-1    │   0-2    +\n",
            "├───+──────────┼──────────┼──────────+\n",
            "│ 1 +   1-0    │   1-1    │   1-2    +\n",
            "├───+──────────┼──────────┼──────────+\n",
            "│ 2 +   2-0    │   2-1    │   2-2    +\n",
            "└───@++++++++++++++++++++++++++++++++.\n",
        ),
    );

    test_highlight!(
        Full.not(Segment::new(0..1, 1..3)),
        concat!(
            "*+++#──────────┬──────────*++++++++++#\n",
            "+ N + column 0 │ column 1 + column 2 +\n",
            "+───@+++++++++++++++++++++.──────────+\n",
            "+ 0 │   0-0    │   0-1    │   0-2    +\n",
            "+───┼──────────┼──────────┼──────────+\n",
            "+ 1 │   1-0    │   1-1    │   1-2    +\n",
            "+───┼──────────┼──────────┼──────────+\n",
            "+ 2 │   2-0    │   2-1    │   2-2    +\n",
            "@++++++++++++++++++++++++++++++++++++.\n",
        ),
    );

    test_highlight!(
        Full.not(Segment::new(1..2, 1..3)),
        concat!(
            "*++++++++++++++++++++++++++++++++++++#\n",
            "+ N │ column 0 │ column 1 │ column 2 +\n",
            "+───*+++++++++++++++++++++#──────────+\n",
            "+ 0 +   0-0    │   0-1    +   0-2    +\n",
            "+───@+++++++++++++++++++++.──────────+\n",
            "+ 1 │   1-0    │   1-1    │   1-2    +\n",
            "+───┼──────────┼──────────┼──────────+\n",
            "+ 2 │   2-0    │   2-1    │   2-2    +\n",
            "@++++++++++++++++++++++++++++++++++++.\n",
        ),
    );

    test_highlight!(
        Cell(0, 0).and(Cell(3, 3)).and(Cell(0, 3)).and(Cell(3, 0)),
        concat!(
            "*+++#──────────┬──────────*++++++++++#\n",
            "+ N + column 0 │ column 1 + column 2 +\n",
            "@+++.──────────┼──────────@++++++++++.\n",
            "│ 0 │   0-0    │   0-1    │   0-2    │\n",
            "├───┼──────────┼──────────┼──────────┤\n",
            "│ 1 │   1-0    │   1-1    │   1-2    │\n",
            "*+++#──────────┼──────────*++++++++++#\n",
            "+ 2 +   2-0    │   2-1    +   2-2    +\n",
            "@+++.──────────┴──────────@++++++++++.\n",
        ),
    );

    test_highlight!(
        Rows::single(0).and(Rows::single(3)),
        concat!(
            "*++++++++++++++++++++++++++++++++++++#\n",
            "+ N │ column 0 │ column 1 │ column 2 +\n",
            "@++++++++++++++++++++++++++++++++++++.\n",
            "│ 0 │   0-0    │   0-1    │   0-2    │\n",
            "├───┼──────────┼──────────┼──────────┤\n",
            "│ 1 │   1-0    │   1-1    │   1-2    │\n",
            "*++++++++++++++++++++++++++++++++++++#\n",
            "+ 2 │   2-0    │   2-1    │   2-2    +\n",
            "@++++++++++++++++++++++++++++++++++++.\n",
        ),
    );

    test_highlight!(
        Columns::single(0).and(Columns::single(3)),
        concat!(
            "*+++#──────────┬──────────*++++++++++#\n",
            "+ N + column 0 │ column 1 + column 2 +\n",
            "+───+──────────┼──────────+──────────+\n",
            "+ 0 +   0-0    │   0-1    +   0-2    +\n",
            "+───+──────────┼──────────+──────────+\n",
            "+ 1 +   1-0    │   1-1    +   1-2    +\n",
            "+───+──────────┼──────────+──────────+\n",
            "+ 2 +   2-0    │   2-1    +   2-2    +\n",
            "@+++.──────────┴──────────@++++++++++.\n",
        ),
    );

    test_highlight!(
        Full.not(Cell(3, 1).and(Cell(3, 2))),
        concat!(
            "*++++++++++++++++++++++++++++++++++++#\n",
            "+ N │ column 0 │ column 1 │ column 2 +\n",
            "+───┼──────────┼──────────┼──────────+\n",
            "+ 0 │   0-0    │   0-1    │   0-2    +\n",
            "+───┼──────────┼──────────┼──────────+\n",
            "+ 1 │   1-0    │   1-1    │   1-2    +\n",
            "+───*+++++++++++++++++++++#──────────+\n",
            "+ 2 +   2-0    │   2-1    +   2-2    +\n",
            "@+++.──────────┴──────────@++++++++++.\n",
        ),
    );

    test_highlight!(
        Rows::single(0)
            .and(Cell(1, 1).and(Cell(1, 2)))
            .and(Cell(2, 3)),
        concat!(
            "*++++++++++++++++++++++++++++++++++++#\n",
            "+ N │ column 0 │ column 1 │ column 2 +\n",
            "@+++#──────────┼──────────*++++++++++.\n",
            "│ 0 +   0-0    │   0-1    +   0-2    │\n",
            "├───@+++++++++++++++++++++*++++++++++#\n",
            "│ 1 │   1-0    │   1-1    +   1-2    +\n",
            "├───┼──────────┼──────────@++++++++++.\n",
            "│ 2 │   2-0    │   2-1    │   2-2    │\n",
            "└───┴──────────┴──────────┴──────────┘\n",
        ),
    );
}
