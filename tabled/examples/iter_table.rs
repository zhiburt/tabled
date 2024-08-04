//! This example demonstrates using [`IterTable`], an [allocation](https://doc.rust-lang.org/nomicon/vec/vec-alloc.html)
//! free [`Table`] alternative that translates an iterator into a display.
//!
//! * Note how [`IterTable`] supports the familiar `.with()` syntax for applying display
//!   modifications.
//!
//! * [`IterTable`] supports manual configuration of:
//!     * Record sniffing (default 1000 rows)
//!     * Row cutoff
//!     * Row height
//!     * Column cutoff
//!     * Column width

use std::{
    fs::File,
    io::{stdout, BufRead, BufReader},
};

use tabled::{settings::Style, tables::IterTable};

fn main() {
    let path = file!();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let iterator = reader.lines().enumerate().map(|(i, line)| match line {
        Ok(line) => [i.to_string(), "ok".into(), line],
        Err(err) => [i.to_string(), "error".into(), err.to_string()],
    });

    let table = IterTable::new(iterator).with(Style::ascii_rounded());

    table.build(stdout()).unwrap();

    println!()
}
