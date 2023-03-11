#![cfg(feature = "std")]

use crate::util::{new_table, test_table};
use tabled::settings::{Shadow, Style};

#[cfg(feature = "color")]
use owo_colors::OwoColorize;
#[cfg(feature = "color")]
use std::convert::TryFrom;
#[cfg(feature = "color")]
use tabled::settings::Color;

mod util;

test_table!(
    test_shadow_bottom_right_0,
    new_table([(123, 456, 789), (234, 567, 891)]).with(Style::psql()).with(Shadow::new(1)),
    " i32 | i32 | i32  "
    "-----+-----+-----▒"
    " 123 | 456 | 789 ▒"
    " 234 | 567 | 891 ▒"
    " ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
);

test_table!(
    test_shadow_bottom_left_0,
    new_table([(123, 456, 789), (234, 567, 891)]).with(Style::psql()).with(Shadow::new(1).set_left()),
    "  i32 | i32 | i32 "
    "▒-----+-----+-----"
    "▒ 123 | 456 | 789 "
    "▒ 234 | 567 | 891 "
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ "
);

test_table!(
    test_shadow_top_right_0,
    new_table([(123, 456, 789), (234, 567, 891)]).with(Style::psql()).with(Shadow::new(1).set_top()),
   " ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"
   " i32 | i32 | i32 ▒"
   "-----+-----+-----▒"
   " 123 | 456 | 789 ▒"
   " 234 | 567 | 891  "
);

test_table!(
    test_shadow_top_left_0,
    new_table([(123, 456, 789), (234, 567, 891)]).with(Style::psql()).with(Shadow::new(1).set_top().set_left()),
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ "
    "▒ i32 | i32 | i32 "
    "▒-----+-----+-----"
    "▒ 123 | 456 | 789 "
    "  234 | 567 | 891 "
);

test_table!(
    test_shadow_set_fill,
    new_table([(123, 456, 789), (234, 567, 891)]).with(Shadow::new(1).set_fill('▓')),
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
    new_table([(123, 456, 789), (234, 567, 891)]).with(Shadow::new(2).set_fill('▓')),
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
    new_table([(123, 456, 789), (234, 567, 891)]).with(Shadow::new(2).set_offset(3)),
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

#[cfg(feature = "color")]
test_table!(
    test_shadow_set_color_0,
    new_table([(123, 456, 789), (234, 567, 891)]).with(Shadow::new(2).set_offset(3).set_color(Color::try_from(' '.red().to_string()).unwrap())),
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
