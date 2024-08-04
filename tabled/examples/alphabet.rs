//! This example demonstrates instantiating a [`Table`] from an [`IntoIterator`] compliant object.
//!
//! * Note how [`Range`] [expression syntax](https://doc.rust-lang.org/reference/expressions/range-expr.html)
//!   is used to idiomatically represent the English alphabet.

fn main() {
    use tabled::{
        settings::{Alignment, Style},
        Table,
    };

    let movies = vec![
        ("The Fall Guy", 2024, 6.9),
        ("Barbie", 2023, 6.8),
        ("The Chase for Carrera", 2023, 7.5),
    ];

    let mut table = Table::new(movies);
    table.with((Alignment::right(), Style::modern()));

    assert_eq!(
        table.to_string(),
        "┌───────────────────────┬──────┬─────┐\n\
         │                  &str │  i32 │ f64 │\n\
         ├───────────────────────┼──────┼─────┤\n\
         │          The Fall Guy │ 2024 │ 6.9 │\n\
         ├───────────────────────┼──────┼─────┤\n\
         │                Barbie │ 2023 │ 6.8 │\n\
         ├───────────────────────┼──────┼─────┤\n\
         │ The Chase for Carrera │ 2023 │ 7.5 │\n\
         └───────────────────────┴──────┴─────┘",
    );
}
