use core::fmt::{self, Display};
use std::borrow::Borrow;

use serde_json::Value;
use tabled::{
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
// todo: rename to ValueTable.
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
                cfg: ColoredConfig::new(configure_grid(), Default::default()),
                array_orientation: Orientation::Vertical,
                object_orientation: Orientation::Vertical,
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
    ///    assert_eq!(
    ///        table,
    ///        concat!(
    ///             "╔═════╦════╗\n",  
    ///             "║  234║ 123║\n",  
    ///             "║     ╠════╣\n",  
    ///             "║     ║ 234║\n",  
    ///             "║     ╠════╣\n",  
    ///             "║     ║ 456║\n",  
    ///             "╠═════╬════╣\n",  
    ///             "║ key1║ 123║\n",  
    ///             "╠═════╬══╦═╣\n",  
    ///             "║key22║k1║1║\n",  
    ///             "║     ╠══╬═╣\n",  
    ///             "║     ║k2║2║\n",  
    ///             "╚═════╩══╩═╝",
    ///        ),
    ///    );
    /// ```
    ///
    /// [`Table`]: tabled::Table
    pub fn with<O>(&mut self, mut option: O) -> &mut Self
    where
        O: TableOption<EmptyRecords, CompleteDimension<'static>, ColoredConfig>,
    {
        let mut records = EmptyRecords::default();
        let mut dims = CompleteDimension::default();
        option.change(&mut records, &mut self.cfg.cfg, &mut dims);

        self
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
    Vertical,
    /// Horizontal mode (from left to right).
    Horizontal,
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
