# `table_to_html`

Provides a interface to convert a `tabled::Table` into a HTML table (`<table>`).

Because of the specifics of HTML it's not considered to be the best approach to supply custom CSS for the table.
Instead of that you can set a custom id for the table and use your on CSS.

# Example

```rust
use std::iter::FromIterator;

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

    let mut table = Table::from_iter(&data);
    table.with(Rows::first().modify().with(Alignment::center()));

    let html_table = HtmlTable::from(table).to_string();

    let expected = r#"<table id="tabled-table" border="1">
    <tr id="tabled-table-0">
        <td id="tabled-table-0-0" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;" style="text-align: center;">
            <p> name </p>
        </td>
        <td id="tabled-table-0-1" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;" style="text-align: center;">
            <p> based_on </p>
        </td>
        <td id="tabled-table-0-2" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;" style="text-align: center;">
            <p> is_active </p>
        </td>
    </tr>
    <tr id="tabled-table-1">
        <td id="tabled-table-1-0" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;">
            <p> Debian </p>
        </td>
        <td id="tabled-table-1-1" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;">
        </td>
        <td id="tabled-table-1-2" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;">
            <p> true </p>
        </td>
    </tr>
    <tr id="tabled-table-2">
        <td id="tabled-table-2-0" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;">
            <p> Arch </p>
        </td>
        <td id="tabled-table-2-1" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;">
        </td>
        <td id="tabled-table-2-2" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;">
            <p> true </p>
        </td>
    </tr>
    <tr id="tabled-table-3">
        <td id="tabled-table-3-0" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;">
            <p> Manjaro </p>
        </td>
        <td id="tabled-table-3-1" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;">
            <p> Arch </p>
        </td>
        <td id="tabled-table-3-2" style="padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;">
            <p> true </p>
        </td>
    </tr>
</table>"#;

    assert_eq!(html_table, expected)
}
```