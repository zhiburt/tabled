//! The example can be run by this command.
//!
//! `cargo run --features color --example padding_color`
//!
//! This example requires a `color` feature.

use std::convert::TryFrom;

use owo_colors::OwoColorize;

use tabled::{
    margin::MarginColor,
    object::{Columns, Object, Rows, Segment},
    padding::PaddingColor,
    style::{Color, Style},
    Alignment, CellOption, Format, Margin, ModifyObject, Padding, Table, Tabled,
};

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct Fundamental {
    quantity: &'static str,
    symbol: &'static str,
    value: &'static str,
    unit: &'static str,
}

impl Fundamental {
    fn new(
        quantity: &'static str,
        symbol: &'static str,
        value: &'static str,
        unit: &'static str,
    ) -> Self {
        Self {
            quantity,
            symbol,
            value,
            unit,
        }
    }
}

fn main() {
    // data source: https://www.britannica.com/science/physical-constant
    let data = [
        Fundamental::new(
            "constant of gravitation",
            "G",
            "6.67384 × 10⁻¹¹",
            "cubic metre per second squared per kilogram",
        ),
        Fundamental::new(
            "speed of light (in a vacuum)",
            "c",
            "2.99792458 × 10⁻⁸",
            "metres per second",
        ),
        Fundamental::new(
            "Planck's constant",
            "h",
            "6.626070040 × 10⁻³⁴",
            "joule second",
        ),
        Fundamental::new(
            "Boltzmann constant",
            "k",
            "1.38064852 × 10⁻²³",
            "joule per kelvin",
        ),
        Fundamental::new(
            "Faraday constant",
            "F",
            "9.648533289 × 10⁴",
            "coulombs per mole",
        ),
    ];

    let pane_color = Color::try_from(' '.bg_rgb::<220, 220, 220>().to_string()).unwrap();
    let border_color = Color::try_from(' '.bg_rgb::<200, 200, 220>().bold().to_string()).unwrap();
    let data_color = Color::try_from(' '.bg_rgb::<200, 200, 220>().to_string()).unwrap();

    let header_settings = Rows::first()
        .modify()
        .with(Padding::new(1, 1, 2, 2))
        .with(MakeMaxPadding)
        .with(PaddingColor::new(
            Color::try_from(' '.on_green().to_string()).unwrap(),
            Color::try_from(' '.on_yellow().to_string()).unwrap(),
            Color::try_from(' '.on_magenta().to_string()).unwrap(),
            Color::try_from(' '.on_cyan().to_string()).unwrap(),
        ))
        .with(Format::new(|s| s.on_black().white().to_string()));

    let data_settings = Rows::first()
        .inverse()
        .modify()
        .with(Alignment::left())
        .with(MakeMaxPadding)
        .with(PaddingColor::new(
            Color::default(),
            Color::default(),
            data_color.clone(),
            data_color.clone(),
        ));

    let symbol_settings = Columns::single(1)
        .not(Rows::first())
        .modify()
        .with(Format::new(|s| s.bold().to_string()));

    let unit_settings = Columns::single(3)
        .not(Rows::first())
        .modify()
        .with(Format::new(|s| s.italic().to_string()));

    let table = Table::new(&data)
        .with(Style::rounded())
        .with(Margin::new(1, 2, 1, 1))
        .with(MarginColor::new(
            pane_color.clone(),
            pane_color.clone(),
            pane_color.clone(),
            pane_color,
        ))
        .with(border_color)
        .with(Segment::all().modify().with(data_color))
        .with(header_settings)
        .with(data_settings)
        .with(symbol_settings)
        .with(unit_settings);

    println!("\n\n{}\n\n", table);
}

struct MakeMaxPadding;

impl CellOption for MakeMaxPadding {
    fn change_cell(&mut self, grid: &mut papergrid::Grid, entity: papergrid::Entity) {
        let widths = grid.build_widths();
        for (row, col) in entity.iter(grid.count_rows(), grid.count_columns()) {
            let width = grid.get_string_width(row, col);
            let column_width = widths[col];

            if width < column_width {
                let available_width = column_width - width;
                let left = available_width / 2;
                let right = available_width - left;

                let mut padding = *grid.get_padding((row, col).into());
                padding.left.size = left;
                padding.right.size = right;

                grid.set_padding((row, col).into(), padding);
            }
        }
    }
}
