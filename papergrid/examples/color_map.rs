//! This example demonstrates using a [`HashMap`] of colors to simplify styling
//! sections of a [`Grid`] without embedding ANSI escape characters into cell values.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Check out [`owo_colors`] for additional styling options available through their API.

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

use owo_colors::{
    colors::{Black, Blue, Red},
    Style as OStyle,
};

use papergrid::{
    ansi::ANSIFmt,
    config::spanned::SpannedConfig,
    config::{Borders, Position},
    dimension::spanned::SpannedGridDimension,
    dimension::Estimate,
    grid::iterable::Grid,
    records::IterRecords,
};

fn main() {
    let records = vec![vec!["Hello", "World"], vec!["Hi", "World"]];
    let records = IterRecords::new(&records, 2, None);

    let cfg = generate_table_config();

    let mut dimension = SpannedGridDimension::default();
    dimension.estimate(records, &cfg);

    let colors = generate_colors();

    let grid = Grid::new(records, &dimension, &cfg, &colors);

    grid.build(UTF8Stdout(io::stdout())).unwrap();
    println!();
}

fn generate_colors() -> HashMap<Position, Style> {
    HashMap::from([
        ((0, 0), Style(OStyle::default().bg::<Red>().fg::<Black>())),
        ((1, 1), Style(OStyle::default().bg::<Blue>())),
    ])
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

#[derive(Debug, Clone, Default)]
struct Style(OStyle);

impl ANSIFmt for Style {
    fn fmt_ansi_prefix<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        struct Prefix<'a>(&'a OStyle);

        impl Display for Prefix<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.0.fmt_prefix(f)
            }
        }

        f.write_fmt(format_args!("{}", Prefix(&self.0)))
    }
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
