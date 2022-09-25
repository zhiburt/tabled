//! The example can be run by this command
//! `cargo run --example builder`

use tabled::{builder::Builder, object::Rows, Modify, Panel, Style, Width};

fn main() {
    let message = r#"The terms "the ocean" or "the sea" used without specification refer to the interconnected body of salt water covering the majority of the Earth's surface"#;
    let link = r#"https://en.wikipedia.org/wiki/Ocean"#;

    let oceans = ["Atlantic", "Pacific", "Indian", "Southern", "Arctic"];

    let mut builder = Builder::default();
    builder.set_columns(["#", "Ocean"]);
    for (i, ocean) in oceans.iter().enumerate() {
        builder.add_record([i.to_string(), ocean.to_string()]);
    }

    let table = builder
        .build()
        .with(Panel::header(message))
        .with(Panel::header(link))
        .with(Panel::horizontal(2).text("=".repeat(link.len())))
        .with(Modify::new(Rows::single(1)).with(Width::wrap(link.len())))
        .with(Style::markdown())
        .to_string();

    println!("{}", table);
}
