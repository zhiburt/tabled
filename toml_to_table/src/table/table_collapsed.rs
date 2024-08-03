use std::iter::repeat;

use tabled::{
    grid::{
        config::{AlignmentHorizontal, AlignmentVertical},
        dimension::{DimensionPriority, PoolTableDimension},
        util::string::{count_lines, get_line_width, get_lines, get_text_width},
    },
    tables::{PoolTable, TableValue},
};

use toml::{Table as TomlMap, Value};

use super::{Orientation, Settings};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct CollapseCtx {
    map_orientation: Orientation,
    list_orientation: Orientation,
    has_horizontal: bool,
    has_vertical: bool,
    alignment_horizontal: AlignmentHorizontal,
    alignment_vertical: AlignmentVertical,
}

pub(super) fn table(value: &Value, cfg: &Settings) -> String {
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
        Value::Table(map) => match ctx.map_orientation {
            Orientation::Row => convert_map_to_row(map, ctx),
            Orientation::Column => convert_map_to_column(map, ctx),
        },
        Value::Array(list) => convert_list(list, ctx),
        Value::Boolean(boolean) => TableValue::Cell(boolean.to_string()),
        Value::Float(float) => TableValue::Cell(float.to_string()),
        Value::Integer(int) => TableValue::Cell(int.to_string()),
        Value::String(text) => TableValue::Cell(text.to_owned()),
        Value::Datetime(datetime) => TableValue::Cell(datetime.to_string()),
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

fn convert_map_to_column(map: &TomlMap, ctx: CollapseCtx) -> TableValue {
    let mut keys = map
        .keys()
        .map(|key| key.to_string())
        .map(|key| {
            let width = get_text_width(&key);
            (key, width)
        })
        .collect::<Vec<_>>();

    let key_width = keys.iter().map(|v| v.1).max().unwrap_or(0);

    keys.iter_mut().for_each(|(key, width)| {
        let left = key_width - *width;
        if left > 0 {
            *key = increase_string_width(key, left, ctx.alignment_horizontal);
        }
    });

    let data = keys
        .into_iter()
        .zip(map.values())
        .map(|((key, _), value)| (key, convert_value_to_table_value(value, ctx)))
        .map(|(key, value)| TableValue::Row(vec![TableValue::Cell(key), value]))
        .collect();

    TableValue::Column(data)
}

fn convert_map_to_row(map: &TomlMap, ctx: CollapseCtx) -> TableValue {
    let mut keys = map
        .keys()
        .map(|key| key.to_string())
        .map(|key| {
            let height = count_lines(&key);
            (key, height)
        })
        .collect::<Vec<_>>();

    let key_height = keys.iter().map(|v| v.1).max().unwrap_or(0);

    keys.iter_mut().for_each(|(key, width)| {
        let left = key_height - *width;
        if left > 0 {
            *key = increase_string_height(key, left, ctx.alignment_vertical)
        }
    });

    let data = keys
        .into_iter()
        .zip(map.values())
        .map(|((key, _), value)| (key, convert_value_to_table_value(value, ctx)))
        .map(|(key, value)| TableValue::Column(vec![TableValue::Cell(key), value]))
        .collect();

    TableValue::Row(data)
}

fn increase_string_width(text: &str, by: usize, ah: AlignmentHorizontal) -> String {
    let mut out = Vec::new();

    for line in get_lines(text) {
        let width = get_line_width(&line);
        let (left, right) = indent_horizontal(ah, width + by, width);

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
