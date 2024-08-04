//! This example demonstrates using colors to stylize [`Grid`] cells.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note that this example uses inline ANSI escape characters to style
//!   grid cells. `Grid::new(_, _, _, NoColors)` indicates that a color
//!   map is not provided. NOT that colors are ignored in the output.

use std::{
    fmt,
    io::{self, Write},
};

use papergrid::{
    colors::NoColors, config::spanned::SpannedConfig, config::Borders,
    dimension::spanned::SpannedGridDimension, dimension::Estimate, grid::iterable::Grid,
    records::IterRecords,
};

fn main() {
    let data = vec![
        vec!["\u{1b}[42mHello\u{1b}[0m", "\u{1b}[43mWorld\u{1b}[0m"],
        vec!["\u{1b}[44mHi\u{1b}[0m", "\u{1b}[45mWorld\u{1b}[0m"],
    ];
    let records = IterRecords::new(data, 2, None);

    let cfg = generate_table_config();

    let mut dimension = SpannedGridDimension::default();
    dimension.estimate(&records, &cfg);

    let grid = Grid::new(&records, &dimension, &cfg, NoColors);

    grid.build(UTF8Stdout(io::stdout())).unwrap();
    println!();
}

fn generate_table_config() -> SpannedConfig {
    let mut cfg = SpannedConfig::default();
    cfg.set_borders(Borders {
        top: Some('-'),
        bottom: Some('-'),
        left: Some('|'),
        right: Some('|'),
        vertical: Some('|'),
        horizontal: Some('-'),
        ..Default::default()
    });
    cfg.set_borders_missing('+');

    cfg
}

struct UTF8Stdout(io::Stdout);

impl fmt::Write for UTF8Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buf = s.as_bytes();
        loop {
            let n = self.0.write(buf).map_err(|_| fmt::Error)?;
            if n == buf.len() {
                break;
            }

            buf = &buf[n..];
        }

        self.0.flush().map_err(|_| fmt::Error)?;

        Ok(())
    }
}
