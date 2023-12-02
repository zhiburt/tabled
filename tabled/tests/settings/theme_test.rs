#![cfg(feature = "std")]

use tabled::settings::{themes::Theme, Alignment, Style};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    theme_0,
    Matrix::new(3, 3).with(Theme::from_style(Style::modern())),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    theme_reverse_data,
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.reverse_rows(true);
        theme
    }),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    theme_reverse_columns,
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.reverse_columns(true);
        theme
    }),
    "┌──────────┬──────────┬──────────┬───┐"
    "│ column 2 │ column 1 │ column 0 │ N │"
    "├──────────┼──────────┼──────────┼───┤"
    "│   0-2    │   0-1    │   0-0    │ 0 │"
    "├──────────┼──────────┼──────────┼───┤"
    "│   1-2    │   1-1    │   1-0    │ 1 │"
    "├──────────┼──────────┼──────────┼───┤"
    "│   2-2    │   2-1    │   2-0    │ 2 │"
    "└──────────┴──────────┴──────────┴───┘"
);

test_table!(
    theme_reverse_columns_and_data,
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.reverse_rows(true);
        theme.reverse_columns(true);
        theme
    }),
    "┌──────────┬──────────┬──────────┬───┐"
    "│ column 2 │ column 1 │ column 0 │ N │"
    "├──────────┼──────────┼──────────┼───┤"
    "│   2-2    │   2-1    │   2-0    │ 2 │"
    "├──────────┼──────────┼──────────┼───┤"
    "│   1-2    │   1-1    │   1-0    │ 1 │"
    "├──────────┼──────────┼──────────┼───┤"
    "│   0-2    │   0-1    │   0-0    │ 0 │"
    "└──────────┴──────────┴──────────┴───┘"
);

test_table!(
    theme_stick_left,
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.align_columns(Alignment::left());
        theme
    }),
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
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.align_columns(Alignment::right());
        theme
    }),
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
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.align_columns(Alignment::bottom());
        theme
    }),
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
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.set_footer(true);
        theme
    }),
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
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.align_columns(Alignment::left());
        theme.set_footer(true);
        theme
    }),
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
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.align_columns(Alignment::right());
        theme.set_footer(true);
        theme
    }),
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
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.align_columns(Alignment::bottom());
        theme.set_footer(true);
        theme
    }),
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
    theme_footer_with_reverse,
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.set_footer(true);
        theme.reverse_rows(true);
        theme
    }),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "└───┴──────────┴──────────┴──────────┘"

);

test_table!(
    theme_stick_left_with_footer_with_reverse,
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.align_columns(Alignment::left());
        theme.set_footer(true);
        theme.reverse_rows(true);
        theme
    }),
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
    theme_stick_right_with_footer_with_reverse,
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.align_columns(Alignment::right());
        theme.set_footer(true);
        theme.reverse_rows(true);
        theme
    }),
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
    theme_stick_bottom_with_footer_with_reverse,
    Matrix::new(3, 3).with({
        let mut theme = Theme::from_style(Style::modern());
        theme.align_columns(Alignment::bottom());
        theme.set_footer(true);
        theme.reverse_rows(true);
        theme
    }),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "└───┴──────────┴──────────┴──────────┘"
);
