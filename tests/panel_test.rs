use tabled::{
    object::{Cell, Object, Rows, Segment},
    style::HorizontalLine,
    Alignment, Border, Highlight, Modify, Panel, Style,
};

use crate::util::{create_table, new_table, test_table};

mod util;

test_table!(
    panel_has_no_style_by_default,
    create_table::<3, 3>().with(Style::psql()).with(Panel::horizontal(0).text("Linux Distributions")),
    "        Linux Distributions         "
    "---+----------+----------+----------"
    " N | column 0 | column 1 | column 2 "
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    highlight_panel_0,
    create_table::<3, 3>()
        .with(Panel::horizontal(0).text("Linux Distributions"))
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
    highlight_panel_1,
    create_table::<3, 3>()
        .with(Panel::horizontal(0).text("Linux Distributions"))
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
    create_table::<3, 3>()
        .with(Panel::horizontal(0).text("Linux Distributions"))
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
    create_table::<3, 3>()
        .with(Panel::horizontal(4).text("Linux Distributions"))
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
    create_table::<3, 3>()
        .with(Panel::horizontal(2).text("Linux Distributions"))
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
    create_table::<3, 3>()
        .with(Panel::header("Linux Distributions"))
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
    create_table::<3, 3>()
        .with(Panel::header("Linux Distributions"))
        .with(Panel::footer("The end"))
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
    new_table([(0, 1)]).with(Panel::horizontal(0).text("Numbers")).with(Style::modern()),
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
    new_table([(0, 1)])
        .with(Panel::horizontal(0).text("Numbers"))
        .with(Style::modern().top_intersection('─').horizontals([HorizontalLine::new(1, Style::modern().get_horizontal()).intersection(Some('┬'))]))
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
    new_table([(0, 1)])
        .with(Panel::horizontal(0).text("Numbers"))
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
    panel_style_change_correct,
    new_table([(0, 1)])
        .with(Panel::horizontal(0).text("Numbers"))
        .with(Style::modern().top_intersection('─').horizontals([HorizontalLine::new(1, Style::modern().get_horizontal()).intersection(Some('┬'))]))
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
    new_table([(0)]).with(Panel::horizontal(0).text("Numbers")).with(Style::modern()),
    "┌─────────┐"
    "│ Numbers │"
    "├─────────┤"
    "│   i32   │"
    "├─────────┤"
    "│    0    │"
    "└─────────┘"
);

test_table!(
    panel_vertical_0,
    create_table::<3, 3>().with(Style::psql()).with(Panel::vertical(0).text("Linux Distributions")),
    " Linux Distributions | N | column 0 | column 1 | column 2 "
    "                     +---+----------+----------+----------"
    "                     | 0 |   0-0    |   0-1    |   0-2    "
    "                     | 1 |   1-0    |   1-1    |   1-2    "
    "                     | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_1,
    create_table::<3, 3>().with(Style::psql()).with(Panel::vertical(1).text("Linux Distributions")),
    " N | Linux Distributions | column 0 | column 1 | column 2 "
    "---+                     +----------+----------+----------"
    " 0 |                     |   0-0    |   0-1    |   0-2    "
    " 1 |                     |   1-0    |   1-1    |   1-2    "
    " 2 |                     |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_2,
    create_table::<3, 3>().with(Style::psql()).with(Panel::vertical(4).text("Linux Distributions")),
    " N | column 0 | column 1 | column 2 | Linux Distributions "
    "---+----------+----------+----------+                     "
    " 0 |   0-0    |   0-1    |   0-2    |                     "
    " 1 |   1-0    |   1-1    |   1-2    |                     "
    " 2 |   2-0    |   2-1    |   2-2    |                     "
);

test_table!(
    panel_vertical_0_wrap,
    create_table::<3, 3>().with(Style::psql()).with(Panel::vertical(0).text("Linux Distributions").text_width(3)),
    " Lin | N | column 0 | column 1 | column 2 "
    " ux  |   |          |          |          "
    " Dis |   |          |          |          "
    " tri +---+----------+----------+----------"
    " but | 0 |   0-0    |   0-1    |   0-2    "
    " ion | 1 |   1-0    |   1-1    |   1-2    "
    " s   | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_0_wrap_0,
    create_table::<3, 3>().with(Style::psql()).with(Panel::vertical(0).text("Linux Distributions").text_width(0)),
    " Linux Distributions | N | column 0 | column 1 | column 2 "
    "                     +---+----------+----------+----------"
    "                     | 0 |   0-0    |   0-1    |   0-2    "
    "                     | 1 |   1-0    |   1-1    |   1-2    "
    "                     | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_0_wrap_100,
    create_table::<3, 3>().with(Style::psql()).with(Panel::vertical(0).text("Linux Distributions").text_width(100)),
    " Linux Distributions | N | column 0 | column 1 | column 2 "
    "                     +---+----------+----------+----------"
    "                     | 0 |   0-0    |   0-1    |   0-2    "
    "                     | 1 |   1-0    |   1-1    |   1-2    "
    "                     | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_set_row_0,
    create_table::<3, 3>().with(Style::psql()).with(Panel::vertical(0).row(1).text("Linux Distributions")),
    "                     | N | column 0 | column 1 | column 2 "
    "---------------------+---+----------+----------+----------"
    " Linux Distributions | 0 |   0-0    |   0-1    |   0-2    "
    "                     | 1 |   1-0    |   1-1    |   1-2    "
    "                     | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_set_row_1,
    create_table::<3, 3>().with(Style::psql()).with(Panel::vertical(1).row(1).text("Linux Distributions")),
    " N |                     | column 0 | column 1 | column 2 "
    "---+---------------------+----------+----------+----------"
    " 0 | Linux Distributions |   0-0    |   0-1    |   0-2    "
    " 1 |                     |   1-0    |   1-1    |   1-2    "
    " 2 |                     |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_set_row_2,
    create_table::<3, 3>().with(Style::psql()).with(Panel::vertical(4).row(1).text("Linux Distributions")),
   " N | column 0 | column 1 | column 2 |                     "
   "---+----------+----------+----------+---------------------"
   " 0 |   0-0    |   0-1    |   0-2    | Linux Distributions "
   " 1 |   1-0    |   1-1    |   1-2    |                     "
   " 2 |   2-0    |   2-1    |   2-2    |                     "
);

test_table!(
    panel_horizontal_set_col_0,
    create_table::<3, 3>().with(Style::psql()).with(Panel::horizontal(0).column(1).text("Linux Distributions")),
    "   |      Linux Distributions       "
    "---+----------+----------+----------"
    " N | column 0 | column 1 | column 2 "
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_horizontal_set_col_1,
    create_table::<3, 3>().with(Style::psql()).with(Panel::horizontal(2).column(1).text("Linux Distributions")),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    "   |      Linux Distributions       "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_horizontal_set_col_2,
    create_table::<3, 3>().with(Style::psql()).with(Panel::horizontal(4).column(1).text("Linux Distributions")),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
    "   |      Linux Distributions       "
);

test_table!(
    panel_horizontal_set_0,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Panel::horizontal(0).text("Linux Distributions"))
        .with(Panel::vertical(0).text("asd")),
    " asd |        Linux Distributions         "
    "     +---+----------+----------+----------"
    "     | N | column 0 | column 1 | column 2 "
    "     | 0 |   0-0    |   0-1    |   0-2    "
    "     | 1 |   1-0    |   1-1    |   1-2    "
    "     | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_horizontal_set_1,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Panel::horizontal(0).text("Linux Distributions"))
        .with(Panel::vertical(0).text("asd"))
        .with(Panel::vertical(5).text("asd"))
        ,
    " asd |        Linux Distributions         | asd "
    "     +---+----------+----------+----------+     "
    "     | N | column 0 | column 1 | column 2 |     "
    "     | 0 |   0-0    |   0-1    |   0-2    |     "
    "     | 1 |   1-0    |   1-1    |   1-2    |     "
    "     | 2 |   2-0    |   2-1    |   2-2    |     "
);

test_table!(
    panel_horizontal_set_2,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Panel::horizontal(0).text("Linux Distributions"))
        .with(Panel::vertical(0).text("asd"))
        .with(Panel::vertical(5).text("asd"))
        .with(Panel::vertical(3).row(1).text("asd")),
    " asd |      Linux Distributions      |          | asd "
    "     +---+----------+-----+----------+----------+     "
    "     | N | column 0 | asd | column 1 | column 2 |     "
    "     | 0 |   0-0    |     |   0-1    |   0-2    |     "
    "     | 1 |   1-0    |     |   1-1    |   1-2    |     "
    "     | 2 |   2-0    |     |   2-1    |   2-2    |     "
);
