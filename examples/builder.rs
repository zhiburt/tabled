//! The example can be run by this command
//! `cargo run --example builder`

use tabled::{Header, MaxWidth, Modify, Panel, Row, Style};

fn main() {
    let message = r#"The terms "the ocean" or "the sea" used without specification refer to the interconnected body of salt water covering the majority of the Earth's surface"#;
    let link = r#"https://en.wikipedia.org/wiki/Ocean"#;

    let oceans = ["Atlantic", "Pacific", "Indian", "Southern", "Arctic"];

    let mut builder = tabled::builder::Builder::default().set_header(["#", "Ocean"]);
    for (i, ocean) in oceans.iter().enumerate() {
        builder = builder.add_row([i.to_string(), ocean.to_string()]);
    }

    let table = builder
        .build()
        .with(Header(message))
        .with(Header(link))
        .with(Panel("=".repeat(link.len()), 2))
        .with(Modify::new(Row(1..2)).with(MaxWidth::wrapping(link.len())))
        .with(Style::GITHUB_MARKDOWN);

    println!("{}", table);
}
