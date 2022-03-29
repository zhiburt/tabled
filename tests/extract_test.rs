use crate::util::create_vector;
use tabled::{object::Full, Alignment, Extract, Format, Modify, Padding, Table};

mod util;

#[test]
fn extract_segment_full_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(.., ..))
        .to_string();

    let expected = concat!(
        "+-------+--------------+--------------+--------------+\n",
        "|   [N] |   [column 0] |   [column 1] |   [column 2] |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extract_segment_skip_top_row_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(1.., ..))
        .to_string();

    let expected = concat!(
        "+-------+---------+---------+---------+\n",
        "|   [0] |   [0-0] |   [0-1] |   [0-2] |\n",
        "+-------+---------+---------+---------+\n",
        "|   [1] |   [1-0] |   [1-1] |   [1-2] |\n",
        "+-------+---------+---------+---------+\n",
        "|   [2] |   [2-0] |   [2-1] |   [2-2] |\n",
        "+-------+---------+---------+---------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extract_segment_skip_left_col_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(.., 1..))
        .to_string();

    let expected = concat!(
        "+--------------+--------------+--------------+\n",
        "|   [column 0] |   [column 1] |   [column 2] |\n",
        "+--------------+--------------+--------------+\n",
        "|   [0-0]      |   [0-1]      |   [0-2]      |\n",
        "+--------------+--------------+--------------+\n",
        "|   [1-0]      |   [1-1]      |   [1-2]      |\n",
        "+--------------+--------------+--------------+\n",
        "|   [2-0]      |   [2-1]      |   [2-2]      |\n",
        "+--------------+--------------+--------------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extract_segment_bottom_right_square_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(2.., 2..))
        .to_string();

    let expected = concat!(
        "+---------+---------+\n",
        "|   [1-1] |   [1-2] |\n",
        "+---------+---------+\n",
        "|   [2-1] |   [2-2] |\n",
        "+---------+---------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extract_segment_middle_section_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(1..3, 1..))
        .to_string();

    let expected = concat!(
        "+---------+---------+---------+\n",
        "|   [0-0] |   [0-1] |   [0-2] |\n",
        "+---------+---------+---------+\n",
        "|   [1-0] |   [1-1] |   [1-2] |\n",
        "+---------+---------+---------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extract_segment_empty_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::segment(1..1, 1..1))
        .to_string();

    assert_eq!(table, "");
}

#[test]
fn extract_rows_full_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::rows(..))
        .to_string();

    let expected = concat!(
        "+-------+--------------+--------------+--------------+\n",
        "|   [N] |   [column 0] |   [column 1] |   [column 2] |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extract_rows_empty_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::rows(0..0))
        .to_string();

    assert_eq!(table, "");
}

#[test]
fn extract_rows_partial_view_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::rows(0..=2))
        .to_string();

    let expected = concat!(
        "+-------+--------------+--------------+--------------+\n",
        "|   [N] |   [column 0] |   [column 1] |   [column 2] |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extract_columns_full_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::columns(..))
        .to_string();

    let expected = concat!(
        "+-------+--------------+--------------+--------------+\n",
        "|   [N] |   [column 0] |   [column 1] |   [column 2] |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
        "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |\n",
        "+-------+--------------+--------------+--------------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extract_columns_empty_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::columns(0..0))
        .to_string();

    assert_eq!(table, "");
}

#[test]
fn extract_columns_partial_view_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format::new(|s| format!("[{}]", s))))
        .with(Extract::columns(0..2))
        .to_string();

    let expected = concat!(
        "+-------+--------------+\n",
        "|   [N] |   [column 0] |\n",
        "+-------+--------------+\n",
        "|   [0] |   [0-0]      |\n",
        "+-------+--------------+\n",
        "|   [1] |   [1-0]      |\n",
        "+-------+--------------+\n",
        "|   [2] |   [2-0]      |\n",
        "+-------+--------------+\n",
    );

    assert_eq!(table, expected);
}
