use tabled::{
    object::{Cell, Object, Rows, Segment},
    style::{Border, Style},
    Alignment, Footer, Header, Highlight, Modify, Panel, Table,
};

use crate::util::{create_vector, test_table};

mod util;

test_table!(
    panel_has_no_style_by_default,
    Table::new(create_vector::<3, 3>()).with(Style::psql()).with(Panel("Linux Distributions", 0)),
    "        Linux Distributions         "
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    highligt_panel_0,
    Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 0))
        .with(Style::psql())
        .with(Highlight::new(Cell(0, 0), Border::filled('#'))),
    "#####                                "
    "#        Linux Distributions         "
    "#####----------+----------+----------"
    "  N | column 0 | column 1 | column 2 "
    "  0 |   0-0    |   0-1    |   0-2    "
    "  1 |   1-0    |   1-1    |   1-2    "
    "  2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    highligt_panel_1,
    Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 0))
        .with(Style::psql())
        .with(Highlight::new(Cell(0, 0), Border::filled('#')))
        .with(Highlight::new(Cell(0, 1), Border::filled('#')))
        .with(Highlight::new(Cell(0, 2), Border::filled('#')))
        .with(Highlight::new(Cell(0, 3), Border::filled('#'))),
    "######################################"
    "#        Linux Distributions         #"
    "######################################"
    "  N | column 0 | column 1 | column 2  "
    "  0 |   0-0    |   0-1    |   0-2     "
    "  1 |   1-0    |   1-1    |   1-2     "
    "  2 |   2-0    |   2-1    |   2-2     "
);

test_table!(
    top_panel,
    Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::psql()),
    "        Linux Distributions         "
    "---+----------+----------+----------"
    " N | column 0 | column 1 | column 2 "
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    bottom_panel,
    Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 4))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::psql()),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
    "        Linux Distributions         "
);

test_table!(
    inner_panel,
    Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 2))
        .with(Modify::new(Rows::new(2..)).with(Alignment::center()))
        .with(Style::psql()),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    "        Linux Distributions         "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    header,
    Table::new(create_vector::<3, 3>())
        .with(Header("Linux Distributions"))
        .with(Style::psql())
        .with(Modify::new(Rows::new(0..1)).with(Alignment::center())),
    "        Linux Distributions         "
    "---+----------+----------+----------"
    " N | column 0 | column 1 | column 2 "
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    footer,
    Table::new(create_vector::<3, 3>())
        .with(Header("Linux Distributions"))
        .with(Footer("The end"))
        .with(Style::psql())
        .with(Modify::new(Rows::first().and(Rows::last())).with(Alignment::center())),
    "        Linux Distributions         "
    "---+----------+----------+----------"
    " N | column 0 | column 1 | column 2 "
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
    "              The end               "
);

test_table!(
    panel_style_uses_most_left_and_right_cell_styles,
    Table::new(&[(0, 1)]).with(tabled::Panel("Numbers", 0)).with(Style::modern()),
    "┌─────┬─────┐"
    "│  Numbers  │"
    "├─────┼─────┤"
    "│ i32 │ i32 │"
    "├─────┼─────┤"
    "│  0  │  1  │"
    "└─────┴─────┘"
);

test_table!(
    panel_style_change,
    Table::new(&[(0, 1)])
        .with(tabled::Panel("Numbers", 0))
        .with(Style::modern().top_intersection('─').lines([(1, Style::modern().get_horizontal().intersection(Some('┬')))]))
        .with(Modify::new(Cell(0, 0)).with(Alignment::center())),
    "┌───────────┐"
    "│  Numbers  │"
    "├─────┬─────┤"
    "│ i32 │ i32 │"
    "├─────┼─────┤"
    "│  0  │  1  │"
    "└─────┴─────┘"
);

test_table!(
    panel_style_uses_most_left_and_right_cell_styles_correct,
    Table::new(&[(0, 1)])
        .with(tabled::Panel("Numbers", 0))
        .with(Style::modern())
        .with(Style::correct_spans()),
    "┌───────────┐"
    "│  Numbers  │"
    "├─────┬─────┤"
    "│ i32 │ i32 │"
    "├─────┼─────┤"
    "│  0  │  1  │"
    "└─────┴─────┘"
);

test_table!(
    panel_style_change_corect,
    Table::new(&[(0, 1)])
        .with(tabled::Panel("Numbers", 0))
        .with(Style::modern().top_intersection('─').lines([(1, Style::modern().get_horizontal().intersection(Some('┬')))]))
        .with(Style::correct_spans())
        .with(Modify::new(Cell(0, 0)).with(Alignment::center())),
    "┌───────────┐"
    "│  Numbers  │"
    "├───────────┤" // it's different because we use a top_intersection char by default when making style for `Panel`s.
    "│ i32 │ i32 │"
    "├─────┼─────┤"
    "│  0  │  1  │"
    "└─────┴─────┘"
);

test_table!(
    panel_in_single_column,
    Table::new(&[(0)]).with(tabled::Panel("Numbers", 0)).with(Style::modern()),
    "┌─────────┐"
    "│ Numbers │"
    "├─────────┤"
    "│   i32   │"
    "├─────────┤"
    "│    0    │"
    "└─────────┘"
);
