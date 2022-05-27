use crate::util::create_vector;
use tabled::{
    object::{Cell, Object, Rows, Segment},
    style::{Border, Style},
    Alignment, Footer, Header, Highlight, Modify, Panel, Table,
};

mod util;

#[test]
fn panel_has_no_style_by_default() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Panel("Linux Distributions", 0))
        .to_string();

    // todo: it would be better if vertical split was not set in panel line
    // it is because it has right border printed
    let expected = concat!(
        "Linux Distributions                 \n",
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn highligt_panel() {
    let border = Border::new('#', '#', '#', '#', '#', '#', '#', '#');
    let table = Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 0))
        .with(Style::psql())
        .with(Highlight::new(Cell(0, 0), border.clone()))
        .to_string();

    // todo: it would be better if vertical split was not set in panel line
    // it is because it has right border printed
    let expected = concat!(
        "#####                                \n",
        "#Linux Distributions                 \n",
        "#####----------+----------+----------\n",
        "  N | column 0 | column 1 | column 2 \n",
        "  0 |   0-0    |   0-1    |   0-2    \n",
        "  1 |   1-0    |   1-1    |   1-2    \n",
        "  2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);

    let table = Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 0))
        .with(Style::psql())
        .with(Highlight::new(Cell(0, 0), border.clone()))
        .with(Highlight::new(Cell(0, 1), border.clone()))
        .with(Highlight::new(Cell(0, 2), border.clone()))
        .with(Highlight::new(Cell(0, 3), border))
        .to_string();

    // todo: it would be better if vertical split was not set in panel line
    // it is because it has right border printed
    let expected = concat!(
        "######################################\n",
        "#Linux Distributions                 #\n",
        "######################################\n",
        "  N | column 0 | column 1 | column 2  \n",
        "  0 |   0-0    |   0-1    |   0-2     \n",
        "  1 |   1-0    |   1-1    |   1-2     \n",
        "  2 |   2-0    |   2-1    |   2-2     \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn top_panel() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::psql())
        .to_string();

    let expected = concat!(
        "        Linux Distributions         \n",
        "---+----------+----------+----------\n",
        " N | column 0 | column 1 | column 2 \n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn bottom_panel() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Panel("Linux Distributions", data.len() + 1))
        .with(Modify::new(Rows::last()).with(Alignment::center()))
        .with(Style::psql())
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
        "        Linux Distributions         \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn inner_panel() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 2))
        .with(Modify::new(Rows::new(2..)).with(Alignment::center()))
        .with(Style::psql())
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        "        Linux Distributions         \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn header() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Header("Linux Distributions"))
        .with(Style::psql())
        .with(Modify::new(Rows::new(0..1)).with(Alignment::center()))
        .to_string();

    let expected = concat!(
        "        Linux Distributions         \n",
        "---+----------+----------+----------\n",
        " N | column 0 | column 1 | column 2 \n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    println!("{table}");

    assert_eq!(table, expected);
}

#[test]
fn footer() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Header("Linux Distributions"))
        .with(Footer("The end"))
        .with(Style::psql())
        .with(Modify::new(Rows::first().and(Rows::last())).with(Alignment::center()))
        .to_string();

    let expected = concat!(
        "        Linux Distributions         \n",
        "---+----------+----------+----------\n",
        " N | column 0 | column 1 | column 2 \n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
        "              The end               \n",
    );

    println!("{table}");

    assert_eq!(table, expected);
}

#[test]
fn panel_style_uses_most_left_and_right_cell_styles() {
    let table = Table::new(&[(0, 1)])
        .with(tabled::Panel("Numbers", 0))
        .with(Style::modern())
        .to_string();

    let expected = concat!(
        "┌─────┬─────┐\n",
        "│Numbers    │\n",
        "├─────┼─────┤\n",
        "│ i32 │ i32 │\n",
        "├─────┼─────┤\n",
        "│  0  │  1  │\n",
        "└─────┴─────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn panel_style_change() {
    let table = Table::new(&[(0, 1)])
        .with(tabled::Panel("Numbers", 0))
        .with(
            Style::modern()
                .top_intersection('─')
                .header_intersection('┬'),
        )
        .with(Modify::new(Cell(0, 0)).with(Alignment::center()))
        .to_string();

    let expected = concat!(
        "┌───────────┐\n",
        "│  Numbers  │\n",
        "├─────┬─────┤\n",
        "│ i32 │ i32 │\n",
        "├─────┼─────┤\n",
        "│  0  │  1  │\n",
        "└─────┴─────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn panel_in_single_column() {
    let table = Table::new(&[(0)])
        .with(tabled::Panel("Numbers", 0))
        .with(Style::modern())
        .to_string();

    let expected = concat!(
        "┌───────┐\n",
        "│Numbers│\n",
        "├───────┤\n",
        "│  i32  │\n",
        "├───────┤\n",
        "│   0   │\n",
        "└───────┘\n",
    );

    assert_eq!(table, expected);
}
