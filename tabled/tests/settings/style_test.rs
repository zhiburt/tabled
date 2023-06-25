#![cfg(feature = "std")]

use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    settings::{
        object::{Columns, Rows, Segment},
        style::{
            Border, BorderChar, BorderColor, BorderSpanCorrection, BorderText, HorizontalLine,
            Line, Offset, RawStyle, Style, VerticalLine,
        },
        Color, Format, Highlight, Modify, Padding, Span,
    },
    Table,
};

use crate::matrix::Matrix;
use testing_table::{static_table, test_table};

#[cfg(feature = "color")]
use ::{owo_colors::OwoColorize, std::convert::TryFrom};

test_table!(
    default_style,
    Matrix::new(3, 3).with(Style::ascii()),
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
    psql_style,
    Matrix::new(3, 3).with(Style::psql()),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    markdown_style,
    Matrix::new(3, 3).with(Style::markdown()),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    modern_style,
    Matrix::new(3, 3).with(Style::modern()),
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
    rounded_style,
    Matrix::new(3, 3).with(Style::rounded()),
    "╭───┬──────────┬──────────┬──────────╮"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "╰───┴──────────┴──────────┴──────────╯"
);

test_table!(
    sharp_style,
    Matrix::new(3, 3).with(Style::sharp()),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    modern_clean_style,
    Matrix::new(3, 3).with(Style::modern().remove_horizontal().horizontals(vec![HorizontalLine::new(1, Style::modern().get_horizontal())])),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    blank_style,
    Matrix::new(3, 3).with(Style::blank()),
    " N   column 0   column 1   column 2 "
    " 0     0-0        0-1        0-2    "
    " 1     1-0        1-1        1-2    "
    " 2     2-0        2-1        2-2    "
);

test_table!(
    extended_style,
    Matrix::new(3, 3).with(Style::extended()),
    "╔═══╦══════════╦══════════╦══════════╗"
    "║ N ║ column 0 ║ column 1 ║ column 2 ║"
    "╠═══╬══════════╬══════════╬══════════╣"
    "║ 0 ║   0-0    ║   0-1    ║   0-2    ║"
    "╠═══╬══════════╬══════════╬══════════╣"
    "║ 1 ║   1-0    ║   1-1    ║   1-2    ║"
    "╠═══╬══════════╬══════════╬══════════╣"
    "║ 2 ║   2-0    ║   2-1    ║   2-2    ║"
    "╚═══╩══════════╩══════════╩══════════╝"
);

test_table!(
    ascii_dots_style,
    Matrix::new(3, 3).with(Style::dots()),
    "......................................"
    ": N : column 0 : column 1 : column 2 :"
    ":...:..........:..........:..........:"
    ": 0 :   0-0    :   0-1    :   0-2    :"
    ":...:..........:..........:..........:"
    ": 1 :   1-0    :   1-1    :   1-2    :"
    ":...:..........:..........:..........:"
    ": 2 :   2-0    :   2-1    :   2-2    :"
    ":...:..........:..........:..........:"
);

test_table!(
    re_structured_text_style,
    Matrix::new(3, 3).with(Style::re_structured_text()),
    "=== ========== ========== =========="
    " N   column 0   column 1   column 2 "
    "=== ========== ========== =========="
    " 0     0-0        0-1        0-2    "
    " 1     1-0        1-1        1-2    "
    " 2     2-0        2-1        2-2    "
    "=== ========== ========== =========="
);

test_table!(
    ascii_rounded_style,
    Matrix::new(3, 3).with(Style::ascii_rounded()),
    ".------------------------------------."
    "| N | column 0 | column 1 | column 2 |"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "'------------------------------------'"
);

test_table!(
    style_head_changes,
    Matrix::new(3, 3).with(Style::modern().remove_horizontal()),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    style_frame_changes,
    Matrix::new(3, 3).with(Style::modern().remove_top().remove_bottom().remove_horizontal()),
    "│ N │ column 0 │ column 1 │ column 2 │"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
);

test_table!(
    custom_style,
    Matrix::new(3, 3)
        .with(Style::blank()
            .bottom('*')
            .vertical('\'')
            .horizontal('`')
            .intersection('\'')
            .intersection_bottom('\'')
            .horizontals(vec![HorizontalLine::new(1, Line::full('x', '*', 'q', 'w'))])),
    "  N ' column 0 ' column 1 ' column 2  "
    "qxxx*xxxxxxxxxx*xxxxxxxxxx*xxxxxxxxxxw"
    "  0 '   0-0    '   0-1    '   0-2     "
    " ```'``````````'``````````'`````````` "
    "  1 '   1-0    '   1-1    '   1-2     "
    " ```'``````````'``````````'`````````` "
    "  2 '   2-0    '   2-1    '   2-2     "
    " ***'**********'**********'********** "
);

test_table!(
    style_single_cell_0,
    Matrix::table(0, 0),
    "+---+"
    "| N |"
    "+---+"
);

test_table!(
    style_single_cell_1,
    Matrix::table(0, 0).with(Style::blank()),
    " N "
);

test_table!(
    top_border_override_first_test,
    Matrix::table(2, 2).with(BorderText::new("-Table").horizontal(Rows::first())),
    "-Table---------+----------+"
    "| N | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    top_border_override_last_test,
    Matrix::table(2, 2).with(BorderText::new("-Table").horizontal(Rows::last())),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "-Table---------+----------+"
);

test_table!(
    top_border_override_new_test,
    Matrix::table(2, 2)
        .with(BorderText::new("-Table").horizontal(1))
        .with(BorderText::new("-Table").horizontal(2)),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "-Table---------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "-Table---------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    top_border_override_new_doesnt_panic_when_index_is_invalid,
    Matrix::table(2, 2).with(BorderText::new("-Table").horizontal(100)),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    top_override_doesnt_work_with_style_with_no_top_border_test,
    Matrix::table(2, 2)
        .with(Style::psql())
        .with(BorderText::new("-Table").horizontal(Rows::first())),
    " N | column 0 | column 1 "
    "---+----------+----------"
    " 0 |   0-0    |   0-1    "
    " 1 |   1-0    |   1-1    "
);

test_table!(
    top_border_override_cleared_after_restyling_test,
    Matrix::table(2, 2)
        .with(BorderText::new("-Table").horizontal(Rows::first()))
        .with(Style::ascii()),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    top_border_override_with_big_string_test,
    Matrix::table(2, 2)
        .with(BorderText::new("-Tableeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee1231").horizontal(Rows::first())),
    "-Tableeeeeeeeeeeeeeeeeeeeee"
    "| N | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_text_0,
    Matrix::table(2, 2)
        .with(Style::empty())
        .with(Modify::new(Rows::first()).with(Border::default().bottom('-')))
        .with(BorderText::new("-Table").horizontal(1)),
    " N  column 0  column 1 "
    "-Table-----------------"
    " 0    0-0       0-1    "
    " 1    1-0       1-1    "
);

test_table!(
    border_color_global,
    { Matrix::table(2, 2).with(BorderColor::default().bottom(Color::FG_RED)) },
    "+---+----------+----------+\n\
     | N | column 0 | column 1 |\n\
     +\u{1b}[31m---\u{1b}[39m+\u{1b}[31m----------\u{1b}[39m+\u{1b}[31m----------\u{1b}[39m+\n\
     | 0 |   0-0    |   0-1    |\n\
     +\u{1b}[31m---\u{1b}[39m+\u{1b}[31m----------\u{1b}[39m+\u{1b}[31m----------\u{1b}[39m+\n\
     | 1 |   1-0    |   1-1    |\n\
     +\u{1b}[31m---\u{1b}[39m+\u{1b}[31m----------\u{1b}[39m+\u{1b}[31m----------\u{1b}[39m+"
);

#[cfg(feature = "color")]
test_table!(
    border_text_colored,
    Matrix::table(2, 2)
        .with(BorderText::new("-Table").horizontal(1))
        .with(BorderText::new("-Table213123").horizontal(2))
        .with(Modify::new(Rows::single(1)).with(BorderColor::default().bottom(Color::FG_RED)))
        .with(Modify::new(Rows::single(2)).with(BorderColor::default().bottom(Color::try_from(" ".blue().on_green().to_string()).unwrap()))),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "-Table---------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "-\u{1b}[31mTab\u{1b}[39ml\u{1b}[31me213123---\u{1b}[39m+\u{1b}[31m----------\u{1b}[39m+"
    "| 1 |   1-0    |   1-1    |"
    "+\u{1b}[34m\u{1b}[42m---\u{1b}[39m\u{1b}[49m+\u{1b}[34m\u{1b}[42m----------\u{1b}[39m\u{1b}[49m+\u{1b}[34m\u{1b}[42m----------\u{1b}[39m\u{1b}[49m+"
);

test_table!(
    border_text_offset_test_0,
    Matrix::table(2, 2).with(BorderText::new("-Table").horizontal(1).offset(Offset::Begin(5))),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+-Table----+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_text_offset_test_1,
    Matrix::table(2, 2).with(BorderText::new("-Table").horizontal(1).offset(Offset::Begin(15))),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+-----------Table-----+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_text_offset_test_2,
    Matrix::table(2, 2).with(BorderText::new("Table").horizontal(1).offset(Offset::End(5))),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+----------+------Table"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_text_offset_test_3,
    Matrix::table(2, 2).with(BorderText::new("Table").horizontal(1).offset(Offset::End(15))),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+-------Table---------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_text_offset_test_4,
    Matrix::table(2, 2).with(BorderText::new("Table").horizontal(1).offset(Offset::End(21))),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+-Table----+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_text_offset_test_5,
    Matrix::table(2, 2).with(BorderText::new("Table").horizontal(1).offset(Offset::End(25))),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+-Table--------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_text_offset_test_6,
    Matrix::table(2, 2).with(BorderText::new("-Table").horizontal(1).offset(Offset::Begin(21))),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+----------+------Table"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_override_color,
    Matrix::table(2, 2).with(BorderText::new("-Table").horizontal(Rows::first()).color(Color::FG_BLUE)),
    "\u{1b}[34m-\u{1b}[39m\u{1b}[34mT\u{1b}[39m\u{1b}[34ma\u{1b}[39m\u{1b}[34mb\u{1b}[39m\u{1b}[34ml\u{1b}[39m\u{1b}[34me\u{1b}[39m---------+----------+"
    "| N | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    empty_style,
    Matrix::new(3, 3)
        .with(Style::empty())
        .with(Modify::new(Segment::all()).with(Padding::zero())),
    "Ncolumn 0column 1column 2"
    "0  0-0     0-1     0-2   "
    "1  1-0     1-1     1-2   "
    "2  2-0     2-1     2-2   "
);

test_table!(
    single_column_style_0,
    Matrix::table(2, 0).with(Style::modern()),
    "┌───┐"
    "│ N │"
    "├───┤"
    "│ 0 │"
    "├───┤"
    "│ 1 │"
    "└───┘"
);

test_table!(
    single_column_style_1,
    Matrix::table(2, 0).with(Style::blank()),
    " N "
    " 0 "
    " 1 "
);

test_table!(
    single_column_last_row_style,
    Matrix::table(3, 0).with(Style::re_structured_text()),
    "==="
    " N "
    "==="
    " 0 "
    " 1 "
    " 2 "
    "==="
);

test_table!(
    single_cell_style,
    Builder::from_iter([[""]]).build().with(Style::modern()),
    "┌──┐"
    "│  │"
    "└──┘"
);

test_table!(
    border_test_0,
    Matrix::table(2, 2).with(Modify::new(Rows::single(1)).with(Border::filled('*').top('#'))),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "*###*##########*##########*"
    "* 0 *   0-0    *   0-1    *"
    "***************************"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_test_1,
    Matrix::table(2, 2)
        .with(Style::empty())
        .with(Modify::new(Rows::single(1)).with(Border::filled('*').top('#'))),
    "  N   column 0   column 1  "
    "*###*##########*##########*"
    "* 0 *   0-0    *   0-1    *"
    "***************************"
    "  1     1-0        1-1     "
);

test_table!(
    style_frame_test_0,
    Matrix::table(2, 2).with(Highlight::new(Rows::single(1), Style::modern().get_frame())),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "┌─────────────────────────┐"
    "│ 0 |   0-0    |   0-1    │"
    "└─────────────────────────┘"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    style_frame_test_1,
    Matrix::table(2, 2)
        .with(Style::blank())
        .with(Highlight::new(Rows::single(0), Style::extended().get_frame()))
        .with(Highlight::new(Rows::single(2), Style::extended().get_frame())),
    "╔═════════════════════════╗"
    "║ N   column 0   column 1 ║"
    "╚═════════════════════════╝"
    "  0     0-0        0-1     "
    "╔═════════════════════════╗"
    "║ 1     1-0        1-1    ║"
    "╚═════════════════════════╝"
);

test_table!(
    single_column_off_horizontal_test,
    Matrix::table(3, 0).with(Style::ascii().remove_horizontal().remove_vertical()),
    "+---+"
    "| N |"
    "| 0 |"
    "| 1 |"
    "| 2 |"
    "+---+"
);

test_table!(
    single_row_test,
    Matrix::table(0, 3).with(Style::modern()),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    empty_border_text_doesnt_panic_test,
    Matrix::table(2, 2).with(BorderText::new("").horizontal(0)),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    span_correct_test_0,
    Matrix::table(6, 4)
        .with(Modify::new((0, 3)).with(Span::column(2)))
        .with(Modify::new((1, 0)).with(Span::column(3)))
        .with(Modify::new((2, 0)).with(Span::column(2)))
        .with(Modify::new((2, 3)).with(Span::column(2)))
        .with(Modify::new((3, 0)).with(Span::column(5)))
        .with(Modify::new((4, 1)).with(Span::column(4)))
        .with(Modify::new((5, 0)).with(Span::column(5)))
        .with(Modify::new((6, 0)).with(Span::column(5)))
        .with(BorderSpanCorrection),
    "+---+----------+----------+-----------+"
    "| N | column 0 | column 1 | column 2  |"
    "+---+----------+----------+-----+-----+"
    "|            0            | 0-2 | 0-3 |"
    "+--------------+----------+-----+-----+"
    "|      1       |   1-1    |    1-2    |"
    "+--------------+----------+-----------+"
    "|                  2                  |"
    "+---+---------------------------------+"
    "| 3 |               3-0               |"
    "+---+---------------------------------+"
    "|                  4                  |"
    "+-------------------------------------+"
    "|                  5                  |"
    "+-------------------------------------+"
);

test_table!(
    span_correct_test_1,
    Matrix::table(6, 4)
        .with(Modify::new((0, 0)).with(Span::column(5)))
        .with(Modify::new((1, 0)).with(Span::column(3)))
        .with(Modify::new((2, 0)).with(Span::column(2)))
        .with(Modify::new((2, 3)).with(Span::column(2)))
        .with(Modify::new((3, 0)).with(Span::column(5)))
        .with(Modify::new((4, 1)).with(Span::column(4)))
        .with(Modify::new((5, 0)).with(Span::column(5)))
        .with(Modify::new((6, 0)).with(Span::column(5)))
        .with(BorderSpanCorrection),
    "+----------------------+"
    "|          N           |"
    "+----------+-----+-----+"
    "|    0     | 0-2 | 0-3 |"
    "+----+-----+-----+-----+"
    "| 1  | 1-1 |    1-2    |"
    "+----+-----+-----------+"
    "|          2           |"
    "+---+------------------+"
    "| 3 |       3-0        |"
    "+---+------------------+"
    "|          4           |"
    "+----------------------+"
    "|          5           |"
    "+----------------------+"
);

test_table!(
    style_settings_usage_test_0,
    Matrix::new(3, 3)
        .insert((1, 1), "a longer string")
        .with({
            let mut style: RawStyle = Style::modern().into();
            style
                .set_bottom(Some('a'))
                .set_left(Some('b'))
                .set_right(None)
                .set_top(None)
                .set_intersection(Some('x'))
                .set_intersection_top(None)
                .set_corner_top_left(None)
                .set_corner_top_right(None);
            style
        }),
    "b N │    column 0     │ column 1 │ column 2  "
    "├───x─────────────────x──────────x──────────┤"
    "b 0 │ a longer string │   0-1    │   0-2     "
    "├───x─────────────────x──────────x──────────┤"
    "b 1 │       1-0       │   1-1    │   1-2     "
    "├───x─────────────────x──────────x──────────┤"
    "b 2 │       2-0       │   2-1    │   2-2     "
    "└aaa┴aaaaaaaaaaaaaaaaa┴aaaaaaaaaa┴aaaaaaaaaa┘"
);

test_table!(
    style_settings_usage_test_1,
    Matrix::new(3, 3)
        .insert((1, 1), "a longer string")
        .with({
            let mut style: RawStyle = Style::modern().into();
            style.set_bottom(None);
            style
        }),
    "┌───┬─────────────────┬──────────┬──────────┐"
    "│ N │    column 0     │ column 1 │ column 2 │"
    "├───┼─────────────────┼──────────┼──────────┤"
    "│ 0 │ a longer string │   0-1    │   0-2    │"
    "├───┼─────────────────┼──────────┼──────────┤"
    "│ 1 │       1-0       │   1-1    │   1-2    │"
    "├───┼─────────────────┼──────────┼──────────┤"
    "│ 2 │       2-0       │   2-1    │   2-2    │"
    "└   ┴                 ┴          ┴          ┘"
);

test_table!(
    style_settings_usage_test_2,
    Matrix::new(3, 3)
        .insert((1, 1), "a longer string")
        .with({
            let mut style: RawStyle = Style::modern().into();
            style.set_bottom(None);
            style
        })
        .with(Modify::new(Rows::last()).with(Border::default().corner_bottom_left('*'))),
    "┌───┬─────────────────┬──────────┬──────────┐"
    "│ N │    column 0     │ column 1 │ column 2 │"
    "├───┼─────────────────┼──────────┼──────────┤"
    "│ 0 │ a longer string │   0-1    │   0-2    │"
    "├───┼─────────────────┼──────────┼──────────┤"
    "│ 1 │       1-0       │   1-1    │   1-2    │"
    "├───┼─────────────────┼──────────┼──────────┤"
    "│ 2 │       2-0       │   2-1    │   2-2    │"
    "*   *                 *          *          ┘"
);

test_table!(
    border_none_test_0,
    Matrix::table(2, 2)
        .with(Style::ascii())
        .with(Modify::new(Rows::single(1)).with(Border::filled('*').top('#')))
        .with(Modify::new(Rows::single(1)).with(Border::empty())),
    "+---+----------+----------+"
    "| N | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 |   0-0    |   0-1    |"
    "+---+----------+----------+"
    "| 1 |   1-0    |   1-1    |"
    "+---+----------+----------+"
);

test_table!(
    border_none_test_1,
    Matrix::table(2, 2)
        .with(Style::empty())
        .with(Modify::new(Rows::single(1)).with(Border::filled('*').top('#')))
        .with(Modify::new(Columns::single(1)).with(Border::empty())),
    "  N  column 0  column 1  "
    "*###          ##########*"
    "* 0    0-0       0-1    *"
    "****          ***********"
    "  1    1-0       1-1     "
);

#[test]
fn custom_style_test() {
    macro_rules! test_style {
        ($style:expr, $expected:expr $(,)*) => {
            let table = Matrix::new(3, 3).with($style).to_string();
            assert_eq!(table, $expected);
        };
    }

    // Single

    test_style!(
        Style::empty().top('-'),
        static_table!(
            "---------------------------------"
            " N  column 0  column 1  column 2 "
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
        ),
    );
    test_style!(
        Style::empty().bottom('-'),
        static_table!(
            " N  column 0  column 1  column 2 "
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "---------------------------------"
        ),
    );
    test_style!(
        Style::empty().left('-'),
        static_table!(
            "- N  column 0  column 1  column 2 "
            "- 0    0-0       0-1       0-2    "
            "- 1    1-0       1-1       1-2    "
            "- 2    2-0       2-1       2-2    "
        ),
    );
    test_style!(
        Style::empty().right('-'),
        static_table!(
            " N  column 0  column 1  column 2 -"
            " 0    0-0       0-1       0-2    -"
            " 1    1-0       1-1       1-2    -"
            " 2    2-0       2-1       2-2    -"
        ),
    );
    test_style!(
        Style::empty().horizontal('-'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            "---------------------------------"
            " 1    1-0       1-1       1-2    "
            "---------------------------------"
            " 2    2-0       2-1       2-2    "
        ),
    );
    test_style!(
        Style::empty().horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('-'))]),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
        ),
    );
    test_style!(
        Style::empty().vertical('-'),
        static_table!(
            " N - column 0 - column 1 - column 2 "
            " 0 -   0-0    -   0-1    -   0-2    "
            " 1 -   1-0    -   1-1    -   1-2    "
            " 2 -   2-0    -   2-1    -   2-2    "
        ),
    );

    // Combinations

    test_style!(
        Style::empty().top('-').bottom('+'),
        static_table!(
            "---------------------------------"
            " N  column 0  column 1  column 2 "
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "+++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().top('-').left('+'),
        static_table!(
            "+---------------------------------"
            "+ N  column 0  column 1  column 2 "
            "+ 0    0-0       0-1       0-2    "
            "+ 1    1-0       1-1       1-2    "
            "+ 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().top('-').right('+'),
        static_table!(
            "---------------------------------+"
            " N  column 0  column 1  column 2 +"
            " 0    0-0       0-1       0-2    +"
            " 1    1-0       1-1       1-2    +"
            " 2    2-0       2-1       2-2    +"
        )
    );
    test_style!(
        Style::empty().top('-').horizontal('+'),
        static_table!(
            "---------------------------------"
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            "+++++++++++++++++++++++++++++++++"
            " 1    1-0       1-1       1-2    "
            "+++++++++++++++++++++++++++++++++"
            " 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().top('-').vertical('+'),
        static_table!(
            "---+----------+----------+----------"
            " N + column 0 + column 1 + column 2 "
            " 0 +   0-0    +   0-1    +   0-2    "
            " 1 +   1-0    +   1-1    +   1-2    "
            " 2 +   2-0    +   2-1    +   2-2    "
        )
    );
    test_style!(
        Style::empty()
            .top('-')
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('+'))]),
        static_table!(
            "---------------------------------"
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
        )
    );

    test_style!(
        Style::empty().bottom('-').top('+'),
        static_table!(
            "+++++++++++++++++++++++++++++++++"
            " N  column 0  column 1  column 2 "
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "---------------------------------"
        )
    );
    test_style!(
        Style::empty().bottom('-').left('+'),
        static_table!(
            "+ N  column 0  column 1  column 2 "
            "+ 0    0-0       0-1       0-2    "
            "+ 1    1-0       1-1       1-2    "
            "+ 2    2-0       2-1       2-2    "
            "+---------------------------------"
        )
    );
    test_style!(
        Style::empty().bottom('-').right('+'),
        static_table!(
            " N  column 0  column 1  column 2 +"
            " 0    0-0       0-1       0-2    +"
            " 1    1-0       1-1       1-2    +"
            " 2    2-0       2-1       2-2    +"
            "---------------------------------+"
        )
    );
    test_style!(
        Style::empty().bottom('-').vertical('+'),
        static_table!(
            " N + column 0 + column 1 + column 2 "
            " 0 +   0-0    +   0-1    +   0-2    "
            " 1 +   1-0    +   1-1    +   1-2    "
            " 2 +   2-0    +   2-1    +   2-2    "
            "---+----------+----------+----------"
        )
    );
    test_style!(
        Style::empty().bottom('-').horizontal('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            "+++++++++++++++++++++++++++++++++"
            " 1    1-0       1-1       1-2    "
            "+++++++++++++++++++++++++++++++++"
            " 2    2-0       2-1       2-2    "
            "---------------------------------"
        )
    );
    test_style!(
        Style::empty()
            .bottom('-')
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('+'))]),
        static_table!(
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "---------------------------------"
        )
    );

    test_style!(
        Style::empty().left('-').top('+'),
        static_table!(
            "++++++++++++++++++++++++++++++++++"
            "- N  column 0  column 1  column 2 "
            "- 0    0-0       0-1       0-2    "
            "- 1    1-0       1-1       1-2    "
            "- 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().left('-').bottom('+'),
        static_table!(
            "- N  column 0  column 1  column 2 "
            "- 0    0-0       0-1       0-2    "
            "- 1    1-0       1-1       1-2    "
            "- 2    2-0       2-1       2-2    "
            "++++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().left('-').right('+'),
        static_table!(
            "- N  column 0  column 1  column 2 +"
            "- 0    0-0       0-1       0-2    +"
            "- 1    1-0       1-1       1-2    +"
            "- 2    2-0       2-1       2-2    +"
        )
    );
    test_style!(
        Style::empty().left('-').vertical('+'),
        static_table!(
            "- N + column 0 + column 1 + column 2 "
            "- 0 +   0-0    +   0-1    +   0-2    "
            "- 1 +   1-0    +   1-1    +   1-2    "
            "- 2 +   2-0    +   2-1    +   2-2    "
        )
    );
    test_style!(
        Style::empty().left('-').horizontal('+'),
        static_table!(
            "- N  column 0  column 1  column 2 "
            "++++++++++++++++++++++++++++++++++"
            "- 0    0-0       0-1       0-2    "
            "++++++++++++++++++++++++++++++++++"
            "- 1    1-0       1-1       1-2    "
            "++++++++++++++++++++++++++++++++++"
            "- 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty()
            .left('-')
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('+'))]),
        static_table!(
            "- N  column 0  column 1  column 2 "
            " +++++++++++++++++++++++++++++++++"
            "- 0    0-0       0-1       0-2    "
            "- 1    1-0       1-1       1-2    "
            "- 2    2-0       2-1       2-2    "
        )
    );

    test_style!(
        Style::empty().right('-').top('+'),
        static_table!(
            "++++++++++++++++++++++++++++++++++"
            " N  column 0  column 1  column 2 -"
            " 0    0-0       0-1       0-2    -"
            " 1    1-0       1-1       1-2    -"
            " 2    2-0       2-1       2-2    -"
        )
    );
    test_style!(
        Style::empty().right('-').bottom('+'),
        static_table!(
            " N  column 0  column 1  column 2 -"
            " 0    0-0       0-1       0-2    -"
            " 1    1-0       1-1       1-2    -"
            " 2    2-0       2-1       2-2    -"
            "++++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().right('-').left('+'),
        static_table!(
            "+ N  column 0  column 1  column 2 -"
            "+ 0    0-0       0-1       0-2    -"
            "+ 1    1-0       1-1       1-2    -"
            "+ 2    2-0       2-1       2-2    -"
        )
    );
    test_style!(
        Style::empty().right('-').vertical('+'),
        static_table!(
            " N + column 0 + column 1 + column 2 -"
            " 0 +   0-0    +   0-1    +   0-2    -"
            " 1 +   1-0    +   1-1    +   1-2    -"
            " 2 +   2-0    +   2-1    +   2-2    -"
        )
    );
    test_style!(
        Style::empty().right('-').horizontal('+'),
        static_table!(
            " N  column 0  column 1  column 2 -"
            "++++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    -"
            "++++++++++++++++++++++++++++++++++"
            " 1    1-0       1-1       1-2    -"
            "++++++++++++++++++++++++++++++++++"
            " 2    2-0       2-1       2-2    -"
        )
    );
    test_style!(
        Style::empty()
            .right('-')
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('+'))]),
        static_table!(
            " N  column 0  column 1  column 2 -"
            "+++++++++++++++++++++++++++++++++ "
            " 0    0-0       0-1       0-2    -"
            " 1    1-0       1-1       1-2    -"
            " 2    2-0       2-1       2-2    -"
        )
    );

    test_style!(
        Style::empty().vertical('-').top('+'),
        static_table!(
            "++++++++++++++++++++++++++++++++++++"
            " N - column 0 - column 1 - column 2 "
            " 0 -   0-0    -   0-1    -   0-2    "
            " 1 -   1-0    -   1-1    -   1-2    "
            " 2 -   2-0    -   2-1    -   2-2    "
        )
    );
    test_style!(
        Style::empty().vertical('-').bottom('+'),
        static_table!(
            " N - column 0 - column 1 - column 2 "
            " 0 -   0-0    -   0-1    -   0-2    "
            " 1 -   1-0    -   1-1    -   1-2    "
            " 2 -   2-0    -   2-1    -   2-2    "
            "++++++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().vertical('-').left('+'),
        static_table!(
            "+ N - column 0 - column 1 - column 2 "
            "+ 0 -   0-0    -   0-1    -   0-2    "
            "+ 1 -   1-0    -   1-1    -   1-2    "
            "+ 2 -   2-0    -   2-1    -   2-2    "
        )
    );
    test_style!(
        Style::empty().vertical('-').right('+'),
        static_table!(
            " N - column 0 - column 1 - column 2 +"
            " 0 -   0-0    -   0-1    -   0-2    +"
            " 1 -   1-0    -   1-1    -   1-2    +"
            " 2 -   2-0    -   2-1    -   2-2    +"
        )
    );
    test_style!(
        Style::empty().vertical('-').horizontal('+'),
        static_table!(
            " N - column 0 - column 1 - column 2 "
            "++++++++++++++++++++++++++++++++++++"
            " 0 -   0-0    -   0-1    -   0-2    "
            "++++++++++++++++++++++++++++++++++++"
            " 1 -   1-0    -   1-1    -   1-2    "
            "++++++++++++++++++++++++++++++++++++"
            " 2 -   2-0    -   2-1    -   2-2    "
        )
    );
    test_style!(
        Style::empty()
            .vertical('-')
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('+'))]),
        static_table!(
            " N - column 0 - column 1 - column 2 "
            "+++ ++++++++++ ++++++++++ ++++++++++"
            " 0 -   0-0    -   0-1    -   0-2    "
            " 1 -   1-0    -   1-1    -   1-2    "
            " 2 -   2-0    -   2-1    -   2-2    "
        )
    );

    test_style!(
        Style::empty().horizontal('-').top('+'),
        static_table!(
            "+++++++++++++++++++++++++++++++++"
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            "---------------------------------"
            " 1    1-0       1-1       1-2    "
            "---------------------------------"
            " 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().horizontal('-').bottom('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            "---------------------------------"
            " 1    1-0       1-1       1-2    "
            "---------------------------------"
            " 2    2-0       2-1       2-2    "
            "+++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty().horizontal('-').left('+'),
        static_table!(
            "+ N  column 0  column 1  column 2 "
            "+---------------------------------"
            "+ 0    0-0       0-1       0-2    "
            "+---------------------------------"
            "+ 1    1-0       1-1       1-2    "
            "+---------------------------------"
            "+ 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty().horizontal('-').right('+'),
        static_table!(
            " N  column 0  column 1  column 2 +"
            "---------------------------------+"
            " 0    0-0       0-1       0-2    +"
            "---------------------------------+"
            " 1    1-0       1-1       1-2    +"
            "---------------------------------+"
            " 2    2-0       2-1       2-2    +"
        )
    );
    test_style!(
        Style::empty().horizontal('-').vertical('+'),
        static_table!(
            " N + column 0 + column 1 + column 2 "
            "---+----------+----------+----------"
            " 0 +   0-0    +   0-1    +   0-2    "
            "---+----------+----------+----------"
            " 1 +   1-0    +   1-1    +   1-2    "
            "---+----------+----------+----------"
            " 2 +   2-0    +   2-1    +   2-2    "
        )
    );
    test_style!(
        Style::empty()
            .horizontal('-')
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('+'))]),
        static_table!(
            " N  column 0  column 1  column 2 "
            "+++++++++++++++++++++++++++++++++"
            " 0    0-0       0-1       0-2    "
            "---------------------------------"
            " 1    1-0       1-1       1-2    "
            "---------------------------------"
            " 2    2-0       2-1       2-2    "
        )
    );

    test_style!(
        Style::empty()
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('-'))])
            .top('+'),
        static_table!(
            "+++++++++++++++++++++++++++++++++"
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty()
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('-'))])
            .bottom('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
            "+++++++++++++++++++++++++++++++++"
        )
    );
    test_style!(
        Style::empty()
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('-'))])
            .left('+'),
        static_table!(
            "+ N  column 0  column 1  column 2 "
            "+---------------------------------"
            "+ 0    0-0       0-1       0-2    "
            "+ 1    1-0       1-1       1-2    "
            "+ 2    2-0       2-1       2-2    "
        )
    );
    test_style!(
        Style::empty()
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('-'))])
            .right('+'),
        static_table!(
            " N  column 0  column 1  column 2 +"
            "---------------------------------+"
            " 0    0-0       0-1       0-2    +"
            " 1    1-0       1-1       1-2    +"
            " 2    2-0       2-1       2-2    +"
        )
    );
    test_style!(
        Style::empty()
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('-'))])
            .vertical('+'),
        static_table!(
            " N + column 0 + column 1 + column 2 "
            "---+----------+----------+----------"
            " 0 +   0-0    +   0-1    +   0-2    "
            " 1 +   1-0    +   1-1    +   1-2    "
            " 2 +   2-0    +   2-1    +   2-2    "
        )
    );
    test_style!(
        Style::empty()
            .horizontals(vec![HorizontalLine::new(1, Line::default()).main(Some('-'))])
            .horizontal('+'),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            "+++++++++++++++++++++++++++++++++"
            " 1    1-0       1-1       1-2    "
            "+++++++++++++++++++++++++++++++++"
            " 2    2-0       2-1       2-2    "
        )
    );

    // Full

    test_style!(
        Style::empty()
            .top('-')
            .bottom('+')
            .left('|')
            .right('*')
            .horizontal('x')
            .horizontals(vec![HorizontalLine::new(1, Line::filled('z'))])
            .vertical('#'),
        static_table!(
            "|---#----------#----------#----------*"
            "| N # column 0 # column 1 # column 2 *"
            "zzzz#zzzzzzzzzz#zzzzzzzzzz#zzzzzzzzzzz"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "xxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxx"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "xxxx#xxxxxxxxxx#xxxxxxxxxx#xxxxxxxxxxx"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "|+++#++++++++++#++++++++++#++++++++++*"
        ),
    );

    let full_style = Style::empty()
        .top('-')
        .bottom('+')
        .left('|')
        .right('*')
        .horizontal('x')
        .horizontals(vec![HorizontalLine::new(1, Line::filled(','))])
        .vertical('#')
        .intersection_bottom('@')
        .intersection_top('!')
        .intersection_left('=')
        .intersection_right('$')
        .intersection('+')
        .corner_top_left(';')
        .corner_bottom_left('?')
        .corner_top_right('.')
        .corner_bottom_right('%');
    test_style!(
        full_style.clone(),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            ",,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,,"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );

    // Overwrite intersections and corners

    test_style!(
        full_style.clone().top('q'),
        static_table!(
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
            "| N # column 0 # column 1 # column 2 *"
            ",,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,,"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().bottom('q'),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            ",,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,,"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
        )
    );
    test_style!(
        full_style.clone().left('w'),
        static_table!(
            "w---!----------!----------!----------."
            "w N # column 0 # column 1 # column 2 *"
            "w,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,,"
            "w 0 #   0-0    #   0-1    #   0-2    *"
            "wxxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "w 1 #   1-0    #   1-1    #   1-2    *"
            "wxxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "w 2 #   2-0    #   2-1    #   2-2    *"
            "w+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().right('i'),
        static_table!(
            ";---!----------!----------!----------i"
            "| N # column 0 # column 1 # column 2 i"
            ",,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,i"
            "| 0 #   0-0    #   0-1    #   0-2    i"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxxi"
            "| 1 #   1-0    #   1-1    #   1-2    i"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxxi"
            "| 2 #   2-0    #   2-1    #   2-2    i"
            "?+++@++++++++++@++++++++++@++++++++++i"
        )
    );
    test_style!(
        full_style.clone().horizontal('q'),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            ",,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,,"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().vertical('q'),
        static_table!(
            ";---q----------q----------q----------."
            "| N q column 0 q column 1 q column 2 *"
            ",,,,q,,,,,,,,,,q,,,,,,,,,,q,,,,,,,,,,,"
            "| 0 q   0-0    q   0-1    q   0-2    *"
            "=xxxqxxxxxxxxxxqxxxxxxxxxxqxxxxxxxxxx$"
            "| 1 q   1-0    q   1-1    q   1-2    *"
            "=xxxqxxxxxxxxxxqxxxxxxxxxxqxxxxxxxxxx$"
            "| 2 q   2-0    q   2-1    q   2-2    *"
            "?+++q++++++++++q++++++++++q++++++++++%"
        )
    );
    test_style!(
        full_style
            .clone()
            .horizontals(vec![HorizontalLine::new(1, Line::filled('q'))]),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );

    // Turn off borders

    let empty_table = static_table!(
        " N  column 0  column 1  column 2 "
        " 0    0-0       0-1       0-2    "
        " 1    1-0       1-1       1-2    "
        " 2    2-0       2-1       2-2    "
    );
    test_style!(Style::empty().top('-').remove_top(), empty_table);
    test_style!(Style::empty().bottom('-').remove_bottom(), empty_table);
    test_style!(Style::empty().right('-').remove_right(), empty_table);
    test_style!(Style::empty().left('-').remove_left(), empty_table);
    test_style!(
        Style::empty().horizontal('-').remove_horizontal(),
        empty_table
    );
    test_style!(Style::empty().vertical('-').remove_vertical(), empty_table);
    test_style!(
        Style::empty().horizontals(vec![HorizontalLine::new(
            1,
            Line::new(Some('-'), None, None, None)
        )]),
        static_table!(
            " N  column 0  column 1  column 2 "
            "---------------------------------"
            " 0    0-0       0-1       0-2    "
            " 1    1-0       1-1       1-2    "
            " 2    2-0       2-1       2-2    "
        )
    );

    test_style!(
        full_style.clone().remove_top(),
        static_table!(
            "| N # column 0 # column 1 # column 2 *"
            ",,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,,"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().remove_bottom(),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            ",,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,,"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
        )
    );
    test_style!(
        full_style.clone().remove_right(),
        static_table!(
            ";---!----------!----------!----------"
            "| N # column 0 # column 1 # column 2 "
            ",,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,"
            "| 0 #   0-0    #   0-1    #   0-2    "
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx"
            "| 1 #   1-0    #   1-1    #   1-2    "
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx"
            "| 2 #   2-0    #   2-1    #   2-2    "
            "?+++@++++++++++@++++++++++@++++++++++"
        )
    );
    test_style!(
        full_style.clone().remove_left(),
        static_table!(
           "---!----------!----------!----------."
           " N # column 0 # column 1 # column 2 *"
           ",,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,,"
           " 0 #   0-0    #   0-1    #   0-2    *"
           "xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
           " 1 #   1-0    #   1-1    #   1-2    *"
           "xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
           " 2 #   2-0    #   2-1    #   2-2    *"
           "+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().remove_horizontal(),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            ",,,,#,,,,,,,,,,#,,,,,,,,,,#,,,,,,,,,,,"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );
    test_style!(
        full_style.clone().remove_vertical(),
        static_table!(
            ";---------------------------------."
            "| N  column 0  column 1  column 2 *"
            ",,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,"
            "| 0    0-0       0-1       0-2    *"
            "=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx$"
            "| 1    1-0       1-1       1-2    *"
            "=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx$"
            "| 2    2-0       2-1       2-2    *"
            "?+++++++++++++++++++++++++++++++++%"
        )
    );
    test_style!(
        full_style.remove_horizontals(),
        static_table!(
            ";---!----------!----------!----------."
            "| N # column 0 # column 1 # column 2 *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 0 #   0-0    #   0-1    #   0-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 1 #   1-0    #   1-1    #   1-2    *"
            "=xxx+xxxxxxxxxx+xxxxxxxxxx+xxxxxxxxxx$"
            "| 2 #   2-0    #   2-1    #   2-2    *"
            "?+++@++++++++++@++++++++++@++++++++++%"
        )
    );
}

#[test]
fn test_default_border_usage() {
    macro_rules! test_border {
        ($modify:expr, $expected:expr) => {
            let table = Matrix::new(3, 3)
                .insert((1, 1), "a longer string")
                .with(Style::empty())
                .with($modify)
                .to_string();

            assert_eq!(table, $expected);
        };
    }

    test_border! {
        Modify::new((3, 2)).with(Border::default().corner_bottom_left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
            "                    *                    "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().corner_bottom_right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
            "                              *          "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().bottom('*')),
        static_table!(
            " N     column 0      column 1  column 2 "
            " 0  a longer string    0-1       0-2    "
            " 1        1-0          1-1       1-2    "
            " 2        2-0          2-1       2-2    "
            "                    **********          "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().bottom('*').corner_bottom_left('#')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
            "                    #**********          "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().bottom('*').corner_bottom_right('#')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
            "                    **********#          "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0       *   2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().corner_top_left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            "                    *                    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().left('#').corner_top_left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            "                    *                    "
            " 2        2-0       #   2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().left('#').corner_bottom_left('@').corner_top_left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            "                    *                    "
            " 2        2-0       #   2-1       2-2    "
            "                    @                    "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1    *   2-2    "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().corner_top_right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            "                              *          "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().right('#').corner_top_right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            "                              *          "
            " 2        2-0          2-1    #   2-2    "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().right('#').corner_top_right('*').corner_bottom_right('@')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            "                              *          "
            " 2        2-0          2-1    #   2-2    "
            "                              @          "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::default().right('#').corner_top_right('*').corner_bottom_left('@')),
        static_table!(
            " N     column 0       column 1   column 2 "
            " 0  a longer string     0-1        0-2    "
            " 1        1-0           1-1        1-2    "
            "                               *          "
            " 2        2-0           2-1    #   2-2    "
            "                    @                     "
        )
    }
    test_border! {
        Modify::new((3, 2)).with(Border::filled('@')),
        static_table!(
            " N     column 0       column 1   column 2 "
            " 0  a longer string     0-1        0-2    "
            " 1        1-0           1-1        1-2    "
            "                    @@@@@@@@@@@@          "
            " 2        2-0       @   2-1    @   2-2    "
            "                    @@@@@@@@@@@@          "
        )
    }

    test_border! {
        Modify::new((1, 2)).with(Border::default().corner_bottom_left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            "                    *                    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().corner_bottom_right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            "                              *          "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().bottom('*')),
        static_table!(
            " N     column 0      column 1  column 2 "
            " 0  a longer string    0-1       0-2    "
            "                    **********          "
            " 1        1-0          1-1       1-2    "
            " 2        2-0          2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().bottom('*').corner_bottom_left('#')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string     0-1       0-2    "
            "                    #**********          "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().bottom('*').corner_bottom_right('#')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            "                    **********#          "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            " 0  a longer string *   0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().corner_top_left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            "                    *                    "
            " 0  a longer string     0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().left('#').corner_top_left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            "                    *                    "
            " 0  a longer string #   0-1       0-2    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().left('#').corner_bottom_left('@').corner_top_left('*')),
        static_table!(
            " N     column 0       column 1  column 2 "
            "                    *                    "
            " 0  a longer string #   0-1       0-2    "
            "                    @                    "
            " 1        1-0           1-1       1-2    "
            " 2        2-0           2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1    *   0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().corner_top_right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              *          "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().right('#').corner_top_right('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              *          "
            " 0  a longer string    0-1    #   0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().right('#').corner_top_right('*').corner_bottom_right('@')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              *          "
            " 0  a longer string    0-1    #   0-2    "
            "                              @          "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::default().right('#').corner_top_right('*').corner_bottom_left('@')),
        static_table!(
            " N     column 0       column 1   column 2 "
            "                               *          "
            " 0  a longer string     0-1    #   0-2    "
            "                    @                     "
            " 1        1-0           1-1        1-2    "
            " 2        2-0           2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((1, 2)).with(Border::filled('@')),
        static_table!(
            " N     column 0       column 1   column 2 "
            "                    @@@@@@@@@@@@          "
            " 0  a longer string @   0-1    @   0-2    "
            "                    @@@@@@@@@@@@          "
            " 1        1-0           1-1        1-2    "
            " 2        2-0           2-1        2-2    "
        )
    }

    test_border! {
        Modify::new((0, 3)).with(Border::default().corner_bottom_left('*')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              *          "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().corner_bottom_right('*')),
        static_table!(
            " N     column 0      column 1  column 2  "
            "                                        *"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().bottom('*')),
        static_table!(
            " N     column 0      column 1  column 2 "
            "                              **********"
            " 0  a longer string    0-1       0-2    "
            " 1        1-0          1-1       1-2    "
            " 2        2-0          2-1       2-2    "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().bottom('*').corner_bottom_left('#')),
        static_table!(
            " N     column 0      column 1   column 2 "
            "                              #**********"
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().bottom('*').corner_bottom_right('#')),
        static_table!(
            " N     column 0      column 1  column 2  "
            "                              **********#"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().left('*')),
        static_table!(
            " N     column 0      column 1 * column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().corner_top_left('*')),
        static_table!(
            "                              *          "
            " N     column 0      column 1   column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().left('#').corner_top_left('*')),
        static_table!(
            "                              *          "
            " N     column 0      column 1 # column 2 "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().left('#').corner_bottom_left('@').corner_top_left('*')),
        static_table!(
            "                              *          "
            " N     column 0      column 1 # column 2 "
            "                              @          "
            " 0  a longer string    0-1        0-2    "
            " 1        1-0          1-1        1-2    "
            " 2        2-0          2-1        2-2    "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().right('*')),
        static_table!(
            " N     column 0      column 1  column 2 *"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().corner_top_right('*')),
        static_table!(
            "                                        *"
            " N     column 0      column 1  column 2  "
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().right('#').corner_top_right('*')),
        static_table!(
            "                                        *"
            " N     column 0      column 1  column 2 #"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().right('#').corner_top_right('*').corner_bottom_right('@')),
        static_table!(
            "                                        *"
            " N     column 0      column 1  column 2 #"
            "                                        @"
            " 0  a longer string    0-1       0-2     "
            " 1        1-0          1-1       1-2     "
            " 2        2-0          2-1       2-2     "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::default().right('#').corner_top_right('*').corner_bottom_left('@')),
        static_table!(
            "                                         *"
            " N     column 0      column 1   column 2 #"
            "                              @           "
            " 0  a longer string    0-1        0-2     "
            " 1        1-0          1-1        1-2     "
            " 2        2-0          2-1        2-2     "
        )
    }
    test_border! {
        Modify::new((0, 3)).with(Border::filled('@')),
        static_table!(
            "                              @@@@@@@@@@@@"
            " N     column 0      column 1 @ column 2 @"
            "                              @@@@@@@@@@@@"
            " 0  a longer string    0-1        0-2     "
            " 1        1-0          1-1        1-2     "
            " 2        2-0          2-1        2-2     "
        )
    }
}

#[cfg(feature = "color")]
#[test]
fn border_colored_test() {
    let table = Matrix::table(2, 2)
        .with(Style::ascii())
        .with(
            Modify::new(Rows::single(1))
                .with(
                    BorderColor::filled(Color::try_from('*'.blue().to_string()).unwrap())
                        .top(Color::try_from('#'.truecolor(12, 220, 100).to_string()).unwrap()),
                )
                .with(Border::filled('*').top('#')),
        )
        .to_string();

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table),
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "*###*##########*##########*"
            "* 0 *   0-0    *   0-1    *"
            "***************************"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
        )
    );

    assert_eq!(
        table,
        static_table!(
            "+---+----------+----------+"
            "| N | column 0 | column 1 |"
            "\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m###\u{1b}[39m\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m##########\u{1b}[39m\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m##########\u{1b}[39m\u{1b}[34m*\u{1b}[39m"
            "\u{1b}[34m*\u{1b}[39m 0 \u{1b}[34m*\u{1b}[39m   0-0    \u{1b}[34m*\u{1b}[39m   0-1    \u{1b}[34m*\u{1b}[39m"
            "\u{1b}[34m***************************\u{1b}[39m"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
        )
    );

    let table = Matrix::table(2, 2)
        .with(Style::empty())
        .with(
            Modify::new(Rows::single(1))
                .with(
                    BorderColor::filled(Color::try_from('*'.blue().to_string()).unwrap())
                        .top(Color::try_from('#'.truecolor(12, 220, 100).to_string()).unwrap()),
                )
                .with(Border::filled('*').top('#')),
        )
        .to_string();

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table),
        static_table!(
            "  N   column 0   column 1  "
            "*###*##########*##########*"
            "* 0 *   0-0    *   0-1    *"
            "***************************"
            "  1     1-0        1-1     "
        )
    );

    assert_eq!(
        table,
        "  N   column 0   column 1  \n\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m###\u{1b}[39m\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m##########\u{1b}[39m\u{1b}[34m*\u{1b}[39m\u{1b}[38;2;12;220;100m##########\u{1b}[39m\u{1b}[34m*\u{1b}[39m\n\u{1b}[34m*\u{1b}[39m 0 \u{1b}[34m*\u{1b}[39m   0-0    \u{1b}[34m*\u{1b}[39m   0-1    \u{1b}[34m*\u{1b}[39m\n\u{1b}[34m***************************\u{1b}[39m\n  1     1-0        1-1     ",
    );
}

#[cfg(feature = "color")]
#[test]
fn style_with_color_test() {
    let mut style: RawStyle = Style::ascii().into();
    style
        .set_left(Some('['))
        .set_right(Some(']'))
        .set_top(Some('-'))
        .set_bottom(Some('-'))
        .set_vertical(Some('|'))
        .set_intersection(Some('+'));
    style
        .set_color_left(Color::FG_RED)
        .set_color_right(Color::FG_RED)
        .set_color_top(Color::FG_BLUE)
        .set_color_bottom(Color::FG_BLUE)
        .set_color_vertical(Color::FG_YELLOW)
        .set_color_intersection(Color::try_from(' '.purple().to_string()).unwrap());

    let table = Matrix::new(3, 3).with(style).to_string();

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table),
        static_table!(
            "+---+----------+----------+----------+"
            "[ N | column 0 | column 1 | column 2 ]"
            "+---+----------+----------+----------+"
            "[ 0 |   0-0    |   0-1    |   0-2    ]"
            "+---+----------+----------+----------+"
            "[ 1 |   1-0    |   1-1    |   1-2    ]"
            "+---+----------+----------+----------+"
            "[ 2 |   2-0    |   2-1    |   2-2    ]"
            "+---+----------+----------+----------+"
        )
    );

    assert_eq!(table, "+\u{1b}[34m---\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\n\u{1b}[31m[\u{1b}[39m N \u{1b}[33m|\u{1b}[39m column 0 \u{1b}[33m|\u{1b}[39m column 1 \u{1b}[33m|\u{1b}[39m column 2 \u{1b}[31m]\u{1b}[39m\n+---\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------+\n\u{1b}[31m[\u{1b}[39m 0 \u{1b}[33m|\u{1b}[39m   0-0    \u{1b}[33m|\u{1b}[39m   0-1    \u{1b}[33m|\u{1b}[39m   0-2    \u{1b}[31m]\u{1b}[39m\n+---\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------+\n\u{1b}[31m[\u{1b}[39m 1 \u{1b}[33m|\u{1b}[39m   1-0    \u{1b}[33m|\u{1b}[39m   1-1    \u{1b}[33m|\u{1b}[39m   1-2    \u{1b}[31m]\u{1b}[39m\n+---\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------\u{1b}[35m+\u{1b}[39m----------+\n\u{1b}[31m[\u{1b}[39m 2 \u{1b}[33m|\u{1b}[39m   2-0    \u{1b}[33m|\u{1b}[39m   2-1    \u{1b}[33m|\u{1b}[39m   2-2    \u{1b}[31m]\u{1b}[39m\n+\u{1b}[34m---\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+\u{1b}[34m----------\u{1b}[39m+");
}

test_table!(
    empty_line_clears_lines,
    Matrix::new(3, 3).with(Style::rounded().remove_horizontals()),
    "╭───┬──────────┬──────────┬──────────╮"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "╰───┴──────────┴──────────┴──────────╯"
);

test_table!(
    empty_line_clears_lines_1,
    Matrix::new(3, 3).with(Style::rounded().remove_horizontals()),
    "╭───┬──────────┬──────────┬──────────╮"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "╰───┴──────────┴──────────┴──────────╯"
);

test_table!(
    border_color,
    {
        use tabled::settings::Color;
        Matrix::new(3, 3).with(Style::psql()).with(Color::BG_GREEN)
    },
    " \u{1b}[42mN\u{1b}[49m | \u{1b}[42mcolumn 0\u{1b}[49m | \u{1b}[42mcolumn 1\u{1b}[49m | \u{1b}[42mcolumn 2\u{1b}[49m \n---+----------+----------+----------\n \u{1b}[42m0\u{1b}[49m |   \u{1b}[42m0-0\u{1b}[49m    |   \u{1b}[42m0-1\u{1b}[49m    |   \u{1b}[42m0-2\u{1b}[49m    \n \u{1b}[42m1\u{1b}[49m |   \u{1b}[42m1-0\u{1b}[49m    |   \u{1b}[42m1-1\u{1b}[49m    |   \u{1b}[42m1-2\u{1b}[49m    \n \u{1b}[42m2\u{1b}[49m |   \u{1b}[42m2-0\u{1b}[49m    |   \u{1b}[42m2-1\u{1b}[49m    |   \u{1b}[42m2-2\u{1b}[49m    "
);

test_table!(
    text_color,
    {
        use tabled::settings::Color;
        Matrix::new(3, 3).with(Style::psql()).with(Modify::new(Segment::all()).with(Color::BG_BLACK))
    },
    " \u{1b}[40mN\u{1b}[49m | \u{1b}[40mcolumn 0\u{1b}[49m | \u{1b}[40mcolumn 1\u{1b}[49m | \u{1b}[40mcolumn 2\u{1b}[49m \n---+----------+----------+----------\n \u{1b}[40m0\u{1b}[49m |   \u{1b}[40m0-0\u{1b}[49m    |   \u{1b}[40m0-1\u{1b}[49m    |   \u{1b}[40m0-2\u{1b}[49m    \n \u{1b}[40m1\u{1b}[49m |   \u{1b}[40m1-0\u{1b}[49m    |   \u{1b}[40m1-1\u{1b}[49m    |   \u{1b}[40m1-2\u{1b}[49m    \n \u{1b}[40m2\u{1b}[49m |   \u{1b}[40m2-0\u{1b}[49m    |   \u{1b}[40m2-1\u{1b}[49m    |   \u{1b}[40m2-2\u{1b}[49m    "
);

test_table!(
    verticals_0,
    Matrix::new(3, 3)
        .with(Style::rounded().verticals(vec![VerticalLine::new(0, Line::filled('+')), VerticalLine::new(4, Line::filled('+'))])),
    "+───┬──────────┬──────────┬──────────+"
    "+ N │ column 0 │ column 1 │ column 2 +"
    "├───┼──────────┼──────────┼──────────┤"
    "+ 0 │   0-0    │   0-1    │   0-2    +"
    "+ 1 │   1-0    │   1-1    │   1-2    +"
    "+ 2 │   2-0    │   2-1    │   2-2    +"
    "+───┴──────────┴──────────┴──────────+"
);

test_table!(
    verticals_1,
    Matrix::new(3, 3)
        .with(Style::rounded().verticals((1..4).map(|i| VerticalLine::new(i, Line::filled('+'))))),
    "╭───+──────────+──────────+──────────╮"
    "│ N + column 0 + column 1 + column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 +   0-0    +   0-1    +   0-2    │"
    "│ 1 +   1-0    +   1-1    +   1-2    │"
    "│ 2 +   2-0    +   2-1    +   2-2    │"
    "╰───+──────────+──────────+──────────╯"
);

test_table!(
    verticals_2,
    Matrix::new(3, 3).with(Style::rounded().verticals(vec![VerticalLine::new(1, Line::filled('+'))])),
    "╭───+──────────┬──────────┬──────────╮"
    "│ N + column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 +   0-0    │   0-1    │   0-2    │"
    "│ 1 +   1-0    │   1-1    │   1-2    │"
    "│ 2 +   2-0    │   2-1    │   2-2    │"
    "╰───+──────────┴──────────┴──────────╯"
);

test_table!(
    verticals_3,
    Matrix::new(3, 3).with(Style::ascii().verticals([VerticalLine::new(1, Line::filled('*'))])),
    "+---*----------+----------+----------+"
    "| N * column 0 | column 1 | column 2 |"
    "+---*----------+----------+----------+"
    "| 0 *   0-0    |   0-1    |   0-2    |"
    "+---*----------+----------+----------+"
    "| 1 *   1-0    |   1-1    |   1-2    |"
    "+---*----------+----------+----------+"
    "| 2 *   2-0    |   2-1    |   2-2    |"
    "+---*----------+----------+----------+"
);

test_table!(
    verticals_4,
    Matrix::new(3, 3).with(Style::ascii().verticals((0..10).map(|i| VerticalLine::new(i, Line::new(Some('*'), Some('x'), Some('c'), Some('2')))))),
    "c---c----------c----------c----------c"
    "* N * column 0 * column 1 * column 2 *"
    "x---x----------x----------x----------x"
    "* 0 *   0-0    *   0-1    *   0-2    *"
    "x---x----------x----------x----------x"
    "* 1 *   1-0    *   1-1    *   1-2    *"
    "x---x----------x----------x----------x"
    "* 2 *   2-0    *   2-1    *   2-2    *"
    "2---2----------2----------2----------2"
);

test_table!(
    vertical_line_0,
    Matrix::new(3, 3)
        .with(HorizontalLine::new(1, Line::new(Some('8'), Some('8'), Some('8'), Some('8'))))
        .with(VerticalLine::new(1, Line::new(Some('*'), Some('x'), Some('c'), Some('2')))),
    "+---c----------+----------+----------+"
    "| N * column 0 | column 1 | column 2 |"
    "88888888888888888888888888888888888888"
    "| 0 *   0-0    |   0-1    |   0-2    |"
    "+---x----------+----------+----------+"
    "| 1 *   1-0    |   1-1    |   1-2    |"
    "+---x----------+----------+----------+"
    "| 2 *   2-0    |   2-1    |   2-2    |"
    "+---2----------+----------+----------+"
);

test_table!(
    vertical_line_1,
    Matrix::new(3, 3)
        .with(Style::empty())
        .with(VerticalLine::new(1, Line::new(Some('*'), Some('x'), Some('c'), Some('2')))),
    "   c                              "
    " N * column 0  column 1  column 2 "
    " 0 *   0-0       0-1       0-2    "
    " 1 *   1-0       1-1       1-2    "
    " 2 *   2-0       2-1       2-2    "
    "   2                              "
);

test_table!(
    vertical_line_2,
    Matrix::new(3, 3)
        .with(Style::empty())
        .with(VerticalLine::new(1, Line::new(None, Some('x'), Some('c'), Some('2')))),
    "   c                              "
    " N   column 0  column 1  column 2 "
    " 0     0-0       0-1       0-2    "
    " 1     1-0       1-1       1-2    "
    " 2     2-0       2-1       2-2    "
    "   2                              "
);

test_table!(
    vertical_line_3,
    Matrix::new(3, 3)
        .with(Style::empty())
        .with(VerticalLine::new(1, Line::new(Some('*'), Some('x'), None, None))),
    " N * column 0  column 1  column 2 "
    " 0 *   0-0       0-1       0-2    "
    " 1 *   1-0       1-1       1-2    "
    " 2 *   2-0       2-1       2-2    "
);

test_table!(
    override_horizontal_border_on_line,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::single(1))
            .with(BorderChar::horizontal(':', Offset::Begin(0)))
            .with(BorderChar::horizontal(':', Offset::End(0)))
    ),
    "| N | column 0 | column 1 | column 2 |"
    "|:-:|:--------:|:--------:|:--------:|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    override_horizontal_border_on_borders,
    Matrix::new(3, 3)
        .with(Modify::new(Rows::new(..5))
            .with(BorderChar::horizontal(':', Offset::Begin(0)))
            .with(BorderChar::horizontal('y', Offset::Begin(3)))
            .with(BorderChar::horizontal(':', Offset::End(0)))
            .with(BorderChar::horizontal('x', Offset::End(3)))
    ),
    "+:-:+:--y--x--:+:--y--x--:+:--y--x--:+"
    "| N | column 0 | column 1 | column 2 |"
    "+:-:+:--y--x--:+:--y--x--:+:--y--x--:+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+:-:+:--y--x--:+:--y--x--:+:--y--x--:+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+:-:+:--y--x--:+:--y--x--:+:--y--x--:+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+:-:+:--y--x--:+:--y--x--:+:--y--x--:+"
);

test_table!(
    override_horizontal_border_on_border,
    Matrix::new(3, 3)
        .with(Modify::new(Rows::new(..5))
            .with(Border::filled('['))
            .with(BorderChar::horizontal(':', Offset::Begin(0)))
            .with(BorderChar::horizontal('y', Offset::Begin(3)))
            .with(BorderChar::horizontal(':', Offset::End(0)))
            .with(BorderChar::horizontal('x', Offset::End(3)))
    ),
    "[:[:[:[[y[[x[[:[:[[y[[x[[:[:[[y[[x[[:["
    "[ N [ column 0 [ column 1 [ column 2 ["
    "[:[:[:[[y[[x[[:[:[[y[[x[[:[:[[y[[x[[:["
    "[ 0 [   0-0    [   0-1    [   0-2    ["
    "[:[:[:[[y[[x[[:[:[[y[[x[[:[:[[y[[x[[:["
    "[ 1 [   1-0    [   1-1    [   1-2    ["
    "[:[:[:[[y[[x[[:[:[[y[[x[[:[:[[y[[x[[:["
    "[ 2 [   2-0    [   2-1    [   2-2    ["
    "[:[:[:[[y[[x[[:[:[[y[[x[[:[:[[y[[x[[:["
);

test_table!(
    override_vertical_border_on_line,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::single(1))
            .with(BorderChar::vertical(':', Offset::Begin(0)))
    ),
    "| N : column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 :   0-0    |   0-1    |   0-2    |"
    "| 1 :   1-0    |   1-1    |   1-2    |"
    "| 2 :   2-0    |   2-1    |   2-2    |"
);

test_table!(
    override_vertical_border_on_line_1,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::single(1))
            .with(BorderChar::vertical(':', Offset::End(0)))
    ),
    "| N : column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 :   0-0    |   0-1    |   0-2    |"
    "| 1 :   1-0    |   1-1    |   1-2    |"
    "| 2 :   2-0    |   2-1    |   2-2    |"
);

test_table!(
    override_vertical_border_on_line_multiline,
    Matrix::new(3, 3)
        .with(Modify::new(Rows::single(1)).with(Format::content(|s| format!("\nsome text\ntext\n{s}\ntext\ntext\n"))))
        .with(Style::markdown())
        .with(Modify::new(Columns::single(1))
            .with(BorderChar::vertical(':', Offset::Begin(4)))
    ),
    "|     N     | column 0  | column 1  | column 2  |"
    "|-----------|-----------|-----------|-----------|"
    "|           |           |           |           |"
    "| some text | some text | some text | some text |"
    "| text      | text      | text      | text      |"
    "| 0         | 0-0       | 0-1       | 0-2       |"
    "| text      : text      | text      | text      |"
    "| text      | text      | text      | text      |"
    "|           |           |           |           |"
    "|     1     |    1-0    |    1-1    |    1-2    |"
    "|     2     |    2-0    |    2-1    |    2-2    |"
);

test_table!(
    override_vertical_border_on_line_multiline_2,
    Matrix::new(3, 3)
        .with(Modify::new(Rows::single(1)).with(Format::content(|s| format!("\nsome text\ntext\n{s}\ntext\ntext\n"))))
        .with(Style::markdown())
        .with(Modify::new(Columns::single(1))
            .with(BorderChar::vertical(':', Offset::End(4)))
    ),
    "|     N     | column 0  | column 1  | column 2  |"
    "|-----------|-----------|-----------|-----------|"
    "|           |           |           |           |"
    "| some text | some text | some text | some text |"
    "| text      : text      | text      | text      |"
    "| 0         | 0-0       | 0-1       | 0-2       |"
    "| text      | text      | text      | text      |"
    "| text      | text      | text      | text      |"
    "|           |           |           |           |"
    "|     1     |    1-0    |    1-1    |    1-2    |"
    "|     2     |    2-0    |    2-1    |    2-2    |"
);

test_table!(
    override_vertical_and_horizontal_border_on_line,
    Matrix::new(3, 3)
        .with(Modify::new(Rows::single(1)).with(Format::content(|s| format!("\nsome text\ntext\n{s}\ntext\ntext\n"))))
        .with(Style::markdown())
        .with(Modify::new(Columns::new(..5))
            .with(BorderChar::vertical('y', Offset::Begin(0)))
            .with(BorderChar::vertical('^', Offset::End(0)))
        )
        .with(Modify::new(Rows::single(1))
            .with(BorderChar::horizontal('x', Offset::Begin(0)))
            .with(BorderChar::horizontal('@', Offset::End(0)))
        ),
    "y     N     y column 0  y column 1  y column 2  y"
    "|x---------@|x---------@|x---------@|x---------@|"
    "y           y           y           y           y"
    "| some text | some text | some text | some text |"
    "| text      | text      | text      | text      |"
    "| 0         | 0-0       | 0-1       | 0-2       |"
    "| text      | text      | text      | text      |"
    "| text      | text      | text      | text      |"
    "^           ^           ^           ^           ^"
    "y     1     y    1-0    y    1-1    y    1-2    y"
    "y     2     y    2-0    y    2-1    y    2-2    y"
);

test_table!(
    table_format_alignment_left_test,
    format!("{:<}", Table::new(vec!["hello", "world", "!"])),
    "+-------+"
    "| &str  |"
    "+-------+"
    "| hello |"
    "+-------+"
    "| world |"
    "+-------+"
    "| !     |"
    "+-------+"
);

test_table!(
    table_format_alignment_right_test,
    format!("{:>}", Table::new(vec!["hello", "world", "!"])),
    "+-------+"
    "|  &str |"
    "+-------+"
    "| hello |"
    "+-------+"
    "| world |"
    "+-------+"
    "|     ! |"
    "+-------+"
);

test_table!(
    table_format_alignment_center_test,
    format!("{:^}", Table::new(vec!["hello", "world", "!"])),
    "+-------+"
    "| &str  |"
    "+-------+"
    "| hello |"
    "+-------+"
    "| world |"
    "+-------+"
    "|   !   |"
    "+-------+"
);

test_table!(
    table_format_width_0_test,
    format!("{:<13}", Table::new(vec!["hello", "world", "!"])),
    "    +-------+"
    "    | &str  |"
    "    +-------+"
    "    | hello |"
    "    +-------+"
    "    | world |"
    "    +-------+"
    "    | !     |"
    "    +-------+"
);

test_table!(
    table_format_width_1_test,
    format!("{:>13}", Table::new(vec!["hello", "world", "!"])),
    "+-------+    "
    "|  &str |    "
    "+-------+    "
    "| hello |    "
    "+-------+    "
    "| world |    "
    "+-------+    "
    "|     ! |    "
    "+-------+    "
);

test_table!(
    table_format_width_2_test,
    format!("{:^13}", Table::new(vec!["hello", "world", "!"])),
    "  +-------+  "
    "  | &str  |  "
    "  +-------+  "
    "  | hello |  "
    "  +-------+  "
    "  | world |  "
    "  +-------+  "
    "  |   !   |  "
    "  +-------+  "
);

test_table!(
    table_format_width_3_test,
    format!("{:x^13}", Table::new(vec!["hello", "world", "!"])),
    "xx+-------+xx"
    "xx| &str  |xx"
    "xx+-------+xx"
    "xx| hello |xx"
    "xx+-------+xx"
    "xx| world |xx"
    "xx+-------+xx"
    "xx|   !   |xx"
    "xx+-------+xx"
);

test_table!(
    table_format_width_4_test,
    format!("{:x<13}", Table::new(vec!["hello", "world", "!"])),
    "xxxx+-------+"
    "xxxx| &str  |"
    "xxxx+-------+"
    "xxxx| hello |"
    "xxxx+-------+"
    "xxxx| world |"
    "xxxx+-------+"
    "xxxx| !     |"
    "xxxx+-------+"
);

test_table!(
    table_format_width_5_test,
    format!("{:x>13}", Table::new(vec!["hello", "world", "!"])),
    "+-------+xxxx"
    "|  &str |xxxx"
    "+-------+xxxx"
    "| hello |xxxx"
    "+-------+xxxx"
    "| world |xxxx"
    "+-------+xxxx"
    "|     ! |xxxx"
    "+-------+xxxx"
);

test_table!(
    table_style_no_bottom_no_new_line,
    Matrix::table(0, 0).with(Style::markdown().remove_horizontals()),
    "| N |"
);
