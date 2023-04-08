//! This example demonstrates using [`IterTable`], an [allocation](https://doc.rust-lang.org/nomicon/vec/vec-alloc.html)
//! free [`Table`] alternative that translates an iterator into a display.
//!
//! * Note how [`IterTable`] supports the familiar `.with()` syntax for applying display
//! modifications.
//!
//! * [`IterTable`] supports manual configuration of:
//!     * Record sniffing (default 1000 rows)
//!     * Row cutoff
//!     * Row height
//!     * Column cutoff
//!     * Column width

use std::io::BufRead;

use tabled::{settings::Style, tables::IterTable};

fn main() {
    let path = file!();
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let iterator = reader.lines().enumerate().map(|(i, line)| match line {
        Ok(line) => [i.to_string(), String::from("ok"), line],
        Err(err) => [i.to_string(), String::from("error"), err.to_string()],
    });

    let table = IterTable::new(iterator).with(Style::ascii_rounded());

    table.build(std::io::stdout()).unwrap();
    println!()
}
