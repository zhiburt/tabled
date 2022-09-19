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
            let t = json_to_table_r(value, &cfg.style, 0);
            table_cell_to_table(t, &cfg.style, 0, 0, true, true, false, false, &[], None)
        }
    }

    fn json_to_table_r(v: &Value, style: &RawStyle, row: usize) -> TableCell {
        match v {
            Value::Null => TableCell::String(String::new()),
            Value::Bool(b) => TableCell::String(b.to_string()),
            Value::Number(n) => TableCell::String(n.to_string()),
            Value::String(s) => TableCell::String(s.to_string()),
            Value::Array(arr) => {
                let tables = arr
                    .iter()
                    .enumerate()
                    .map(|(i, value)| json_to_table_r(value, style, row + i))
                    .collect();

                TableCell::List(tables)
            }
            Value::Object(map) => {
                let tables = map
                    .into_iter()
                    .enumerate()
                    .map(|(i, (key, value))| {
                        (key.to_owned(), json_to_table_r(value, style, row + i))
                    })
                    .collect();

                TableCell::Map(tables)
            }
        }
    }

    #[derive(Debug, Clone)]
    enum TableCell {
        String(String),
        Map(Vec<(String, TableCell)>),
        List(Vec<TableCell>),
    }

    #[allow(clippy::too_many_arguments)]
    fn table_cell_to_table(
        table: TableCell,
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
        match table {
            TableCell::String(s) => {
                let mut table = col![s]
                    .with(style)
                    .with(Width::increase(width.unwrap_or(0)));

                {
                    table = table.with(SetBottomChars(used_splits));
                }

                table
            }
            TableCell::Map(m) => {
                let map_length = m.len();

                let mut keys = Vec::new();
                for (key, _) in m.iter() {
                    let mut key = col![key];

                    key = key.with(style);

                    if row != 0 {
                        key = key.with(NoTopBorders);
                    }

                    key = key.with(NoRightBorders);

                    keys.push(key);
                }

                let max_keys_width = keys.iter().map(|key| key.total_width()).max().unwrap_or(0);

                let mut map = Vec::new();
                for i in 0..map_length {
                    let row = row + i;

                    let (key, value) = &m[i];

                    let mut was_intersection_touched = false;
                    let intersections = if i + 1 < map_length {
                        let (_, value) = &m[i + 1];
                        find_top_intersection(value, style)
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

                    let width = width.map(|w| w - max_keys_width);

                    let is_last = is_last && i + 1 == map_length;

                    let mut value = table_cell_to_table(
                        value.clone(),
                        style,
                        row,
                        column + 2,
                        is_last,
                        i + 1 == map_length,
                        false,
                        was_intersection_touched,
                        &intersections,
                        width,
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

                        if i + 1 == map_length && is_in_list && !is_last {
                            key = key.with(BottomLeftChangeSplit);
                        }

                        if i + 1 == map_length && is_prev_row_last && column != 0 {
                            key = key.with(BottomLeftChangeSplit3);
                        }

                        if is_last && column != 0 {
                            key = key.with(BottomLeftChangeSplit3);
                        }

                        if change_key_split {
                            key = key.with(BottomLeftChangeSplitToIntersection);
                        }
                    }

                    map.push((key, value));
                }

                match width {
                    Some(width) => {
                        let width = width - max_keys_width;

                        map = map
                            .into_iter()
                            .map(|(key, mut value)| {
                                value = value.with(Width::increase(width));

                                (key, value)
                            })
                            .collect::<Vec<_>>();

                        let mut b = Builder::new();
                        for (mut key, value) in map {
                            let value_height = value.total_height();

                            key = key
                                .with(Width::increase(max_keys_width))
                                .with(Height::increase(value_height));

                            b.add_record([key.to_string(), value.to_string()]);
                        }

                        let mut table = b.build();
                        table = table.with(Style::empty()).with(Padding::zero());
                        table
                    }
                    None => {
                        // need to rebuild the values with a known width
                        let width = map
                            .into_iter()
                            .map(|(_, value)| value.total_width())
                            .max()
                            .unwrap_or(0);

                        table_cell_to_table(
                            TableCell::Map(m),
                            style,
                            row,
                            column,
                            is_last,
                            is_prev_row_last,
                            false,
                            change_key_split,
                            used_splits,
                            Some(width + max_keys_width),
                        )
                    }
                }
            }
            TableCell::List(list) => {
                let map_length = list.len();
                let mut map = Vec::new();
                for (i, value) in list.clone().into_iter().enumerate() {
                    let row = row + i;

                    let intersections = if i + 1 < map_length {
                        let value = &list[i + 1];
                        find_top_intersection(value, style)
                    } else {
                        used_splits.to_owned()
                    };

                    let is_last = is_last && i + 1 == map_length;
                    let mut value = table_cell_to_table(
                        value,
                        style,
                        row,
                        column,
                        is_last,
                        i + 1 == map_length,
                        true,
                        false,
                        &intersections,
                        width,
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

                    map.push(value);
                }

                match width {
                    Some(width) => {
                        map = map
                            .into_iter()
                            .map(|value| value.with(Width::increase(width)))
                            .collect();

                        let mut b = Builder::new();
                        for value in map {
                            b.add_record([value.to_string()]);
                        }

                        b.build().with(Style::empty()).with(Padding::zero())
                    }
                    None => {
                        // need to rebuild the values with a known width
                        let width = map
                            .into_iter()
                            .map(|value| value.total_width())
                            .max()
                            .unwrap_or(0);

                        table_cell_to_table(
                            TableCell::List(list),
                            style,
                            row,
                            column,
                            is_last,
                            is_prev_row_last,
                            true,
                            false,
                            used_splits,
                            Some(width),
                        )
                    }
                }
            }
        }
    }

    fn find_top_intersection(table: &TableCell, style: &RawStyle) -> Vec<usize> {
        let mut intersections = Vec::new();
        find_top_intersection_r(table, style, &mut intersections);

        intersections
    }

    fn find_top_intersection_r(table: &TableCell, style: &RawStyle, chars: &mut Vec<usize>) {
        match table {
            TableCell::String(_) => (),
            TableCell::Map(m) => {
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
            TableCell::List(list) => {
                if let Some(value) = list.first() {
                    find_top_intersection_r(value, style, chars);
                }
            }
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

    // fn json_table_length(value: &Value) -> usize {
    //     match value {
    //         Value::Null => 1,
    //         Value::Bool(_) => 1,
    //         Value::Number(_) => 1,
    //         Value::String(_) => 1,
    //         Value::Array(values) => values.iter().map(json_table_length).max().unwrap_or(1),
    //         Value::Object(map) => 1 + map.values().map(json_table_length).max().unwrap_or(1),
    //     }
    // }

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
