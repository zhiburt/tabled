#![cfg(feature = "std")]

use tabled::{
    grid::{
        config::CompactConfig, dimension::CompactGridDimension, dimension::Estimate,
        records::IterRecords,
    },
    tables::CompactTable,
};
use testing::{Matrix, test_table};

test_table!(
    compact_new,
    CompactTable::new(Matrix::new(3, 3).to_vec()).to_string(),
    ""
);

test_table!(
    compact_with_dimension,
    {
        let data = Matrix::new(3, 3);
        let mut dims = CompactGridDimension::default();
        dims.estimate(IterRecords::new(&data, 3, None), &CompactConfig::default());
        CompactTable::with_dimension(data, dims).columns(3).to_string()
    },
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
    CompactTable::new(Matrix::new(3, 3)).columns(3).width(5).to_string(),
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
    CompactTable::new(Matrix::new(3, 3)).columns(3).width(3).to_string(),
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
    CompactTable::new(Matrix::new(3, 3)).columns(3).width(10).to_string(),
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
    CompactTable::new(Matrix::new(3, 3)).columns(3).to_string(),
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
    CompactTable::new(Matrix::new(3, 3))
        .columns(0)
        .to_string(),
    ""
);

test_table!(
    compact_cols_less,
    CompactTable::new(Matrix::new(3, 3))
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
    CompactTable::new(Matrix::new(3, 3))
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
