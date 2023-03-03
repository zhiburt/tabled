#![deny(unused_must_use)]
#![warn(
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    missing_debug_implementations,
    unreachable_pub,
    missing_docs
)]
#![allow(clippy::uninlined_format_args)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]

//! # table_to_html
//!
//! The library provides a interface to build a HTML table (`<table>`).
//!
//! ## Example building a table from iterator
//!
//! ```rust
//! use table_to_html::HtmlTable;
//!
//! let data = vec![
//!     vec!["Debian", "", "0"],
//!     vec!["Arch", "", "0"],
//!     vec!["Manjaro", "Arch", "0"],
//! ];
//!
//! let html_table = HtmlTable::new(data);
//!
//! assert_eq!(
//!     html_table.to_string(),
//!     concat!(
//!         "<table>\n",
//!         "    <tbody>\n",
//!         "        <tr>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Debian\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        \n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        0\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "        <tr>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Arch\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        \n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        0\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "        <tr>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Manjaro\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Arch\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        0\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "    </tbody>\n",
//!         "</table>"
//!     ),
//! )
//! ```
//!
//! ## Example building a table using [`Tabled`].
//!
//! ```rust
//! use table_to_html::{HtmlTable, Alignment, Entity};
//! use tabled::{Table, Tabled};
//!
//! #[derive(Debug, Tabled)]
//! struct Distribution {
//!     name: &'static str,
//!     based_on: &'static str,
//!     is_active: bool,
//! }
//!
//! impl Distribution {
//!     fn new(name: &'static str, base: &'static str, is_active: bool) -> Self {
//!         Self {
//!             based_on: base,
//!             is_active,
//!             name,
//!         }
//!     }
//! }
//!
//! let data = [
//!     Distribution::new("Debian", "", true),
//!     Distribution::new("Arch", "", true),
//!     Distribution::new("Manjaro", "Arch", true),
//! ];
//!
//! let mut html_table = HtmlTable::from(Table::builder(&data));
//! html_table.set_alignment(Entity::Row(1), Alignment::center());
//!
//! assert_eq!(
//!     html_table.to_string(),
//!     concat!(
//!         "<style>\n",
//!         "    tbody > :nth-child(2) > td, thead > :nth-child(2) > th {\n",
//!         "      text-align: center;\n",
//!         "    }\n",
//!         "</style>\n",
//!         "<table>\n",
//!         "    <thead>\n",
//!         "        <tr>\n",
//!         "            <th>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        name\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </th>\n",
//!         "            <th>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        based_on\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </th>\n",
//!         "            <th>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        is_active\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </th>\n",
//!         "        </tr>\n",
//!         "    </thead>\n",
//!         "    <tbody>\n",
//!         "        <tr>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Debian\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        \n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        true\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "        <tr>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Arch\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        \n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        true\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "        <tr>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Manjaro\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Arch\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td>\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        true\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "    </tbody>\n",
//!         "</table>",
//!     ),
//! )
//! ```
//!
//! The default table might look not very represenative.
//! But it's considered that you might improve it by suplying your own CSS.
//!
//! In a mean time there's some regular style options.
//!
//! Also notice that table elements does not have any special `id`, `class` attributes.
//! It's supposed that you might add them if nessary your self, by using [`HtmlTable::visit_mut`]
//!
//! ## Adding custom ids example.
//!
//! ```rust
//! use table_to_html::{HtmlTable, html::{Attribute, HtmlVisitorMut, HtmlElement}};
//!
//! struct CellIdIndex {
//!     i: usize
//! }
//!
//! impl HtmlVisitorMut for CellIdIndex {
//!     fn visit_element_mut(&mut self, e: &mut HtmlElement) -> bool {
//!         if e.tag() == "td" {
//!             let mut attrs = e.attrs().to_vec();
//!             attrs.push(Attribute::new("id", self.i.to_string()));
//!             *e = HtmlElement::new("td", attrs, e.value().cloned());
//!             self.i += 1;
//!         } else if e.tag() == "th" {
//!             let mut attrs = e.attrs().to_vec();
//!             attrs.push(Attribute::new("id", self.i.to_string()));
//!             *e = HtmlElement::new("th", attrs, e.value().cloned());
//!             self.i += 1;
//!         }
//!
//!         true
//!     }
//! }
//!
//! let data = vec![
//!     vec!["Debian", "", "0"],
//!     vec!["Arch", "", "0"],
//!     vec!["Manjaro", "Arch", "0"],
//! ];
//!
//! let mut html_table = HtmlTable::new(data);
//! html_table.visit_mut(CellIdIndex{ i: 0 });
//!
//! assert_eq!(
//!     html_table.to_string(),
//!     concat!(
//!         "<table>\n",
//!         "    <tbody>\n",
//!         "        <tr>\n",
//!         "            <td id=\"0\">\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Debian\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td id=\"1\">\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        \n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td id=\"2\">\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        0\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "        <tr>\n",
//!         "            <td id=\"3\">\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Arch\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td id=\"4\">\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        \n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td id=\"5\">\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        0\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "        <tr>\n",
//!         "            <td id=\"6\">\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Manjaro\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td id=\"7\">\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        Arch\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "            <td id=\"8\">\n",
//!         "                <div>\n",
//!         "                    <p>\n",
//!         "                        0\n",
//!         "                    </p>\n",
//!         "                </div>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "    </tbody>\n",
//!         "</table>"
//!     ),
//! )
//! ```
//!
//! [`Tabled`]: tabled::Tabled
//! [`HtmlTable::visit_mut`]: HtmlTable::visit_mut

pub mod html;

use std::{
    collections::BTreeMap,
    fmt::{Display, Write},
    iter::FromIterator,
};

use html::{HtmlElement, HtmlValue, HtmlVisitor, HtmlVisitorMut};
use tabled::{
    builder::Builder,
    grid::{config::Sides, util::string::get_lines},
};

use crate::html::Attribute;

/// A Html element padding in PX.
pub type Padding = Sides<usize>;
/// A Html element margin in PX.
pub type Margin = Sides<usize>;

pub use tabled::grid::config::Entity;
pub use tabled::grid::config::Position;
pub use tabled::settings::alignment::Alignment;

/// The structure represents an HTML `<table>`.
#[derive(Debug, Clone)]
pub struct HtmlTable {
    table: HtmlElement,
    css: BTreeMap<String, BTreeMap<String, String>>,
}

impl HtmlTable {
    /// Creates a new html table from a given elements.
    pub fn new<I, R, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self::from(Builder::from_iter(
            iter.into_iter()
                .map(|row| row.into_iter().map(|s| s.into())),
        ))
    }

    /// Set a padding for a given target.
    pub fn set_padding(&mut self, target: Entity, pad: Padding) {
        let target = entity_target(target);
        let css = padding_css(pad);

        self.css.insert(target, css);
    }

    /// Set a alignment for a given target.
    pub fn set_alignment(&mut self, target: Entity, val: Alignment) {
        let target = entity_target(target);
        let css = alignment_css(val);

        self.css.insert(target, css);
    }

    /// Set a column span for a given cell.
    pub fn set_column_span(&mut self, pos: Position, size: usize) {
        set_cell_attribute(
            &mut self.table,
            pos,
            Attribute::new("colspan", size.to_string()),
        );
    }

    /// Set a row span for a given cell.
    pub fn set_row_span(&mut self, pos: Position, size: usize) {
        set_cell_attribute(
            &mut self.table,
            pos,
            Attribute::new("rowspan", size.to_string()),
        );
    }

    /// Set a margin for a whole table.
    pub fn set_margin(&mut self, margin: Margin) {
        let mut m = BTreeMap::new();
        m.insert(String::from("margin-top"), margin.top.to_string());
        m.insert(String::from("margin-bottom"), margin.bottom.to_string());
        m.insert(String::from("margin-left"), margin.left.to_string());
        m.insert(String::from("margin-right"), margin.right.to_string());

        self.css.insert(String::from("table"), m);
    }

    /// Set a border for a whole table.
    pub fn set_border(&mut self, size: usize) {
        let mut m = BTreeMap::new();
        m.insert(String::from("border"), format!("{size}px solid"));

        self.css.insert(String::from("table, th, td"), m);
    }

    /// Uses the visitor to traverse a table.
    pub fn visit<V: HtmlVisitor>(&self, visitor: V) {
        self.table.visit(visitor);
    }

    /// Uses the visitor to traverse a table while mutating it.
    pub fn visit_mut<V: HtmlVisitorMut>(&mut self, visitor: V) {
        self.table.visit_mut(visitor);
    }
}

impl From<HtmlTable> for HtmlElement {
    fn from(value: HtmlTable) -> Self {
        value.table
    }
}

impl From<Builder> for HtmlTable {
    fn from(value: Builder) -> Self {
        let has_header = value.has_header();
        let data: Vec<Vec<_>> = value.into();
        let table = build_table(data, has_header);

        Self {
            table,
            css: BTreeMap::default(),
        }
    }
}

impl Display for HtmlTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.css.is_empty() {
            let css = build_css(&self.css);
            css.fmt(f)?;
            f.write_char('\n')?;
        }

        self.table.fmt(f)
    }
}

fn set_cell_attribute(table: &mut HtmlElement, pos: Position, attr: Attribute) {
    struct Setter {
        pos: Position,
        attr: Attribute,
        cursor: Position,
        is_started: bool,
    }

    impl HtmlVisitorMut for Setter {
        fn visit_element_mut(&mut self, e: &mut HtmlElement) -> bool {
            if self.cursor.0 != self.pos.0 {
                // looking for a row
                if e.tag() == "tr" {
                    if self.is_started {
                        self.cursor.0 += 1;
                    } else {
                        self.is_started = true;
                    }
                }
            } else {
                // loking for a column

                if e.tag() == "td" || e.tag() == "th" {
                    if self.cursor == self.pos {
                        let mut attrs = e.attrs().to_vec();
                        attrs.push(self.attr.clone());

                        let val = e.value().cloned();

                        if e.tag() == "td" {
                            *e = HtmlElement::new("td", attrs, val);
                        } else {
                            *e = HtmlElement::new("th", attrs, val);
                        }

                        return false;
                    }

                    self.cursor.1 += 1;
                }
            }

            true
        }
    }

    table.visit_mut(&mut Setter {
        attr,
        pos,
        cursor: (0, 0),
        is_started: false,
    });
}

fn build_table(mut data: Vec<Vec<String>>, has_header: bool) -> HtmlElement {
    let mut elements = vec![];
    if has_header && !data.is_empty() {
        let row = data.remove(0);
        let th_row = build_th(row);

        elements.push(HtmlElement::new(
            "thead",
            vec![],
            Some(HtmlValue::Elements(vec![th_row])),
        ))
    }

    let tr_list = data.into_iter().map(build_tr).collect::<Vec<_>>();
    elements.push(HtmlElement::new(
        "tbody",
        vec![],
        Some(HtmlValue::Elements(tr_list)),
    ));

    HtmlElement::new("table", vec![], Some(HtmlValue::Elements(elements)))
}

fn build_tr(row: Vec<String>) -> HtmlElement {
    build_row(row, "td")
}

fn build_th(row: Vec<String>) -> HtmlElement {
    build_row(row, "th")
}

fn build_row(row: Vec<String>, tag: &str) -> HtmlElement {
    let th_list = row
        .into_iter()
        .map(|content| {
            HtmlValue::Elements(vec![HtmlElement::new(
                "div",
                vec![],
                Some(HtmlValue::Elements(
                    get_lines(&content)
                        .into_iter()
                        .map(|line| HtmlValue::Content(line.to_string()))
                        .map(|content| HtmlElement::new("p", vec![], Some(content)))
                        .collect(),
                )),
            )])
        })
        .map(|content| HtmlElement::new(tag, vec![], Some(content)))
        .collect();

    HtmlElement::new("tr", vec![], Some(HtmlValue::Elements(th_list)))
}

fn entity_target(target: Entity) -> String {
    match target {
        Entity::Global => String::from("tbody > tr > td, thead > tr > th"),
        Entity::Column(col) => {
            let col = col + 1;
            format!("tbody > tr > :nth-child({col}), thead > tr > :nth-child({col})")
        }
        Entity::Row(row) => {
            let row = row + 1;
            format!("tbody > :nth-child({row}) > td, thead > :nth-child({row}) > th")
        }
        Entity::Cell(row, col) => {
            let row = row + 1;
            let col = col + 1;

            if row == 1 {
                format!("table > thead:first-child > :nth-child({row}) > :nth-child({col}), table > tbody:first-child > :nth-child({row}) > :nth-child({col})")
            } else {
                let prev_row = row - 1;
                format!("table:has(thead) > tbody > :nth-child({prev_row}) > :nth-child({col}), table:not(:has(thead)) > tbody > :nth-child({row}) > :nth-child({col})")
            }
        }
    }
}

fn padding_css(pad: Sides<usize>) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    m.insert(String::from("padding-top"), format!("{}px", pad.top));
    m.insert(String::from("padding-bottom"), format!("{}px", pad.bottom));
    m.insert(String::from("padding-left"), format!("{}px", pad.left));
    m.insert(String::from("padding-right"), format!("{}px", pad.right));

    m
}

fn alignment_css(val: Alignment) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();

    match val {
        _ if val == Alignment::left() => {
            m.insert(String::from("text-align"), String::from("left"));
        }
        _ if val == Alignment::right() => {
            m.insert(String::from("text-align"), String::from("right"));
        }
        _ if val == Alignment::center() => {
            m.insert(String::from("text-align"), String::from("center"));
        }
        _ if val == Alignment::top() => {
            m.insert(String::from("vertical-align"), String::from("top"));
        }
        _ if val == Alignment::bottom() => {
            m.insert(String::from("vertical-align"), String::from("top"));
        }
        _ if val == Alignment::center_vertical() => {
            m.insert(String::from("vertical-align"), String::from("center"));
        }
        _ => {}
    }

    m
}

fn build_css(css: &BTreeMap<String, BTreeMap<String, String>>) -> HtmlElement {
    HtmlElement::new(
        "style",
        vec![],
        Some(HtmlValue::Content(
            css.iter()
                .map(|(target, style)| build_css_config(target, style))
                .collect::<Vec<_>>()
                .join("\n"),
        )),
    )
}

fn build_css_config(target: &str, values: &BTreeMap<String, String>) -> String {
    let mut buf = String::new();
    let _ = writeln!(buf, "{target} {{");

    for (key, val) in values {
        let _ = writeln!(buf, "  {key}: {val};");
    }

    let _ = write!(buf, "}}");

    buf
}
