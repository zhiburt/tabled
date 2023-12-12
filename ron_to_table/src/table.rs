use std::{
    cmp,
    iter::{repeat, FromIterator},
};

use ron::{Number, Value};
use tabled::{
    builder::Builder,
    grid::{
        config::{
            AlignmentHorizontal, AlignmentVertical, CompactMultilineConfig, Indent, Sides,
            SpannedConfig,
        },
        dimension::{CompleteDimension, DimensionPriority, PoolTableDimension},
        records::EmptyRecords,
        util::string::{count_lines, get_lines, string_width, string_width_multiline},
    },
    settings::{style::Style, TableOption},
    tables::{PoolTable, TableValue},
    Table,
};

use crate::Orientation;

/// Converter of [`Value`] to a table,
/// with a set of configurations.
#[derive(Debug, Clone)]
pub struct RonTable {
    cfg: CompactMultilineConfig,
    plain: bool,
    object_orientation: Orientation,
    array_orientation: Orientation,
}

impl Default for RonTable {
    fn default() -> Self {
        Self {
            plain: true,
            cfg: configure_grid(),
            array_orientation: Orientation::Column,
            object_orientation: Orientation::Column,
        }
    }
}

impl RonTable {
    /// Creates a default ron configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Collapse tables out instead of tables within tables.
    pub fn collapse(&mut self) -> &mut Self {
        self.plain = false;
        self
    }

    /// Set a table mode for a [`ron::Value::Map`].
    pub fn map_orientation(&mut self, mode: Orientation) -> &mut Self {
        self.object_orientation = mode;
        self
    }

    /// Set a table mode for a [`ron::Value::Seq`].
    pub fn seq_orientation(&mut self, mode: Orientation) -> &mut Self {
        self.array_orientation = mode;
        self
    }

    /// Apply settings to the table.
    pub fn with<O>(&mut self, option: O) -> &mut Self
    where
        O: TableOption<EmptyRecords, CompactMultilineConfig, CompleteDimension<'static>>,
    {
        let mut records = EmptyRecords::default();
        let mut dims = CompleteDimension::default();
        option.change(&mut records, &mut self.cfg, &mut dims);

        self
    }

    /// Build a table.
    pub fn build(&self, value: &Value) -> String {
        match self.plain {
            true => plain_table(value, self),
            false => collapsed_table(value, self),
        }
    }
}

fn configure_grid() -> CompactMultilineConfig {
    let pad = Sides::new(
        Indent::spaced(1),
        Indent::spaced(1),
        Indent::default(),
        Indent::default(),
    );

    let mut cfg = CompactMultilineConfig::new();
    cfg.set_padding(pad);
    cfg.set_alignment_horizontal(AlignmentHorizontal::Left);
    cfg.set_borders(tabled::grid::config::Borders::from(Style::ascii()));

    cfg
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct CollapseCtx {
    map_orientation: Orientation,
    list_orientation: Orientation,
    has_horizontal: bool,
    has_vertical: bool,
    alignment_horizontal: AlignmentHorizontal,
    alignment_vertical: AlignmentVertical,
}

fn collapsed_table(value: &Value, cfg: &RonTable) -> String {
    let ctx = CollapseCtx {
        map_orientation: cfg.object_orientation,
        list_orientation: cfg.array_orientation,
        has_horizontal: cfg.cfg.get_borders().has_top(),
        has_vertical: cfg.cfg.get_borders().has_left(),
        alignment_horizontal: cfg.cfg.get_alignment_horizontal(),
        alignment_vertical: cfg.cfg.get_alignment_vertical(),
    };
    let value = convert_value_to_table_value(value, ctx);

    PoolTable::from(value)
        .with(cfg.cfg)
        .with(PoolTableDimension::new(
            DimensionPriority::Last,
            DimensionPriority::Last,
        ))
        .to_string()
}

fn convert_value_to_table_value(value: &Value, ctx: CollapseCtx) -> TableValue {
    match value {
        Value::Map(map) => match ctx.map_orientation {
            Orientation::Row => convert_map_to_row(map, ctx),
            Orientation::Column => convert_map_to_column(map, ctx),
        },
        Value::Option(opt) => match opt {
            Some(value) => convert_value_to_table_value(value, ctx),
            None => TableValue::Cell(String::new()),
        },
        Value::Seq(list) => convert_list(list, ctx),
        Value::Bool(value) => TableValue::Cell(value.to_string()),
        Value::Char(char) => TableValue::Cell(char.to_string()),
        Value::Number(Number::Integer(number)) => TableValue::Cell(number.to_string()),
        Value::Number(Number::Float(number)) => TableValue::Cell(number.get().to_string()),
        Value::String(text) => TableValue::Cell(text.to_owned()),
        Value::Unit => TableValue::Cell(String::new()),
    }
}

fn convert_list(list: &[Value], ctx: CollapseCtx) -> TableValue {
    let list = list
        .iter()
        .map(|value| convert_value_to_table_value(value, ctx))
        .collect();

    match ctx.list_orientation {
        Orientation::Row => TableValue::Row(list),
        Orientation::Column => TableValue::Column(list),
    }
}

fn convert_map_to_column(map: &ron::Map, ctx: CollapseCtx) -> TableValue {
    let mut keys = map
        .keys()
        .map(|key| convert_value_to_table_value(key, ctx))
        .map(|key| {
            let width = table_value_width(&key, ctx.has_vertical);
            (key, width)
        })
        .collect::<Vec<_>>();

    let key_width = keys.iter().map(|v| v.1).max().unwrap_or(0);

    keys.iter_mut().for_each(|(key, width)| {
        let left = key_width - *width;
        if left > 0 {
            table_value_increase_width(key, left, ctx.alignment_horizontal);
        }
    });

    let data = keys
        .into_iter()
        .zip(map.values())
        .map(|((key, _), value)| (key, convert_value_to_table_value(value, ctx)))
        .map(|(key, value)| TableValue::Row(vec![key, value]))
        .collect();

    TableValue::Column(data)
}

fn convert_map_to_row(map: &ron::Map, ctx: CollapseCtx) -> TableValue {
    let mut keys = map
        .keys()
        .map(|key| convert_value_to_table_value(key, ctx))
        .map(|key| {
            let height = table_value_height(&key, ctx.has_horizontal);
            (key, height)
        })
        .collect::<Vec<_>>();

    let key_height = keys.iter().map(|v| v.1).max().unwrap_or(0);

    keys.iter_mut().for_each(|(key, width)| {
        let left = key_height - *width;
        if left > 0 {
            table_value_increase_height(key, left, ctx.alignment_vertical);
        }
    });

    let data = keys
        .into_iter()
        .zip(map.values())
        .map(|((key, _), value)| (key, convert_value_to_table_value(value, ctx)))
        .map(|(key, value)| TableValue::Column(vec![key, value]))
        .collect();

    TableValue::Row(data)
}

fn table_value_width(value: &TableValue, has_vertical: bool) -> usize {
    match value {
        TableValue::Row(list) => {
            list.iter()
                .map(|value| table_value_width(value, has_vertical))
                .sum::<usize>()
                + (cmp::max(list.len(), 1) - 1) * has_vertical as usize
        }
        TableValue::Column(list) => list
            .iter()
            .map(|value| table_value_width(value, has_vertical))
            .max()
            .unwrap_or(0),
        TableValue::Cell(string) => string_width_multiline(string),
    }
}

fn table_value_height(value: &TableValue, has_horizontal: bool) -> usize {
    match value {
        TableValue::Row(list) => list
            .iter()
            .map(|value| table_value_width(value, has_horizontal))
            .max()
            .unwrap_or(0),
        TableValue::Column(list) => {
            list.iter()
                .map(|value| table_value_height(value, has_horizontal))
                .sum::<usize>()
                + (cmp::max(list.len(), 1) - 1) * has_horizontal as usize
        }
        TableValue::Cell(string) => count_lines(string),
    }
}

fn table_value_increase_width(value: &mut TableValue, by: usize, ah: AlignmentHorizontal) {
    match value {
        TableValue::Row(list) => {
            let mut left = by;
            while left > 0 {
                for value in list.iter_mut() {
                    left -= 1;
                    table_value_increase_width(value, 1, ah);
                }
            }
        }
        TableValue::Column(list) => {
            for value in list.iter_mut() {
                table_value_increase_width(value, by, ah);
            }
        }
        TableValue::Cell(string) => *string = increase_string_width(string, by, ah),
    }
}

fn table_value_increase_height(value: &mut TableValue, by: usize, av: AlignmentVertical) {
    match value {
        TableValue::Row(list) => {
            for value in list.iter_mut() {
                table_value_increase_height(value, by, av);
            }
        }
        TableValue::Column(list) => {
            let mut left = by;
            while left > 0 {
                for value in list.iter_mut() {
                    left -= 1;
                    table_value_increase_height(value, 1, av);
                }
            }
        }
        TableValue::Cell(string) => *string = increase_string_height(string, by, av),
    }
}

fn increase_string_width(text: &str, by: usize, ah: AlignmentHorizontal) -> String {
    let mut out = Vec::new();

    for line in get_lines(text) {
        let w = string_width(&line);
        let (left, right) = indent_horizontal(ah, w + by, w);

        let mut buf = String::new();
        buf.extend(repeat(' ').take(left));
        buf.push_str(&line);
        buf.extend(repeat(' ').take(right));

        out.push(buf);
    }

    out.join("\n")
}

fn increase_string_height(text: &str, by: usize, av: AlignmentVertical) -> String {
    let mut out = Vec::new();

    let count_lines = count_lines(text);

    let (top, bottom) = indent_vertical(av, count_lines + by, count_lines);

    out.extend(repeat(String::new()).take(top));

    for line in get_lines(text) {
        out.push(line.into_owned());
    }

    out.extend(repeat(String::new()).take(bottom));

    out.join("\n")
}

fn indent_vertical(al: AlignmentVertical, available: usize, real: usize) -> (usize, usize) {
    let top = indent_top(al, available, real);
    let bottom = available - real - top;
    (top, bottom)
}

fn indent_horizontal(al: AlignmentHorizontal, available: usize, real: usize) -> (usize, usize) {
    let top = indent_left(al, available, real);
    let right = available - real - top;
    (top, right)
}

fn indent_top(al: AlignmentVertical, available: usize, real: usize) -> usize {
    match al {
        AlignmentVertical::Top => 0,
        AlignmentVertical::Bottom => available - real,
        AlignmentVertical::Center => (available - real) / 2,
    }
}

fn indent_left(al: AlignmentHorizontal, available: usize, real: usize) -> usize {
    match al {
        AlignmentHorizontal::Left => 0,
        AlignmentHorizontal::Right => available - real,
        AlignmentHorizontal::Center => (available - real) / 2,
    }
}

fn plain_table(value: &Value, cfg: &RonTable) -> String {
    _plain_table(value, cfg, true)
}

fn _plain_table(value: &Value, cfg: &RonTable, outer: bool) -> String {
    let config: SpannedConfig = cfg.cfg.into();

    match value {
        Value::Seq(arr) => match cfg.array_orientation {
            Orientation::Column => seq_column_table(arr, cfg, &config),
            Orientation::Row => seq_row_table(arr, cfg, &config),
        },
        Value::Map(map) => match cfg.object_orientation {
            Orientation::Column => map_column_table(map, cfg, &config),
            Orientation::Row => map_row_table(map, cfg, &config),
        },
        Value::Option(opt) => match opt {
            Some(value) => _plain_table(value, cfg, outer),
            None => String::new(),
        },
        Value::Unit => String::new(),
        Value::String(text) => string_table(text.to_owned(), config, outer),
        Value::Bool(val) => string_table(val.to_string(), config, outer),
        Value::Char(char) => string_table(char.to_string(), config, outer),
        Value::Number(Number::Integer(num)) => string_table(num.to_string(), config, outer),
        Value::Number(Number::Float(num)) => string_table(num.get().to_string(), config, outer),
    }
}

fn seq_column_table(arr: &Vec<Value>, cfg: &RonTable, config: &SpannedConfig) -> String {
    let mut buf = Builder::with_capacity(1, 1);
    for value in arr {
        let val = _plain_table(value, cfg, false);
        buf.push_record([val]);
    }

    buf.build().with(config).to_string()
}

fn seq_row_table(arr: &Vec<Value>, cfg: &RonTable, config: &SpannedConfig) -> String {
    let mut buf = Vec::with_capacity(arr.len());
    for value in arr {
        let val = _plain_table(value, cfg, false);
        buf.push(val);
    }

    Builder::from(vec![buf]).build().with(config).to_string()
}

fn map_column_table(map: &ron::Map, cfg: &RonTable, config: &SpannedConfig) -> String {
    let mut buf = Builder::with_capacity(map.len(), 2);
    for (key, value) in map.iter() {
        let key = _plain_table(key, cfg, false);
        let val = _plain_table(value, cfg, false);
        buf.push_record([key, val]);
    }

    buf.build().with(config).to_string()
}

fn map_row_table(map: &ron::Map, cfg: &RonTable, config: &SpannedConfig) -> String {
    let mut keys = Vec::with_capacity(map.len());
    let mut vals = Vec::with_capacity(map.len());
    for (key, value) in map.iter() {
        let key = _plain_table(key, cfg, false);
        let val = _plain_table(value, cfg, false);
        vals.push(val);
        keys.push(key);
    }

    Builder::from(vec![keys, vals])
        .build()
        .with(config)
        .to_string()
}

fn string_table(val: String, config: SpannedConfig, outer: bool) -> String {
    let mut table = Table::from_iter([[val]]);
    table.with(config);

    if !outer {
        table.with(Style::empty());
    }

    table.to_string()
}
