//! This example demonstrates how [`tabled`] is an excellent tool for creating
//! dataset visualizations.
//!
//! * 🚀 When native display solutions, such as the [`Debug`] trait and [pretty printing](https://doc.rust-lang.org/std/fmt/#sign0)
//! options, aren't enough, [`tabled`] is a great choice for improving the quality of your displays.

use tabled::{settings::Style, Table};

fn matrix<const N: usize>() -> [[usize; N]; N] {
    let mut matrix = [[0; N]; N];

    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        for j in 0..N {
            matrix[i][j] = (i + 1) * (j + 1);
        }
    }

    matrix
}

fn main() {
    let data = matrix::<10>();
    let table = Table::new(data).with(Style::modern()).to_string();

    println!("{table}");
}
