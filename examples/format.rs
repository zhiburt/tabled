//! The example can be run by this command
//! `cargo run --example format`
//!
//! The example shows a usage of [`tabled::Format`]/[`tabled::FormatWithIndex`].

use tabled::{
    object::{Columns, Object, Rows},
    Format, ModifyObject, Style, Table, Tabled,
};

#[derive(Tabled)]
struct Commit {
    id: &'static str,
    header: &'static str,
    message: &'static str,
}

fn main() {
    let data = [
        Commit {
            header: "bypass open-source transmitter",
            message: "index neural panel",
            id: "8ae4e8957caeaa467acbce963701e227af00a1c7",
        },
        Commit {
            header: "program online alarm",
            message: "copy bluetooth card",
            id: "48c76de71bd685486d97dc8f4f05aa6fcc0c3f86",
        },
        Commit {
            header: "CSV",
            message: "reboot mobile capacitor",
            id: "6ffc2a2796229fc7bf59471ad907f58b897005d0",
        },
    ];

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(
            Rows::first()
                .modify()
                .with(Format::with_index(|_, (_, column)| column.to_string())),
        )
        .with(
            Columns::first()
                .not(Rows::first())
                .modify()
                .with(|s: &str| format!("{}...", s)),
        );

    println!("{}", table);
}
