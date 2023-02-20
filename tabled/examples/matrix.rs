//! The example can be run by this command
//! `cargo run --example print_matrix`

use std::iter::FromIterator;

use tabled::{settings::style::Style, Table};

fn matrix<const N: usize>() -> Vec<Vec<String>> {
    let mut matrix = vec![vec![String::new(); N]; N];

    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        for j in 0..N {
            let n = (i + 1) * (j + 1);
            let s = str::repeat("abc\nden", n);
            matrix[i][j] = s;
        }
    }

    matrix
}

fn main() {
    let data = matrix::<100>();
    let _table = Table::from_iter(data).with(Style::modern()).to_string();

    // println!("{table}");
}
