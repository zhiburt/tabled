use core::fmt::{self, Display};

use serde_json::Value;
use tabled::{papergrid::GridConfig, style::RawStyle, Style, Table};

/// Converter of [`Value`] to a table,
/// with a set of configurations.
#[derive(Debug, Clone)]
pub struct JsonTable<'a> {
    value: &'a Value,
    cfg: Config,
}

#[derive(Debug, Clone, Default)]
struct Config {
    plain: bool,
    style: Option<RawStyle>,
    cfg: Option<GridConfig>,
}

impl JsonTable<'_> {
    /// Creates a new [`JsonTable`] object.
    pub fn new(value: &Value) -> JsonTable<'_> {
        JsonTable {
            value,
            cfg: Config {
                plain: true,
                style: None,
                cfg: None,
            },
        }
    }

    /// Set a style which will be used,
    /// default is [`Style::ascii`].
    pub fn set_style(&mut self, style: impl Into<RawStyle>) -> &mut Self {
        self.cfg.style = Some(style.into());
        self
    }

    /// Collapse tables out instead of tables within tables.
    pub fn collapse(&mut self) -> &mut Self {
        self.cfg.plain = false;
        self
    }

    /// Set a config which will be used.
    ///
    /// You can obtain a config from a [`Table`].
    ///
    /// # Example
    ///
    /// ```
    /// use json_to_table::json_to_table;
    /// use serde_json::json;
    /// use tabled::{Alignment, Padding, Style, Table};
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
    /// let cfg = Table::new([""])
    ///     .with(Padding::zero())
    ///     .with(Alignment::right())
    ///     .with(Style::extended())
    ///     .get_config()
    ///     .clone();
    ///
    /// let table = json_to_table(&value)
    ///     .set_config(cfg)
    ///     .collapse()
    ///     .to_string();
    ///
    /// println!("{}", table);
    ///
    ///    assert_eq!(
    ///        table,
    ///        concat!(
    ///             "╔═══════╦══════╗\n",  
    ///             "║    234║   123║\n",  
    ///             "║       ╠══════╣\n",  
    ///             "║       ║   234║\n",  
    ///             "║       ╠══════╣\n",  
    ///             "║       ║   456║\n",  
    ///             "╠═══════╬══════╣\n",  
    ///             "║   key1║   123║\n",  
    ///             "╠═══════╬════╦═╣\n",  
    ///             "║  key22║  k1║1║\n",  
    ///             "║       ╠════╬═╣\n",  
    ///             "║       ║  k2║2║\n",  
    ///             "╚═══════╩════╩═╝",
    ///        ),
    ///    );
    /// ```
    ///
    /// [`Table`]: tabled::Table
    pub fn set_config(&mut self, cfg: GridConfig) -> &mut Self {
        self.cfg.cfg = Some(cfg);
        self
    }
}

impl Display for JsonTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let table = json_to_table::json_to_table(self.value, &self.cfg);
        table.fmt(f)
    }
}

impl From<JsonTable<'_>> for Table {
    fn from(t: JsonTable<'_>) -> Self {
        json_to_table::json_to_table(t.value, &t.cfg)
    }
}

mod json_to_table {
    use std::cmp;

    use tabled::{
        builder::Builder,
        col,
        papergrid::{records::Records, util::string_width_multiline},
        Height, Padding, TableOption, Width,
    };

    use super::*;

    pub(super) fn json_to_table(value: &Value, cfg: &Config) -> Table {
        if cfg.plain {
            json_to_table_f(value, cfg.style.as_ref(), cfg.cfg.as_ref(), true)
        } else {
            json_to_table_r(
                value,
                cfg.style.as_ref(),
                cfg.cfg.as_ref(),
                0,
                0,
                true,
                true,
                false,
                false,
                &[],
                None,
            )
        }
    }

    fn json_to_table_f(
        v: &Value,
        style: Option<&RawStyle>,
        cfg: Option<&GridConfig>,
        outer: bool,
    ) -> Table {
        match v {
            Value::Null => {
                let mut table = col![];
                set_table_style(&mut table, style, cfg);

                if !outer {
                    table.with(Style::empty());
                }

                table
            }
            Value::Bool(b) => {
                let mut table = col![b];
                set_table_style(&mut table, style, cfg);

                if !outer {
                    table.with(Style::empty());
                }

                table
            }
            Value::Number(n) => {
                let mut table = col![n];
                set_table_style(&mut table, style, cfg);

                if !outer {
                    table.with(Style::empty());
                }

                table
            }
            Value::String(s) => {
                let mut table = col![s];
                set_table_style(&mut table, style, cfg);

                if !outer {
                    table.with(Style::empty());
                }

                table
            }
            Value::Array(arr) => {
                let mut builder = Builder::new();
                for value in arr {
                    let val = json_to_table_f(value, style, cfg, false).to_string();
                    builder.add_record([val]);
                }

                let mut table = builder.build();
                set_table_style(&mut table, style, cfg);

                table
            }
            Value::Object(map) => {
                let mut builder = Builder::new();
                for (key, value) in map {
                    let val = json_to_table_f(value, style, cfg, false).to_string();
                    builder.add_record([key.clone(), val]);
                }

                let mut table = builder.build();
                set_table_style(&mut table, style, cfg);

                table
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn json_to_table_r(
        value: &Value,
        style: Option<&RawStyle>,
        cfg: Option<&GridConfig>,
        row: usize,
        column: usize,
        is_last: bool,
        is_prev_row_last: bool,
        is_in_list: bool,
        change_key_split: bool,
        used_splits: &[usize],
        width: Option<usize>,
    ) -> Table {
        match value {
            Value::String(..) | Value::Bool(..) | Value::Number(..) | Value::Null => {
                let mut table = match value {
                    Value::String(s) => col![s],
                    Value::Bool(b) => col![b],
                    Value::Number(n) => col![n],
                    Value::Null => col![""],
                    _ => unreachable!(),
                };

                set_table_style(&mut table, style, cfg);

                table.with(Width::increase(width.unwrap_or(0)));
                table.with(SetBottomChars(
                    used_splits,
                    table
                        .get_config()
                        .get_borders()
                        .top_intersection
                        .unwrap_or(' '),
                ));

                table
            }
            Value::Object(obj) => {
                let map_length = obj.len();

                // we build a table here to not make any assumptions about style.
                // but we could to reduce allocations.
                let max_keys_width = obj
                    .iter()
                    .map(|(key, _)| col![key].with(NoRightBorders).total_width())
                    .max()
                    .unwrap_or(0);

                let width = match width {
                    Some(width) => width,
                    None => {
                        // build dummy table
                        let map = obj.iter().enumerate().map(|(i, (key, value))| {
                            let is_last = is_last && i + 1 == map_length;

                            let mut key = col![key];
                            key.with(NoRightBorders);

                            let value = json_to_table_r(
                                value,
                                style,
                                cfg,
                                row,
                                column + 2,
                                is_last,
                                i + 1 == map_length,
                                false,
                                false,
                                &[],
                                None,
                            );

                            (key, value)
                        });

                        // need to rebuild the values with a known width
                        let width = map
                            .into_iter()
                            .map(|(_, value)| value.total_width())
                            .max()
                            .unwrap_or(0);

                        width + max_keys_width
                    }
                };

                let mut builder = Builder::new();
                let mut iter = obj.iter().enumerate().peekable();
                while let Some((i, value)) = iter.next() {
                    let row = row + i;
                    let (key, value) = value;

                    let mut was_intersection_touched = false;
                    let intersections = if i + 1 < map_length {
                        let (_, (_, value)) = iter.peek().unwrap();
                        find_top_intersection(value)
                    } else {
                        let mut splits = used_splits.to_owned();
                        if !splits.is_empty() {
                            let mut current_width = 0;
                            while !splits.is_empty() {
                                current_width += splits[0];
                                if current_width >= max_keys_width {
                                    splits[0] = current_width - max_keys_width;
                                    break;
                                }

                                splits.remove(0);
                                current_width += 1;

                                // means we must change a split char
                                if current_width == max_keys_width {
                                    was_intersection_touched = true;
                                    break;
                                }
                            }
                        }

                        splits
                    };

                    let is_last = is_last && i + 1 == map_length;
                    let width = width - max_keys_width;
                    let mut value = json_to_table_r(
                        value,
                        style,
                        cfg,
                        row,
                        column + 2,
                        is_last,
                        i + 1 == map_length,
                        false,
                        was_intersection_touched,
                        &intersections,
                        Some(width),
                    );
                    {
                        value.with(TopLeftChangeSplit);

                        if row != 0 {
                            value.with(NoTopBorders);
                        }

                        if !is_last {
                            value.with(BottomRightChangeToRight);
                        }

                        if i + 1 == map_length {
                            value.with(BottomLeftChangeToBottomIntersection);
                        } else {
                            value.with(BottomLeftChangeSplitToIntersection);
                        }

                        if was_intersection_touched {
                            value.with(BottomLeftChangeSplitToIntersection);
                        }
                    }

                    let mut key = col![key];
                    set_table_style(&mut key, style, cfg);

                    let top_intersection = key
                        .get_config()
                        .get_borders()
                        .top_intersection
                        .unwrap_or(' ');

                    {
                        key.with(NoRightBorders);

                        if row != 0 {
                            key.with(NoTopBorders);
                        }

                        if row == 0 && column != 0 {
                            key.with(TopLeftChangeSplit);
                        }

                        if column > 0 {
                            if i + 1 == map_length {
                                if is_in_list {
                                    if is_last {
                                        key.with(BottomLeftChangeToBottomIntersection);
                                    } else if is_prev_row_last {
                                        key.with(BottomLeftChangeSplitToIntersection);
                                    } else {
                                        key.with(BottomLeftChangeSplit);
                                    }
                                } else if is_prev_row_last {
                                    key.with(BottomLeftChangeToBottomIntersection);
                                } else {
                                    key.with(BottomLeftChangeSplitToIntersection);
                                }
                            } else {
                                key.with(BottomLeftChangeSplit);
                            }
                        } else if !is_last {
                            key.with(BottomLeftChangeSplit);
                        }

                        if change_key_split {
                            key.with(BottomLeftChangeSplitToIntersection);
                        }
                    }

                    {
                        let value_height = value.total_height();

                        key.with(Width::increase(max_keys_width))
                            .with(Height::increase(value_height));
                    }

                    {
                        // set custom chars
                        if i + 1 == map_length {
                            // set for the key
                            key.with(SetBottomChars(used_splits, top_intersection));
                        }
                    }

                    builder.add_record([key.to_string(), value.to_string()]);
                }

                let mut table = builder.build();
                table.with(Style::empty()).with(Padding::zero());
                table
            }
            Value::Array(list) => {
                let width = match width {
                    Some(width) => width,
                    None => {
                        // build a dummy tables
                        let list = list.iter().enumerate().map(|(i, value)| {
                            let is_last = is_last && i + 1 == list.len();
                            json_to_table_r(
                                value,
                                style,
                                cfg,
                                row,
                                column,
                                is_last,
                                i + 1 == list.len(),
                                true,
                                false,
                                &[],
                                None,
                            )
                        });

                        // need to rebuild the values with a known width
                        list.into_iter()
                            .map(|value| value.total_width())
                            .max()
                            .unwrap_or(0)
                    }
                };

                let map_length = list.len();
                let mut builder = Builder::new();
                for (i, value) in list.iter().enumerate() {
                    let row = row + i;

                    let intersections = if i + 1 < map_length {
                        let value = &list[i + 1];
                        find_top_intersection(value)
                    } else {
                        used_splits.to_owned()
                    };

                    let is_last = is_last && i + 1 == map_length;

                    let mut is_last_in_list = i + 1 == list.len();
                    if is_in_list {
                        is_last_in_list = is_last_in_list && is_prev_row_last;
                    }

                    let mut value = json_to_table_r(
                        value,
                        style,
                        cfg,
                        row,
                        column,
                        is_last,
                        is_last_in_list,
                        true,
                        false,
                        &intersections,
                        Some(width),
                    );

                    if column != 0 {
                        value.with(TopLeftChangeSplit);
                    }

                    if row > 0 {
                        value.with(NoTopBorders);
                    }

                    if !is_last {
                        value.with(BottomRightChangeToRight);
                    }

                    if i + 1 < map_length {
                        value.with(BottomLeftChangeSplit);
                    }

                    if i + 1 == map_length && !is_last {
                        value.with(BottomLeftChangeSplitToIntersection);
                    }

                    if i + 1 == map_length && !is_last && is_prev_row_last {
                        value.with(BottomLeftChangeToBottomIntersection);
                    }

                    if column == 0 && !is_last {
                        value.with(BottomLeftChangeSplit);
                    }

                    if is_last && column != 0 {
                        value.with(BottomLeftChangeToBottomIntersection);
                    }

                    value.with(Width::increase(width));

                    builder.add_record([value.to_string()]);
                }

                let mut table = builder.build();
                table.with(Style::empty()).with(Padding::zero());
                table
            }
        }
    }

    fn find_top_intersection(table: &Value) -> Vec<usize> {
        let mut intersections = Vec::new();
        find_top_intersection_r(table, &mut intersections);

        intersections
    }

    fn find_top_intersection_r(table: &Value, chars: &mut Vec<usize>) {
        match table {
            Value::String(_) | Value::Bool(_) | Value::Number(_) | Value::Null => (),
            Value::Object(m) => {
                if m.is_empty() {
                    return;
                }

                let mut max_keys_width = 0;
                for (key, _) in m.iter() {
                    let width = string_width_multiline(key) + 2; // + padding
                    max_keys_width = cmp::max(max_keys_width, width);
                }

                chars.push(max_keys_width);

                let (_, value) = m.iter().next().unwrap();
                find_top_intersection_r(value, chars);
            }
            Value::Array(list) => {
                if let Some(value) = list.first() {
                    find_top_intersection_r(value, chars);
                }
            }
        }
    }

    fn set_table_style(table: &mut Table, style: Option<&RawStyle>, cfg: Option<&GridConfig>) {
        if let Some(cfg) = cfg {
            *table.get_config_mut() = cfg.clone();
        }

        if let Some(style) = style {
            table.with(style);
        }
    }

    struct NoTopBorders;

    impl<R> TableOption<R> for NoTopBorders {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.top = None;
            borders.top_intersection = None;
            borders.top_left = None;
            borders.top_right = None;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct NoRightBorders;

    impl<R> TableOption<R> for NoRightBorders {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.top_right = None;
            borders.bottom_right = None;
            borders.vertical_right = None;
            borders.horizontal_right = None;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct TopLeftChangeSplit;

    impl<R> TableOption<R> for TopLeftChangeSplit {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.top_left = borders.top_intersection;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct BottomLeftChangeSplit;

    impl<R> TableOption<R> for BottomLeftChangeSplit {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom_left = borders.horizontal_left;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct BottomLeftChangeSplitToIntersection;

    impl<R> TableOption<R> for BottomLeftChangeSplitToIntersection {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom_left = borders.intersection;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct BottomRightChangeToRight;

    impl<R> TableOption<R> for BottomRightChangeToRight {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom_right = borders.horizontal_right;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct BottomLeftChangeToBottomIntersection;

    impl<R> TableOption<R> for BottomLeftChangeToBottomIntersection {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom_left = borders.bottom_intersection;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct SetBottomChars<'a>(&'a [usize], char);

    impl<R> TableOption<R> for SetBottomChars<'_>
    where
        R: Records,
    {
        fn change(&mut self, table: &mut Table<R>) {
            let split_char = self.1;

            let table_width = table.total_width();
            let mut current_width = 0;
            for pos in self.0 {
                current_width += pos;
                if current_width > table_width {
                    break;
                }

                table.get_config_mut().override_horizontal_border(
                    (1, 0),
                    split_char,
                    tabled::papergrid::Offset::Begin(current_width),
                );

                current_width += 1;
            }
        }
    }
}
