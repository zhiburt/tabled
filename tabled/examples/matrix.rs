//! The example can be run by this command
//! `cargo run --example print_matrix`

use tabled::{settings::style::Style, Table};

fn main() {
    let data = matrix::<10>();
    let table = Table::new(&data).with(Style::modern()).to_string();

    println!("{}", table);
}

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
