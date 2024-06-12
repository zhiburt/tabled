use std::{
    cmp::{self, max},
    collections::HashMap,
    iter::repeat,
};

use serde_json::Map;
use tabled::{
    builder::Builder,
    grid::{
        config::{AlignmentHorizontal, AlignmentVertical, ColoredConfig, Entity, Offset},
        dimension::{Dimension, Estimate},
        records::Records,
        util::string::{count_lines, get_lines, string_dimension, string_width},
    },
    settings::{Padding, TableOption},
};

use super::*;

#[derive(Debug, Default)]
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
    no_left: bool,
    no_right: bool,
    no_bottom: bool,
    lean_top: bool,
    top_intersection: bool,
    top_left: bool,
    intersections_horizontal: Vec<usize>,
    intersections_vertical: Vec<usize>,
    size: Dim,
}

struct CellData {
    content: String,
    intersections_horizontal: Vec<usize>,
    intersections_vertical: Vec<usize>,
}

impl CellData {
    fn new(content: String, i_horizontal: Vec<usize>, i_vertical: Vec<usize>) -> Self {
        Self {
            content,
            intersections_horizontal: i_horizontal,
            intersections_vertical: i_vertical,
        }
    }
}

pub(super) fn collapsed_table(value: &Value, cfg: &Config) -> String {
    let dims = collect_table_dimensions(value, cfg);
    let ctx = PrintContext {
        is_last_col: true,
        is_last_row: true,
        is_first_col: true,
        is_first_row: true,
        size: *dims.all.get(&0).unwrap(),
        ..Default::default()
    };
    _collapsed_table(value, cfg, &dims, ctx).content
}

fn _collapsed_table(val: &Value, cfg: &Config, dims: &Dimensions, ctx: PrintContext) -> CellData {
    match val {
        Value::String(..) | Value::Bool(..) | Value::Number(..) | Value::Null => {
            let value = match val {
                Value::String(val) => val.to_string(),
                Value::Bool(val) => val.to_string(),
                Value::Number(val) => val.to_string(),
                Value::Null => String::new(),
                _ => unreachable!(),
            };

            generate_value_cell(&value, cfg, ctx)
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                // a corner case where the object must behave as empty string
                return _collapsed_table(&Value::Null, cfg, dims, ctx);
            }

            match cfg.object_orientation {
                Orientation::Column => generate_vertical_object(obj, cfg, dims, ctx),
                Orientation::Row => generate_horizontal_object(obj, cfg, dims, ctx),
            }
        }
        Value::Array(list) => {
            if list.is_empty() {
                // a corner case where the list must behave as empty string
                return _collapsed_table(&Value::Null, cfg, dims, ctx);
            }

            match cfg.array_orientation {
                Orientation::Column => generate_vertical_array(list, cfg, dims, ctx),
                Orientation::Row => generate_horizontal_array(list, cfg, dims, ctx),
            }
        }
    }
}

fn generate_vertical_array(
    list: &[Value],
    cfg: &Config,
    dims: &Dimensions,
    ctx: PrintContext,
) -> CellData {
    let array_dims = dims.arrays.get(&ctx.pos).unwrap();

    let height = dims.all.get(&ctx.pos).unwrap().height;
    let additional_height = ctx.size.height - height;
    let (chunk_height, rest_height) = split_value(additional_height, list.len());

    let mut intersections_horizontal = ctx.intersections_horizontal;
    let mut intersections_vertical = ctx.intersections_vertical;
    let mut next_vsplit = false;
    let mut next_intersections_vertical = vec![];

    let mut builder = Builder::new();
    for (i, val) in list.iter().enumerate() {
        let val_pos = *array_dims.index.get(&i).unwrap();

        let mut height = dims.all.get(&val_pos).unwrap().height + chunk_height;
        if i == 0 {
            height += rest_height;
        }
        let size = Dim::new(ctx.size.width, height);

        let (split, intersections_vertical) =
            short_splits3(&mut intersections_vertical, size.height);
        let old_split = next_vsplit;
        next_vsplit = split;

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
            no_left: ctx.no_left,
            no_right: ctx.no_right,
            no_bottom: ctx.no_bottom && i + 1 == list.len(),
            lean_top: ctx.lean_top && i == 0,
            top_intersection: (ctx.top_intersection && i == 0) || old_split,
            top_left: ctx.top_left || i > 0,
            intersections_horizontal,
            intersections_vertical,
            size,
        };

        let data = _collapsed_table(val, cfg, dims, valctx);
        intersections_horizontal = data.intersections_horizontal;
        next_intersections_vertical.extend(data.intersections_vertical);

        builder.push_record([data.content]);
    }

    let table = builder
        .build()
        .with(Style::empty())
        .with(Padding::zero())
        .to_string();

    CellData::new(table, intersections_horizontal, next_intersections_vertical)
}

fn generate_horizontal_array(
    list: &[Value],
    cfg: &Config,
    dims: &Dimensions,
    ctx: PrintContext,
) -> CellData {
    let array_dims = dims.arrays.get(&ctx.pos).unwrap();

    let list_width = dims.all.get(&ctx.pos).unwrap().width;
    let additional_width = ctx.size.width - list_width;
    let (chunk_width, rest_width) = split_value(additional_width, list.len());

    let mut intersections_horizontal = ctx.intersections_horizontal;
    let mut intersections_vertical = ctx.intersections_vertical;
    let mut new_intersections_horizontal = vec![];
    let mut split_next = false;

    let mut buf = Vec::with_capacity(list.len());
    for (i, val) in list.iter().enumerate() {
        let val_pos = *array_dims.index.get(&i).unwrap();

        let mut width = dims.all.get(&val_pos).unwrap().width + chunk_width;
        if i == 0 {
            width += rest_width;
        }
        let size = Dim::new(width, ctx.size.height);

        let (split, intersections_horizontal) = short_splits3(&mut intersections_horizontal, width);
        let old_split = split_next;
        split_next = split;

        let is_prev_list_not_first = ctx.list && !ctx.list_is_first;
        let valctx = PrintContext {
            pos: val_pos,
            is_first_col: ctx.is_first_col && i == 0,
            is_last_col: ctx.is_last_col && i + 1 == list.len(),
            is_last_row: ctx.is_last_row,
            is_first_row: ctx.is_first_row,
            kv: false,
            kv_is_first: false,
            list: false,
            list_is_first: !is_prev_list_not_first,
            no_left: false,
            no_right: !(ctx.is_last_col && i + 1 == list.len()),
            no_bottom: false,
            lean_top: !(ctx.is_first_col && i == 0),
            top_intersection: (ctx.top_intersection && i == 0) || old_split,
            top_left: ctx.top_left && i == 0,
            intersections_horizontal,
            intersections_vertical,
            size,
        };

        let val = _collapsed_table(val, cfg, dims, valctx);

        intersections_vertical = val.intersections_vertical;

        new_intersections_horizontal.extend(val.intersections_horizontal.iter());
        let value = val.content;

        buf.push(value);
    }

    let mut builder = Builder::with_capacity(1, buf.len());
    builder.push_record(buf);

    let table = builder
        .build()
        .with(Style::empty())
        .with(Padding::zero())
        .to_string();

    CellData::new(table, new_intersections_horizontal, intersections_vertical)
}

fn generate_vertical_object(
    obj: &Map<String, Value>,
    cfg: &Config,
    dims: &Dimensions,
    ctx: PrintContext,
) -> CellData {
    let map_dims = dims.maps.get(&ctx.pos).unwrap();
    let max_key_width = map_dims.key_max.width;

    let has_vertical = cfg.cfg.get_borders().has_left();
    let has_horizontal = cfg.cfg.get_borders().has_top();

    let value_width = ctx.size.width - max_key_width - has_vertical as usize;

    let mut intersections_horizontal = ctx.intersections_horizontal;
    let mut intersections_vertical = ctx.intersections_vertical;
    let mut next_vertical_intersections = vec![];

    let (is_key_intersect, key_splits) =
        short_splits3(&mut intersections_horizontal, max_key_width);

    let mut builder = Builder::new();
    for (i, (key, val)) in obj.iter().enumerate() {
        let val_pos = *map_dims.index.get(&i).unwrap();
        let key_pos = ctx.pos + i + 1;

        let key_height = dims.all.get(&key_pos).unwrap().height;
        let val_height = dims.all.get(&val_pos).unwrap().height;
        let entry_height = cmp::max(key_height, val_height);

        let is_last_row = ctx.is_last_row && i + 1 == obj.len();
        let is_first_row = ctx.is_first_row && i == 0;

        let valctx = PrintContext {
            pos: val_pos,
            is_first_row,
            is_last_row,
            is_first_col: false,
            is_last_col: ctx.is_last_col,
            kv: true,
            kv_is_first: i == 0,
            list: false,
            list_is_first: false,
            no_left: false,
            no_right: !ctx.is_last_col,
            no_bottom: false,
            lean_top: i == 0,
            top_intersection: i > 0 || (i == 0 && is_key_intersect),
            top_left: false,
            intersections_horizontal,
            intersections_vertical: vec![],
            size: Dim::new(value_width, entry_height),
        };

        let val = _collapsed_table(val, cfg, dims, valctx);
        intersections_horizontal = val.intersections_horizontal;
        next_vertical_intersections.extend(val.intersections_vertical);

        let (_, key_vsplits) = short_splits3(&mut intersections_vertical, entry_height);

        let key = config_string(key, &cfg.cfg, max_key_width, entry_height);

        let mut key = tabled::builder::Builder::from(vec![vec![key]]).build();
        key.with(cfg.cfg.clone());
        key.with(NoRightBorders);

        if !is_last_row {
            key.with(NoBottomBorders);
        }

        if !ctx.is_first_col && is_last_row {
            key.with(BottomLeftChangeToBottomIntersection);
        }

        if !is_first_row {
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

        if !ctx.is_first_col && is_first_row {
            key.with(TopLeftChangeTopIntersection);
        }

        if i == 0 && ctx.lean_top {
            key.with(TopLeftChangeTopIntersection);
        }

        if i == 0 && ctx.top_intersection {
            key.with(TopLeftChangeIntersection);
        }

        if i == 0 && has_vertical && !key_splits.is_empty() {
            let c = cfg.cfg.get_borders().bottom_intersection.unwrap_or(' ');
            key.with(SetTopChars(&key_splits, c));
        }

        if has_horizontal && !key_vsplits.is_empty() {
            let c = cfg.cfg.get_borders().right_intersection.unwrap_or(' ');
            key.with(SetLeftChars(&key_vsplits, c));
        }

        builder.push_record([key.to_string(), val.content]);
    }

    intersections_horizontal.insert(0, max_key_width);

    let table = builder
        .build()
        .with(Style::empty())
        .with(Padding::zero())
        .to_string();

    CellData::new(table, intersections_horizontal, next_vertical_intersections)
}

fn generate_horizontal_object(
    obj: &Map<String, Value>,
    cfg: &Config,
    dims: &Dimensions,
    ctx: PrintContext,
) -> CellData {
    let map_dims = dims.maps.get(&ctx.pos).unwrap();

    let key_height = map_dims.key_max.height;
    let has_horizontal = cfg.cfg.get_borders().has_top();
    let val_height = ctx.size.height - key_height - has_horizontal as usize;

    let map_width = dims.all.get(&ctx.pos).unwrap().width;
    let additional_width = ctx.size.width - map_width;
    let (chunk_width, rest_width) = split_value(additional_width, obj.len());

    let mut intersections_horizontal = ctx.intersections_horizontal;
    let mut intersections_vertical = ctx.intersections_vertical;
    let mut split_next = false;

    let (vsplit, mut first_key_intersections_horizontal) =
        short_splits3(&mut intersections_vertical, key_height);

    let mut row1 = Vec::with_capacity(obj.len());
    for (i, val) in obj.keys().enumerate() {
        let key_pos = ctx.pos + i + 1;
        let val_pos = *map_dims.index.get(&i).unwrap();

        let val_width = dims.all.get(&val_pos).unwrap().width;
        let key_width = dims.all.get(&key_pos).unwrap().width;

        let mut width = max(val_width, key_width) + chunk_width;
        if i == 0 {
            width += rest_width;
        }

        let (split, intersections_horizontal) = short_splits3(&mut intersections_horizontal, width);
        let old_split = split_next;
        split_next = split;

        let size = Dim::new(width, key_height);
        let is_prev_list_not_first = ctx.list && !ctx.list_is_first;
        let valctx = PrintContext {
            pos: key_pos,
            is_first_col: ctx.is_first_col && i == 0,
            is_last_col: ctx.is_last_col && i + 1 == obj.len(),
            is_last_row: false,
            is_first_row: ctx.is_first_row,
            kv: false,
            kv_is_first: false,
            list: false,
            list_is_first: !is_prev_list_not_first,
            no_left: false,
            no_right: !(ctx.is_last_col && i + 1 == obj.len()),
            no_bottom: true,
            lean_top: !(ctx.is_first_col && i == 0),
            top_intersection: (ctx.top_intersection && i == 0) || old_split,
            top_left: ctx.top_left && i == 0,
            intersections_horizontal,
            intersections_vertical: first_key_intersections_horizontal,
            size,
        };

        first_key_intersections_horizontal = vec![];

        let val = generate_value_cell(val, cfg, valctx);
        let value = val.content;

        row1.push(value);
    }

    let mut next_intersections_horizontal = vec![];

    let mut row2 = Vec::with_capacity(obj.len());
    for (i, val) in obj.values().enumerate() {
        let key_pos = ctx.pos + i + 1;
        let val_pos = *map_dims.index.get(&i).unwrap();

        let val_width = dims.all.get(&val_pos).unwrap().width;
        let key_width = dims.all.get(&key_pos).unwrap().width;

        let mut width = max(val_width, key_width) + chunk_width;
        if i == 0 {
            width += rest_width;
        }

        let size = Dim::new(width, val_height);
        let is_prev_list_not_first = ctx.list && !ctx.list_is_first;
        let valctx = PrintContext {
            pos: val_pos,
            is_first_col: ctx.is_first_col && i == 0,
            is_last_col: ctx.is_last_col && i + 1 == obj.len(),
            is_last_row: ctx.is_last_row,
            is_first_row: false,
            kv: true,
            kv_is_first: false,
            list: true,
            list_is_first: !is_prev_list_not_first,
            no_left: false,
            no_right: !(ctx.is_last_col && i + 1 == obj.len()),
            no_bottom: false,
            lean_top: false,
            top_intersection: i > 0 || (i == 0 && vsplit),
            top_left: i == 0,
            intersections_horizontal: vec![],
            intersections_vertical,
            size,
        };

        let val = _collapsed_table(val, cfg, dims, valctx);

        intersections_vertical = val.intersections_vertical;
        next_intersections_horizontal.extend(val.intersections_horizontal);

        row2.push(val.content);
    }

    intersections_vertical.insert(0, key_height);

    let mut builder = Builder::with_capacity(2, obj.len());
    builder.push_record(row1);
    builder.push_record(row2);

    let table = builder
        .build()
        .with(Style::empty())
        .with(Padding::zero())
        .to_string();

    CellData::new(table, next_intersections_horizontal, intersections_vertical)
}

fn generate_value_cell(value: &str, cfg: &Config, ctx: PrintContext) -> CellData {
    let value = config_string(value, &cfg.cfg, ctx.size.width, ctx.size.height);

    let mut table = tabled::builder::Builder::from(vec![vec![value]]).build();
    table.with(cfg.cfg.clone());

    if !ctx.is_last_row || ctx.no_bottom {
        table.with(NoBottomBorders);
    }

    if ctx.no_right {
        table.with(NoRightBorders);
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

    if ctx.lean_top {
        table.with(TopLeftChangeTopIntersection);
    }

    if ctx.top_left {
        table.with(TopLeftChangeToLeft);
    }

    if ctx.top_intersection {
        table.with(TopLeftChangeIntersection);
    }

    let has_vertical = cfg.cfg.get_borders().has_left();
    if !ctx.intersections_horizontal.is_empty() && has_vertical {
        let mut splits = ctx.intersections_horizontal;
        let mut splits = short_splits(&mut splits, ctx.size.width);
        squash_splits(&mut splits);

        let c = cfg.cfg.get_borders().bottom_intersection.unwrap_or(' ');
        table.with(SetTopChars(&splits, c));
    }

    let has_horizontal = cfg.cfg.get_borders().has_top();
    if !ctx.intersections_vertical.is_empty() && has_horizontal {
        let mut splits = ctx.intersections_vertical;
        let mut splits = short_splits(&mut splits, ctx.size.width);
        squash_splits(&mut splits);

        let c = cfg.cfg.get_borders().right_intersection.unwrap_or(' ');
        table.with(SetLeftChars(&splits, c));
    }

    let table = table.to_string();

    CellData::new(table, vec![ctx.size.width], vec![ctx.size.height])
}

#[allow(dead_code)]
struct NoTopBorders;

impl<R, D> TableOption<R, ColoredConfig, D> for NoTopBorders {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top = None;
        borders.top_intersection = None;
        borders.top_left = None;
        borders.top_right = None;

        cfg.set_borders(borders);
    }
}

struct NoBottomBorders;

impl<R, D> TableOption<R, ColoredConfig, D> for NoBottomBorders {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.bottom = None;
        borders.bottom_intersection = None;
        borders.bottom_left = None;
        borders.bottom_right = None;

        cfg.set_borders(borders);
    }
}

struct NoRightBorders;

impl<R, D> TableOption<R, ColoredConfig, D> for NoRightBorders {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_right = None;
        borders.bottom_right = None;
        borders.right = None;
        borders.right_intersection = None;

        cfg.set_borders(borders);
    }
}

#[allow(dead_code)]
struct NoLeftBorders;

impl<R, D> TableOption<R, ColoredConfig, D> for NoLeftBorders {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_left = None;
        borders.bottom_left = None;
        borders.left = None;
        borders.left_intersection = None;

        cfg.set_borders(borders);
    }
}

struct TopLeftChangeTopIntersection;

impl<R, D> TableOption<R, ColoredConfig, D> for TopLeftChangeTopIntersection {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_left = borders.top_intersection;

        cfg.set_borders(borders);
    }
}

struct TopLeftChangeIntersection;

impl<R, D> TableOption<R, ColoredConfig, D> for TopLeftChangeIntersection {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_left = borders.intersection;

        cfg.set_borders(borders);
    }
}

struct TopLeftChangeToLeft;

impl<R, D> TableOption<R, ColoredConfig, D> for TopLeftChangeToLeft {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_left = borders.left_intersection;

        cfg.set_borders(borders);
    }
}

struct TopRightChangeToRight;

impl<R, D> TableOption<R, ColoredConfig, D> for TopRightChangeToRight {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.top_right = borders.right_intersection;

        cfg.set_borders(borders);
    }
}

#[allow(dead_code)]
struct BottomLeftChangeSplit;

impl<R, D> TableOption<R, ColoredConfig, D> for BottomLeftChangeSplit {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.bottom_left = borders.left_intersection;

        cfg.set_borders(borders);
    }
}

#[allow(dead_code)]
struct BottomLeftChangeSplitToIntersection;

impl<R, D> TableOption<R, ColoredConfig, D> for BottomLeftChangeSplitToIntersection {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.bottom_left = borders.intersection;

        cfg.set_borders(borders);
    }
}

#[allow(dead_code)]
struct BottomRightChangeToRight;

impl<R, D> TableOption<R, ColoredConfig, D> for BottomRightChangeToRight {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.bottom_right = borders.right_intersection;

        cfg.set_borders(borders);
    }
}

struct BottomLeftChangeToBottomIntersection;

impl<R, D> TableOption<R, ColoredConfig, D> for BottomLeftChangeToBottomIntersection {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.bottom_left = borders.bottom_intersection;

        cfg.set_borders(borders);
    }
}

#[allow(dead_code)]
struct SetBottomChars<'a>(&'a [usize], char);

impl<R, D> TableOption<R, ColoredConfig, D> for SetBottomChars<'_>
where
    R: Records,
    for<'a> &'a R: Records,
    for<'a> D: Dimension + Estimate<&'a R, SpannedConfig>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
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

impl<R, D> TableOption<R, ColoredConfig, D> for SetTopChars<'_> {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        for &split in self.0 {
            let offset = split;
            cfg.set_horizontal_char((0, 0), self.1, Offset::Begin(offset));
        }
    }
}

struct SetLeftChars<'a>(&'a [usize], char);

impl<R, D> TableOption<R, ColoredConfig, D> for SetLeftChars<'_> {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        for &offset in self.0 {
            cfg.set_vertical_char((0, 0), self.1, Offset::Begin(offset));
        }
    }
}

struct GetTopIntersection(char);

impl<R, D> TableOption<R, ColoredConfig, D> for &mut GetTopIntersection {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        self.0 = cfg.get_borders().top_intersection.unwrap_or(' ');
    }
}

struct GetBottomIntersection(char);

impl<R, D> TableOption<R, ColoredConfig, D> for &mut GetBottomIntersection {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
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
            let mut total_width = 0;
            let mut count_elements = obj.len() * 2;
            let mut val_pos = pos + 1 + obj.len();
            for (i, (key, val)) in obj.iter().enumerate() {
                let key_pos = pos + i + 1;

                let key = str_dimension(key, cfg);
                let (val, elements) = __collect_table_dims(buf, val, cfg, val_pos);
                count_elements += elements;

                total_height += max(key.height, val.height);
                total_width += max(key.width, val.width);

                index.key_max.width = max(index.key_max.width, key.width);
                index.key_max.height = max(index.key_max.height, key.height);
                index.value_max.width = max(index.value_max.width, val.width);
                index.value_max.height = max(index.value_max.height, val.height);

                buf.all.insert(key_pos, key);
                buf.all.insert(val_pos, val);

                index.index.insert(i, val_pos);

                val_pos += elements + 1;
            }

            let key_max = index.key_max;
            let val_max = index.value_max;

            buf.maps.insert(pos, index);

            let has_vertical = cfg.cfg.get_borders().has_left();
            let has_horizontal = cfg.cfg.get_borders().has_top();

            match cfg.object_orientation {
                Orientation::Column => {
                    let total_width = key_max.width + val_max.width + has_vertical as usize;
                    total_height += has_horizontal as usize * (obj.len() - 1);

                    (Dim::new(total_width, total_height), count_elements)
                }
                Orientation::Row => {
                    let total_height = key_max.height + val_max.height + has_horizontal as usize;
                    total_width += has_vertical as usize * (obj.len() - 1);

                    (Dim::new(total_width, total_height), count_elements)
                }
            }
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
            let mut total_width = 0;

            let mut count_elements = list.len();
            let mut val_pos = pos + 1;
            for (i, value) in list.iter().enumerate() {
                let (dim, elements) = __collect_table_dims(buf, value, cfg, val_pos);
                count_elements += elements;

                total_height += dim.height;
                total_width += dim.width;

                index.max.width = max(index.max.width, dim.width);
                index.max.height = max(index.max.height, dim.height);

                buf.all.insert(val_pos, dim);

                index.index.insert(i, val_pos);

                val_pos += 1 + elements;
            }

            let max_width = index.max.width;
            let max_height = index.max.height;

            buf.arrays.insert(pos, index);

            match cfg.array_orientation {
                Orientation::Column => {
                    let has_horizontal = cfg.cfg.get_borders().has_top();
                    total_height += has_horizontal as usize * (list.len() - 1);

                    (Dim::new(max_width, total_height), count_elements)
                }
                Orientation::Row => {
                    let has_vertical = cfg.cfg.get_borders().has_left();
                    total_width += has_vertical as usize * (list.len() - 1);

                    (Dim::new(total_width, max_height), count_elements)
                }
            }
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
    pad.left.size + pad.right.size
}

fn get_padding_vertical(cfg: &Config) -> usize {
    let pad = cfg.cfg.get_padding(Entity::Global);
    pad.top.size + pad.bottom.size
}

fn split_value(value: usize, by: usize) -> (usize, usize) {
    let val = value / by;
    let rest = value - val * by;
    (val, rest)
}

fn config_string(value: &str, cfg: &ColoredConfig, width: usize, height: usize) -> String {
    let pad = cfg.get_padding(Entity::Global);
    let width = width - pad.left.size - pad.right.size;
    let height = height - pad.bottom.size - pad.top.size;
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
        let rest = width - pos;
        splits[0] -= rest;
    }

    out
}

fn short_splits3(splits: &mut Vec<usize>, width: usize) -> (bool, Vec<usize>) {
    if splits.is_empty() {
        return (false, Vec::new());
    }

    let mut out = Vec::new();
    let mut pos = 0;
    for &split in splits.iter() {
        if pos + split >= width {
            break;
        }

        pos += split + 1;
        out.push(split);
    }

    splits.drain(..out.len());

    if splits.is_empty() {
        return (false, out);
    }

    if pos <= width {
        splits[0] -= width - pos;
        if splits[0] > 0 {
            splits[0] -= 1;
        } else {
            splits.remove(0);
            return (true, out);
        }
    }

    (false, out)
}

fn squash_splits(splits: &mut [usize]) {
    splits.iter_mut().enumerate().for_each(|(i, s)| *s += i);
}
