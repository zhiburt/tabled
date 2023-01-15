//! The example can be run by this command
//! `cargo run --example custom_cell_logic`

use std::iter::FromIterator;

use table_to_html::HtmlTable;
use tabled::{object::Rows, papergrid::records::Records, Alignment, ModifyObject, Table, Tabled};

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

    let mut table = Table::from_iter(&data);
    table.with(Rows::first().modify().with(Alignment::center()));

    let mut html_table = HtmlTable::from(table);
    html_table.override_cell_elements(|t, row, col| {
        let text = t.get_records().get_text((row, col));

        format!(
            r#"
<ul>
    <li>Coffee</li>
    <li>Tea</li>
    <li>{text}</li>
</ul>
"#
        )
    });

    println!("{html_table}");
}
