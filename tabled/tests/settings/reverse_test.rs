#![cfg(feature = "std")]
#![cfg(feature = "assert")]

use tabled::{assert::test_table, grid::config::Offset, settings::Reverse};

use crate::matrix::Matrix;

test_table!(
    test_0x0_reverse_rows,
    Matrix::empty().with(Reverse::rows(0)),
    ""
);

test_table!(
    test_0x0_reverse_columns,
    Matrix::empty().with(Reverse::columns(0)),
    ""
);

test_table!(
    test_3x3_reverse_rows,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Reverse::rows(0)),
    "+-----+-----+-----+"
    "| 234 | 567 | 891 |"
    "+-----+-----+-----+"
    "| 123 | 456 | 789 |"
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
);

test_table!(
    test_3x3_reverse_rows_skip_start,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Reverse::rows(1)),
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
    "| 234 | 567 | 891 |"
    "+-----+-----+-----+"
    "| 123 | 456 | 789 |"
    "+-----+-----+-----+"
);

test_table!(
    test_3x3_reverse_rows_skip_end,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Reverse::rows(0).limit(Offset::End(1))),
    "+-----+-----+-----+"
    "| 123 | 456 | 789 |"
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
    "| 234 | 567 | 891 |"
    "+-----+-----+-----+"
);

test_table!(
    test_3x3_reverse_rows_skip_end_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891), (345, 678, 912)])
        .with(Reverse::rows(1).limit(Offset::Begin(2))),
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
    "| 234 | 567 | 891 |"
    "+-----+-----+-----+"
    "| 123 | 456 | 789 |"
    "+-----+-----+-----+"
    "| 345 | 678 | 912 |"
    "+-----+-----+-----+"
);

test_table!(
    test_4x4_reverse_rows_skip_start_and_end,
    Matrix::iter([(123, 456, 789), (234, 567, 891), (345, 678, 901)]).with(Reverse::rows(1).limit(Offset::End(1))),
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
    "| 234 | 567 | 891 |"
    "+-----+-----+-----+"
    "| 123 | 456 | 789 |"
    "+-----+-----+-----+"
    "| 345 | 678 | 901 |"
    "+-----+-----+-----+"
);

test_table!(
    test_3x3_reverse_columns,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Reverse::columns(0)),
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
    "| 789 | 456 | 123 |"
    "+-----+-----+-----+"
    "| 891 | 567 | 234 |"
    "+-----+-----+-----+"
);

test_table!(
    test_3x3_reverse_columns_skip_start,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Reverse::columns(1)),
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
    "| 123 | 789 | 456 |"
    "+-----+-----+-----+"
    "| 234 | 891 | 567 |"
    "+-----+-----+-----+"
);

test_table!(
    test_3x3_reverse_columns_skip_end,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Reverse::columns(0).limit(Offset::End(1))),
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
    "| 456 | 123 | 789 |"
    "+-----+-----+-----+"
    "| 567 | 234 | 891 |"
    "+-----+-----+-----+"
);

test_table!(
    test_3x3_reverse_columns_skip_end_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891), (345, 678, 912)])
        .with(Reverse::columns(0).limit(Offset::Begin(2))),
    "+-----+-----+-----+"
    "| i32 | i32 | i32 |"
    "+-----+-----+-----+"
    "| 456 | 123 | 789 |"
    "+-----+-----+-----+"
    "| 567 | 234 | 891 |"
    "+-----+-----+-----+"
    "| 678 | 345 | 912 |"
    "+-----+-----+-----+"
);

test_table!(
    test_4x3_reverse_columns_skip_start_and_end,
    Matrix::iter([(123, 456, 789, 123), (234, 567, 891, 234)]).with(Reverse::columns(1).limit(Offset::End(1))),
    "+-----+-----+-----+-----+"
    "| i32 | i32 | i32 | i32 |"
    "+-----+-----+-----+-----+"
    "| 123 | 789 | 456 | 123 |"
    "+-----+-----+-----+-----+"
    "| 234 | 891 | 567 | 234 |"
    "+-----+-----+-----+-----+"
);
