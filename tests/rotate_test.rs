// todo: add method for SPACING between cells.

use tabled::{
    object::{Cell, Rows},
    style::{Border, Style},
    Highlight, Rotate, Table,
};

use crate::util::static_table;

mod util;

#[test]
fn test_rotate() {
    let table = || Table::new([(123, 456, 789), (234, 567, 891)]);

    assert_eq!(
        table()
            .with(Rotate::Left)
            .with(Rotate::Left)
            .with(Rotate::Left)
            .with(Rotate::Left)
            .to_string(),
        table().to_string()
    );
    assert_eq!(
        table()
            .with(Rotate::Right)
            .with(Rotate::Right)
            .with(Rotate::Right)
            .with(Rotate::Right)
            .to_string(),
        table().to_string()
    );
    assert_eq!(
        table().with(Rotate::Right).with(Rotate::Left).to_string(),
        table().to_string()
    );
    assert_eq!(
        table().with(Rotate::Left).with(Rotate::Right).to_string(),
        table().to_string()
    );
    assert_eq!(
        table().with(Rotate::Bottom).with(Rotate::Top).to_string(),
        table().to_string()
    );
    assert_eq!(
        table()
            .with(Rotate::Bottom)
            .with(Rotate::Bottom)
            .to_string(),
        table().to_string()
    );
    assert_eq!(
        table().with(Rotate::Top).with(Rotate::Top).to_string(),
        table().to_string()
    );
}

#[test]
fn test_3x3_box() {
    let table = Table::new([(123, 456, 789), (234, 567, 891)]);

    let table = table.with(Rotate::Left);
    assert_eq!(
        table.to_string(),
        static_table!(
            "+-----+-----+-----+"
            "| i32 | 789 | 891 |"
            "+-----+-----+-----+"
            "| i32 | 456 | 567 |"
            "+-----+-----+-----+"
            "| i32 | 123 | 234 |"
            "+-----+-----+-----+"
        )
    );

    let table = table.with(Rotate::Right).with(Rotate::Right);
    assert_eq!(
        table.to_string(),
        static_table!(
            "+-----+-----+-----+"
            "| 234 | 123 | i32 |"
            "+-----+-----+-----+"
            "| 567 | 456 | i32 |"
            "+-----+-----+-----+"
            "| 891 | 789 | i32 |"
            "+-----+-----+-----+"
        )
    );
}

#[test]
fn test_left_rotate() {
    let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);

    let table = table.with(Rotate::Left);
    assert_eq!(
        table.to_string(),
        static_table!(
            "+-----+-----+-----+-----+"
            "| i32 | 789 | 891 | 333 |"
            "+-----+-----+-----+-----+"
            "| i32 | 456 | 567 | 222 |"
            "+-----+-----+-----+-----+"
            "| i32 | 123 | 234 | 111 |"
            "+-----+-----+-----+-----+"
        ),
    )
}

#[test]
fn test_right_rotate() {
    let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);

    let table = table.with(Rotate::Right);
    assert_eq!(
        table.to_string(),
        static_table!(
            "+-----+-----+-----+-----+"
            "| 111 | 234 | 123 | i32 |"
            "+-----+-----+-----+-----+"
            "| 222 | 567 | 456 | i32 |"
            "+-----+-----+-----+-----+"
            "| 333 | 891 | 789 | i32 |"
            "+-----+-----+-----+-----+"
        )
    );
}

#[test]
fn test_bottom_rotate() {
    let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);

    let table = table.with(Rotate::Bottom);
    assert_eq!(
        table.to_string(),
        static_table!(
            "+-----+-----+-----+"
            "| 111 | 222 | 333 |"
            "+-----+-----+-----+"
            "| 234 | 567 | 891 |"
            "+-----+-----+-----+"
            "| 123 | 456 | 789 |"
            "+-----+-----+-----+"
            "| i32 | i32 | i32 |"
            "+-----+-----+-----+"
        )
    );
}

#[test]
fn test_top_rotate() {
    let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);

    let table = table.with(Rotate::Top);
    assert_eq!(
        table.to_string(),
        static_table!(
            "+-----+-----+-----+"
            "| 111 | 222 | 333 |"
            "+-----+-----+-----+"
            "| 234 | 567 | 891 |"
            "+-----+-----+-----+"
            "| 123 | 456 | 789 |"
            "+-----+-----+-----+"
            "| i32 | i32 | i32 |"
            "+-----+-----+-----+"
        )
    );
}

#[test]
fn rotate_preserve_border_styles_test() {
    let data = [(123, 456, 789), (234, 567, 891), (111, 222, 333)];

    let table = Table::new(&data)
        .with(Style::ascii())
        .with(Highlight::new(Rows::single(0), Border::default().top('*')))
        .with(Rotate::Left)
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "******+-----+-----+-----+"
            "| i32 | 789 | 891 | 333 |"
            "+-----+-----+-----+-----+"
            "| i32 | 456 | 567 | 222 |"
            "+-----+-----+-----+-----+"
            "| i32 | 123 | 234 | 111 |"
            "+-----+-----+-----+-----+"
        ),
    );

    let table = Table::new(&data)
        .with(Style::ascii())
        .with(Highlight::new(Cell(0, 2), Border::default().bottom('*')))
        .with(Rotate::Left)
        .to_string();

    // it's a correct behaviour because
    // when we sen bottom border of cell(0, 2) we also set top border of cell(1, 2)
    //
    // todo: determine if it's correct
    assert_eq!(
        table,
        static_table!(
            "+-----+*****+-----+-----+"
            "| i32 | 789 | 891 | 333 |"
            "+*****+-----+-----+-----+"
            "| i32 | 456 | 567 | 222 |"
            "+-----+-----+-----+-----+"
            "| i32 | 123 | 234 | 111 |"
            "+-----+-----+-----+-----+"
        ),
    );
}
