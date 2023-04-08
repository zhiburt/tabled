//! This example demonstrates inserting text into the borders
//! of a [`Table`] with [`BorderText`]; a powerful labeling tool.
//!
//! * [`BorderText`] currently supports:
//!     * Horizontal border placement
//!     * Placement starting column offset
//!     * Text colorization
//!
//! * Note how the flexibility of [`Style`] is utilized
//! to remove horizontal borders from the table entirely,
//! and then granularly reinserts one for a highly customized
//! visualization.
//!
//! * Note how the [`Rows`] utility object is used to idiomatically
//! reference the first and last rows of a [`Table`] without writing
//! the necessary logic by hand.
//!
//! * 🚀 Combining several easy-to-use tools,
//! to create unique data representations is what makes [`tabled`] great!

use tabled::{
    settings::{
        object::Rows,
        style::{BorderText, HorizontalLine, Style},
    },
    Table,
};

fn main() {
    let data = [[5, 6, 7, 8, 9], [10, 11, 12, 13, 14]];

    let table = Table::new(data)
        .with(
            Style::modern()
                .remove_horizontal()
                .horizontals([HorizontalLine::new(1, Style::modern().get_horizontal())]),
        )
        .with(BorderText::new(" Numbers ").horizontal(Rows::first()))
        .with(BorderText::new(" More numbers ").horizontal(1))
        .with(BorderText::new(" end. ").horizontal(Rows::last()))
        .to_string();

    println!("{table}");
}
