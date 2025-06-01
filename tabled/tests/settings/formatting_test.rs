#![cfg(feature = "std")]
#![cfg(feature = "assert")]

use tabled::{
    assert::test_table,
    settings::{formatting::Justification, object::Columns, Color, Modify},
    Table,
};

use crate::util::Matrix;

test_table!(
    justification,
    Matrix::new(3, 3).with(Justification::new('#')),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 | ##0-0### | ##0-1### | ##0-2### |"
    "+---+----------+----------+----------+"
    "| 1 | ##1-0### | ##1-1### | ##1-2### |"
    "+---+----------+----------+----------+"
    "| 2 | ##2-0### | ##2-1### | ##2-2### |"
    "+---+----------+----------+----------+"
);

test_table!(
    justification_color,
    Matrix::new(3, 3).with(Justification::new('#').color(Color::BG_RED)),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 | \u{1b}[41m##\u{1b}[49m0-0\u{1b}[41m###\u{1b}[49m | \u{1b}[41m##\u{1b}[49m0-1\u{1b}[41m###\u{1b}[49m | \u{1b}[41m##\u{1b}[49m0-2\u{1b}[41m###\u{1b}[49m |"
    "+---+----------+----------+----------+"
    "| 1 | \u{1b}[41m##\u{1b}[49m1-0\u{1b}[41m###\u{1b}[49m | \u{1b}[41m##\u{1b}[49m1-1\u{1b}[41m###\u{1b}[49m | \u{1b}[41m##\u{1b}[49m1-2\u{1b}[41m###\u{1b}[49m |"
    "+---+----------+----------+----------+"
    "| 2 | \u{1b}[41m##\u{1b}[49m2-0\u{1b}[41m###\u{1b}[49m | \u{1b}[41m##\u{1b}[49m2-1\u{1b}[41m###\u{1b}[49m | \u{1b}[41m##\u{1b}[49m2-2\u{1b}[41m###\u{1b}[49m |"
    "+---+----------+----------+----------+"
);

test_table!(
    justification_columns,
    Matrix::new(3, 3)
        .with(Modify::new(Columns::one(1)).with(Justification::new('#')))
        .with(Modify::new(Columns::one(2)).with(Justification::new('@')))
        .with(Modify::new(Columns::one(3)).with(Justification::new('$'))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 | ##0-0### | @@0-1@@@ | $$0-2$$$ |"
    "+---+----------+----------+----------+"
    "| 1 | ##1-0### | @@1-1@@@ | $$1-2$$$ |"
    "+---+----------+----------+----------+"
    "| 2 | ##2-0### | @@2-1@@@ | $$2-2$$$ |"
    "+---+----------+----------+----------+"
);

test_table!(
    justification_color_columns,
    Matrix::new(3, 3)
        .with(Modify::new(Columns::one(1)).with(Justification::new('#').color(Color::BG_BLUE)))
        .with(Modify::new(Columns::one(2)).with(Justification::new('@').color(Color::BG_RED)))
        .with(Modify::new(Columns::one(3)).with(Justification::new('$').color(Color::BG_WHITE))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 | \u{1b}[44m##\u{1b}[49m0-0\u{1b}[44m###\u{1b}[49m | \u{1b}[41m@@\u{1b}[49m0-1\u{1b}[41m@@@\u{1b}[49m | \u{1b}[47m$$\u{1b}[49m0-2\u{1b}[47m$$$\u{1b}[49m |"
    "+---+----------+----------+----------+"
    "| 1 | \u{1b}[44m##\u{1b}[49m1-0\u{1b}[44m###\u{1b}[49m | \u{1b}[41m@@\u{1b}[49m1-1\u{1b}[41m@@@\u{1b}[49m | \u{1b}[47m$$\u{1b}[49m1-2\u{1b}[47m$$$\u{1b}[49m |"
    "+---+----------+----------+----------+"
    "| 2 | \u{1b}[44m##\u{1b}[49m2-0\u{1b}[44m###\u{1b}[49m | \u{1b}[41m@@\u{1b}[49m2-1\u{1b}[41m@@@\u{1b}[49m | \u{1b}[47m$$\u{1b}[49m2-2\u{1b}[47m$$$\u{1b}[49m |"
    "+---+----------+----------+----------+"
);

test_table!(
    justification_multiline,
    Table::new(["Hello\nW\nor\nld", "yes", "no"])
        .with(Justification::new('*')),
    "+-------+"
    "| &str* |"
    "+-------+"
    "| Hello |"
    "| W**** |"
    "| or*** |"
    "| ld*** |"
    "+-------+"
    "| yes** |"
    "+-------+"
    "| no*** |"
    "+-------+"
);
