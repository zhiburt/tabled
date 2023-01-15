//! The example can be run by this command
//! `cargo run --example attributes`

use table_to_html::HtmlTable;
use tabled::{object::Rows, Alignment, ModifyObject, Table, Tabled};

#[derive(Debug, Tabled)]
struct Distribution {
    name: &'static str,
    based_on: &'static str,
    is_active: bool,
}

impl Distribution {
    fn new(name: &'static str, base: &'static str, is_active: bool) -> Self {
        Self {
            based_on: base,
            is_active,
            name,
        }
    }
}

fn main() {
    let data = [
        Distribution::new("Debian", "", true),
        Distribution::new("Arch", "", true),
        Distribution::new("Manjaro", "Arch", true),
    ];

    let mut table = Table::new(&data);
    table.with(Rows::first().modify().with(Alignment::center()));

    let mut html_table = HtmlTable::from(table);
    html_table.set_id("example.table");
    html_table.add_tr_attr("style", "background-color: #D6EEEE;");
    html_table.add_td_attr("style", "background-color: rgba(150, 212, 212, 0.4);");

    println!("{html_table}");
}
