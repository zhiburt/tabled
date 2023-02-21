#![cfg(feature = "std")]

mod util;

use tabled::grid::compact::ExactDimension;
use tabled::tables::compact::CompactTable;
use util::{create_matrix, test_table};

test_table!(
    compact_new,
    CompactTable::new(create_matrix::<3, 3>()).to_string(),
    ""
);

test_table!(
    compact_with_dimension,
    CompactTable::with_dimension(create_matrix::<3, 3>(), ExactDimension::default()).columns(3).to_string(),
    "+-----+-----+-----+"
    "| 0-0 | 0-1 | 0-2 |"
    "|-----+-----+-----|"
    "| 1-0 | 1-1 | 1-2 |"
    "|-----+-----+-----|"
    "| 2-0 | 2-1 | 2-2 |"
    "+-----+-----+-----+"
);

test_table!(
    compact_width,
    CompactTable::new(create_matrix::<3, 3>()).columns(3).width(5).to_string(),
    "+-----+-----+-----+"
    "| 0-0 | 0-1 | 0-2 |"
    "|-----+-----+-----|"
    "| 1-0 | 1-1 | 1-2 |"
    "|-----+-----+-----|"
    "| 2-0 | 2-1 | 2-2 |"
    "+-----+-----+-----+"
);

test_table!(
    compact_width_pad_not_included,
    CompactTable::new(create_matrix::<3, 3>()).columns(3).width(3).to_string(),
    "+---+---+---+"
    "| 0-0 | 0-1 | 0-2 |"
    "|---+---+---|"
    "| 1-0 | 1-1 | 1-2 |"
    "|---+---+---|"
    "| 2-0 | 2-1 | 2-2 |"
    "+---+---+---+"
);

test_table!(
    compact_width_bigger,
    CompactTable::new(create_matrix::<3, 3>()).columns(3).width(10).to_string(),
    "+----------+----------+----------+"
    "| 0-0      | 0-1      | 0-2      |"
    "|----------+----------+----------|"
    "| 1-0      | 1-1      | 1-2      |"
    "|----------+----------+----------|"
    "| 2-0      | 2-1      | 2-2      |"
    "+----------+----------+----------+"
);

test_table!(
    compact_columns,
    CompactTable::new(create_matrix::<3, 3>()).columns(3).to_string(),
    "+--+--+--+"
    "| 0-0 | 0-1 | 0-2 |"
    "|--+--+--|"
    "| 1-0 | 1-1 | 1-2 |"
    "|--+--+--|"
    "| 2-0 | 2-1 | 2-2 |"
    "+--+--+--+"
);

test_table!(
    compact_cols_zero,
    CompactTable::new(create_matrix::<3, 3>())
        .columns(0)
        .to_string(),
    ""
);

test_table!(
    compact_cols_less,
    CompactTable::new(create_matrix::<3, 3>())
        .columns(1)
        .to_string(),
    "+--+"
    "| 0-0 |"
    "|--|"
    "| 1-0 |"
    "|--|"
    "| 2-0 |"
    "+--+"
);

test_table!(
    compact_cols_more,
    CompactTable::new(create_matrix::<3, 3>())
        .columns(5)
        .to_string(),
    "+--+--+--+--+--+"
    "| 0-0 | 0-1 | 0-2 |"
    "|--+--+--+--+--|"
    "| 1-0 | 1-1 | 1-2 |"
    "|--+--+--+--+--|"
    "| 2-0 | 2-1 | 2-2 |"
    "+--+--+--+--+--+"
);
