#![cfg(feature = "std")]

use tabled::{
    builder::Builder,
    settings::{
        object::{Rows, Segment},
        Alignment, Disable, Extract, Format, Modify, Padding,
    },
};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    extract_segment_full_test,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]"))))
        .with(Extract::segment(.., ..)),
    "+-------+--------------+--------------+--------------+"
    "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
    "+-------+--------------+--------------+--------------+"
    "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
    "+-------+--------------+--------------+--------------+"
    "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
    "+-------+--------------+--------------+--------------+"
    "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |"
    "+-------+--------------+--------------+--------------+"
);

test_table!(
    extract_segment_skip_top_row_test,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]"))))
        .with(Extract::segment(1.., ..)),
    "+-------+---------+---------+---------+"
    "|   [0] |   [0-0] |   [0-1] |   [0-2] |"
    "+-------+---------+---------+---------+"
    "|   [1] |   [1-0] |   [1-1] |   [1-2] |"
    "+-------+---------+---------+---------+"
    "|   [2] |   [2-0] |   [2-1] |   [2-2] |"
    "+-------+---------+---------+---------+"
);

test_table!(
    extract_segment_skip_column_test,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]"))))
        .with(Extract::segment(.., 1..)),
    "+--------------+--------------+--------------+"
    "|   [column 0] |   [column 1] |   [column 2] |"
    "+--------------+--------------+--------------+"
    "|   [0-0]      |   [0-1]      |   [0-2]      |"
    "+--------------+--------------+--------------+"
    "|   [1-0]      |   [1-1]      |   [1-2]      |"
    "+--------------+--------------+--------------+"
    "|   [2-0]      |   [2-1]      |   [2-2]      |"
    "+--------------+--------------+--------------+"
);

test_table!(
    extract_segment_bottom_right_square_test,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]"))))
        .with(Extract::segment(2.., 2..)),
    "+---------+---------+"
    "|   [1-1] |   [1-2] |"
    "+---------+---------+"
    "|   [2-1] |   [2-2] |"
    "+---------+---------+"
);

test_table!(
    extract_segment_middle_section_test,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]"))))
        .with(Extract::segment(1..3, 1..)),
        "+---------+---------+---------+"
        "|   [0-0] |   [0-1] |   [0-2] |"
        "+---------+---------+---------+"
        "|   [1-0] |   [1-1] |   [1-2] |"
        "+---------+---------+---------+"
);

test_table!(
    extract_segment_empty_test,
    Matrix::new(3, 3).with(Extract::segment(1..1, 1..1)),
    ""
);

test_table!(
    extract_rows_full_test,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]"))))
        .with(Extract::rows(..)),
    "+-------+--------------+--------------+--------------+"
    "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
    "+-------+--------------+--------------+--------------+"
    "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
    "+-------+--------------+--------------+--------------+"
    "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
    "+-------+--------------+--------------+--------------+"
    "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |"
    "+-------+--------------+--------------+--------------+"
);

test_table!(
    extract_rows_empty_test,
    Matrix::new(3, 3).with(Extract::rows(0..0)),
    ""
);

test_table!(
    extract_rows_partial_view_test,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]"))))
        .with(Extract::rows(0..=2)),
    "+-------+--------------+--------------+--------------+"
    "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
    "+-------+--------------+--------------+--------------+"
    "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
    "+-------+--------------+--------------+--------------+"
    "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
    "+-------+--------------+--------------+--------------+"
);

test_table!(
    extract_columns_full_test,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]"))))
        .with(Extract::columns(..)),
    "+-------+--------------+--------------+--------------+"
    "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
    "+-------+--------------+--------------+--------------+"
    "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
    "+-------+--------------+--------------+--------------+"
    "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
    "+-------+--------------+--------------+--------------+"
    "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |"
    "+-------+--------------+--------------+--------------+"
);

test_table!(
    extract_columns_empty_test,
    Matrix::new(3, 3).with(Extract::columns(0..0)),
    ""
);

test_table!(
    extract_columns_partial_view_test,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]"))))
        .with(Extract::columns(0..2)),
    "+-------+--------------+"
    "|   [N] |   [column 0] |"
    "+-------+--------------+"
    "|   [0] |   [0-0]      |"
    "+-------+--------------+"
    "|   [1] |   [1-0]      |"
    "+-------+--------------+"
    "|   [2] |   [2-0]      |"
    "+-------+--------------+"
);

test_table!(
    extract_inside_test,
    Matrix::new(3, 3).with(Disable::row(Rows::first())).with(Extract::segment(1..2, 1..2)),
    "+-----+"
    "| 1-0 |"
    "+-----+"
);

test_table!(
    extract_left_test,
    Matrix::new(3, 3).with(Disable::row(Rows::first())).with(Extract::segment(.., ..1)),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
    "| 2 |"
    "+---+"
);

test_table!(
    extract_right_test,
    Matrix::new(3, 3).with(Disable::row(Rows::first())).with(Extract::segment(.., 2..)),
    "+-----+-----+"
    "| 0-1 | 0-2 |"
    "+-----+-----+"
    "| 1-1 | 1-2 |"
    "+-----+-----+"
    "| 2-1 | 2-2 |"
    "+-----+-----+"
);

test_table!(
    extract_top_test,
    Matrix::new(3, 3).with(Disable::row(Rows::first())).with(Extract::segment(..1, ..)),
    "+---+-----+-----+-----+"
    "| 0 | 0-0 | 0-1 | 0-2 |"
    "+---+-----+-----+-----+"
);

test_table!(
    extract_bottom_test,
    Matrix::new(3, 3).with(Disable::row(Rows::first())).with(Extract::segment(2.., ..)),
    "+---+-----+-----+-----+"
    "| 2 | 2-0 | 2-1 | 2-2 |"
    "+---+-----+-----+-----+"
);

test_table!(
    extract_all_test,
    Matrix::new(3, 3)
        .with(Disable::row(Rows::first()))
        .with(Extract::segment(3.., 3..)),
    ""
);

test_table!(
    extract_empty_test,
    Builder::default().build().with(Extract::segment(.., ..)),
    ""
);
