use core::fmt::{self, Display};

use serde_json::Value;
use tabled::{style::RawStyle, Style, Table};

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
    style: RawStyle,
}

impl JsonTable<'_> {
    /// Creates a new [`JsonTable`] object.
    pub fn new(value: &Value) -> JsonTable<'_> {
        JsonTable {
            value,
            cfg: Config {
                plain: true,
                style: Style::ascii().into(),
            },
        }
    }

    /// Set a style which will be used,
    /// default is [`Style::ascii`].
    pub fn set_style(&mut self, style: impl Into<RawStyle>) -> &mut Self {
        self.cfg.style = style.into();
        self
    }

    /// Collapse tables out instead of tables within tables.
    pub fn collapse(&mut self) -> &mut Self {
        self.cfg.plain = false;
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
    use std::{cmp, collections::HashMap};

    use tabled::{
        builder::Builder,
        col,
        papergrid::{records::Records, util::string_width_multiline},
        Height, Padding, TableOption, Width,
    };

    use super::*;

    pub(super) fn json_to_table(value: &Value, cfg: &Config) -> Table {
        if cfg.plain {
            json_to_table_f(value, &cfg.style, true)
        } else {
            json_to_table_r(value, &cfg.style, 0, 0, true, true, false, false, &[], None)
        }
    }

    fn json_to_table_f(v: &Value, style: &RawStyle, outer: bool) -> Table {
        match v {
            Value::Null => {
                let mut table = col![].with(style);
                if !outer {
                    table = table.with(Style::empty());
                }

                table
            }
            Value::Bool(b) => {
                let mut table = col![b].with(style);
                if !outer {
                    table = table.with(Style::empty());
                }

                table
            }
            Value::Number(n) => {
                let mut table = col![n].with(style);
                if !outer {
                    table = table.with(Style::empty());
                }

                table
            }
            Value::String(s) => {
                let mut table = col![s].with(style);
                if !outer {
                    table = table.with(Style::empty());
                }

                table
            }
            Value::Array(arr) => {
                let mut b = Builder::new();
                for value in arr {
                    b.add_record([json_to_table_f(value, style, false).to_string()]);
                }

                b.build().with(style)
            }
            Value::Object(map) => {
                let mut b = Builder::new();
                for (key, value) in map {
                    b.add_record([
                        key.clone(),
                        json_to_table_f(value, style, false).to_string(),
                    ]);
                }

                b.build().with(style)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn json_to_table_r(
        value: &Value,
        style: &RawStyle,
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

                table = table.with(style).with(Width::increase(width.unwrap_or(0)));
                table = table.with(SetBottomChars(used_splits));

                table
            }
            Value::Object(obj) => {
                let map_length = obj.len();

                // we build a table here to not make any assumptions about style.
                // but we could to reduce allocations.
                let max_keys_width = obj
                    .iter()
                    .map(|(key, _)| col![key].with(NoRightBorders))
                    .map(|key| key.total_width())
                    .max()
                    .unwrap_or(0);

                let width = match width {
                    Some(width) => width,
                    None => {
                        // build dummy table
                        let map = obj.iter().enumerate().map(|(i, (key, value))| {
                            let is_last = is_last && i + 1 == map_length;

                            let key = col![key].with(NoRightBorders);
                            let value = json_to_table_r(
                                value,
                                style,
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
                        find_top_intersection(value, style)
                    } else {
                        let mut splits = used_splits.to_owned();

                        println!("after={:?}", splits);

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

                        println!("after={:?} {}", splits, was_intersection_touched);

                        splits
                    };

                    let is_last = is_last && i + 1 == map_length;
                    let width = width - max_keys_width;
                    let mut value = json_to_table_r(
                        value,
                        style,
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
                        value = value.with(TopLeftChangeSplit);

                        if row != 0 {
                            value = value.with(NoTopBorders);
                        }

                        if !is_last {
                            value = value.with(BottomRightChangeSplit2);
                        }

                        if i + 1 == map_length {
                            value = value.with(BottomLeftChangeSplit3);
                        } else {
                            value = value.with(BottomLeftChangeSplitToIntersection);
                        }

                        if was_intersection_touched {
                            value = value.with(BottomLeftChangeSplitToIntersection);
                        }
                    }

                    let mut key = col![key];
                    key = key.with(style);

                    {
                        // set custom chars
                        if i + 1 == map_length {
                            // set for the key
                            key = key.with(SetBottomChars(used_splits));
                        }
                    }

                    {
                        if row != 0 {
                            key = key.with(NoTopBorders);
                        }

                        if row == 0 && column != 0 {
                            key = key.with(TopLeftChangeSplit);
                        }

                        key = key.with(NoRightBorders);

                        if i + 1 < map_length {
                            key = key.with(BottomLeftChangeSplit);
                        }

                        if i + 1 == map_length && !is_last {
                            key = key.with(BottomLeftChangeSplitToIntersection);
                        }

                        if i + 1 == map_length && is_in_list && !is_last && !is_prev_row_last {
                            key = key.with(BottomLeftChangeSplit);
                        }

                        if i + 1 == map_length && is_prev_row_last && column != 0 {
                            key = key.with(BottomLeftChangeSplit3);
                        }

                        if is_last && column != 0 {
                            key = key.with(BottomLeftChangeSplit3);
                        }

                        if i + 1 == map_length
                            && !is_last
                            && is_in_list
                            && is_prev_row_last
                            && column > 0
                        {
                            key = key.with(BottomLeftChangeSplitToIntersection);
                        }

                        if change_key_split {
                            key = key.with(BottomLeftChangeSplitToIntersection);
                        }
                    }

                    {
                        let value_height = value.total_height();

                        key = key
                            .with(Width::increase(max_keys_width))
                            .with(Height::increase(value_height));
                    }

                    builder.add_record([key.to_string(), value.to_string()]);
                }

                let mut table = builder.build();
                table = table.with(Style::empty()).with(Padding::zero());
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
                        find_top_intersection(value, style)
                    } else {
                        used_splits.to_owned()
                    };

                    let is_last = is_last && i + 1 == map_length;
                    let mut value = json_to_table_r(
                        value,
                        style,
                        row,
                        column,
                        is_last,
                        i + 1 == map_length,
                        true,
                        false,
                        &intersections,
                        Some(width),
                    );

                    if column != 0 {
                        value = value.with(TopLeftChangeSplit);
                    }

                    if row > 0 {
                        value = value.with(NoTopBorders);
                    }

                    if !is_last {
                        value = value.with(BottomRightChangeSplit2);
                    }

                    if i + 1 < map_length {
                        value = value.with(BottomLeftChangeSplit);
                    }

                    if i + 1 == map_length && !is_last {
                        value = value.with(BottomLeftChangeSplitToIntersection);
                    }

                    if is_last && column != 0 {
                        value = value.with(BottomLeftChangeSplit3);
                    }

                    value = value.with(Width::increase(width));

                    builder.add_record([value.to_string()]);
                }

                builder.build().with(Style::empty()).with(Padding::zero())
            }
        }
    }

    fn find_top_intersection(table: &Value, style: &RawStyle) -> Vec<usize> {
        let mut intersections = Vec::new();
        find_top_intersection_r(table, style, &mut intersections);

        intersections
    }

    fn find_top_intersection_r(table: &Value, style: &RawStyle, chars: &mut Vec<usize>) {
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
                find_top_intersection_r(value, style, chars);
            }
            Value::Array(list) => {
                if let Some(value) = list.first() {
                    find_top_intersection_r(value, style, chars);
                }
            }
        }
    }

    struct NoOuterBorders;

    impl<R> TableOption<R> for NoOuterBorders {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom = None;
            borders.bottom_intersection = None;
            borders.bottom_left = None;
            borders.bottom_right = None;
            borders.top = None;
            borders.top_intersection = None;
            borders.top_left = None;
            borders.top_right = None;
            borders.horizontal_left = None;
            borders.vertical_left = None;
            borders.horizontal_right = None;
            borders.vertical_right = None;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct NoInnerBorders;

    impl<R> TableOption<R> for NoInnerBorders {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom_left = None;
            borders.bottom_right = None;
            borders.bottom_intersection = None;
            borders.top_intersection = None;
            borders.top_left = None;
            borders.top_right = None;
            borders.vertical = None;
            borders.horizontal = None;
            borders.intersection = None;
            borders.vertical_left = None;
            borders.vertical_right = None;
            borders.horizontal_left = None;
            borders.horizontal_right = None;

            table.get_config_mut().set_borders(borders);
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

    struct NoLeftBorders;

    impl<R> TableOption<R> for NoLeftBorders {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.top_left = None;
            borders.bottom_left = None;
            borders.vertical_left = None;
            borders.horizontal_left = None;

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

    struct NoBottomBorders;

    impl<R> TableOption<R> for NoBottomBorders {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom = None;
            borders.bottom_intersection = None;
            borders.bottom_left = None;
            borders.bottom_right = None;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct BottomSplitChange;

    impl<R> TableOption<R> for BottomSplitChange {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom_left = borders.horizontal_left;
            borders.bottom_right = borders.intersection;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct TopRightChangeSplit;

    impl<R> TableOption<R> for TopRightChangeSplit {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.top_right = borders.top_intersection;

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

    struct BottomRightChangeSplit;

    impl<R> TableOption<R> for BottomRightChangeSplit {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom_right = borders.bottom_intersection;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct BottomRightChangeSplit2;

    impl<R> TableOption<R> for BottomRightChangeSplit2 {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom_right = borders.horizontal_right;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct BottomLeftChangeSplit3;

    impl<R> TableOption<R> for BottomLeftChangeSplit3 {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = table.get_config().get_borders().clone();
            borders.bottom_left = borders.bottom_intersection;

            table.get_config_mut().set_borders(borders);
        }
    }

    struct ConvertStyleToBorders;

    impl<R> TableOption<R> for ConvertStyleToBorders
    where
        R: Records,
    {
        fn change(&mut self, table: &mut Table<R>) {
            let mut borders = HashMap::with_capacity(table.count_rows() * table.count_columns());
            for row in 0..table.count_rows() {
                for col in 0..table.count_columns() {
                    let border = table.get_config().get_border((row, col), table.shape());
                    borders.insert((row, col), border);
                }
            }

            table.get_config_mut().clear_theme();

            for (pos, border) in borders {
                table.get_config_mut().set_border(pos, border);
            }
        }
    }

    struct SetBottomChars<'a>(&'a [usize]);

    impl<R> TableOption<R> for SetBottomChars<'_>
    where
        R: Records,
    {
        fn change(&mut self, table: &mut Table<R>) {
            let split_char = table
                .get_config()
                .get_borders()
                .top_intersection
                .unwrap_or('@');

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
