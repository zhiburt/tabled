mod orientation;
mod table_collapsed;
mod table_plain;

pub use orientation::Orientation;

use std::{
    borrow::Borrow,
    fmt::{self, Display, Formatter},
};

use tabled::{
    grid::{
        config::{AlignmentHorizontal, Borders, CompactMultilineConfig, Indent, Sides},
        dimension::CompleteDimension,
        records::EmptyRecords,
    },
    settings::{style::Style, TableOption},
};

use toml::Value;

/// Converter of [`Value`] to a table,
/// with a set of configurations.
#[derive(Debug, Clone)]
pub struct TomlTable<V> {
    value: V,
    settings: Settings,
}

#[derive(Debug, Clone)]
struct Settings {
    cfg: CompactMultilineConfig,
    plain: bool,
    object_orientation: Orientation,
    array_orientation: Orientation,
}

impl<V> TomlTable<V>
where
    V: Borrow<Value>,
{
    /// Creates a default table configuration.
    pub fn new(value: V) -> Self {
        Self {
            value,
            settings: Settings {
                plain: true,
                cfg: configure_grid(),
                array_orientation: Orientation::Column,
                object_orientation: Orientation::Column,
            },
        }
    }

    /// Collapse tables out instead of tables within tables.
    pub fn collapse(&mut self) -> &mut Self {
        self.settings.plain = false;
        self
    }

    /// Set a table mode for a [`Value::Table`].
    pub fn map_orientation(&mut self, mode: Orientation) -> &mut Self {
        self.settings.object_orientation = mode;
        self
    }

    /// Set a table mode for a [`Value::Array`].
    pub fn seq_orientation(&mut self, mode: Orientation) -> &mut Self {
        self.settings.array_orientation = mode;
        self
    }

    /// Apply settings to the table.
    pub fn with<O>(&mut self, option: O) -> &mut Self
    where
        O: TableOption<EmptyRecords, CompactMultilineConfig, CompleteDimension<'static>>,
    {
        let mut records = EmptyRecords::default();
        let mut dims = CompleteDimension::default();
        option.change(&mut records, &mut self.settings.cfg, &mut dims);

        self
    }
}

impl<V> Display for TomlTable<V>
where
    V: Borrow<Value>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value = self.value.borrow();
        let table = match self.settings.plain {
            true => table_plain::table(value, &self.settings),
            false => table_collapsed::table(value, &self.settings),
        };

        table.fmt(f)
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
    cfg.set_borders(Borders::from(Style::ascii()));

    cfg
}
