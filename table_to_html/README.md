# `table_to_html`

Provides a interface to convert a `tabled::Table` into a HTML table (`<table>`).

Because of the specifics of HTML it's not considered to be the best approach to supply custom CSS for the table.
Istead of that you can set a custom id for the table and use your on CSS.

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

let data = [
    Distribution::new("Debian", "", true),
    Distribution::new("Arch", "", true),
    Distribution::new("Manjaro", "Arch", true),
];

let mut table = Table::from_iter(&data);
table.with(Rows::first().modify().with(Alignment::center()));

let html_table = HtmlTable::from(table);

assert_eq!(
    html_table.to_string(),
    concat!(
        "<table id=\"tabled-table\" border=\"1\">\n",
        "    <tr id=\"tabled-table-0\">\n",
        "        <td id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" style=\"text-align: center;\">\n",
        "            <p> name </p>\n",
        "        </td>\n",
        "        <td id=\"tabled-table-0-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" style=\"text-align: center;\">\n",
        "            <p> based_on </p>\n",
        "        </td>\n",
        "        <td id=\"tabled-table-0-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" style=\"text-align: center;\">\n",
        "            <p> is_active </p>\n",
        "        </td>\n",
        "    </tr>\n",
        "    <tr id=\"tabled-table-1\">\n",
        "        <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
        "            <p> Debian </p>\n",
        "        </td>\n",
        "        <td id=\"tabled-table-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
        "        </td>\n",
        "        <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
        "            <p> true </p>\n",
        "        </td>\n",
        "    </tr>\n",
        "    <tr id=\"tabled-table-2\">\n",
        "        <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
        "            <p> Arch </p>\n",
        "        </td>\n",
        "        <td id=\"tabled-table-2-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
        "        </td>\n",
        "        <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
        "            <p> true </p>\n",
        "        </td>\n",
        "    </tr>\n",
        "    <tr id=\"tabled-table-3\">\n",
        "        <td id=\"tabled-table-3-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
        "            <p> Manjaro </p>\n",
        "        </td>\n",
        "        <td id=\"tabled-table-3-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
        "            <p> Arch </p>\n",
        "        </td>\n",
        "        <td id=\"tabled-table-3-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
        "            <p> true </p>\n",
        "        </td>\n",
        "    </tr>\n",
        "</table>"
    ),
)
```