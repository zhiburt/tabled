#![cfg(feature = "std")]

use tabled::settings::{themes::Layout, Alignment, Style};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    theme_stick_left,
    Matrix::new(3, 3).with(Style::modern()).with(Layout::new(Alignment::left(), false)),
    "┌──────────┬─────┬─────┬─────┐"
    "│    N     │  0  │  1  │  2  │"
    "├──────────┼─────┼─────┼─────┤"
    "│ column 0 │ 0-0 │ 1-0 │ 2-0 │"
    "├──────────┼─────┼─────┼─────┤"
    "│ column 1 │ 0-1 │ 1-1 │ 2-1 │"
    "├──────────┼─────┼─────┼─────┤"
    "│ column 2 │ 0-2 │ 1-2 │ 2-2 │"
    "└──────────┴─────┴─────┴─────┘"
);

test_table!(
    theme_stick_right,
    Matrix::new(3, 3).with(Style::modern()).with(Layout::new(Alignment::right(), false)),
    "┌─────┬─────┬─────┬──────────┐"
    "│  2  │  1  │  0  │    N     │"
    "├─────┼─────┼─────┼──────────┤"
    "│ 2-0 │ 1-0 │ 0-0 │ column 0 │"
    "├─────┼─────┼─────┼──────────┤"
    "│ 2-1 │ 1-1 │ 0-1 │ column 1 │"
    "├─────┼─────┼─────┼──────────┤"
    "│ 2-2 │ 1-2 │ 0-2 │ column 2 │"
    "└─────┴─────┴─────┴──────────┘"
);

test_table!(
    theme_stick_bottom,
    Matrix::new(3, 3).with(Style::modern()).with(Layout::new(Alignment::bottom(), false)),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    theme_footer,
    Matrix::new(3, 3).with(Style::modern()).with(Layout::new(Alignment::top(), true)),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    theme_stick_left_with_footer,
    Matrix::new(3, 3).with(Style::modern()).with(Layout::new(Alignment::left(), true)),
    "┌──────────┬─────┬─────┬─────┬──────────┐"
    "│    N     │  0  │  1  │  2  │    N     │"
    "├──────────┼─────┼─────┼─────┼──────────┤"
    "│ column 0 │ 0-0 │ 1-0 │ 2-0 │ column 0 │"
    "├──────────┼─────┼─────┼─────┼──────────┤"
    "│ column 1 │ 0-1 │ 1-1 │ 2-1 │ column 1 │"
    "├──────────┼─────┼─────┼─────┼──────────┤"
    "│ column 2 │ 0-2 │ 1-2 │ 2-2 │ column 2 │"
    "└──────────┴─────┴─────┴─────┴──────────┘"
);

test_table!(
    theme_stick_right_with_footer,
    Matrix::new(3, 3).with(Style::modern()).with(Layout::new(Alignment::right(), true)),
    "┌──────────┬─────┬─────┬─────┬──────────┐"
    "│    N     │  2  │  1  │  0  │    N     │"
    "├──────────┼─────┼─────┼─────┼──────────┤"
    "│ column 0 │ 2-0 │ 1-0 │ 0-0 │ column 0 │"
    "├──────────┼─────┼─────┼─────┼──────────┤"
    "│ column 1 │ 2-1 │ 1-1 │ 0-1 │ column 1 │"
    "├──────────┼─────┼─────┼─────┼──────────┤"
    "│ column 2 │ 2-2 │ 1-2 │ 0-2 │ column 2 │"
    "└──────────┴─────┴─────┴─────┴──────────┘"
);

test_table!(
    theme_stick_bottom_with_footer,
    Matrix::new(3, 3).with(Style::modern()).with(Layout::new(Alignment::bottom(), true)),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    theme_stick_1x1,
    Matrix::new(0, 0).with(Layout::new(Alignment::left(), false)),
    "+---+"
    "| N |"
    "+---+"
);

test_table!(
    theme_stick_empty,
    Matrix::empty().with(Layout::new(Alignment::left(), false)),
    ""
);
