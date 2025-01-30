#![cfg(feature = "std")]

use tabled::settings::{Shadow, Style};

use crate::matrix::Matrix;
use testing_table::test_table;

#[cfg(feature = "ansi")]
use tabled::settings::Color;

test_table!(
    test_shadow_bottom_right_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Style::psql()).with(Shadow::new(1)),
    " i32 | i32 | i32  "
    "-----+-----+-----▒"
    " 123 | 456 | 789 ▒"
    " 234 | 567 | 891 ▒"
    " ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
);

test_table!(
    test_shadow_bottom_left_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Style::psql()).with(Shadow::new(1).set_left()),
    "  i32 | i32 | i32 "
    "▒-----+-----+-----"
    "▒ 123 | 456 | 789 "
    "▒ 234 | 567 | 891 "
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ "
);

test_table!(
    test_shadow_top_right_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Style::psql()).with(Shadow::new(1).set_top()),
   " ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
   " i32 | i32 | i32 ▒"
   "-----+-----+-----▒"
   " 123 | 456 | 789 ▒"
   " 234 | 567 | 891  "
);

test_table!(
    test_shadow_top_left_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Style::psql()).with(Shadow::new(1).set_top().set_left()),
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ "
    "▒ i32 | i32 | i32 "
    "▒-----+-----+-----"
    "▒ 123 | 456 | 789 "
    "  234 | 567 | 891 "
);

test_table!(
    test_shadow_set_fill,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Shadow::new(1).set_fill('▓')),
    "+-----+-----+-----+ "
    "| i32 | i32 | i32 |▓"
    "+-----+-----+-----+▓"
    "| 123 | 456 | 789 |▓"
    "+-----+-----+-----+▓"
    "| 234 | 567 | 891 |▓"
    "+-----+-----+-----+▓"
    " ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓"
);

test_table!(
    test_shadow_size_1,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Shadow::new(2).set_fill('▓')),
    "+-----+-----+-----+  "
    "| i32 | i32 | i32 |▓▓"
    "+-----+-----+-----+▓▓"
    "| 123 | 456 | 789 |▓▓"
    "+-----+-----+-----+▓▓"
    "| 234 | 567 | 891 |▓▓"
    "+-----+-----+-----+▓▓"
    " ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓"
    " ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓"
);

test_table!(
    test_shadow_set_offset_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891)]).with(Shadow::new(2).set_offset(3)),
    "+-----+-----+-----+  "
    "| i32 | i32 | i32 |  "
    "+-----+-----+-----+  "
    "| 123 | 456 | 789 |▒▒"
    "+-----+-----+-----+▒▒"
    "| 234 | 567 | 891 |▒▒"
    "+-----+-----+-----+▒▒"
    "   ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
    "   ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
);

#[cfg(feature = "ansi")]
test_table!(
    test_shadow_set_color_0,
    Matrix::iter([(123, 456, 789), (234, 567, 891)])
        .with(Shadow::new(2).set_offset(3).set_color(Color::FG_RED)),
    "+-----+-----+-----+  "
    "| i32 | i32 | i32 |  "
    "+-----+-----+-----+  "
    "| 123 | 456 | 789 |\u{1b}[31m▒▒\u{1b}[39m"
    "+-----+-----+-----+\u{1b}[31m▒▒\u{1b}[39m"
    "| 234 | 567 | 891 |\u{1b}[31m▒▒\u{1b}[39m"
    "+-----+-----+-----+\u{1b}[31m▒▒\u{1b}[39m"
    "   \u{1b}[31m▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒\u{1b}[39m"
    "   \u{1b}[31m▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒\u{1b}[39m"
);
