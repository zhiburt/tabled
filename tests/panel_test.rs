use tabled::{
    object::{Cell, Object, Rows, Segment},
    style::{Border, Style},
    Alignment, Footer, Header, Highlight, Modify, Panel, Table,
};

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn panel_has_no_style_by_default() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Panel("Linux Distributions", 0))
        .to_string();

    // todo: it would be better if vertical split was not set in panel line
    // it is because it has right border printed

    assert_eq!(
        table,
        static_table!(
            "Linux Distributions                 "
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |   2-1    |   2-2    "
        )
    );
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

    assert_eq!(
        table,
        static_table!(
            "#####                                "
            "#Linux Distributions                 "
            "#####----------+----------+----------"
            "  N | column 0 | column 1 | column 2 "
            "  0 |   0-0    |   0-1    |   0-2    "
            "  1 |   1-0    |   1-1    |   1-2    "
            "  2 |   2-0    |   2-1    |   2-2    "
        )
    );

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
    assert_eq!(
        table,
        static_table!(
            "######################################"
            "#Linux Distributions                 #"
            "######################################"
            "  N | column 0 | column 1 | column 2  "
            "  0 |   0-0    |   0-1    |   0-2     "
            "  1 |   1-0    |   1-1    |   1-2     "
            "  2 |   2-0    |   2-1    |   2-2     "
        )
    );
}

#[test]
fn top_panel() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::psql())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "        Linux Distributions         "
            "---+----------+----------+----------"
            " N | column 0 | column 1 | column 2 "
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |   2-1    |   2-2    "
        )
    );
}

#[test]
fn bottom_panel() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Panel("Linux Distributions", data.len() + 1))
        .with(Modify::new(Rows::last()).with(Alignment::center()))
        .with(Style::psql())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |   2-1    |   2-2    "
            "        Linux Distributions         "
        )
    );
}

#[test]
fn inner_panel() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 2))
        .with(Modify::new(Rows::new(2..)).with(Alignment::center()))
        .with(Style::psql())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |   0-0    |   0-1    |   0-2    "
            "        Linux Distributions         "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |   2-1    |   2-2    "
        )
    );
}

#[test]
fn header() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Header("Linux Distributions"))
        .with(Style::psql())
        .with(Modify::new(Rows::new(0..1)).with(Alignment::center()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "        Linux Distributions         "
            "---+----------+----------+----------"
            " N | column 0 | column 1 | column 2 "
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |   2-1    |   2-2    "
        )
    );
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

    assert_eq!(
        table,
        static_table!(
            "        Linux Distributions         "
            "---+----------+----------+----------"
            " N | column 0 | column 1 | column 2 "
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |   2-1    |   2-2    "
            "              The end               "
        )
    );
}

#[test]
fn panel_style_uses_most_left_and_right_cell_styles() {
    let table = Table::new(&[(0, 1)])
        .with(tabled::Panel("Numbers", 0))
        .with(Style::modern())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌─────┬─────┐"
            "│Numbers    │"
            "├─────┼─────┤"
            "│ i32 │ i32 │"
            "├─────┼─────┤"
            "│  0  │  1  │"
            "└─────┴─────┘"
        )
    );
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

    assert_eq!(
        table,
        static_table!(
            "┌───────────┐"
            "│  Numbers  │"
            "├─────┬─────┤" // it's different because we use a top_intersection char by default when making style for `Panel`s.
            "│ i32 │ i32 │"
            "├─────┼─────┤"
            "│  0  │  1  │"
            "└─────┴─────┘"
        )
    );
}

#[test]
fn panel_style_uses_most_left_and_right_cell_styles_correct() {
    let table = Table::new(&[(0, 1)])
        .with(tabled::Panel("Numbers", 0))
        .with(Style::modern())
        .with(Style::correct_spans())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───────────┐"
            "│Numbers    │"
            "├─────┬─────┤"
            "│ i32 │ i32 │"
            "├─────┼─────┤"
            "│  0  │  1  │"
            "└─────┴─────┘"
        )
    );
}

#[test]
fn panel_style_change_corect() {
    let table = Table::new(&[(0, 1)])
        .with(tabled::Panel("Numbers", 0))
        .with(
            Style::modern()
                .top_intersection('─')
                .header_intersection('┬'),
        )
        .with(Style::correct_spans())
        .with(Modify::new(Cell(0, 0)).with(Alignment::center()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───────────┐"
            "│  Numbers  │"
            "├───────────┤" // it's different because we use a top_intersection char by default when making style for `Panel`s.
            "│ i32 │ i32 │"
            "├─────┼─────┤"
            "│  0  │  1  │"
            "└─────┴─────┘"
        )
    );
}

#[test]
fn panel_in_single_column() {
    let table = Table::new(&[(0)])
        .with(tabled::Panel("Numbers", 0))
        .with(Style::modern())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───────┐"
            "│Numbers│"
            "├───────┤"
            "│  i32  │"
            "├───────┤"
            "│   0   │"
            "└───────┘"
        )
    );
}
