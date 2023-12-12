use std::iter::FromIterator;

use tabled::{builder::Builder, grid::config::SpannedConfig, settings::Style, Table};
use toml::{Table as TomlMap, Value};

use super::{Orientation, Settings};

pub(super) fn table(value: &Value, cfg: &Settings) -> String {
    _plain_table(value, cfg, true)
}

fn _plain_table(value: &Value, cfg: &Settings, outer: bool) -> String {
    let config: SpannedConfig = cfg.cfg.into();

    match value {
        Value::Array(arr) => match cfg.array_orientation {
            Orientation::Column => seq_column_table(arr, cfg, config),
            Orientation::Row => seq_row_table(arr, cfg, config),
        },
        Value::Table(map) => match cfg.object_orientation {
            Orientation::Column => map_column_table(map, cfg, config),
            Orientation::Row => map_row_table(map, cfg, config),
        },
        Value::String(string) => string_table(string.to_owned(), config, outer),
        Value::Boolean(boolean) => string_table(boolean.to_string(), config, outer),
        Value::Float(float) => string_table(float.to_string(), config, outer),
        Value::Integer(int) => string_table(int.to_string(), config, outer),
        Value::Datetime(datetime) => string_table(datetime.to_string(), config, outer),
    }
}

fn seq_column_table(arr: &Vec<Value>, cfg: &Settings, config: SpannedConfig) -> String {
    let mut buf = Builder::with_capacity(1, 1);
    for value in arr {
        let val = _plain_table(value, cfg, false);
        buf.push_record([val]);
    }

    buf.build().with(config).to_string()
}

fn seq_row_table(arr: &Vec<Value>, cfg: &Settings, config: SpannedConfig) -> String {
    let mut buf = Vec::with_capacity(arr.len());
    for value in arr {
        let val = _plain_table(value, cfg, false);
        buf.push(val);
    }

    Builder::from(vec![buf]).build().with(config).to_string()
}

fn map_column_table(map: &TomlMap, cfg: &Settings, config: SpannedConfig) -> String {
    let mut buf = Builder::with_capacity(map.len(), 2);
    for (key, value) in map.iter() {
        let val = _plain_table(value, cfg, false);
        buf.push_record([key.to_owned(), val]);
    }

    buf.build().with(config).to_string()
}

fn map_row_table(map: &TomlMap, cfg: &Settings, config: SpannedConfig) -> String {
    let mut keys = Vec::with_capacity(map.len());
    let mut vals = Vec::with_capacity(map.len());
    for (key, value) in map.iter() {
        let val = _plain_table(value, cfg, false);
        vals.push(val);
        keys.push(key.to_owned());
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
