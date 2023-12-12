use std::iter::FromIterator;

use tabled::builder::Builder;

use super::*;

pub(super) fn plain_table(v: &Value, cfg: &Config) -> String {
    _plain_table(v, cfg, true)
}

fn _plain_table(v: &Value, cfg: &Config, outer: bool) -> String {
    match v {
        Value::Array(arr) => match cfg.array_orientation {
            Orientation::Column => {
                let mut buf = Builder::with_capacity(1, 1);
                for value in arr {
                    let val = _plain_table(value, cfg, false);
                    buf.push_record([val]);
                }

                buf.build().with(cfg.cfg.clone()).to_string()
            }
            Orientation::Row => {
                let mut buf = Vec::with_capacity(arr.len());
                for value in arr {
                    let val = _plain_table(value, cfg, false);
                    buf.push(val);
                }

                Builder::from(vec![buf])
                    .build()
                    .with(cfg.cfg.clone())
                    .to_string()
            }
        },
        Value::Object(map) => match cfg.object_orientation {
            Orientation::Column => {
                let mut buf = Builder::with_capacity(map.len(), 2);
                for (key, value) in map {
                    let val = _plain_table(value, cfg, false);
                    buf.push_record([key.clone(), val]);
                }

                buf.build().with(cfg.cfg.clone()).to_string()
            }
            Orientation::Row => {
                let mut keys = Vec::with_capacity(map.len());
                let mut vals = Vec::with_capacity(map.len());
                for (key, value) in map {
                    let val = _plain_table(value, cfg, false);
                    vals.push(val);
                    keys.push(key.clone());
                }

                Builder::from(vec![keys, vals])
                    .build()
                    .with(cfg.cfg.clone())
                    .to_string()
            }
        },
        Value::Null => String::new(),
        value => {
            let val = match value {
                Value::String(text) => text.to_owned(),
                Value::Bool(val) => val.to_string(),
                Value::Number(num) => num.to_string(),
                _ => unreachable!(),
            };

            let mut table = Table::from_iter([[val]]);
            table.with(cfg.cfg.clone());

            if !outer {
                table.with(Style::empty());
            }

            table.to_string()
        }
    }
}
