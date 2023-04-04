use std::{
    cmp::{self, max},
    collections::HashMap,
    iter::repeat,
};

use tabled::{
    builder::Builder,
    col,
    grid::{
        config::{AlignmentHorizontal, AlignmentVertical, ColoredConfig, Entity, Offset},
        dimension::{Dimension, Estimate},
        records::Records,
        util::string::{count_lines, get_lines, string_dimension, string_width},
    },
    settings::{Padding, TableOption},
};

use super::*;

struct PrintContext {
    pos: usize,
    is_last_col: bool,
    is_last_row: bool,
    is_first_col: bool,
    is_first_row: bool,
    kv: bool,
    kv_is_first: bool,
    list: bool,
    list_is_first: bool,
    splits: Vec<usize>,
    size: Dim,
}

pub(super) fn collapsed_table(value: &Value, cfg: &Config) -> String {
    let dims = collect_table_dimensions(value, cfg);
    let ctx = PrintContext {
        pos: 0,
        is_last_col: true,
        is_last_row: true,
        is_first_col: true,
        is_first_row: true,
        kv: false,
        kv_is_first: false,
        list: false,
        list_is_first: false,
        splits: Vec::new(),
        size: *dims.all.get(&0).unwrap(),
    };
    _collapsed_table(value, cfg, &dims, ctx)
}

fn _collapsed_table(val: &Value, cfg: &Config, dims: &Dimensions, ctx: PrintContext) -> String {
    match val {
        Value::String(..) | Value::Bool(..) | Value::Number(..) | Value::Null => {
            let value = match val {
                Value::String(val) => val.to_string(),
                Value::Bool(val) => val.to_string(),
                Value::Number(val) => val.to_string(),
                Value::Null => String::new(),
                _ => unreachable!(),
            };

            let value = config_string(&value, &cfg.cfg, ctx.size.width, ctx.size.height);

            let mut table = col![value];
            table.with(&cfg.cfg);

            if !ctx.is_last_row {
                table.with(NoBottomBorders);
            }

            if ctx.kv && ctx.kv_is_first {
                table.with(TopLeftChangeTopIntersection);
            }

            if ctx.kv && !ctx.kv_is_first {
                table.with(TopLeftChangeIntersection);
            }

            if ctx.kv && ctx.list && !ctx.list_is_first {
                table.with(TopLeftChangeToLeft);
            }

            if ctx.is_last_col && !ctx.is_first_row {
                table.with(TopRightChangeToRight);
            }

            if !ctx.is_first_col && ctx.is_last_row {
                table.with(BottomLeftChangeToBottomIntersection);
            }

            if ctx.is_first_col && !ctx.is_first_row {
                table.with(TopLeftChangeToLeft);
            }

            let has_vertical = cfg.cfg.get_borders().has_left();
            if !ctx.splits.is_empty() && has_vertical {
                let mut splits = ctx.splits;
                let splits = short_splits(&mut splits, ctx.size.width);

                let c = cfg.cfg.get_borders().bottom_intersection.unwrap_or(' ');
                set_top_intersections(&mut table, &splits, c);
            }

            table.to_string()
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                // a corner case where the object must behave as empty string
                return _collapsed_table(&Value::Null, cfg, dims, ctx);
            }

            let map_dims = dims.maps.get(&ctx.pos).unwrap();
            let max_key_width = map_dims.key_max.width;

            let has_vertical = cfg.cfg.get_borders().has_left();
            let value_width = ctx.size.width - max_key_width - has_vertical as usize;

            let mut splits = ctx.splits;
            let key_splits = short_splits(&mut splits, max_key_width);

            let mut builder = Builder::new();
            for (i, (key, val)) in obj.iter().enumerate() {
                let val_pos = *map_dims.index.get(&i).unwrap();
                let key_pos = ctx.pos + i + 1;

                let intersections = splits;
                splits = get_splits(val, dims, val_pos, i, obj.len());

                let key_height = dims.all.get(&key_pos).unwrap().height;
                let val_height = dims.all.get(&val_pos).unwrap().height;

                let entry_height = cmp::max(key_height, val_height);

                let valctx = PrintContext {
                    pos: val_pos,
                    is_last_col: ctx.is_last_col,
                    is_last_row: ctx.is_last_row && i + 1 == obj.len(),
                    is_first_col: false,
                    is_first_row: ctx.is_first_row && i == 0,
                    kv: true,
                    kv_is_first: i == 0,
                    list: false,
                    list_is_first: false,
                    splits: intersections,
                    size: Dim::new(value_width, entry_height),
                };

                let key = config_string(key, &cfg.cfg, max_key_width, entry_height);

                let mut key = col![key];
                key.with(&cfg.cfg);
                key.with(NoRightBorders);

                if !valctx.is_last_row {
                    key.with(NoBottomBorders);
                }

                if !ctx.is_first_col && valctx.is_last_row {
                    key.with(BottomLeftChangeToBottomIntersection);
                }

                if !valctx.is_first_row {
                    key.with(TopLeftChangeToLeft);
                }

                if ctx.kv && !ctx.kv_is_first && i == 0 {
                    key.with(TopLeftChangeIntersection);
                }

                if ctx.kv && ctx.kv_is_first && i == 0 {
                    key.with(TopLeftChangeTopIntersection);
                }

                if ctx.list && !ctx.list_is_first {
                    key.with(TopLeftChangeToLeft);
                }

                if !ctx.is_first_col && valctx.is_first_row {
                    key.with(TopLeftChangeTopIntersection);
                }

                if i == 0 && has_vertical {
                    let c = cfg.cfg.get_borders().bottom_intersection.unwrap_or(' ');
                    set_top_intersections(&mut key, &key_splits, c);
                }

                let val = _collapsed_table(val, cfg, dims, valctx);

                builder.push_record([key.to_string(), val]);
            }

            let mut table = builder.build();
            table.with(Style::empty()).with(Padding::zero());
            table.to_string()
        }
        Value::Array(list) => {
            if list.is_empty() {
                // a corner case where the list must behave as empty string
                return _collapsed_table(&Value::Null, cfg, dims, ctx);
            }

            let array_dims = dims.arrays.get(&ctx.pos).unwrap();

            let height = dims.all.get(&ctx.pos).unwrap().height;
            let additional_height = ctx.size.height - height;
            let (chunk_height, rest_height) = split_value(additional_height, list.len());

            let mut splits = ctx.splits;
            let mut builder = Builder::new();
            for (i, val) in list.iter().enumerate() {
                let val_pos = *array_dims.index.get(&i).unwrap();

                let intersections = splits;
                splits = get_splits(val, dims, val_pos, i, list.len());

                let size = {
                    let mut height = dims.all.get(&val_pos).unwrap().height + chunk_height;
                    if i == 0 {
                        height += rest_height;
                    }
                    Dim::new(ctx.size.width, height)
                };
                let is_prev_list_not_first = ctx.list && !ctx.list_is_first;
                let valctx = PrintContext {
                    pos: val_pos,
                    is_last_col: ctx.is_last_col,
                    is_last_row: ctx.is_last_row && i + 1 == list.len(),
                    is_first_col: ctx.is_first_col,
                    is_first_row: ctx.is_first_row && i == 0,
                    kv: ctx.kv,
                    kv_is_first: ctx.kv_is_first,
                    list: true,
                    list_is_first: i == 0 && !is_prev_list_not_first,
                    splits: intersections,
                    size,
                };

                let val = _collapsed_table(val, cfg, dims, valctx);
                builder.push_record([val]);
            }

            let mut table = builder.build();
            table.with(Style::empty()).with(Padding::zero());
            table.to_string()
        }
    }
}

fn get_splits(val: &Value, dims: &Dimensions, pos: usize, i: usize, len: usize) -> Vec<usize> {
    if i + 1 == len {
        Vec::new()
    } else {
        collect_splits(val, dims, pos)
    }
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

struct TopLeftChangeTopIntersection;

impl<R, D> TableOption<R, D, ColoredConfig> for TopLeftChangeTopIntersection {
    fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_left = borders.top_intersection;

        cfg.set_borders(borders);
    }
}

struct TopLeftChangeIntersection;

impl<R, D> TableOption<R, D, ColoredConfig> for TopLeftChangeIntersection {
    fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_left = borders.intersection;

        cfg.set_borders(borders);
    }
}

struct TopLeftChangeToLeft;

impl<R, D> TableOption<R, D, ColoredConfig> for TopLeftChangeToLeft {
    fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_left = borders.left_intersection;

        cfg.set_borders(borders);
    }
}

struct TopRightChangeToRight;

impl<R, D> TableOption<R, D, ColoredConfig> for TopRightChangeToRight {
    fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_right = borders.right_intersection;

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
    for<'a> D: Dimension + Estimate<&'a R, SpannedConfig>,
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
            cfg.set_horizontal_char((1, 0), split_char, Offset::Begin(current_width));

            current_width += 1;
        }
    }
}

struct SetTopChars<'a>(&'a [usize], char);

impl<R, D> TableOption<R, D, ColoredConfig> for SetTopChars<'_> {
    fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        for &split in self.0 {
            let offset = split - 1;
            cfg.set_horizontal_char((0, 0), self.1, Offset::Begin(offset));
        }
    }
}

struct GetTopIntersection(char);

impl<R, D> TableOption<R, D, ColoredConfig> for GetTopIntersection {
    fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        self.0 = cfg.get_borders().top_intersection.unwrap_or(' ');
    }
}

struct GetBottomIntersection(char);

impl<R, D> TableOption<R, D, ColoredConfig> for GetBottomIntersection {
    fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        self.0 = cfg.get_borders().bottom_intersection.unwrap_or(' ');
    }
}

#[derive(Debug, Default)]
struct Dimensions {
    all: HashMap<usize, Dim>,
    maps: HashMap<usize, MapDimensions>,
    arrays: HashMap<usize, ArrayDimensions>,
}

#[derive(Debug, Default, Clone, Copy)]
struct Dim {
    width: usize,
    height: usize,
}

impl Dim {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Default)]
struct MapDimensions {
    key_max: Dim,
    value_max: Dim,
    index: HashMap<usize, usize>,
}

#[derive(Debug, Default)]
struct ArrayDimensions {
    max: Dim,
    index: HashMap<usize, usize>,
}

fn collect_table_dimensions(val: &Value, cfg: &Config) -> Dimensions {
    let mut buf = Dimensions::default();
    let (dim, _) = __collect_table_dims(&mut buf, val, cfg, 0);
    buf.all.insert(0, dim);
    buf
}

fn __collect_table_dims(
    buf: &mut Dimensions,
    val: &Value,
    cfg: &Config,
    pos: usize,
) -> (Dim, usize) {
    match val {
        Value::String(text) => (str_dimension(text, cfg), 0),
        Value::Bool(b) => (bool_dimension(b, cfg), 0),
        Value::Number(num) => (num_dimension(num, cfg), 0),
        Value::Null => (empty_dimension(cfg), 0),
        Value::Object(obj) => {
            if obj.is_empty() {
                return (empty_dimension(cfg), 0);
            }

            let mut index = MapDimensions {
                index: HashMap::with_capacity(obj.len()),
                key_max: Dim::default(),
                value_max: Dim::default(),
            };
            let mut total_height = 0;
            let mut count_elements = obj.len() * 2;
            let mut val_pos = pos + 1 + obj.len();
            for (i, (key, val)) in obj.iter().enumerate() {
                let key_pos = pos + i + 1;

                let key = str_dimension(key, cfg);
                let (val, elements) = __collect_table_dims(buf, val, cfg, val_pos);
                count_elements += elements;

                println!(
                        "-----> key={:?} pos={:?} key_pos={:?} val_pos={:?} elements_total={:?} elements_val={:?} i={}",
                        obj.keys().nth(i).unwrap(),
                        pos,
                        key_pos,
                        val_pos,
                        count_elements,
                        elements,
                        i
                    );

                total_height += max(key.height, val.height);

                index.key_max.width = max(index.key_max.width, key.width);
                index.key_max.height = max(index.key_max.height, key.height);
                index.value_max.width = max(index.value_max.width, val.width);
                index.value_max.height = max(index.value_max.height, val.height);

                buf.all.insert(key_pos, key);
                buf.all.insert(val_pos, val);

                index.index.insert(i, val_pos);

                val_pos += elements + 1;
            }

            let has_vertical = cfg.cfg.get_borders().has_left();
            let total_width = index.key_max.width + index.value_max.width + has_vertical as usize;

            let has_horizontal = cfg.cfg.get_borders().has_top();
            total_height += has_horizontal as usize * (obj.len() - 1);

            buf.maps.insert(pos, index);

            (Dim::new(total_width, total_height), count_elements)
        }
        Value::Array(list) => {
            if list.is_empty() {
                return (empty_dimension(cfg), 0);
            }

            let mut index = ArrayDimensions {
                max: Dim::default(),
                index: HashMap::with_capacity(list.len()),
            };

            let mut total_height = 0;
            let mut count_elements = list.len();
            let mut val_pos = pos + 1;
            for (i, value) in list.iter().enumerate() {
                let (dim, elements) = __collect_table_dims(buf, value, cfg, val_pos);
                count_elements += elements;

                println!(
                    "-----arr> pos={:?} val_pos={:?} elements_total={:?} elements_val={:?} i={}",
                    pos, val_pos, count_elements, elements, i
                );

                total_height += dim.height;

                index.max.width = max(index.max.width, dim.width);
                index.max.height = max(index.max.height, dim.height);

                buf.all.insert(val_pos, dim);

                index.index.insert(i, val_pos);

                val_pos += 1 + elements;
            }

            println!("xxxx - {} {}", count_elements, pos);

            let has_horizontal = cfg.cfg.get_borders().has_top();
            total_height += has_horizontal as usize * (list.len() - 1);

            let max_width = index.max.width;

            buf.arrays.insert(pos, index);

            (Dim::new(max_width, total_height), count_elements)
        }
    }
}

fn bool_dimension(b: &bool, cfg: &Config) -> Dim {
    let width = if *b { 4 } else { 5 };
    Dim::new(
        width + get_padding_horizontal(cfg),
        1 + get_padding_vertical(cfg),
    )
}

fn num_dimension(num: &serde_json::Number, cfg: &Config) -> Dim {
    Dim::new(
        num.to_string().len() + get_padding_horizontal(cfg),
        1 + get_padding_vertical(cfg),
    )
}

fn empty_dimension(cfg: &Config) -> Dim {
    Dim::new(get_padding_horizontal(cfg), 1 + get_padding_vertical(cfg))
}

fn str_dimension(text: &str, cfg: &Config) -> Dim {
    let (count_lines, width) = string_dimension(text);
    let w = width + get_padding_horizontal(cfg);
    let h = count_lines + get_padding_vertical(cfg);
    Dim::new(w, h)
}

fn get_padding_horizontal(cfg: &Config) -> usize {
    let pad = cfg.cfg.get_padding(Entity::Global);
    pad.left.indent.size + pad.right.indent.size
}

fn get_padding_vertical(cfg: &Config) -> usize {
    let pad = cfg.cfg.get_padding(Entity::Global);
    pad.top.indent.size + pad.bottom.indent.size
}

fn split_value(value: usize, by: usize) -> (usize, usize) {
    let val = value / by;
    let rest = value - val;

    (val, rest)
}

fn config_string(value: &str, cfg: &ColoredConfig, width: usize, height: usize) -> String {
    let pad = cfg.get_padding(Entity::Global);
    let width = width - pad.left.indent.size - pad.right.indent.size;
    let height = height - pad.bottom.indent.size - pad.top.indent.size;
    let ah = *cfg.get_alignment_horizontal(Entity::Global);
    let av = *cfg.get_alignment_vertical(Entity::Global);
    set_string_dimension(value, width, height, ah, av)
}

fn set_string_dimension(
    text: &str,
    width: usize,
    height: usize,
    ah: AlignmentHorizontal,
    av: AlignmentVertical,
) -> String {
    let mut out = Vec::with_capacity(height);

    let count_lines = count_lines(text);

    let (top, bottom) = indent_vertical(av, height, count_lines);

    out.extend(repeat(String::new()).take(top));

    for line in get_lines(text) {
        let w = string_width(&line);
        let (left, right) = indent_horizontal(ah, width, w);

        let mut buf = String::new();
        buf.extend(repeat(' ').take(left));
        buf.push_str(&line);
        buf.extend(repeat(' ').take(right));

        out.push(buf);
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

fn short_splits(splits: &mut Vec<usize>, width: usize) -> Vec<usize> {
    if splits.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut pos = 0;
    for &split in splits.iter() {
        if pos + split >= width {
            break;
        }

        pos += split;
        out.push(pos);
    }

    splits.drain(..out.len());

    if !splits.is_empty() && pos <= width {
        let rest = width - pos + 1;
        splits[0] -= rest;
    }

    out
}

fn intersections_to_splits(splits: &mut [usize]) {
    for split in splits.iter_mut() {
        *split += 1;
    }
}

fn set_top_intersections(table: &mut Table, mut splits: &[usize], c: char) {
    if splits.first().copied() == Some(0) {
        splits = &splits[1..];
        table.with(TopLeftChangeIntersection);
    }

    table.with(SetTopChars(splits, c));
}

fn collect_splits(val: &Value, dims: &Dimensions, pos: usize) -> Vec<usize> {
    let mut widths = collect_intersections(val, dims, pos);
    intersections_to_splits(&mut widths);
    widths
}

fn collect_intersections(val: &Value, dims: &Dimensions, pos: usize) -> Vec<usize> {
    let mut intersections = Vec::new();
    __find_intersection(&mut intersections, val, dims, pos);
    intersections
}

fn __find_intersection(chars: &mut Vec<usize>, table: &Value, dims: &Dimensions, pos: usize) {
    match table {
        Value::String(_) | Value::Bool(_) | Value::Number(_) | Value::Null => (),
        Value::Object(map) => {
            if map.is_empty() {
                return;
            }

            let map_dims = dims.maps.get(&pos).unwrap();

            chars.push(map_dims.key_max.width);

            let val_index = *map_dims.index.get(&(map.len() - 1)).unwrap();
            let value = map.values().last().unwrap();

            __find_intersection(chars, value, dims, val_index);
        }
        Value::Array(list) => {
            if list.is_empty() {
                return;
            }

            let array_dims = dims.arrays.get(&pos).unwrap();
            let val_index = *array_dims.index.get(&(list.len() - 1)).unwrap();
            let value = list.last().unwrap();

            __find_intersection(chars, value, dims, val_index);
        }
    }
}
