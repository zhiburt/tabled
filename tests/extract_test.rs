use tabled::{object::Segment, Alignment, Disable, Extract, Format, Modify, Padding, Table};

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

#[test]
fn extract_inside_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Disable::Row(..1))
        .with(Extract::segment(1..2, 1..2));

    assert_eq!(
        table.to_string(),
        "+-----+\n\
         | 1-0 |\n\
         +-----+"
    )
}

#[test]
fn extract_left_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Disable::Row(..1))
        .with(Extract::segment(.., ..1));

    assert_eq!(
        table.to_string(),
        "+---+\n\
         | 0 |\n\
         +---+\n\
         | 1 |\n\
         +---+\n\
         | 2 |\n\
         +---+"
    )
}

#[test]
fn extract_right_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Disable::Row(..1))
        .with(Extract::segment(.., 2..));

    assert_eq!(
        table.to_string(),
        "+-----+-----+\n\
         | 0-1 | 0-2 |\n\
         +-----+-----+\n\
         | 1-1 | 1-2 |\n\
         +-----+-----+\n\
         | 2-1 | 2-2 |\n\
         +-----+-----+"
    )
}

#[test]
fn extract_top_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Disable::Row(..1))
        .with(Extract::segment(..1, ..));

    assert_eq!(
        table.to_string(),
        "+---+-----+-----+-----+\n\
         | 0 | 0-0 | 0-1 | 0-2 |\n\
         +---+-----+-----+-----+"
    )
}

#[test]
fn extract_bottom_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Disable::Row(..1))
        .with(Extract::segment(2.., ..));

    assert_eq!(
        table.to_string(),
        "+---+-----+-----+-----+\n\
         | 2 | 2-0 | 2-1 | 2-2 |\n\
         +---+-----+-----+-----+",
    )
}

#[test]
fn extract_all_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Disable::Row(..1))
        .with(Extract::segment(3.., 3..));

    assert_eq!(table.to_string(), "");
}

#[test]
fn extract_empty_test() {
    let table = tabled::builder::Builder::default()
        .build()
        .with(Extract::segment(.., ..));

    assert_eq!(table.to_string(), "");
}
