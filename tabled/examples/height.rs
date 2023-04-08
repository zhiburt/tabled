//! This example demonstrates using the [`Height`] [`TableOption`] for adjusting
//! the height of a [`Table`].
//!
//! * [`Height`] supports three key features:
//!     * [`CellHeightIncrease`] spreads new whitespace between the [`Table`]
//! rows up to the specified line count.
//!     * [`CellHeightLimit`] removes lines from the [`Table`] rows fairly, until
//! it has no choice but to remove single-line-rows entirely, bottom up.
//!     * [`HeightList`] accepts an array of height specifications that are applied
//! to the rows with the same index. This is helpful for granularly specifying individual
//! row heights irrespective of [`Padding`] or [`Margin`].

use tabled::{
    settings::{peaker::PriorityMax, Height, Style},
    Table,
};

fn main() {
    let data = vec![("Multi\nline\nstring", 123), ("Single line", 234)];

    let mut table = Table::builder(data).build();
    table.with(Style::markdown());

    println!("Table\n");
    println!("{table}");
    println!();

    let table_ = table.clone().with(Height::increase(10)).to_string();

    println!("Table increase height to 10\n");
    println!("{table_}");
    println!();

    let table_ = table
        .clone()
        .with(Height::limit(4).priority::<PriorityMax>())
        .to_string();

    println!("Table decrease height to 4\n");
    println!("{table_}");

    let table_ = table
        .clone()
        .with(Height::limit(0).priority::<PriorityMax>())
        .to_string();

    println!("Table decrease height to 0\n");
    println!("{table_}");
}
