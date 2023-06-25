#![cfg(feature = "std")]

use tabled::settings::{
    object::{Cell, Object, Rows, Segment},
    style::{BorderSpanCorrection, HorizontalLine},
    Alignment, Border, Highlight, Modify, Panel, Span, Style, Width,
};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    panel_has_no_style_by_default,
    Matrix::new(3, 3).with(Style::psql()).with(Panel::horizontal(0,"Linux Distributions")),
    "        Linux Distributions         "
    "---+----------+----------+----------"
    " N | column 0 | column 1 | column 2 "
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    highlight_panel_0,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0,"Linux Distributions"))
        .with(Style::psql())
        .with(Highlight::new(Cell::new(0, 0), Border::filled('#'))),
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
    Matrix::new(3, 3)
        .with(Panel::horizontal(0,"Linux Distributions"))
        .with(Style::psql())
        .with(Highlight::new(Cell::new(0, 0), Border::filled('#')))
        .with(Highlight::new(Cell::new(0, 1), Border::filled('#')))
        .with(Highlight::new(Cell::new(0, 2), Border::filled('#')))
        .with(Highlight::new(Cell::new(0, 3), Border::filled('#'))),
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
    Matrix::new(3, 3)
        .with(Panel::horizontal(0,"Linux Distributions"))
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
    Matrix::new(3, 3)
        .with(Panel::horizontal(4,"Linux Distributions"))
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
    Matrix::new(3, 3)
        .with(Panel::horizontal(2,"Linux Distributions"))
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
    Matrix::new(3, 3)
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
    Matrix::new(3, 3)
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
    Matrix::iter([(0, 1)]).with(Panel::horizontal(0,"Numbers")).with(Style::modern()),
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
    Matrix::iter([(0, 1)])
        .with(Panel::horizontal(0,"Numbers"))
        .with(Style::modern().intersection_top('─').horizontals([HorizontalLine::new(1, Style::modern().get_horizontal()).intersection(Some('┬'))]))
        .with(Modify::new(Cell::new(0, 0)).with(Alignment::center())),
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
    Matrix::iter([(0, 1)])
        .with(Panel::horizontal(0,"Numbers"))
        .with(Style::modern())
        .with(BorderSpanCorrection),
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
    Matrix::iter([(0, 1)])
        .with(Panel::horizontal(0,"Numbers"))
        .with(Style::modern().intersection_top('─').horizontals([HorizontalLine::new(1, Style::modern().get_horizontal()).intersection(Some('┬'))]))
        .with(BorderSpanCorrection)
        .with(Modify::new(Cell::new(0, 0)).with(Alignment::center())),
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
    #[allow(clippy::needless_borrow)]
    Matrix::iter(&[(0)]).with(Panel::horizontal(0,"Numbers")).with(Style::modern()),
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
    Matrix::new(3, 3).with(Style::psql()).with(Panel::vertical(0,"Linux Distributions")),
    " Linux Distributions | N | column 0 | column 1 | column 2 "
    "                     +---+----------+----------+----------"
    "                     | 0 |   0-0    |   0-1    |   0-2    "
    "                     | 1 |   1-0    |   1-1    |   1-2    "
    "                     | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_1,
    Matrix::new(3, 3).with(Style::psql()).with(Panel::vertical(1,"Linux Distributions")),
    " N | Linux Distributions | column 0 | column 1 | column 2 "
    "---+                     +----------+----------+----------"
    " 0 |                     |   0-0    |   0-1    |   0-2    "
    " 1 |                     |   1-0    |   1-1    |   1-2    "
    " 2 |                     |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_2,
    Matrix::new(3, 3).with(Style::psql()).with(Panel::vertical(4,"Linux Distributions")),
    " N | column 0 | column 1 | column 2 | Linux Distributions "
    "---+----------+----------+----------+                     "
    " 0 |   0-0    |   0-1    |   0-2    |                     "
    " 1 |   1-0    |   1-1    |   1-2    |                     "
    " 2 |   2-0    |   2-1    |   2-2    |                     "
);

test_table!(
    panel_vertical_0_wrap,
    Matrix::new(3, 3).with(Style::psql()).with(Panel::vertical(0,"Linux Distributions")).with(Modify::new(Cell::new(0, 0)).with(Width::wrap(3))),
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
    Matrix::new(3, 3).with(Style::psql()).with(Panel::vertical(0,"Linux Distributions")).with(Modify::new(Cell::new(0, 0)).with(Width::wrap(0))),
    "  | N | column 0 | column 1 | column 2 "
    "  +---+----------+----------+----------"
    "  | 0 |   0-0    |   0-1    |   0-2    "
    "  | 1 |   1-0    |   1-1    |   1-2    "
    "  | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_vertical_0_wrap_100,
    Matrix::new(3, 3).with(Style::psql()).with(Panel::vertical(0,"Linux Distributions")).with(Modify::new(Cell::new(0, 0)).with(Width::wrap(100))),
    " Linux Distributions | N | column 0 | column 1 | column 2 "
    "                     +---+----------+----------+----------"
    "                     | 0 |   0-0    |   0-1    |   0-2    "
    "                     | 1 |   1-0    |   1-1    |   1-2    "
    "                     | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_horizontal_set_0,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Panel::horizontal(0,"Linux Distributions"))
        .with(Panel::vertical(0,"asd")),
    " asd |        Linux Distributions         "
    "     +---+----------+----------+----------"
    "     | N | column 0 | column 1 | column 2 "
    "     | 0 |   0-0    |   0-1    |   0-2    "
    "     | 1 |   1-0    |   1-1    |   1-2    "
    "     | 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    panel_horizontal_set_1,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Panel::horizontal(0,"Linux Distributions"))
        .with(Panel::vertical(0,"asd"))
        .with(Panel::vertical(5,"asd"))
        ,
    " asd |        Linux Distributions         | asd "
    "     +---+----------+----------+----------+     "
    "     | N | column 0 | column 1 | column 2 |     "
    "     | 0 |   0-0    |   0-1    |   0-2    |     "
    "     | 1 |   1-0    |   1-1    |   1-2    |     "
    "     | 2 |   2-0    |   2-1    |   2-2    |     "
);

test_table!(
    ignore_col_span_intersect_with_other_span,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Panel::horizontal(0,"Linux Distributions"))
        .with(Panel::vertical(0,"asd"))
        .with(Panel::vertical(5,"zxc"))
        .with(Modify::new((1, 3)).with(Span::column(3)).with("wwwww")),
    " asd |       Linux Distributions       | zxc "
    "     +---+----------+-------+----------+     "
    "     | N | column 0 | wwwww | column 2 |     "
    "     | 0 |   0-0    |  0-1  |   0-2    |     "
    "     | 1 |   1-0    |  1-1  |   1-2    |     "
    "     | 2 |   2-0    |  2-1  |   2-2    |     "
);

test_table!(
    panel_horizontal_x_2,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Panel::horizontal(0,"Linux Distributions"))
        .with(Panel::vertical(0,"asd"))
        .with(Panel::vertical(5,"zxc"))
        .with(Modify::new((1, 3)).with(Span::column(2)).with("wwwww")),
        " asd |   Linux Distributions    | zxc "
        "     +---+----------+-----+-----+     "
        "     | N | column 0 |   wwwww   |     "
        "     | 0 |   0-0    | 0-1 | 0-2 |     "
        "     | 1 |   1-0    | 1-1 | 1-2 |     "
        "     | 2 |   2-0    | 2-1 | 2-2 |     "
);

test_table!(
    ignore_row_span_intersect_with_other_span,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Panel::horizontal(2,"Linux Distributions"))
        .with(Panel::vertical(0,"asd"))
        .with(Panel::vertical(5,"zxc"))
        .with(Modify::new((0, 3)).with(Span::row(4)).with("xxxxx")),
    " asd | N | column 0 | xxxxx | column 2 | zxc "
    "     +---+----------+-------+----------+     "
    "     | 0 |   0-0    |  0-1  |   0-2    |     "
    "     |       Linux Distributions       |     "
    "     | 1 |   1-0    |  1-1  |   1-2    |     "
    "     | 2 |   2-0    |  2-1  |   2-2    |     "
);
