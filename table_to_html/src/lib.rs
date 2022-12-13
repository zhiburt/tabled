#![warn(
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    missing_debug_implementations,
    unreachable_pub,
    missing_docs
)]
#![deny(unused_must_use)]

//! The library provides a interface to build a HTML table (`<table>`) from a [`Table`].
//!
//! Because of the specifics of HTML it's not considered to be the best approach to supply custom CSS for the table.
//! Instead of that you can set a custom id for the table and use your on CSS.
//!
//! # Example
//!
//! ```rust
//! use std::iter::FromIterator;
//!
//! use table_to_html::HtmlTable;
//! use tabled::{object::Rows, Alignment, ModifyObject, Table, Tabled};
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
//! let mut table = Table::from_iter(&data);
//! table.with(Rows::first().modify().with(Alignment::center()));
//!
//! let html_table = HtmlTable::from(table);
//!
//! assert_eq!(
//!     html_table.to_string(),
//!     concat!(
//!         "<table id=\"tabled-table\" border=\"1\">\n",
//!         "    <tbody>\n",
//!         "        <tr id=\"tabled-table-0\">\n",
//!         "            <td id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" style=\"text-align: center;\">\n",
//!         "                <p> name </p>\n",
//!         "            </td>\n",
//!         "            <td id=\"tabled-table-0-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" style=\"text-align: center;\">\n",
//!         "                <p> based_on </p>\n",
//!         "            </td>\n",
//!         "            <td id=\"tabled-table-0-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" style=\"text-align: center;\">\n",
//!         "                <p> is_active </p>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "        <tr id=\"tabled-table-1\">\n",
//!         "            <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
//!         "                <p> Debian </p>\n",
//!         "            </td>\n",
//!         "            <td id=\"tabled-table-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
//!         "            </td>\n",
//!         "            <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
//!         "                <p> true </p>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "        <tr id=\"tabled-table-2\">\n",
//!         "            <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
//!         "                <p> Arch </p>\n",
//!         "            </td>\n",
//!         "            <td id=\"tabled-table-2-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
//!         "            </td>\n",
//!         "            <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
//!         "                <p> true </p>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "        <tr id=\"tabled-table-3\">\n",
//!         "            <td id=\"tabled-table-3-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
//!         "                <p> Manjaro </p>\n",
//!         "            </td>\n",
//!         "            <td id=\"tabled-table-3-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
//!         "                <p> Arch </p>\n",
//!         "            </td>\n",
//!         "            <td id=\"tabled-table-3-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n",
//!         "                <p> true </p>\n",
//!         "            </td>\n",
//!         "        </tr>\n",
//!         "    </tbody>\n",
//!         "</table>",
//!     ),
//! )
//! ```

use std::{
    borrow::Cow,
    fmt::{self, Display, Result, Write},
    ops::{Deref, DerefMut},
};

use tabled::{
    object::Entity,
    papergrid::{records::Records, AlignmentHorizontal, AlignmentVertical, Margin, Padding},
    Table,
};

/// The structure represents an HTML `<table>`.
///
/// You can create it using [From] [Table].
pub struct HtmlTable<T = Table> {
    id: String,
    border_size: usize,
    unit: Unit,
    custom_table_atributes: Vec<Attr<'static, String>>,
    custom_td_atributes: Vec<Attr<'static, String>>,
    custom_tr_atributes: Vec<Attr<'static, String>>,
    custom_cell_print: Option<DynCellPrint<T>>,
    table: T,
}

type DynCellPrint<T> = Box<dyn Fn(&T, usize, usize) -> String>;

impl<T> HtmlTable<T> {
    /// Set an `id` attribute of a `<table>`.
    pub fn set_id(&mut self, id: impl Into<String>) {
        self.id = id.into();
    }

    /// Set a unit measurment which will be used for padding set.
    pub fn set_unit(&mut self, unit: Unit) {
        self.unit = unit;
    }

    /// Set a border size.
    ///
    /// Default value is `1`.
    pub fn set_border_size(&mut self, size: usize) {
        self.border_size = size;
    }

    /// Adds an attribute to a `<table>`.
    pub fn add_table_attr(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        let val = value.into();
        let attr = Attr::new(key, val);

        self.custom_table_atributes.push(attr);
    }

    /// Adds an attribute to all `<td>` inside a `<table>`.
    pub fn add_td_attr(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        let val = value.into();
        let attr = Attr::new(key, val);

        self.custom_td_atributes.push(attr);
    }

    /// Adds an attribute to all `<tr>` inside a `<table>`.
    pub fn add_tr_attr(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        let val = value.into();
        let attr = Attr::new(key, val);

        self.custom_tr_atributes.push(attr);
    }

    /// Overrides cell output to the given function.
    ///
    /// todo: We could create a trait like a Visitor which would be called when building the table.
    ///       and a user could influence the build by overriding the emit logic, setting attribute etc.
    pub fn override_cell_elements<F>(&mut self, f: F)
    where
        F: Fn(&T, usize, usize) -> String + 'static,
    {
        self.custom_cell_print = Some(Box::new(f));
    }
}

impl<R> From<Table<R>> for HtmlTable<Table<R>> {
    fn from(table: Table<R>) -> Self {
        Self {
            id: "tabled-table".into(),
            table,
            border_size: 1,
            unit: Unit::Rem,
            custom_table_atributes: Vec::new(),
            custom_td_atributes: Vec::new(),
            custom_tr_atributes: Vec::new(),
            custom_cell_print: None,
        }
    }
}

impl fmt::Debug for HtmlTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        f.debug_struct("HtmlTable")
            .field("id", &self.id)
            .field("border_size", &self.border_size)
            .field("unit", &self.unit)
            .field("custom_table_atributes", &self.custom_table_atributes)
            .field("custom_td_atributes", &self.custom_td_atributes)
            .field("custom_tr_atributes", &self.custom_tr_atributes)
            .field(
                "custom_cell_print",
                &self
                    .custom_cell_print
                    .as_ref()
                    .map(|f| core::ptr::addr_of!(f)),
            )
            .field("table", &self.table)
            .finish()
    }
}

impl<R> Display for HtmlTable<Table<R>>
where
    R: Records,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        convert_to_html_table(
            f,
            &self.table,
            &self.id,
            self.unit,
            self.border_size,
            &self.custom_table_atributes,
            &self.custom_tr_atributes,
            &self.custom_td_atributes,
            &self.custom_cell_print,
        )
    }
}

/// Unit represents a HTML measure values for different attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Unit {
    /// `em`
    Em,
    /// `rem`
    Rem,
    /// `px`
    Px,
}

impl From<Unit> for &'static str {
    fn from(val: Unit) -> Self {
        match val {
            Unit::Em => "em",
            Unit::Rem => "rem",
            Unit::Px => "px",
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        let s: &str = (*self).into();
        s.fmt(f)
    }
}

#[allow(clippy::too_many_arguments)]
fn convert_to_html_table<R>(
    f: &mut fmt::Formatter<'_>,
    table: &Table<R>,
    table_id: &str,
    unit: Unit,
    border_size: usize,
    table_attrs: &[Attr<'static, String>],
    tr_attrs: &[Attr<'static, String>],
    td_attrs: &[Attr<'static, String>],
    print_custom_cell: &Option<impl Fn(&Table<R>, usize, usize) -> String>,
) -> fmt::Result
where
    R: Records,
{
    if table.has_header() {
        let body = (0..2).map(|i| {
            let (body_tag, inner_tag, (row_start, row_end)) = if i == 0 {
                ("thead", "th", (0, 1))
            } else {
                ("tbody", "td", (1, table.count_rows()))
            };

            let rows = (row_start..row_end).map(move |row| {
                let columns = (0..table.count_columns())
                    .filter(move |col| {
                        table
                            .get_config()
                            .is_cell_visible((row, *col), table.shape())
                    })
                    .map(move |col| {
                        let td_attr = if body_tag == "tbody" { td_attrs } else { &[] };
                        let attrs = create_cell_attrs(table, table_id, row, col, unit, td_attr);

                        let text = match print_custom_cell.as_ref() {
                            Some(f) => {
                                let text = (f)(table, row, col);
                                Paragraph::NoEdit(text)
                            }
                            None => {
                                let text = table.get_records().get_text((row, col));
                                let text = html_escape_text(text);
                                Paragraph::General(text)
                            }
                        };

                        tag(inner_tag, attrs, text)
                    });

                let td = block(columns);

                let mut attrs = vec![attr("id", id(table_id, [row]).to_string())];
                attrs.extend(tr_attrs.iter().cloned());

                tag("tr", attrs, td)
            });
            let inner = block(rows);
            tag(body_tag, [Attr::default(); 0], inner)
        });

        let mut attrs = vec![attr("id", table_id.to_string())];

        let margin = table.get_config().get_margin();
        if *margin != Margin::default() {
            let margin = format!(
                "margin: {}{} {}{} {}{} {}{};",
                margin.top.size,
                unit,
                margin.right.size,
                unit,
                margin.bottom.size,
                unit,
                margin.left.size,
                unit
            );

            let attr = attr("style", margin);
            attrs.push(attr);
        }

        if border_size > 0 {
            attrs.push(attr("border", border_size.to_string()));
        }

        attrs.extend(table_attrs.iter().cloned());

        let table = tag("table", attrs, block(body));

        let mut ctx = Context::new(0, 4, f);
        table.display(&mut ctx)?;

        return Ok(());
    }

    let rows = (0..table.count_rows()).map(|row| {
        let columns = (0..table.count_columns())
            .filter(move |col| {
                table
                    .get_config()
                    .is_cell_visible((row, *col), table.shape())
            })
            .map(move |col| {
                let attrs = create_cell_attrs(table, table_id, row, col, unit, td_attrs);

                let text = match print_custom_cell.as_ref() {
                    Some(f) => {
                        let text = (f)(table, row, col);
                        Paragraph::NoEdit(text)
                    }
                    None => {
                        let text = table.get_records().get_text((row, col));
                        let text = html_escape_text(text);
                        Paragraph::General(text)
                    }
                };

                tag("td", attrs, text)
            });

        let td = block(columns);

        let mut attrs = vec![attr("id", id(table_id, [row]).to_string())];
        attrs.extend(tr_attrs.iter().cloned());

        tag("tr", attrs, td)
    });
    let inner = block(rows);
    let tbody = tag("tbody", [Attr::default(); 0], inner);

    let mut attrs = vec![attr("id", table_id.to_string())];

    let margin = table.get_config().get_margin();
    if *margin != Margin::default() {
        let margin = format!(
            "margin: {}{} {}{} {}{} {}{};",
            margin.top.size,
            unit,
            margin.right.size,
            unit,
            margin.bottom.size,
            unit,
            margin.left.size,
            unit
        );

        let attr = attr("style", margin);
        attrs.push(attr);
    }

    if border_size > 0 {
        attrs.push(attr("border", border_size.to_string()));
    }

    attrs.extend(table_attrs.iter().cloned());

    let table = tag("table", attrs, tbody);

    let mut ctx = Context::new(0, 4, f);
    table.display(&mut ctx)
}

fn create_cell_attrs<R>(
    table: &Table<R>,
    table_id: &str,
    row: usize,
    col: usize,
    unit: Unit,
    td_attrs: &[Attr<'static, String>],
) -> Vec<Attr<'static, String>>
where
    R: Records,
{
    let id = attr("id", id(table_id, [row, col]).to_string());

    let mut attrs = vec![id];

    let padding = table.get_config().get_padding(Entity::Cell(row, col));
    add_attr_padding(*padding, unit, &mut attrs);

    let halignment = table
        .get_config()
        .get_alignment_horizontal(Entity::Cell(row, col));
    add_attr_horizontal_alignment(halignment, &mut attrs);

    let valignment = table
        .get_config()
        .get_alignment_vertical(Entity::Cell(row, col));
    add_attr_vertical_alignment(valignment, &mut attrs);

    let hspan = table
        .get_config()
        .get_column_span((row, col), table.shape());
    add_attr_horizontal_span(hspan, &mut attrs);

    let vspan = table.get_config().get_row_span((row, col), table.shape());
    add_attr_vertical_span(vspan, &mut attrs);

    attrs.extend(td_attrs.iter().cloned());

    attrs
}

fn add_attr_horizontal_alignment(
    alignment: &AlignmentHorizontal,
    attrs: &mut Vec<Attr<'static, String>>,
) {
    if !matches!(alignment, AlignmentHorizontal::Left) {
        let alignment = match alignment {
            AlignmentHorizontal::Center => "center",
            AlignmentHorizontal::Left => "left",
            AlignmentHorizontal::Right => "right",
        };

        let attr = attr("style", format!("text-align: {};", alignment));
        attrs.push(attr);
    }
}

fn add_attr_vertical_alignment(
    alignment: &AlignmentVertical,
    attrs: &mut Vec<Attr<'static, String>>,
) {
    if !matches!(alignment, AlignmentVertical::Top) {
        let valignment = match alignment {
            AlignmentVertical::Center => "center",
            AlignmentVertical::Bottom => "bottom",
            AlignmentVertical::Top => "top",
        };

        let attr = attr("style", format!("vertical-align: {};", valignment));
        attrs.push(attr);
    }
}

fn add_attr_vertical_span(span: Option<usize>, attrs: &mut Vec<Attr<'static, String>>) {
    if let Some(span) = span {
        let attr = attr("rowspan", span.to_string());
        attrs.push(attr);
    }
}

fn add_attr_horizontal_span(span: Option<usize>, attrs: &mut Vec<Attr<'static, String>>) {
    if let Some(span) = span {
        let attr = attr("colspan", span.to_string());
        attrs.push(attr);
    }
}

fn add_attr_padding(padding: Padding, unit: Unit, attrs: &mut Vec<Attr<'static, String>>) {
    if padding != Padding::default() {
        let padding = attr(
            "style",
            format!(
                "padding-top: {}{}; padding-bottom: {}{}; padding-left: {}{}; padding-right: {}{};",
                padding.top.size,
                unit,
                padding.bottom.size,
                unit,
                padding.left.size,
                unit,
                padding.right.size,
                unit
            ),
        );
        attrs.push(padding);
    }
}

fn id<T>(table_id: &str, tail: T) -> ElementID<'_, T> {
    ElementID::new(table_id, tail)
}

#[derive(Debug, Clone)]
struct ElementID<'a, T> {
    table_id: &'a str,
    tail: T,
}

impl<'a, T> ElementID<'a, T> {
    fn new(table_id: &'a str, tail: T) -> Self {
        Self { table_id, tail }
    }
}

impl<T> Display for ElementID<'_, T>
where
    T: IntoIterator + Clone,
    T::Item: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.table_id.is_empty() {
            f.write_str(self.table_id)?;
        }

        for (i, part) in self.tail.clone().into_iter().enumerate() {
            if i != 0 || !self.table_id.is_empty() {
                f.write_char('-')?;
            }

            part.fmt(f)?;
        }

        Ok(())
    }
}

fn attr<D>(name: &str, value: D) -> Attr<'_, D> {
    Attr::new(name, value)
}

#[derive(Debug, Clone)]
struct Attr<'a, D> {
    name: Cow<'a, str>,
    value: D,
}

impl Default for Attr<'static, &'static str> {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl<'a, D> Attr<'a, D> {
    fn new(name: impl Into<Cow<'a, str>>, value: D) -> Self {
        let name = name.into();
        Self { name, value }
    }
}

impl<T> Display for Attr<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)?;
        f.write_char('=')?;
        f.write_char('"')?;
        self.value.fmt(f)?;
        f.write_char('"')?;

        Ok(())
    }
}

fn tag<'a, 'b, A, I, D>(name: &'a str, attrs: A, inner: I) -> Tag<'a, A, I>
where
    A: IntoIterator<Item = Attr<'b, D>> + Clone,
    D: Display,
{
    Tag::new(name, attrs, inner)
}

struct Tag<'a, A, I> {
    name: Cow<'a, str>,
    attrs: A,
    inner: I,
}

impl<'a, 'b, A, I> Tag<'a, A, I> {
    fn new<D>(name: impl Into<Cow<'a, str>>, attrs: A, inner: I) -> Self
    where
        A: IntoIterator<Item = Attr<'b, D>> + Clone,
        D: Display,
    {
        let name = name.into();
        Self { name, attrs, inner }
    }
}

impl<'b, A, D, I> Element for Tag<'_, A, I>
where
    A: IntoIterator<Item = Attr<'b, D>> + Clone,
    D: Display,
    I: Element,
{
    fn display(&self, ctx: &mut Context<'_, '_>) -> fmt::Result {
        ctx.make_tab()?;
        ctx.write_char('<')?;
        ctx.write_str(&self.name)?;

        for attr in self.attrs.clone() {
            ctx.write_char(' ')?;
            attr.fmt(ctx.deref_mut())?;
        }

        ctx.write_str(">\n")?;

        if !self.inner.is_empty() {
            let mut ctx = ctx.dive();
            self.inner.display(&mut ctx)?;
            ctx.write_str("\n")?;
        }

        ctx.make_tab()?;
        ctx.write_str("</")?;
        ctx.write_str(&self.name)?;
        ctx.write_char('>')?;

        Ok(())
    }

    fn is_empty(&self) -> bool {
        false
    }
}

/// A type which represents either a text which will be emited inside `<p></p>` tags or
/// A custom HTML which will be left as it is.
#[derive(Debug)]
pub enum Paragraph {
    /// Text inside paragraphs.
    General(String),
    /// HTML which will be emited.
    NoEdit(String),
}

impl Element for Paragraph {
    fn display(&self, ctx: &mut Context<'_, '_>) -> fmt::Result {
        match self {
            Paragraph::General(text) => {
                for (i, line) in text.lines().enumerate() {
                    if i > 0 {
                        ctx.write_str("\n")?;
                    }

                    ctx.make_tab()?;
                    ctx.write_str("<p> ")?;
                    ctx.write_str(line)?;
                    ctx.write_str(" </p>")?;
                }
            }
            Paragraph::NoEdit(text) => text.display(ctx)?,
        }

        Ok(())
    }

    fn is_empty(&self) -> bool {
        match self {
            Paragraph::General(text) => text.is_empty(),
            Paragraph::NoEdit(text) => text.is_empty(),
        }
    }
}

impl Element for &str {
    fn display(&self, ctx: &mut Context<'_, '_>) -> fmt::Result {
        ctx.write_str(self)
    }

    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl Element for String {
    fn display(&self, ctx: &mut Context<'_, '_>) -> fmt::Result {
        self.as_str().display(ctx)
    }

    fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }
}

fn block<F>(f: F) -> Block<F>
where
    F: IntoIterator + Clone,
    F::Item: Element,
{
    Block::new(f)
}

struct Block<F> {
    f: F,
}

impl<F> Block<F> {
    fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F> Element for Block<F>
where
    F: IntoIterator + Clone,
    F::Item: Element,
{
    fn display(&self, ctx: &mut Context<'_, '_>) -> fmt::Result {
        for (i, element) in self.f.clone().into_iter().enumerate() {
            if i != 0 {
                writeln!(ctx.f)?;
            }

            element.display(ctx)?;
        }

        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.f.clone().into_iter().count() == 0
    }
}

/// An HTML element.
pub trait Element {
    /// Display element.
    fn display(&self, ctx: &mut Context<'_, '_>) -> fmt::Result;
    /// Is element empty and can be skipped.
    fn is_empty(&self) -> bool;
}

/// Context has some general structure.
pub struct Context<'a, 'b> {
    deep: usize,
    deep_step: usize,
    f: &'a mut fmt::Formatter<'b>,
}

impl fmt::Debug for Context<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        f.debug_struct("Context")
            .field("deep", &self.deep)
            .field("deep_step", &self.deep_step)
            .field("f", &std::ptr::addr_of!(self.f))
            .finish()
    }
}

impl<'a, 'b> Context<'a, 'b> {
    fn new(deep: usize, deep_step: usize, f: &'a mut fmt::Formatter<'b>) -> Self {
        Self { deep, deep_step, f }
    }

    /// Increase tab size
    pub fn dive<'c>(&'c mut self) -> Context<'c, 'b> {
        Context::new(self.deep + self.deep_step, self.deep_step, self.f)
    }
}

impl Context<'_, '_> {
    /// Print tab.
    pub fn make_tab(&mut self) -> fmt::Result {
        for _ in 0..self.deep {
            self.write_char(' ')?;
        }

        Ok(())
    }
}

impl<'a, 'b> Deref for Context<'a, 'b> {
    type Target = fmt::Formatter<'b>;

    fn deref(&self) -> &Self::Target {
        self.f
    }
}

impl<'a, 'b> DerefMut for Context<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.f
    }
}

/// Escape HTML text
pub fn html_escape_text(text: &str) -> String {
    let mut buf = String::new();
    for c in text.chars() {
        match c {
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '&' => buf.push_str("&amp;"),
            '\'' => buf.push_str("&apos;"),
            '\"' => buf.push_str("&quot;"),
            c => buf.push(c),
        }
    }

    buf
}

#[cfg(test)]
mod tests {
    use tabled::Panel;

    use super::*;

    #[test]
    fn basic() {
        let table = Table::new([["123", "324", "zxc"], ["123", "324", "zxc"]]);
        let table = HtmlTable::from(table).to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\">\n    <thead>\n        <tr id=\"tabled-table-0\">\n            <th id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 0 </p>\n            </th>\n            <th id=\"tabled-table-0-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n            </th>\n            <th id=\"tabled-table-0-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 2 </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\">\n            <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-2\">\n            <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-2-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn basic_multiline() {
        let table = Table::new([["1\n2\n3", "324", "zxc"], ["123", "324", "zxc"]]);
        let table = HtmlTable::from(table).to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\">\n    <thead>\n        <tr id=\"tabled-table-0\">\n            <th id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 0 </p>\n            </th>\n            <th id=\"tabled-table-0-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n            </th>\n            <th id=\"tabled-table-0-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 2 </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\">\n            <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n                <p> 2 </p>\n                <p> 3 </p>\n            </td>\n            <td id=\"tabled-table-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-2\">\n            <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-2-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn set_id() {
        let table = Table::new([["123", "324", "zxc"], ["123", "324", "zxc"]]);
        let mut table = HtmlTable::from(table);
        table.set_id("custom.id.0");

        let table = table.to_string();

        assert_eq!(table, "<table id=\"custom.id.0\" border=\"1\">\n    <thead>\n        <tr id=\"custom.id.0-0\">\n            <th id=\"custom.id.0-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 0 </p>\n            </th>\n            <th id=\"custom.id.0-0-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n            </th>\n            <th id=\"custom.id.0-0-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 2 </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"custom.id.0-1\">\n            <td id=\"custom.id.0-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"custom.id.0-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"custom.id.0-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"custom.id.0-2\">\n            <td id=\"custom.id.0-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"custom.id.0-2-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"custom.id.0-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn set_unit() {
        let table = Table::new([["123", "324", "zxc"], ["123", "324", "zxc"]]);
        let mut table = HtmlTable::from(table);
        table.set_unit(Unit::Em);

        let table = table.to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\">\n    <thead>\n        <tr id=\"tabled-table-0\">\n            <th id=\"tabled-table-0-0\" style=\"padding-top: 0em; padding-bottom: 0em; padding-left: 1em; padding-right: 1em;\">\n                <p> 0 </p>\n            </th>\n            <th id=\"tabled-table-0-1\" style=\"padding-top: 0em; padding-bottom: 0em; padding-left: 1em; padding-right: 1em;\">\n                <p> 1 </p>\n            </th>\n            <th id=\"tabled-table-0-2\" style=\"padding-top: 0em; padding-bottom: 0em; padding-left: 1em; padding-right: 1em;\">\n                <p> 2 </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\">\n            <td id=\"tabled-table-1-0\" style=\"padding-top: 0em; padding-bottom: 0em; padding-left: 1em; padding-right: 1em;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-1-1\" style=\"padding-top: 0em; padding-bottom: 0em; padding-left: 1em; padding-right: 1em;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-1-2\" style=\"padding-top: 0em; padding-bottom: 0em; padding-left: 1em; padding-right: 1em;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-2\">\n            <td id=\"tabled-table-2-0\" style=\"padding-top: 0em; padding-bottom: 0em; padding-left: 1em; padding-right: 1em;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-2-1\" style=\"padding-top: 0em; padding-bottom: 0em; padding-left: 1em; padding-right: 1em;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-2-2\" style=\"padding-top: 0em; padding-bottom: 0em; padding-left: 1em; padding-right: 1em;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn set_tr_attrs() {
        let table = Table::new([["123", "324", "zxc"], ["123", "324", "zxc"]]);
        let mut table = HtmlTable::from(table);
        table.add_tr_attr("custom-attr", "custom-val");

        let table = table.to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\">\n    <thead>\n        <tr id=\"tabled-table-0\" custom-attr=\"custom-val\">\n            <th id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 0 </p>\n            </th>\n            <th id=\"tabled-table-0-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n            </th>\n            <th id=\"tabled-table-0-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 2 </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\" custom-attr=\"custom-val\">\n            <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-2\" custom-attr=\"custom-val\">\n            <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-2-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn set_td_attrs() {
        let table = Table::new([["123", "324", "zxc"], ["123", "324", "zxc"]]);
        let mut table = HtmlTable::from(table);
        table.add_td_attr("custom-attr", "custom-val");

        let table = table.to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\">\n    <thead>\n        <tr id=\"tabled-table-0\">\n            <th id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 0 </p>\n            </th>\n            <th id=\"tabled-table-0-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n            </th>\n            <th id=\"tabled-table-0-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 2 </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\">\n            <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" custom-attr=\"custom-val\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" custom-attr=\"custom-val\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" custom-attr=\"custom-val\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-2\">\n            <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" custom-attr=\"custom-val\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-2-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" custom-attr=\"custom-val\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" custom-attr=\"custom-val\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn set_table_attrs() {
        let table = Table::new([["123", "324", "zxc"], ["123", "324", "zxc"]]);
        let mut table = HtmlTable::from(table);
        table.add_table_attr("custom-attr", "custom-val");

        let table = table.to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\" custom-attr=\"custom-val\">\n    <thead>\n        <tr id=\"tabled-table-0\">\n            <th id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 0 </p>\n            </th>\n            <th id=\"tabled-table-0-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n            </th>\n            <th id=\"tabled-table-0-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 2 </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\">\n            <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-2\">\n            <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-2-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn raw_span() {
        let mut table = Table::new([["123", "324", "zxc"], ["123", "324", "zxc"]]);
        table.with(Panel::header("Hello World!"));

        let table = HtmlTable::from(table).to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\">\n    <thead>\n        <tr id=\"tabled-table-0\">\n            <th id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" colspan=\"3\">\n                <p> Hello World! </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\">\n            <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 0 </p>\n            </td>\n            <td id=\"tabled-table-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n            </td>\n            <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 2 </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-2\">\n            <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-2-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-3\">\n            <td id=\"tabled-table-3-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-3-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-3-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn col_span() {
        let mut table = Table::new([["123", "324", "zxc"], ["123", "324", "zxc"]]);
        table.with(Panel::vertical(1).text("Hello World!").text_width(1));

        let table = HtmlTable::from(table).to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\">\n    <thead>\n        <tr id=\"tabled-table-0\">\n            <th id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 0 </p>\n            </th>\n            <th id=\"tabled-table-0-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" rowspan=\"3\">\n                <p> H </p>\n                <p> e </p>\n                <p> l </p>\n                <p> l </p>\n                <p> o </p>\n                <p>   </p>\n                <p> W </p>\n                <p> o </p>\n                <p> r </p>\n                <p> l </p>\n                <p> d </p>\n                <p> ! </p>\n            </th>\n            <th id=\"tabled-table-0-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n            </th>\n            <th id=\"tabled-table-0-3\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 2 </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\">\n            <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-1-3\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-2\">\n            <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-2-3\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn row_and_col_span() {
        let mut table = Table::new([["123", "324", "zxc"], ["123", "324", "zxc"]]);
        table.with(Panel::header("Hello World!"));
        table.with(Panel::vertical(1).row(1).text("Hello World!").text_width(1));

        let table = HtmlTable::from(table).to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\">\n    <thead>\n        <tr id=\"tabled-table-0\">\n            <th id=\"tabled-table-0-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" colspan=\"3\">\n                <p> Hello World! </p>\n            </th>\n            <th id=\"tabled-table-0-3\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\">\n            <td id=\"tabled-table-1-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 0 </p>\n            </td>\n            <td id=\"tabled-table-1-1\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\" rowspan=\"3\">\n                <p> H </p>\n                <p> e </p>\n                <p> l </p>\n                <p> l </p>\n                <p> o </p>\n                <p>   </p>\n                <p> W </p>\n                <p> o </p>\n                <p> r </p>\n                <p> l </p>\n                <p> d </p>\n                <p> ! </p>\n            </td>\n            <td id=\"tabled-table-1-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 1 </p>\n            </td>\n            <td id=\"tabled-table-1-3\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 2 </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-2\">\n            <td id=\"tabled-table-2-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-2-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-2-3\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n        <tr id=\"tabled-table-3\">\n            <td id=\"tabled-table-3-0\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 123 </p>\n            </td>\n            <td id=\"tabled-table-3-2\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> 324 </p>\n            </td>\n            <td id=\"tabled-table-3-3\" style=\"padding-top: 0rem; padding-bottom: 0rem; padding-left: 1rem; padding-right: 1rem;\">\n                <p> zxc </p>\n            </td>\n        </tr>\n    </tbody>\n</table>")
    }

    #[test]
    fn text_escape() {
        let mut table = Table::new([["&&&<<<>>>'''\"\"\""]]);
        table.with(tabled::Padding::zero());
        let table = HtmlTable::from(table).to_string();

        assert_eq!(table, "<table id=\"tabled-table\" border=\"1\">\n    <thead>\n        <tr id=\"tabled-table-0\">\n            <th id=\"tabled-table-0-0\">\n                <p> 0 </p>\n            </th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr id=\"tabled-table-1\">\n            <td id=\"tabled-table-1-0\">\n                <p> &amp;&amp;&amp;&lt;&lt;&lt;&gt;&gt;&gt;&apos;&apos;&apos;&quot;&quot;&quot; </p>\n            </td>\n        </tr>\n    </tbody>\n</table>");
    }
}
