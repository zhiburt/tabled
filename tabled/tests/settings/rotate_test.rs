#![cfg(feature = "std")]

// todo: add method for SPACING between cells.

use tabled::settings::{
    object::{Cell, Rows},
    Border, Highlight, Rotate,
};

use crate::matrix::Matrix;
use testing_table::test_table;

#[test]
fn test_rotate() {
    let table = || Matrix::iter([(123, 456, 789), (234, 567, 891)]);

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

test_table!(
    test_3x3_box_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Rotate::Left),
    "+-----+-----+-----+"
    "| i32 | 789 | 891 |"
    "+-----+-----+-----+"
    "| i32 | 456 | 567 |"
    "+-----+-----+-----+"
    "| i32 | 123 | 234 |"
    "+-----+-----+-----+"
);

test_table!(
    test_3x3_box_1,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Rotate::Left).with(Rotate::Right).with(Rotate::Right),
    "+-----+-----+-----+"
    "| 234 | 123 | i32 |"
    "+-----+-----+-----+"
    "| 567 | 456 | i32 |"
    "+-----+-----+-----+"
    "| 891 | 789 | i32 |"
    "+-----+-----+-----+"
);

test_table!(
    test_left_rotate,
    Matrix::iter([(123, 456, 789), (234, 567, 891), (111, 222, 333)]).with(Rotate::Left),
    "+-----+-----+-----+-----+"
    "| i32 | 789 | 891 | 333 |"
    "+-----+-----+-----+-----+"
    "| i32 | 456 | 567 | 222 |"
    "+-----+-----+-----+-----+"
    "| i32 | 123 | 234 | 111 |"
    "+-----+-----+-----+-----+"
);

test_table!(
    test_right_rotate,
    Matrix::iter([(123, 456, 789), (234, 567, 891), (111, 222, 333)]).with(Rotate::Right),
    "+-----+-----+-----+-----+"
    "| 111 | 234 | 123 | i32 |"
    "+-----+-----+-----+-----+"
    "| 222 | 567 | 456 | i32 |"
    "+-----+-----+-----+-----+"
    "| 333 | 891 | 789 | i32 |"
    "+-----+-----+-----+-----+"
);

test_table!(
    test_bottom_rotate,
    Matrix::iter([(123, 456, 789), (234, 567, 891), (111, 222, 333)]).with(Rotate::Bottom),
    "+-----+-----+-----+"
    "| 111 | 222 | 333 |"
    "+-----+-----+-----+"
    "| 234 | 567 | 891 |"
    "+-----+-----+-----+"
    "| 123 | 456 | 789 |"
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
);

test_table!(
    test_top_rotate,
    Matrix::iter([(123, 456, 789), (234, 567, 891), (111, 222, 333)]).with(Rotate::Top),
    "+-----+-----+-----+"
    "| 111 | 222 | 333 |"
    "+-----+-----+-----+"
    "| 234 | 567 | 891 |"
    "+-----+-----+-----+"
    "| 123 | 456 | 789 |"
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
);

test_table!(
    rotate_preserve_border_styles_test_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891), (111, 222, 333)])
        .with(Highlight::new(Rows::single(0), Border::default().top('*')))
        .with(Rotate::Left),
    "+*****************+-----+"
    "| i32 | 789 | 891 | 333 |"
    "+-----+-----+-----+-----+"
    "| i32 | 456 | 567 | 222 |"
    "+-----+-----+-----+-----+"
    "| i32 | 123 | 234 | 111 |"
    "+-----+-----+-----+-----+"
);

// it's a correct behaviour because
// when we sen bottom border of cell(0, 2) we also set top border of cell(1, 2)
//
// todo: determine if it's correct
test_table!(
    rotate_preserve_border_styles_test_1,
    Matrix::iter([(123, 456, 789), (234, 567, 891), (111, 222, 333)])
        .with(Highlight::new(Cell::new(0, 2), Border::default().bottom('*')))
        .with(Rotate::Left),
        "+-----+-----+-----+-----+"
        "| i32 | 789 | 891 | 333 |"
        "+-----+-----+*****+-----+"
        "| i32 | 456 | 567 | 222 |"
        "+-----+-----+-----+-----+"
        "| i32 | 123 | 234 | 111 |"
        "+-----+-----+-----+-----+"
);

test_table!(
    test_left_rotate_1,
    Matrix::iter([(0, 1, 2, 3, 4, 5), (0, 1, 2, 3, 4, 5)]).with(Rotate::Left),
    "+-----+---+---+"
    "| i32 | 5 | 5 |"
    "+-----+---+---+"
    "| i32 | 4 | 4 |"
    "+-----+---+---+"
    "| i32 | 3 | 3 |"
    "+-----+---+---+"
    "| i32 | 2 | 2 |"
    "+-----+---+---+"
    "| i32 | 1 | 1 |"
    "+-----+---+---+"
    "| i32 | 0 | 0 |"
    "+-----+---+---+"
);

test_table!(
    test_right_rotate_1,
    Matrix::iter([(0, 1, 2, 3, 4, 5), (0, 1, 2, 3, 4, 5)]).with(Rotate::Right),
    "+---+---+-----+"
    "| 0 | 0 | i32 |"
    "+---+---+-----+"
    "| 1 | 1 | i32 |"
    "+---+---+-----+"
    "| 2 | 2 | i32 |"
    "+---+---+-----+"
    "| 3 | 3 | i32 |"
    "+---+---+-----+"
    "| 4 | 4 | i32 |"
    "+---+---+-----+"
    "| 5 | 5 | i32 |"
    "+---+---+-----+"
);

test_table!(
    test_bottom_rotate_1,
    Matrix::iter([(0, 1, 2, 3, 4, 5), (0, 1, 2, 3, 4, 5)]).with(Rotate::Bottom),
    "+-----+-----+-----+-----+-----+-----+"
    "|  0  |  1  |  2  |  3  |  4  |  5  |"
    "+-----+-----+-----+-----+-----+-----+"
    "|  0  |  1  |  2  |  3  |  4  |  5  |"
    "+-----+-----+-----+-----+-----+-----+"
    "| i32 | i32 | i32 | i32 | i32 | i32 |"
    "+-----+-----+-----+-----+-----+-----+"
);
