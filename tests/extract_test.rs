use tabled::{object::Segment, Alignment, Extract, Format, Modify, Padding, Table};

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn extract_segment_full_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(.., ..))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-------+--------------+--------------+--------------+"
            "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
            "+-------+--------------+--------------+--------------+"
            "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
            "+-------+--------------+--------------+--------------+"
            "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
            "+-------+--------------+--------------+--------------+"
            "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |"
            "+-------+--------------+--------------+--------------+"
        )
    );
}

#[test]
fn extract_segment_skip_top_row_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(1.., ..))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-------+---------+---------+---------+"
            "|   [0] |   [0-0] |   [0-1] |   [0-2] |"
            "+-------+---------+---------+---------+"
            "|   [1] |   [1-0] |   [1-1] |   [1-2] |"
            "+-------+---------+---------+---------+"
            "|   [2] |   [2-0] |   [2-1] |   [2-2] |"
            "+-------+---------+---------+---------+"
        )
    );
}

#[test]
fn extract_segment_skip_left_col_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(.., 1..))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+--------------+--------------+--------------+"
            "|   [column 0] |   [column 1] |   [column 2] |"
            "+--------------+--------------+--------------+"
            "|   [0-0]      |   [0-1]      |   [0-2]      |"
            "+--------------+--------------+--------------+"
            "|   [1-0]      |   [1-1]      |   [1-2]      |"
            "+--------------+--------------+--------------+"
            "|   [2-0]      |   [2-1]      |   [2-2]      |"
            "+--------------+--------------+--------------+"
        )
    );
}

#[test]
fn extract_segment_bottom_right_square_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(2.., 2..))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+---------+---------+"
            "|   [1-1] |   [1-2] |"
            "+---------+---------+"
            "|   [2-1] |   [2-2] |"
            "+---------+---------+"
        )
    );
}

#[test]
fn extract_segment_middle_section_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(1..3, 1..))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+---------+---------+---------+"
            "|   [0-0] |   [0-1] |   [0-2] |"
            "+---------+---------+---------+"
            "|   [1-0] |   [1-1] |   [1-2] |"
            "+---------+---------+---------+"
        )
    );
}

#[test]
fn extract_segment_empty_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(1..1, 1..1))
        .to_string();

    assert_eq!(table, "");
}

#[test]
fn extract_rows_full_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::rows(..))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-------+--------------+--------------+--------------+"
            "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
            "+-------+--------------+--------------+--------------+"
            "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
            "+-------+--------------+--------------+--------------+"
            "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
            "+-------+--------------+--------------+--------------+"
            "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |"
            "+-------+--------------+--------------+--------------+"
        )
    );
}

#[test]
fn extract_rows_empty_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::rows(0..0))
        .to_string();

    assert_eq!(table, "");
}

#[test]
fn extract_rows_partial_view_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::rows(0..=2))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-------+--------------+--------------+--------------+"
            "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
            "+-------+--------------+--------------+--------------+"
            "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
            "+-------+--------------+--------------+--------------+"
            "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
            "+-------+--------------+--------------+--------------+"
        )
    );
}

#[test]
fn extract_columns_full_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::columns(..))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-------+--------------+--------------+--------------+"
            "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
            "+-------+--------------+--------------+--------------+"
            "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
            "+-------+--------------+--------------+--------------+"
            "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
            "+-------+--------------+--------------+--------------+"
            "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |"
            "+-------+--------------+--------------+--------------+"
        )
    );
}

#[test]
fn extract_columns_empty_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::columns(0..0))
        .to_string();

    assert_eq!(table, "");
}

#[test]
fn extract_columns_partial_view_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::columns(0..2))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-------+--------------+"
            "|   [N] |   [column 0] |"
            "+-------+--------------+"
            "|   [0] |   [0-0]      |"
            "+-------+--------------+"
            "|   [1] |   [1-0]      |"
            "+-------+--------------+"
            "|   [2] |   [2-0]      |"
            "+-------+--------------+"
        )
    );
}
