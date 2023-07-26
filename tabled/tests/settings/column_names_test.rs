#![cfg(feature = "std")]

use tabled::{
    grid::config::AlignmentHorizontal,
    settings::{themes::ColumnNames, Color},
    Table,
};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    new,
    Matrix::new(3, 3).with(ColumnNames::new(["1", "2", "3", "4"])),
    "+1--+2---------+3---------+4---------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    new_more_names_then_columns,
    Matrix::new(3, 3).with(ColumnNames::new(["1", "2", "3", "4", "5", "6", "7"])),
    "+1--+2---------+3---------+4---------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    new_less_names_then_columns,
    Matrix::new(3, 3).with(ColumnNames::new(["1", "2"])),
    "+1--+2---------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    new_empty,
    Matrix::new(3, 3).with(ColumnNames::new([""; 0])),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    default,
    Matrix::new(3, 3).with(ColumnNames::default()),
    "+N--+column 0+column 1+column 2+"
    "| 0 |  0-0   |  0-1   |  0-2   |"
    "+---+--------+--------+--------+"
    "| 1 |  1-0   |  1-1   |  1-2   |"
    "+---+--------+--------+--------+"
    "| 2 |  2-0   |  2-1   |  2-2   |"
    "+---+--------+--------+--------+"
);

test_table!(
    alignment_left,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().set_alignment(AlignmentHorizontal::Left)),
    "+&str---+&str------+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    alignment_right,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().set_alignment(AlignmentHorizontal::Right)),
    "+---&str+------&str+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    alignment_center,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().set_alignment(AlignmentHorizontal::Center)),
    "+-&str--+---&str---+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    alignment_array,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().set_alignment(vec![AlignmentHorizontal::Right, AlignmentHorizontal::Center])),
    "+---&str+---&str---+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    set_line,
    Matrix::new(3, 3).with(ColumnNames::default().set_line(1)),
    "+---+--------+--------+--------+"
    "| 0 |  0-0   |  0-1   |  0-2   |"
    "+N--+column 0+column 1+column 2+"
    "| 1 |  1-0   |  1-1   |  1-2   |"
    "+---+--------+--------+--------+"
    "| 2 |  2-0   |  2-1   |  2-2   |"
    "+---+--------+--------+--------+"
);

test_table!(
    set_line_max_out,
    Matrix::new(3, 3).with(ColumnNames::default().set_line(100)),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    set_line_0,
    Matrix::new(3, 3).with(ColumnNames::default().set_line(0)),
    "+N--+column 0+column 1+column 2+"
    "| 0 |  0-0   |  0-1   |  0-2   |"
    "+---+--------+--------+--------+"
    "| 1 |  1-0   |  1-1   |  1-2   |"
    "+---+--------+--------+--------+"
    "| 2 |  2-0   |  2-1   |  2-2   |"
    "+---+--------+--------+--------+"
);

test_table!(
    set_colors_some_some,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().set_color(vec![Color::BG_BLACK, Color::BG_BLUE])),
    "+\u{1b}[40m&\u{1b}[49m\u{1b}[40ms\u{1b}[49m\u{1b}[40mt\u{1b}[49m\u{1b}[40mr\u{1b}[49m---+\u{1b}[44m&\u{1b}[49m\u{1b}[44ms\u{1b}[49m\u{1b}[44mt\u{1b}[49m\u{1b}[44mr\u{1b}[49m------+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    set_colors_none_some,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().set_color(vec![Color::default(), Color::BG_BLUE])),
        "+&str---+\u{1b}[44m&\u{1b}[49m\u{1b}[44ms\u{1b}[49m\u{1b}[44mt\u{1b}[49m\u{1b}[44mr\u{1b}[49m------+"
        "| Hello | World    |"
        "+-------+----------+"
        "| and   | looooong |"
        "|       | word     |"
        "+-------+----------+"
);

test_table!(
    set_colors_none_none,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().set_color(vec![Color::default(), Color::default()])),
    "+&str---+&str------+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    set_colors_empty,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().set_color(vec![Color::default(); 0])),
    "+&str---+&str------+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);
