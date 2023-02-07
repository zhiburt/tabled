//! This module contains a main table representation of this crate [`Table`].

use std::{borrow::Cow, fmt, io, iter::FromIterator};

use papergrid::{
    config::{Formatting, Indent, Padding},
    records::{IntoRecords, IterRecords},
    util::string::string_width_multiline_tab,
};

use crate::{
    builder::Builder,
    grid::config::AlignmentHorizontal,
    grid::{
        config::{Entity, GridConfig},
        dimension::{Dimension, ExactDimension},
        grid_projection::GridProjection,
        Grid,
    },
    records::{EmptyRecords, ExactRecords, Records, VecRecords},
    settings::width::Truncate,
    Style, TableOption, Tabled,
};

#[derive(Debug, Clone)]
pub struct Table<I> {
    records: I,
    cfg: GridConfig,
    table: TableConfig,
}

#[derive(Debug, Clone)]
struct TableConfig {
    sniff: usize,
    height: usize,
    width: usize,
    count_columns: usize,
    count_rows: Option<usize>,
}

impl<I> Table<I> {
    pub fn new(iter: I) -> Self
    where
        I: IntoRecords,
    {
        Self {
            records: iter,
            cfg: create_config(),
            table: TableConfig {
                sniff: 1000,
                height: 1,
                width: 0,
                count_columns: 0,
                count_rows: None,
            },
        }
    }

    /// With is a generic function which applies options to the [`Table`].
    pub fn with<O>(mut self, option: O) -> Self
    where
        for<'a> O: TableOption<IterRecords<&'a I>, TableDimension<'static>>,
    {
        let mut dimension = TableDimension {
            height: self.table.height,
            width: Width::Exact(self.table.width),
        };

        let mut records = IterRecords::new(
            &self.records,
            self.table.count_columns,
            self.table.count_rows,
        );

        let mut option = option;
        option.change(&mut records, &mut self.cfg, &mut dimension);

        self
    }

    pub fn cols(mut self, count_columns: usize) -> Self {
        self.table.count_columns = count_columns;
        self
    }

    pub fn rows(mut self, count_rows: usize) -> Self {
        self.table.count_rows = Some(count_rows);
        self
    }

    pub fn sniff(mut self, count: usize) -> Self {
        self.table.sniff = count;
        self
    }

    pub fn height(mut self, size: usize) -> Self {
        self.table.height = size;
        self
    }

    pub fn width(mut self, size: usize) -> Self {
        self.table.width = size;
        self
    }

    pub fn fmt<W: fmt::Write>(mut self, writer: W) -> fmt::Result
    where
        I: IntoRecords,
    {
        let mut config = self.cfg;
        clean_config(&mut config);

        let padding = config.get_padding(Entity::Global);
        let padding = padding.left.size + padding.right.size;
        let tab_size = config.get_tab_width();

        if self.table.width != 0 && self.table.count_columns != 0 {
            let width = self.table.width;
            let widths = self.table.width - padding;
            let columns_widths = Width::Exact(width);
            let columns_content_widths = Width::Exact(widths);

            let count_columns = self.table.count_columns;

            let records = BufRecords::new(self.records, 0);
            let records = TruncatedRecords::new(records, columns_content_widths, tab_size);
            let records = ColumnLimitRecords::new(records, count_columns);
            let records = IterRecords::new(records, count_columns, self.table.count_rows);

            let dimension = TableDimension {
                height: self.table.height,
                width: columns_widths,
            };

            Grid::new(records, &config, &dimension).build(writer)
        // } else if self.table.width != 0 {
        // } else if self.table.count_columns != 0 {
        } else {
            let records = BufRecords::new(self.records, self.table.sniff);
            let records = BufRecords2::from(records);

            let count_columns = records
                .as_slice()
                .iter()
                .map(|row| row.len())
                .max()
                .unwrap_or(0);

            let buf_records = IterRecords::new(records.as_slice(), count_columns, None);
            let columns_width = crate::grid::dimension::ExactDimension::width(buf_records, &config);

            let width = Width::Columns(Cow::Borrowed(&columns_width));

            let records = TruncatedRecords::new(records, width.clone(), tab_size);
            let records = ColumnLimitRecords::new(records, count_columns);
            let records = IterRecords::new(records, count_columns, self.table.count_rows);

            let dimension = TableDimension {
                height: self.table.height,
                width,
            };

            Grid::new(records, &config, &dimension).build(writer)
        }
    }

    pub fn to_string(self) -> String
    where
        I: IntoRecords,
    {
        let mut buf = String::new();
        self.fmt(&mut buf).expect("safe");

        buf
    }

    pub fn build<W: io::Write>(self, writer: W) -> io::Result<()>
    where
        I: IntoRecords,
    {
        let writer = UTF8Writer(writer);
        self.fmt(writer)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }
}

// Table::default().cols(count_columns).rows(rows).sniff(4).height(5).width(10).build(&mut String::new());

#[derive(Debug)]
pub struct TableDimension<'a> {
    width: Width<'a>,
    height: usize,
}

#[derive(Debug, Clone)]
enum Width<'a> {
    Exact(usize),
    Columns(Cow<'a, [usize]>),
}

impl Width<'_> {
    fn get(&self, col: usize) -> usize {
        match self {
            Width::Exact(val) => *val,
            Width::Columns(cols) => cols[col],
        }
    }
}

impl Dimension for TableDimension<'_> {
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig) {}

    fn get_width(&self, column: usize) -> usize {
        match &self.width {
            Width::Exact(width) => *width,
            Width::Columns(columns) => columns[column],
        }
    }

    fn get_height(&self, _: usize) -> usize {
        self.height
    }
}

fn create_config() -> GridConfig {
    let mut cfg = GridConfig::default();
    cfg.set_tab_width(4);
    cfg.set_padding(
        Entity::Global,
        Padding::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::default(),
            Indent::default(),
        ),
    );
    cfg.set_alignment_horizontal(Entity::Global, AlignmentHorizontal::Left);
    cfg.set_formatting(Entity::Global, Formatting::new(false, false, false));
    cfg.set_borders(*Style::ascii().get_borders());

    cfg
}

fn clean_config(cfg: &mut GridConfig) {
    cfg.clear_span_column();
    cfg.clear_span_row();

    // todo: leave only global options...
}

struct UTF8Writer<W>(W);

impl<W> fmt::Write for UTF8Writer<W>
where
    W: io::Write,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut buf = s.as_bytes();
        loop {
            let n = self.0.write(buf).map_err(|_| std::fmt::Error::default())?;
            if n == buf.len() {
                break;
            }

            buf = &buf[n..];
        }

        Ok(())
    }
}

// todo: table::table/iter/extended

#[derive(Debug)]
struct BufRecords<I, T> {
    iter: I,
    buf: Vec<T>,
}

impl BufRecords<(), ()> {
    fn new<I>(
        records: I,
        sniff: usize,
    ) -> BufRecords<<I::IterRows as IntoIterator>::IntoIter, I::IterColumns>
    where
        I: IntoRecords,
    {
        let mut buf = vec![];

        let mut iter = records.iter_rows().into_iter();
        for _ in 0..sniff {
            match iter.next() {
                Some(row) => buf.push(row),
                None => break,
            }
        }

        BufRecords { iter, buf }
    }
}

impl<I, T> BufRecords<I, T> {
    pub fn as_slice(&self) -> &[T] {
        &self.buf
    }
}

impl<I, T> From<BufRecords<I, T>> for BufRecords2<I>
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    fn from(value: BufRecords<I, T>) -> Self {
        let buf = value
            .buf
            .into_iter()
            .map(|row| row.into_iter().map(|s| s.as_ref().to_string()).collect())
            .collect();

        BufRecords2 {
            iter: value.iter,
            buf,
        }
    }
}

impl<I, T> IntoRecords for BufRecords<I, T>
where
    I: Iterator<Item = T>,
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    type Cell = T::Item;
    type IterColumns = T;
    type IterRows = BufRecordsIter<I, T>;

    fn iter_rows(self) -> Self::IterRows {
        BufRecordsIter {
            buf: self.buf.into_iter(),
            iter: self.iter,
        }
    }
}

pub struct BufRecordsIter<I, T> {
    buf: std::vec::IntoIter<T>,
    iter: I,
}

impl<I, T> Iterator for BufRecordsIter<I, T>
where
    I: Iterator<Item = T>,
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.next() {
            Some(i) => Some(i),
            None => self.iter.next(),
        }
    }
}

#[derive(Debug)]
struct BufRecords2<I> {
    iter: I,
    buf: Vec<Vec<String>>,
}

impl BufRecords2<()> {
    fn new<I>(records: I, sniff: usize) -> BufRecords2<<I::IterRows as IntoIterator>::IntoIter>
    where
        I: IntoRecords,
    {
        let mut buf = vec![];

        let mut iter = records.iter_rows().into_iter();
        for _ in 0..sniff {
            match iter.next() {
                Some(row) => {
                    let row = row
                        .into_iter()
                        .map(|cell| cell.as_ref().to_string())
                        .collect::<Vec<_>>();
                    buf.push(row)
                }
                None => break,
            }
        }

        BufRecords2 { iter, buf }
    }
}

impl<I> BufRecords2<I> {
    pub fn as_slice(&self) -> &[Vec<String>] {
        &self.buf
    }
}

impl<I> IntoRecords for BufRecords2<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Cell = SomeString<<I::Item as IntoIterator>::Item>;
    type IterColumns = SomeRowIterator<<I::Item as IntoIterator>::IntoIter>;
    type IterRows = BufRecordsIter2<I>;

    fn iter_rows(self) -> Self::IterRows {
        BufRecordsIter2 {
            buf: self.buf.into_iter(),
            iter: self.iter,
        }
    }
}

pub struct BufRecordsIter2<I> {
    buf: std::vec::IntoIter<Vec<String>>,
    iter: I,
}

impl<I> Iterator for BufRecordsIter2<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = SomeRowIterator<<I::Item as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.next() {
            Some(i) => Some(SomeRowIterator::Owned(i.into_iter())),
            None => self
                .iter
                .next()
                .map(|i| SomeRowIterator::Ref(i.into_iter())),
        }
    }
}

pub enum SomeString<T> {
    Owned(String),
    T(T),
}

impl<T> AsRef<str> for SomeString<T>
where
    T: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        match self {
            SomeString::Owned(s) => s.as_ref(),
            SomeString::T(s) => s.as_ref(),
        }
    }
}

pub enum SomeRowIterator<I> {
    Owned(std::vec::IntoIter<String>),
    Ref(I),
}

impl<I> Iterator for SomeRowIterator<I>
where
    I: Iterator,
{
    type Item = SomeString<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SomeRowIterator::Owned(iter) => iter.next().map(SomeString::Owned),
            SomeRowIterator::Ref(iter) => iter.next().map(SomeString::T),
        }
    }
}

#[derive(Debug)]
struct TruncatedRecords<'a, I> {
    records: I,
    width: Width<'a>,
    tab_size: usize,
}

impl TruncatedRecords<'_, ()> {
    fn new<I>(records: I, width: Width<'_>, tab_size: usize) -> TruncatedRecords<'_, I::IterRows>
    where
        I: IntoRecords,
    {
        TruncatedRecords {
            records: records.iter_rows(),
            width,
            tab_size,
        }
    }
}

impl<'a, I> IntoRecords for TruncatedRecords<'a, I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Cell = SomeString<<I::Item as IntoIterator>::Item>;
    type IterColumns = TruncatedRecordsColumnsIter<'a, <I::Item as IntoIterator>::IntoIter>;
    type IterRows = TruncatedRecordsIter<'a, I>;

    fn iter_rows(self) -> Self::IterRows {
        TruncatedRecordsIter {
            iter: self.records,
            width: self.width.clone(),
            tab_size: self.tab_size,
        }
    }
}

pub struct TruncatedRecordsIter<'a, I> {
    iter: I,
    width: Width<'a>,
    tab_size: usize,
}

impl<'a, I> Iterator for TruncatedRecordsIter<'a, I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = TruncatedRecordsColumnsIter<'a, <I::Item as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.iter.next()?;
        Some(TruncatedRecordsColumnsIter {
            iter: iter.into_iter(),
            current: 0,
            width: self.width.clone(),
            tab_size: self.tab_size,
        })
    }
}

pub struct TruncatedRecordsColumnsIter<'a, I> {
    iter: I,
    width: Width<'a>,
    tab_size: usize,
    current: usize,
}

impl<I> Iterator for TruncatedRecordsColumnsIter<'_, I>
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    type Item = SomeString<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.iter.next()?;

        let width = self.width.get(self.current);
        self.current += 1;

        let text_width = string_width_multiline_tab(s.as_ref(), self.tab_size);
        if text_width <= width {
            return Some(SomeString::T(s));
        }

        let text = Truncate::truncate_text(s.as_ref(), width, self.tab_size);
        let text = text.into_owned();
        let text = SomeString::Owned(text);

        Some(text)
    }
}

#[derive(Debug)]
struct ColumnLimitRecords<I> {
    records: I,
    limit: usize,
}

impl ColumnLimitRecords<()> {
    fn new<I>(records: I, limit: usize) -> ColumnLimitRecords<I::IterRows>
    where
        I: IntoRecords,
    {
        ColumnLimitRecords {
            records: records.iter_rows(),
            limit,
        }
    }
}

impl<I> IntoRecords for ColumnLimitRecords<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Cell = <I::Item as IntoIterator>::Item;
    type IterColumns = ColumnLimitRecordsColumnsIter<<I::Item as IntoIterator>::IntoIter>;
    type IterRows = ColumnLimitRecordsIter<I>;

    fn iter_rows(self) -> Self::IterRows {
        ColumnLimitRecordsIter {
            iter: self.records,
            limit: self.limit,
        }
    }
}

pub struct ColumnLimitRecordsIter<I> {
    iter: I,
    limit: usize,
}

impl<I> Iterator for ColumnLimitRecordsIter<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = ColumnLimitRecordsColumnsIter<<I::Item as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.iter.next()?;
        Some(ColumnLimitRecordsColumnsIter {
            iter: iter.into_iter(),
            limit: self.limit,
        })
    }
}

pub struct ColumnLimitRecordsColumnsIter<I> {
    iter: I,
    limit: usize,
}

impl<I> Iterator for ColumnLimitRecordsColumnsIter<I>
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.limit == 0 {
            return None;
        }

        self.limit -= 1;

        self.iter.next()
    }
}
