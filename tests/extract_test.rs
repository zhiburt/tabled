use crate::util::create_vector;
use tabled::{Alignment, Extract, Format, Full, Indent, Modify, Table};

mod util;

#[test]
fn extract_full_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Indent::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .with(Extract::new(.., ..))
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
fn extract_skip_top_row_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Indent::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .with(Extract::new(1.., ..))
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
fn extract_skip_left_col_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Indent::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .with(Extract::new(.., 1..))
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
fn extract_bottom_right_square_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Indent::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .with(Extract::new(2.., 2..))
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
fn extract_middle_section_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Indent::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .with(Extract::new(1..3, 1..))
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
fn extract_empty_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(Indent::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .with(Extract::new(1..1, 1..1))
        .to_string();

    assert_eq!(table, "");
}
