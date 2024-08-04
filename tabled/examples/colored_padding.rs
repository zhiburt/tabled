//! This example demonstrates using the [`Padding::colorize()`] function in several ways
//! to give a [`Table`] display a vibrant aesthetic.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note how the [`Color`] [setting](tabled::settings) is used to simplify creating
//!   reusable themes for text, backgrounds, padded whitespace, and borders.
//!
//! * Note how a unique color can be set for each direction.

use owo_colors::OwoColorize;

use tabled::{
    settings::{
        object::{Columns, Object, Rows, Segment},
        style::BorderColor,
        Alignment, Color, Format, Margin, MarginColor, Modify, Padding, PaddingColor, Style,
    },
    Table, Tabled,
};

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct Fundamental {
    quantity: String,
    value: String,
    unit: String,
    symbol: char,
}

impl Fundamental {
    fn new(quantity: &str, symbol: char, value: &str, unit: &str) -> Self {
        Self {
            symbol,
            quantity: quantity.to_string(),
            value: value.to_string(),
            unit: unit.to_string(),
        }
    }
}

fn main() {
    // data source: https://www.britannica.com/science/physical-constant
    #[rustfmt::skip]
    let data = [
        Fundamental::new("constant of gravitation", 'G', "6.67384 × 10⁻¹¹", "cubic metre per second squared per kilogram"),
        Fundamental::new("speed of light (in a vacuum)", 'c', "2.99792458 × 10⁻⁸", "metres per second"),
        Fundamental::new("Planck's constant", 'h', "6.626070040 × 10⁻³⁴", "joule second"),
        Fundamental::new("Boltzmann constant", 'k', "1.38064852 × 10⁻²³", "joule per kelvin"),
        Fundamental::new("Faraday constant",    'F',    "9.648533289 × 10⁴",    "coulombs per mole"),
    ];

    let pane_color = Color::parse(' '.bg_rgb::<220, 220, 220>().to_string());
    let border_color = Color::parse(' '.bg_rgb::<200, 200, 220>().bold().to_string());
    let data_color = Color::parse(' '.bg_rgb::<200, 200, 220>().to_string());

    let header_settings = Modify::new(Rows::first())
        .with(Padding::new(1, 1, 2, 2))
        .with(PaddingColor::new(
            Color::BG_GREEN,
            Color::BG_YELLOW,
            Color::BG_MAGENTA,
            Color::BG_CYAN,
        ))
        .with(Alignment::center())
        .with(Padding::expand(true))
        .with(Format::content(|s| s.on_black().white().to_string()));

    let data_settings = Modify::new(Rows::first().inverse())
        .with(Alignment::center())
        .with(Padding::expand(true))
        .with(PaddingColor::filled(data_color.clone()));

    let symbol_settings = Modify::new(Columns::single(1).not(Rows::first()))
        .with(Format::content(|s| s.bold().to_string()));

    let unit_settings = Modify::new(Columns::single(3).not(Rows::first()))
        .with(Format::content(|s| s.italic().to_string()));

    let table = Table::new(data)
        .with(Style::rounded())
        .with(Margin::new(1, 2, 1, 1))
        .with(MarginColor::filled(pane_color))
        .with(BorderColor::filled(border_color))
        .with(Modify::new(Segment::all()).with(data_color))
        .with(header_settings)
        .with(data_settings)
        .with(symbol_settings)
        .with(unit_settings)
        .to_string();

    println!("\n\n{table}\n\n");
}
