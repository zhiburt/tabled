use core::fmt::{self, Display};
use std::borrow::Borrow;

use serde_json::Value;
use tabled::{
    builder::Builder,
    grid::{
        config::{AlignmentHorizontal, ColoredConfig, Entity, Indent, Sides, SpannedConfig},
        dimension::CompleteDimension,
        records::EmptyRecords,
    },
    settings::{style::Style, TableOption},
    Table,
};

mod collapsed_table;
mod plain_table;

/// Converter of [`Value`] to a table,
/// with a set of configurations.
#[derive(Debug, Clone)]
pub struct JsonTable<T> {
    value: T,
    cfg: Config,
}

impl<T> JsonTable<T> {
    /// Creates a new [`JsonTable`] object.
    pub fn new(value: T) -> Self {
        JsonTable {
            value,
            cfg: Config {
                plain: true,
                cfg: ColoredConfig::new(configure_grid()),
                array_orientation: Orientation::Column,
                object_orientation: Orientation::Column,
            },
        }
    }

    /// Collapse tables out instead of tables within tables.
    pub fn collapse(&mut self) -> &mut Self {
        self.cfg.plain = false;
        self
    }

    /// Set a table mode for a [`serde_json::Value::Object`].
    pub fn object_orientation(&mut self, mode: Orientation) -> &mut Self {
        self.cfg.object_orientation = mode;
        self
    }

    /// Set a table mode for a [`serde_json::Value::Array`].
    pub fn array_orientation(&mut self, mode: Orientation) -> &mut Self {
        self.cfg.array_orientation = mode;
        self
    }

    /// Set a config which will be used.
    ///
    /// You can obtain a config from a [`Table`].
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use json_to_table::json_to_table;
    /// use tabled::{
    ///     settings::{Alignment, Padding, Style},
    ///     Table
    /// };
    ///
    /// let value = json!({
    ///     "key1": 123,
    ///     "234": ["123", "234", "456"],
    ///     "key22": {
    ///         "k1": 1,
    ///         "k2": 2,
    ///     }
    /// });
    ///
    /// let table = json_to_table(&value)
    ///     .with(Padding::zero())
    ///     .with(Alignment::right())
    ///     .with(Style::extended())
    ///     .collapse()
    ///     .to_string();
    ///
    /// println!("{}", table);
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///          "╔═════╦════╗\n",  
    ///          "║  234║ 123║\n",  
    ///          "║     ╠════╣\n",  
    ///          "║     ║ 234║\n",  
    ///          "║     ╠════╣\n",  
    ///          "║     ║ 456║\n",  
    ///          "╠═════╬════╣\n",  
    ///          "║ key1║ 123║\n",  
    ///          "╠═════╬══╦═╣\n",  
    ///          "║key22║k1║1║\n",  
    ///          "║     ╠══╬═╣\n",  
    ///          "║     ║k2║2║\n",  
    ///          "╚═════╩══╩═╝",
    ///     ),
    /// );
    /// ```
    ///
    /// [`Table`]: tabled::Table
    pub fn with<O>(&mut self, option: O) -> &mut Self
    where
        O: TableOption<EmptyRecords, CompleteDimension<'static>, ColoredConfig>,
    {
        let mut records = EmptyRecords::default();
        let mut dims = CompleteDimension::default();
        option.change(&mut records, &mut self.cfg.cfg, &mut dims);

        self
    }

    /// Convert the table into a [`Table`].
    ///
    /// It does not recognizes collapsed mode.
    /// 
    /// 
    /// ```
    /// use tabled::settings::style::Style;
    /// use json_to_table::Orientation;
    /// 
    /// let json = serde_json::json!({
    ///     "key1": "value1",
    ///     "key2": {
    ///         "key1": 123,
    ///         "key2": [1, 2, 3, 4, 5],
    ///     },
    ///     "key3": [
    ///         {"key": 123.3},
    ///         2,
    ///         "asd"
    ///     ],
    ///     "key4": 1234.567 
    /// });
    ///
    /// let table = json_to_table::json_to_table(&json)
    ///     .with(Style::modern())
    ///     .array_orientation(Orientation::Row)
    ///     .into_table()
    ///     .with(Style::markdown())
    ///     .to_string();
    /// 
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "| key1 | ┌────────┐                                 |\n",
    ///         "|      | │ value1 │                                 |\n",
    ///         "|      | └────────┘                                 |\n",
    ///         "|------|--------------------------------------------|\n",
    ///         "| key2 | ┌──────┬─────────────────────────────────┐ |\n",
    ///         "|      | │ key1 │  123                            │ |\n",
    ///         "|      | ├──────┼─────────────────────────────────┤ |\n",
    ///         "|      | │ key2 │ ┌─────┬─────┬─────┬─────┬─────┐ │ |\n",
    ///         "|      | │      │ │  1  │  2  │  3  │  4  │  5  │ │ |\n",
    ///         "|      | │      │ └─────┴─────┴─────┴─────┴─────┘ │ |\n",
    ///         "|      | └──────┴─────────────────────────────────┘ |\n",
    ///         "| key3 | ┌───────────────────┬─────┬───────┐        |\n",
    ///         "|      | │ ┌─────┬─────────┐ │  2  │  asd  │        |\n",
    ///         "|      | │ │ key │  123.3  │ │     │       │        |\n",
    ///         "|      | │ └─────┴─────────┘ │     │       │        |\n",
    ///         "|      | └───────────────────┴─────┴───────┘        |\n",
    ///         "| key4 | ┌──────────┐                               |\n",
    ///         "|      | │ 1234.567 │                               |\n",
    ///         "|      | └──────────┘                               |",
    ///     ),
    /// )
    /// ```
    pub fn into_table(&self) -> Table
    where
        T: Borrow<Value>,
    {
        match self.value.borrow() {
            Value::Array(array) => {
                let list = array
                    .iter()
                    .map(|value| json_to_table(value, &self.cfg))
                    .collect::<Vec<_>>();

                match self.cfg.array_orientation {
                    Orientation::Row => Builder::from(vec![list]).build(),
                    Orientation::Column => {
                        let list = list
                            .into_iter()
                            .map(|value| vec![value])
                            .collect::<Vec<_>>();
                        Builder::from(list).build()
                    }
                }
            }
            Value::Object(map) => {
                let list = map
                    .iter()
                    .map(|(key, value)| vec![key.clone(), json_to_table(value, &self.cfg)])
                    .collect::<Vec<_>>();

                match self.cfg.object_orientation {
                    Orientation::Row => {
                        let (keys, values) = list.into_iter().fold(
                            (Vec::with_capacity(map.len()), Vec::with_capacity(map.len())),
                            |(mut keys, mut values), mut row| {
                                let value = row.pop().unwrap();
                                let key = row.pop().unwrap();
                                keys.push(key);
                                values.push(value);

                                (keys, values)
                            },
                        );
                        let list = vec![keys, values];

                        Builder::from(list).build()
                    }
                    Orientation::Column => Builder::from(list).build(),
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
                let value = json_to_table(self.value.borrow(), &self.cfg);
                Builder::from(vec![vec![value]]).build()
            }
        }
    }
}

impl<T> Display for JsonTable<T>
where
    T: Borrow<Value>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let table = json_to_table(self.value.borrow(), &self.cfg);
        table.fmt(f)
    }
}

#[derive(Debug, Clone)]
struct Config {
    plain: bool,
    cfg: ColoredConfig,
    object_orientation: Orientation,
    array_orientation: Orientation,
}

/// The structure represents a table mode for a given entity,
/// either it will be rendered vertically or horizontally.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Orientation {
    /// Vertical mode (from top to bottom).
    Row,
    /// Horizontal mode (from left to right).
    Column,
}

fn configure_grid() -> SpannedConfig {
    let mut cfg = SpannedConfig::default();
    cfg.set_padding(
        Entity::Global,
        Sides::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::default(),
            Indent::default(),
        ),
    );
    cfg.set_alignment_horizontal(Entity::Global, AlignmentHorizontal::Left);
    cfg.set_borders(*Style::ascii().get_borders());

    cfg
}

fn json_to_table(value: &Value, cfg: &Config) -> String {
    match cfg.plain {
        true => plain_table::plain_table(value, cfg),
        false => collapsed_table::collapsed_table(value, cfg),
    }
}
