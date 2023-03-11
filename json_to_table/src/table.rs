use core::fmt::{self, Display};

use serde_json::Value;
use tabled::{
    grid::spanned::GridConfig,
    settings::style::{RawStyle, Style},
    tables::table::ColoredConfig,
    Table,
};

/// Converter of [`Value`] to a table,
/// with a set of configurations.
#[derive(Debug, Clone)]
pub struct JsonTable<'val, ModeVisitor = fn(&Value) -> Orientation> {
    value: &'val Value,
    cfg: Config,
    mode_visitor: Option<ModeVisitor>,
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
                array_orientation: Orientation::Vertical,
                object_orientation: Orientation::Vertical,
            },
            mode_visitor: None,
        }
    }
}

impl<'val, ModeVisitor> JsonTable<'val, ModeVisitor> {
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

    /// Set a table mode for a [`serde_json::Value::Object`].
    ///
    /// BE AWARE: The setting works only in not collapsed mode.
    pub fn set_object_mode(&mut self, mode: Orientation) -> &mut Self {
        self.cfg.object_orientation = mode;
        self
    }

    /// Set a table mode for a [`serde_json::Value::Array`].
    ///
    /// BE AWARE: The setting works only in not collapsed mode.
    pub fn set_array_mode(&mut self, mode: Orientation) -> &mut Self {
        self.cfg.array_orientation = mode;
        self
    }

    /// Set a visitor which can configure table mode at processing time.
    ///
    /// BE AWARE: The setting works only in not collapsed mode.
    pub fn set_mode_visitor<F>(self, visitor: F) -> JsonTable<'val, F>
    where
        F: FnMut(&Value) -> Orientation,
    {
        JsonTable {
            cfg: self.cfg,
            mode_visitor: Some(visitor),
            value: self.value,
        }
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
    pub fn set_config(&mut self, cfg: ColoredConfig) -> &mut Self {
        self.cfg.cfg = Some(cfg);
        self
    }
}

impl<ModeVisitor> Display for JsonTable<'_, ModeVisitor>
where
    ModeVisitor: FnMut(&Value) -> Orientation + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut mode_visitor = self.mode_visitor.clone();
        let table = json_to_table::json_to_table(self.value, &self.cfg, mode_visitor.as_mut());
        table.fmt(f)
    }
}

impl<ModeVisitor> From<JsonTable<'_, ModeVisitor>> for Table
where
    ModeVisitor: FnMut(&Value) -> Orientation,
{
    fn from(mut t: JsonTable<'_, ModeVisitor>) -> Self {
        json_to_table::json_to_table(t.value, &t.cfg, t.mode_visitor.as_mut())
    }
}

#[derive(Debug, Clone)]
struct Config {
    plain: bool,
    style: Option<RawStyle>,
    cfg: Option<ColoredConfig>,
    object_orientation: Orientation,
    array_orientation: Orientation,
}

/// The structure represents a table mode for a given entity,
/// either it will be rendered vertically or horizontally.
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    /// Vertical mode (from top to bottom).
    Vertical,
    /// Horizontal mode (from left to right).
    Horizontal,
}

mod json_to_table {
    #![allow(clippy::too_many_arguments)]

    use std::cmp;

    use tabled::{
        builder::Builder,
        col,
        grid::{
            config::Entity,
            dimension::{Dimension, Estimate},
            spanned::config::Offset,
            util::string::string_width_multiline,
        },
        records::Records,
        settings::{Format, Height, Padding, Settings, TableOption, Width},
        tables::table::ColoredConfig,
    };

    use super::*;

    pub(super) fn json_to_table<F>(
        value: &Value,
        cfg: &Config,
        mut mode_visitor: Option<&mut F>,
    ) -> Table
    where
        F: FnMut(&Value) -> Orientation,
    {
        if cfg.plain {
            json_to_table_f(value, cfg, &mut mode_visitor, true)
        } else {
            json_to_table_r(value, cfg, 0, 0, true, true, false, false, &[], None)
        }
    }

    fn json_to_table_f<F>(
        v: &Value,
        config: &Config,
        mode_visitor: &mut Option<&mut F>,
        outer: bool,
    ) -> Table
    where
        F: FnMut(&Value) -> Orientation,
    {
        match v {
            Value::Array(arr) => {
                let mut builder = Builder::new();

                let orientation = mode_visitor
                    .as_mut()
                    .map(|f| (f)(v))
                    .unwrap_or(config.array_orientation);

                match orientation {
                    Orientation::Vertical => {
                        for value in arr {
                            let val =
                                json_to_table_f(value, config, mode_visitor, false).to_string();
                            builder.push_record([val]);
                        }
                    }
                    Orientation::Horizontal => {
                        let mut row = Vec::with_capacity(arr.len());
                        for value in arr {
                            let val =
                                json_to_table_f(value, config, mode_visitor, false).to_string();
                            row.push(val);
                        }

                        builder.hint_column_size(row.len());
                        builder.push_record(row);
                    }
                }

                set_table_style(builder.build(), config)
            }
            Value::Object(map) => {
                let mut builder = Builder::new();

                let orientation = mode_visitor
                    .as_mut()
                    .map(|f| (f)(v))
                    .unwrap_or(config.object_orientation);

                match orientation {
                    Orientation::Vertical => {
                        for (key, value) in map {
                            let val =
                                json_to_table_f(value, config, mode_visitor, false).to_string();
                            builder.push_record([key.clone(), val]);
                        }
                    }
                    Orientation::Horizontal => {
                        let mut keys = Vec::with_capacity(map.len());
                        let mut vals = Vec::with_capacity(map.len());
                        for (key, value) in map {
                            let val =
                                json_to_table_f(value, config, mode_visitor, false).to_string();
                            vals.push(val);
                            keys.push(key.clone());
                        }

                        builder.hint_column_size(map.len());
                        builder.push_record(keys);
                        builder.push_record(vals);
                    }
                }

                set_table_style(builder.build(), config)
            }
            value => {
                let value = match value {
                    Value::String(text) => Some(text.clone()),
                    Value::Bool(val) => Some(val.to_string()),
                    Value::Number(num) => Some(num.to_string()),
                    Value::Null => None,
                    _ => unreachable!(),
                };

                let mut builder = Builder::new();

                if let Some(value) = value {
                    builder.hint_column_size(1);
                    builder.push_record([value]);
                }

                let mut table = set_table_style(builder.build(), config);

                if !outer {
                    table.with(Style::empty());
                }

                table
            }
        }
    }

    fn json_to_table_r(
        value: &Value,
        config: &Config,
        row: usize,
        column: usize,
        is_last: bool,
        is_prev_row_last: bool,
        is_in_list: bool,
        change_key_split: bool,
        used_splits: &[usize],
        width: Option<usize>,
    ) -> Table {
        let wpad = config
            .cfg
            .as_ref()
            .map(|cfg| {
                let pad = cfg.get_padding(Entity::Global);
                pad.left.indent.size + pad.right.indent.size
            })
            .unwrap_or(2);

        match value {
            Value::String(..) | Value::Bool(..) | Value::Number(..) | Value::Null => {
                let table = match value {
                    Value::String(s) => col![s],
                    Value::Bool(b) => col![b],
                    Value::Number(n) => col![n],
                    Value::Null => col![""],
                    _ => unreachable!(),
                };

                let mut table = set_table_style(table, config);

                let mut top_intersection = GetTopIntersection(' ');
                table.with(&mut top_intersection);
                let top_intersection = top_intersection.0;

                table.with(
                    Settings::default()
                        .with(Width::increase(width.unwrap_or(0)))
                        .with(SetBottomChars(used_splits, top_intersection)),
                );

                table
            }
            Value::Object(obj) => {
                if obj.is_empty() {
                    // a corner case where the object must behave as empty string

                    return json_to_table_r(
                        &Value::String(String::new()),
                        config,
                        row,
                        column,
                        is_last,
                        is_prev_row_last,
                        is_in_list,
                        change_key_split,
                        used_splits,
                        width,
                    );
                }

                let map_length = obj.len();
                let max_keys_width = obj
                    .iter()
                    .map(|(key, _)| {
                        set_table_style(col![key], config)
                            .with(NoRightBorders)
                            .total_width()
                    })
                    .max()
                    .unwrap_or_default();

                let width = match width {
                    Some(width) => width,
                    None => {
                        // build dummy table
                        let map = obj.iter().enumerate().map(|(i, (key, value))| {
                            let is_last = is_last && i + 1 == map_length;

                            let mut key = set_table_style(col![key], config);
                            key.with(NoRightBorders);

                            let value = json_to_table_r(
                                value,
                                config,
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
                            .unwrap_or_default();

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
                        find_top_intersection(value, wpad)
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
                        config,
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

                    let key = col![key];
                    let mut key = set_table_style(key, config);

                    let mut top_intersection = GetTopIntersection(' ');
                    key.with(&mut top_intersection);
                    let top_intersection = top_intersection.0;

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

                    let value_height = value.total_height();
                    let key_height = key.total_height();
                    let height = cmp::max(key_height, value_height);

                    value.with(Settings::new(
                        Width::increase(width),
                        Height::increase(height),
                    ));

                    {
                        // set custom chars
                        if i + 1 == map_length {
                            // set for the key
                            key.with(
                                Settings::default()
                                    .with(Width::increase(max_keys_width))
                                    .with(Height::increase(height))
                                    .with(SetBottomChars(used_splits, top_intersection)),
                            );
                        } else {
                            key.with(Settings::new(
                                Width::increase(max_keys_width),
                                Height::increase(height),
                            ));
                        }
                    }

                    builder.push_record([key.to_string(), value.to_string()]);
                }

                let mut table = builder.build();
                table.with(Style::empty()).with(Padding::zero());
                table
            }
            Value::Array(list) => {
                if list.is_empty() {
                    // a corner case where the list must behave as empty string

                    return json_to_table_r(
                        &Value::String(String::new()),
                        config,
                        row,
                        column,
                        is_last,
                        is_prev_row_last,
                        is_in_list,
                        change_key_split,
                        used_splits,
                        width,
                    );
                }

                let width = match width {
                    Some(width) => width,
                    None => {
                        // build a dummy tables
                        let list = list.iter().enumerate().map(|(i, value)| {
                            let is_last_element = i + 1 == list.len();
                            let is_last = is_last && is_last_element;
                            json_to_table_r(
                                value,
                                config,
                                row,
                                column,
                                is_last,
                                is_last_element,
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
                            .unwrap_or_default()
                    }
                };
                let map_length = list.len();
                let mut builder = Builder::new();
                for (i, value) in list.iter().enumerate() {
                    let row = row + i;

                    let intersections = if i + 1 < map_length {
                        let value = &list[i + 1];
                        find_top_intersection(value, wpad)
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
                        config,
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

                    builder.push_record([value.to_string()]);
                }

                let mut table = builder.build();
                table.with(Style::empty()).with(Padding::zero());
                table
            }
        }
    }

    fn find_top_intersection(table: &Value, padding: usize) -> Vec<usize> {
        let mut intersections = Vec::new();
        find_top_intersection_r(table, &mut intersections, padding);

        intersections
    }

    fn find_top_intersection_r(table: &Value, chars: &mut Vec<usize>, padding: usize) {
        match table {
            Value::String(_) | Value::Bool(_) | Value::Number(_) | Value::Null => (),
            Value::Object(m) => {
                if m.is_empty() {
                    return;
                }

                let mut max_keys_width = 0;
                for (key, _) in m.iter() {
                    let width = string_width_multiline(key) + padding;
                    max_keys_width = cmp::max(max_keys_width, width);
                }

                chars.push(max_keys_width);

                let (_, value) = m.iter().next().unwrap();
                find_top_intersection_r(value, chars, padding);
            }
            Value::Array(list) => {
                if let Some(value) = list.first() {
                    find_top_intersection_r(value, chars, padding);
                }
            }
        }
    }

    fn set_table_style(mut table: Table, config: &Config) -> Table {
        if let Some(cfg) = config.cfg.as_ref() {
            table.with(Format::config(|c: &mut ColoredConfig| *c = cfg.clone()));
        }

        if let Some(style) = config.style.as_ref() {
            table.with(style);
        }

        table
    }

    struct NoTopBorders;

    impl<R, D> TableOption<R, D, ColoredConfig> for NoTopBorders {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.top = None;
            borders.top_intersection = None;
            borders.top_left = None;
            borders.top_right = None;

            cfg.set_borders(borders);
        }
    }

    struct NoBottomBorders;

    impl<R, D> TableOption<R, D, ColoredConfig> for NoBottomBorders {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.bottom = None;
            borders.bottom_intersection = None;
            borders.bottom_left = None;
            borders.bottom_right = None;

            cfg.set_borders(borders);
        }
    }

    struct NoRightBorders;

    impl<R, D> TableOption<R, D, ColoredConfig> for NoRightBorders {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.top_right = None;
            borders.bottom_right = None;
            borders.right = None;
            borders.right_intersection = None;

            cfg.set_borders(borders);
        }
    }

    struct NoLeftBorders;

    impl<R, D> TableOption<R, D, ColoredConfig> for NoLeftBorders {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.top_left = None;
            borders.bottom_left = None;
            borders.left = None;
            borders.left_intersection = None;

            cfg.set_borders(borders);
        }
    }

    struct TopLeftChangeSplit;

    impl<R, D> TableOption<R, D, ColoredConfig> for TopLeftChangeSplit {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.top_left = borders.top_intersection;

            cfg.set_borders(borders);
        }
    }

    struct BottomLeftChangeSplit;

    impl<R, D> TableOption<R, D, ColoredConfig> for BottomLeftChangeSplit {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.bottom_left = borders.left_intersection;

            cfg.set_borders(borders);
        }
    }

    struct BottomLeftChangeSplitToIntersection;

    impl<R, D> TableOption<R, D, ColoredConfig> for BottomLeftChangeSplitToIntersection {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.bottom_left = borders.intersection;

            cfg.set_borders(borders);
        }
    }

    struct BottomRightChangeToRight;

    impl<R, D> TableOption<R, D, ColoredConfig> for BottomRightChangeToRight {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.bottom_right = borders.right_intersection;

            cfg.set_borders(borders);
        }
    }

    struct BottomLeftChangeToBottomIntersection;

    impl<R, D> TableOption<R, D, ColoredConfig> for BottomLeftChangeToBottomIntersection {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.bottom_left = borders.bottom_intersection;

            cfg.set_borders(borders);
        }
    }

    struct SetBottomChars<'a>(&'a [usize], char);

    impl<R, D> TableOption<R, D, ColoredConfig> for SetBottomChars<'_>
    where
        R: Records,
        for<'a> &'a R: Records,
        D: Dimension + Estimate<GridConfig>,
    {
        fn change(&mut self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
            dims.estimate(&*records, cfg);

            let table_width = (0..records.count_columns())
                .map(|col| dims.get_width(col))
                .sum::<usize>()
                + cfg.count_vertical(records.count_columns());
            let mut current_width = 0;

            for pos in self.0 {
                current_width += pos;
                if current_width > table_width {
                    break;
                }

                let split_char = self.1;
                cfg.override_horizontal_border((1, 0), split_char, Offset::Begin(current_width));

                current_width += 1;
            }
        }
    }

    struct GetTopIntersection(char);

    impl<R, D> TableOption<R, D, ColoredConfig> for GetTopIntersection {
        fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            self.0 = cfg.get_borders().top_intersection.unwrap_or(' ');
        }
    }
}
