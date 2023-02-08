mod dimension;
mod utf8_writer;

use std::{borrow::Cow, cmp, fmt, io};

use crate::{
    grid::config::AlignmentHorizontal,
    grid::{
        config::{Entity, Formatting, GridConfig, Indent, Padding},
        dimension::ExactDimension,
        Grid,
    },
    records::{
        into_records::{
            truncate_records::Width, BufRecords, BufRecords2, ColumnLimitRecords, RowLimitRecords,
            TruncatedRecords,
        },
        IntoRecords, IterRecords,
    },
    settings::{style::Style, TableOption},
};

use self::{dimension::IterTableDimension, utf8_writer::UTF8Writer};

#[derive(Debug, Clone)]
pub struct IterTable<I> {
    records: I,
    cfg: GridConfig,
    table: TableConfig,
}

#[derive(Debug, Clone)]
struct TableConfig {
    sniff: usize,
    height: usize,
    width: Option<usize>,
    count_columns: Option<usize>,
    count_rows: Option<usize>,
}

impl<I> IterTable<I> {
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
                width: None,
                count_columns: None,
                count_rows: None,
            },
        }
    }

    /// With is a generic function which applies options to the [`Table`].
    pub fn with<O>(mut self, option: O) -> Self
    where
        for<'a> O: TableOption<IterRecords<&'a I>, IterTableDimension<'static>>,
    {
        let mut dimension = IterTableDimension::new(
            Width::Exact(self.table.width.unwrap_or(0)),
            self.table.height,
        );

        let mut records = IterRecords::new(
            &self.records,
            self.table.count_columns.unwrap_or(0),
            self.table.count_rows,
        );

        let mut option = option;
        option.change(&mut records, &mut self.cfg, &mut dimension);

        self
    }

    pub fn cols(mut self, count_columns: usize) -> Self {
        self.table.count_columns = Some(count_columns);
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
        self.table.width = Some(size);
        self
    }

    pub fn fmt<W: fmt::Write>(self, writer: W) -> fmt::Result
    where
        I: IntoRecords,
    {
        let mut config = self.cfg;
        clean_config(&mut config);

        build_grid(writer, self.records, &config, &self.table)
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
        let writer = UTF8Writer::new(writer);
        self.fmt(writer)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }
}

fn build_grid<W: fmt::Write, I: IntoRecords>(
    writer: W,
    records: I,
    config: &GridConfig,
    iter_cfg: &TableConfig,
) -> Result<(), fmt::Error> {
    let tab_size = config.get_tab_width();
    let count_rows = iter_cfg.count_rows;

    let padding = config.get_padding(Entity::Global);
    let padding = padding.left.size + padding.right.size;

    let dont_sniff = iter_cfg.width.is_some() && iter_cfg.count_columns.is_some();
    if dont_sniff {
        let width = iter_cfg.width.unwrap();
        let count_columns = iter_cfg.count_columns.unwrap();

        let dims_width = Width::Exact(cmp::max(width, padding));
        let dimension = IterTableDimension::new(dims_width, iter_cfg.height);

        let width = width.saturating_sub(padding);
        let content_width = Width::Exact(width);

        match count_rows {
            Some(limit) => {
                let records = RowLimitRecords::new(records, limit);
                let records =
                    build_records(records, content_width, count_columns, count_rows, tab_size);
                return Grid::new(records, config, &dimension).build(writer);
            }
            None => {
                let records =
                    build_records(records, content_width, count_columns, count_rows, tab_size);
                return Grid::new(records, config, &dimension).build(writer);
            }
        }
    }

    let records = BufRecords::new(records, iter_cfg.sniff);
    let records = BufRecords2::from(records);

    let count_columns = match iter_cfg.count_columns {
        Some(size) => size,
        None => records
            .as_slice()
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or(0),
    };

    #[allow(unused_assignments)]
    let mut content_width = Vec::new();
    #[allow(unused_assignments)]
    let mut dims_width = Vec::new();

    let (contentw, dimsw) = match iter_cfg.width {
        Some(width) => {
            let contentwidth = width.saturating_sub(padding);
            let dims_width = cmp::max(width, padding);

            (Width::Exact(contentwidth), Width::Exact(dims_width))
        }
        None => {
            let records = ColumnLimitRecords::new(records.as_slice(), count_columns);
            let records = IterRecords::new(records, count_columns, None);
            let width = ExactDimension::width(records, config);

            dims_width = width.iter().map(|i| cmp::max(*i, padding)).collect();
            content_width = width.iter().map(|i| i.saturating_sub(padding)).collect();

            (
                Width::List(Cow::Borrowed(&content_width)),
                Width::List(Cow::Borrowed(&dims_width)),
            )
        }
    };

    let dimension = IterTableDimension::new(dimsw, iter_cfg.height);

    match count_rows {
        Some(limit) => {
            let records = RowLimitRecords::new(records, limit);
            let records = build_records(records, contentw, count_columns, count_rows, tab_size);
            Grid::new(records, &config, &dimension).build(writer)
        }
        None => {
            let records = build_records(records, contentw, count_columns, count_rows, tab_size);
            Grid::new(records, &config, &dimension).build(writer)
        }
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

fn build_records<I: IntoRecords>(
    records: I,
    width: Width<'_>,
    count_columns: usize,
    count_rows: Option<usize>,
    tab_size: usize,
) -> IterRecords<ColumnLimitRecords<TruncatedRecords<'_, I>>> {
    let records = TruncatedRecords::new(records, width, tab_size);
    let records = ColumnLimitRecords::new(records, count_columns);
    IterRecords::new(records, count_columns, count_rows)
}
