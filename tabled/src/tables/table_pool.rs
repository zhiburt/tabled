use core::fmt::{self, Display, Formatter};

use crate::{
    grid::{
        config::{AlignmentHorizontal, CompactMultilineConfig, Indent, Sides},
        dimension::{DimensionPriority, PoolTableDimension},
        records::EmptyRecords,
        records::IntoRecords,
    },
    settings::{Style, TableOption},
};

/// [`PoolTable`] is a table which allows a greater set of possibilities for cell alignment.
/// It's data is not aligned in any way by default.
///
/// It works similar to the main [`Table`] by default.
///
///
/// ```
/// use tabled::tables::PoolTable;
///
/// let data = vec![
///     vec!["Hello", "World", "!"],
///     vec!["Salve", "mondo", "!"],
///     vec!["Hola", "mundo", "!"],
/// ];
///
/// let table = PoolTable::new(data).to_string();
///
/// assert_eq!(
///     table,
///     "+-------+-------+---+\n\
///      | Hello | World | ! |\n\
///      +-------+-------+---+\n\
///      | Salve | mondo | ! |\n\
///      +-------+-------+---+\n\
///      | Hola  | mundo | ! |\n\
///      +-------+-------+---+"
/// )
/// ```
///
/// But it allows you to have a different number of columns inside the rows.
///
/// ```
/// use tabled::tables::PoolTable;
///
/// let data = vec![
///     vec!["Hello", "World", "!"],
///     vec!["Salve, mondo!"],
///     vec!["Hola", "mundo", "", "", "!"],
/// ];
///
/// let table = PoolTable::new(data).to_string();
///
/// assert_eq!(
///     table,
///     "+---------+---------+----+\n\
///      | Hello   | World   | !  |\n\
///      +---------+---------+----+\n\
///      | Salve, mondo!          |\n\
///      +------+-------+--+--+---+\n\
///      | Hola | mundo |  |  | ! |\n\
///      +------+-------+--+--+---+"
/// )
/// ```
///
/// Notice that you also can build a custom table layout by using [`TableValue`].
///
/// ```
/// use tabled::tables::{PoolTable, TableValue};
///
/// let message = "Hello\nWorld";
///
/// let data = TableValue::Column(vec![
///     TableValue::Row(vec![
///         TableValue::Column(vec![
///             TableValue::Cell(String::from(message)),
///         ]),
///         TableValue::Column(vec![
///             TableValue::Cell(String::from(message)),
///             TableValue::Row(vec![
///                 TableValue::Cell(String::from(message)),
///                 TableValue::Cell(String::from(message)),
///                 TableValue::Cell(String::from(message)),
///             ])
///         ]),
///     ]),
///     TableValue::Cell(String::from(message)),
/// ]);
///
/// let table = PoolTable::from(data).to_string();
///
/// assert_eq!(
///     table,
///     "+-------+-----------------------+\n\
///      | Hello | Hello                 |\n\
///      | World | World                 |\n\
///      |       +-------+-------+-------+\n\
///      |       | Hello | Hello | Hello |\n\
///      |       | World | World | World |\n\
///      +-------+-------+-------+-------+\n\
///      | Hello                         |\n\
///      | World                         |\n\
///      +-------------------------------+"
/// )
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PoolTable {
    config: CompactMultilineConfig,
    dims: PoolTableDimension,
    value: TableValue,
}

impl PoolTable {
    /// Creates a [`PoolTable`] out from a record iterator.
    pub fn new<I: IntoRecords>(iter: I) -> Self {
        let value = TableValue::Column(
            iter.iter_rows()
                .into_iter()
                .map(|row| {
                    TableValue::Row(
                        row.into_iter()
                            .map(|cell| cell.as_ref().to_string())
                            .map(TableValue::Cell)
                            .collect(),
                    )
                })
                .collect(),
        );

        Self {
            config: configure_grid(),
            dims: PoolTableDimension::new(DimensionPriority::List, DimensionPriority::List),
            value,
        }
    }

    /// A is a generic function which applies options to the [`PoolTable`] configuration.
    ///
    /// Notice that it has a limited support of options.
    ///
    /// ```
    /// use tabled::tables::PoolTable;
    /// use tabled::settings::{Style, Padding};
    ///
    /// let data = vec![
    ///     vec!["Hello", "World", "!"],
    ///     vec!["Salve", "mondo", "!"],
    ///     vec!["Hola", "mundo", "!"],
    /// ];
    ///
    /// let table = PoolTable::new(data)
    ///     .with(Style::extended())
    ///     .with(Padding::zero())
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "╔═════╦═════╦═╗\n\
    ///      ║Hello║World║!║\n\
    ///      ╠═════╬═════╬═╣\n\
    ///      ║Salve║mondo║!║\n\
    ///      ╠═════╬═════╬═╣\n\
    ///      ║Hola ║mundo║!║\n\
    ///      ╚═════╩═════╩═╝"
    /// )
    /// ```
    pub fn with<O>(&mut self, option: O) -> &mut Self
    where
        O: TableOption<EmptyRecords, PoolTableDimension, CompactMultilineConfig>,
    {
        let mut records = EmptyRecords::default();
        option.change(&mut records, &mut self.config, &mut self.dims);

        self
    }
}

impl From<TableValue> for PoolTable {
    fn from(value: TableValue) -> Self {
        Self {
            config: configure_grid(),
            dims: PoolTableDimension::new(DimensionPriority::List, DimensionPriority::List),
            value,
        }
    }
}

impl Display for PoolTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        print::build_table(&self.value, &self.config, self.dims).fmt(f)
    }
}

/// [`TableValue`] a structure which is responsible for a [`PoolTable`] layout.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableValue {
    /// A horizontal row.
    Row(Vec<TableValue>),
    /// A vertical column.
    Column(Vec<TableValue>),
    /// A single cell.
    Cell(String),
}

fn configure_grid() -> CompactMultilineConfig {
    let pad = Sides::new(
        Indent::spaced(1),
        Indent::spaced(1),
        Indent::default(),
        Indent::default(),
    );

    CompactMultilineConfig::default()
        .set_padding(pad)
        .set_alignment_horizontal(AlignmentHorizontal::Left)
        .set_borders(*Style::ascii().get_borders())
}

impl<R, C> TableOption<R, PoolTableDimension, C> for PoolTableDimension {
    fn change(self, _: &mut R, _: &mut C, dimension: &mut PoolTableDimension) {
        *dimension = self;
    }
}

impl<R, D> TableOption<R, D, CompactMultilineConfig> for CompactMultilineConfig {
    fn change(self, _: &mut R, config: &mut CompactMultilineConfig, _: &mut D) {
        *config = self;
    }
}

mod print {
    use std::{cmp::max, collections::HashMap, iter::repeat};

    use papergrid::{
        color::StaticColor,
        config::{Border, Borders},
        util::string::string_width_multiline,
    };

    use crate::{
        builder::Builder,
        grid::{
            config::{
                AlignmentHorizontal, AlignmentVertical, ColoredConfig, CompactMultilineConfig,
                Indent, Offset, Sides,
            },
            dimension::{Dimension, DimensionPriority, Estimate, PoolTableDimension},
            records::Records,
            util::string::{count_lines, get_lines, string_dimension, string_width},
        },
        settings::{Padding, Style, TableOption},
    };

    use super::TableValue;

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

    pub(super) fn build_table(
        val: &TableValue,
        cfg: &CompactMultilineConfig,
        dims_priority: PoolTableDimension,
    ) -> String {
        let dims = collect_table_dimensions(val, cfg);
        let ctx = PrintContext {
            is_last_col: true,
            is_last_row: true,
            is_first_col: true,
            is_first_row: true,
            size: *dims.all.get(&0).unwrap(),
            ..Default::default()
        };

        let data = _build_table(val, cfg, &dims, dims_priority, ctx);
        let mut table = data.content;

        let margin = cfg.get_margin();
        let has_margin = margin.top.size > 0
            || margin.bottom.size > 0
            || margin.left.size > 0
            || margin.right.size > 0;
        if has_margin {
            let color = convert_border_colors(cfg.get_margin_color());
            table = set_margin(&table, *margin, color);
        }

        table
    }

    fn _build_table(
        val: &TableValue,
        cfg: &CompactMultilineConfig,
        dims: &Dimensions,
        priority: PoolTableDimension,
        ctx: PrintContext,
    ) -> CellData {
        match val {
            TableValue::Cell(text) => generate_value_cell(text, cfg, ctx),
            TableValue::Row(list) => {
                if list.is_empty() {
                    return generate_value_cell("", cfg, ctx);
                }

                generate_table_row(list, cfg, dims, priority, ctx)
            }
            TableValue::Column(list) => {
                if list.is_empty() {
                    return generate_value_cell("", cfg, ctx);
                }

                generate_table_column(list, cfg, dims, priority, ctx)
            }
        }
    }

    fn generate_table_column(
        list: &Vec<TableValue>,
        cfg: &CompactMultilineConfig,
        dims: &Dimensions,
        priority: PoolTableDimension,
        ctx: PrintContext,
    ) -> CellData {
        let array_dims = dims.arrays.get(&ctx.pos).unwrap();

        let height = dims.all.get(&ctx.pos).unwrap().height;
        let additional_height = ctx.size.height - height;
        let (chunk_height, mut rest_height) = split_value(additional_height, list.len());

        let mut intersections_horizontal = ctx.intersections_horizontal;
        let mut intersections_vertical = ctx.intersections_vertical;
        let mut next_vsplit = false;
        let mut next_intersections_vertical = vec![];

        let mut builder = Builder::new();
        for (i, val) in list.iter().enumerate() {
            let val_pos = *array_dims.index.get(&i).unwrap();

            let mut height = dims.all.get(&val_pos).unwrap().height;
            match priority.height() {
                DimensionPriority::First => {
                    if i == 0 {
                        height += additional_height;
                    }
                }
                DimensionPriority::Last => {
                    if i + 1 == list.len() {
                        height += additional_height;
                    }
                }
                DimensionPriority::List => {
                    height += chunk_height;

                    if rest_height > 0 {
                        height += 1;
                        rest_height -= 1; // must be safe
                    }
                }
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

            let data = _build_table(val, cfg, dims, priority, valctx);
            intersections_horizontal = data.intersections_horizontal;
            next_intersections_vertical.extend(data.intersections_vertical);

            let _ = builder.push_record([data.content]);
        }

        let table = builder
            .build()
            .with(Style::empty())
            .with(Padding::zero())
            .to_string();

        CellData::new(table, intersections_horizontal, next_intersections_vertical)
    }

    fn generate_table_row(
        list: &Vec<TableValue>,
        cfg: &CompactMultilineConfig,
        dims: &Dimensions,
        priority: PoolTableDimension,
        ctx: PrintContext,
    ) -> CellData {
        let array_dims = dims.arrays.get(&ctx.pos).unwrap();

        let list_width = dims.all.get(&ctx.pos).unwrap().width;
        let additional_width = ctx.size.width - list_width;
        let (chunk_width, mut rest_width) = split_value(additional_width, list.len());

        let mut intersections_horizontal = ctx.intersections_horizontal;
        let mut intersections_vertical = ctx.intersections_vertical;
        let mut new_intersections_horizontal = vec![];
        let mut split_next = false;

        let mut buf = Vec::with_capacity(list.len());
        for (i, val) in list.iter().enumerate() {
            let val_pos = *array_dims.index.get(&i).unwrap();

            let mut width = dims.all.get(&val_pos).unwrap().width;
            match priority.width() {
                DimensionPriority::First => {
                    if i == 0 {
                        width += additional_width;
                    }
                }
                DimensionPriority::Last => {
                    if i + 1 == list.len() {
                        width += additional_width;
                    }
                }
                DimensionPriority::List => {
                    width += chunk_width;

                    if rest_width > 0 {
                        width += 1;
                        rest_width -= 1; // must be safe
                    }
                }
            }

            let size = Dim::new(width, ctx.size.height);

            let (split, intersections_horizontal) =
                short_splits3(&mut intersections_horizontal, width);
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

            let val = _build_table(val, cfg, dims, priority, valctx);
            intersections_vertical = val.intersections_vertical;
            new_intersections_horizontal.extend(val.intersections_horizontal.iter());
            let value = val.content;

            buf.push(value);
        }

        let mut b = Builder::with_capacity(1);
        let _ = b.hint_column_size(buf.len()).push_record(buf);
        let table = b
            .build()
            .with(Style::empty())
            .with(Padding::zero())
            .to_string();

        CellData::new(table, new_intersections_horizontal, intersections_vertical)
    }

    fn generate_value_cell(
        text: &str,
        cfg: &CompactMultilineConfig,
        ctx: PrintContext,
    ) -> CellData {
        let width = ctx.size.width;
        let height = ctx.size.height;
        let table = generate_value_table(text, cfg, ctx);
        CellData::new(table, vec![width], vec![height])
    }

    fn generate_value_table(
        text: &str,
        cfg: &CompactMultilineConfig,
        mut ctx: PrintContext,
    ) -> String {
        if ctx.size.width == 0 || ctx.size.height == 0 {
            return String::new();
        }

        let halignment = cfg.get_alignment_horizontal();
        let valignment = cfg.get_alignment_vertical();
        let pad = cfg.get_padding();
        let pad_color = cfg.get_padding_color();
        let pad_color = convert_border_colors(pad_color);
        let lines_alignemnt = cfg.get_formatting().allow_lines_alignment;

        let mut borders = *cfg.get_borders();

        let bottom_intesection = cfg.get_borders().bottom_intersection.unwrap_or(' ');
        let mut horizontal_splits = short_splits(&mut ctx.intersections_horizontal, ctx.size.width);
        squash_splits(&mut horizontal_splits);

        let right_intersection = borders.right_intersection.unwrap_or(' ');
        let mut vertical_splits = short_splits(&mut ctx.intersections_vertical, ctx.size.height);
        squash_splits(&mut vertical_splits);

        config_borders(&mut borders, &ctx);
        let border = create_border(borders);

        let borders_colors = *cfg.get_borders_color();
        let border_color = create_border(borders_colors);

        let mut height = ctx.size.height;
        height -= pad.top.size + pad.bottom.size;

        let mut width = ctx.size.width;
        width -= pad.left.size + pad.right.size;

        let count_lines = count_lines(text);
        let (top, bottom) = indent_vertical(valignment, height, count_lines);

        let mut buf = String::new();
        print_top_line(
            &mut buf,
            border,
            border_color,
            &horizontal_splits,
            bottom_intesection,
            ctx.size.width,
        );

        let mut line_index = 0;
        let mut vertical_splits = &vertical_splits[..];

        for _ in 0..top {
            let mut border = border;
            if vertical_splits.first() == Some(&line_index) {
                border.left = Some(right_intersection);
                vertical_splits = &vertical_splits[1..];
            }

            print_line(&mut buf, border, border_color, None, ' ', ctx.size.width);
            line_index += 1;
        }

        for _ in 0..pad.top.size {
            let mut border = border;
            if vertical_splits.first() == Some(&line_index) {
                border.left = Some(right_intersection);
                vertical_splits = &vertical_splits[1..];
            }

            print_line(
                &mut buf,
                border,
                border_color,
                pad_color.top,
                pad.top.fill,
                ctx.size.width,
            );
            line_index += 1;
        }

        if lines_alignemnt {
            for line in get_lines(text) {
                let line_width = string_width(&line);
                let (left, right) = indent_horizontal(halignment, width, line_width);

                if border.has_left() {
                    let mut c = border.left.unwrap_or(' ');
                    if vertical_splits.first() == Some(&line_index) {
                        c = right_intersection;
                        vertical_splits = &vertical_splits[1..];
                    }

                    print_char(&mut buf, c, border_color.left);
                }

                print_chars(&mut buf, pad.left.fill, pad_color.left, pad.left.size);
                buf.extend(repeat(' ').take(left));
                buf.push_str(&line);
                buf.extend(repeat(' ').take(right));
                print_chars(&mut buf, pad.right.fill, pad_color.right, pad.right.size);

                if border.has_right() {
                    print_char(&mut buf, border.right.unwrap_or(' '), border_color.right);
                }

                buf.push('\n');

                line_index += 1;
            }
        } else {
            let text_width = string_width_multiline(text);
            let (left, _) = indent_horizontal(halignment, width, text_width);

            for line in get_lines(text) {
                let line_width = string_width(&line);
                let right = width - line_width - left;

                if border.has_left() {
                    let mut c = border.left.unwrap_or(' ');
                    if vertical_splits.first() == Some(&line_index) {
                        c = right_intersection;
                        vertical_splits = &vertical_splits[1..];
                    }

                    print_char(&mut buf, c, border_color.left);
                }

                print_chars(&mut buf, pad.left.fill, pad_color.left, pad.left.size);
                buf.extend(repeat(' ').take(left));
                buf.push_str(&line);
                buf.extend(repeat(' ').take(right));
                print_chars(&mut buf, pad.right.fill, pad_color.right, pad.right.size);

                if border.has_right() {
                    print_char(&mut buf, border.right.unwrap_or(' '), border_color.right);
                }

                buf.push('\n');

                line_index += 1;
            }
        }

        for _ in 0..pad.bottom.size {
            let mut border = border;
            if vertical_splits.first() == Some(&line_index) {
                border.left = Some(right_intersection);
                vertical_splits = &vertical_splits[1..];
            }

            print_line(
                &mut buf,
                border,
                border_color,
                pad_color.bottom,
                pad.bottom.fill,
                ctx.size.width,
            );

            line_index += 1;
        }

        for _ in 0..bottom {
            let mut border = border;
            if vertical_splits.first() == Some(&line_index) {
                border.left = Some(right_intersection);
                vertical_splits = &vertical_splits[1..];
            }

            print_line(&mut buf, border, border_color, None, ' ', ctx.size.width);
            line_index += 1;
        }

        print_bottom_line(&mut buf, border, border_color, ctx.size.width);

        let _ = buf.remove(buf.len() - 1);

        buf
    }

    fn print_chars(buf: &mut String, c: char, color: Option<StaticColor>, width: usize) {
        match color {
            Some(color) => {
                buf.push_str(color.get_prefix());
                buf.extend(repeat(c).take(width));
                buf.push_str(color.get_suffix());
            }
            None => buf.extend(repeat(c).take(width)),
        }
    }

    fn print_char(buf: &mut String, c: char, color: Option<StaticColor>) {
        match color {
            Some(color) => {
                buf.push_str(color.get_prefix());
                buf.push(c);
                buf.push_str(color.get_suffix());
            }
            None => buf.push(c),
        }
    }

    fn print_line(
        buf: &mut String,
        border: Border<char>,
        border_color: Border<StaticColor>,
        color: Option<StaticColor>,
        c: char,
        width: usize,
    ) {
        if border.has_left() {
            let c = border.left.unwrap_or(' ');
            print_char(buf, c, border_color.left);
        }

        print_chars(buf, c, color, width);

        if border.has_right() {
            let c = border.right.unwrap_or(' ');
            print_char(buf, c, border_color.right);
        }

        buf.push('\n');
    }

    fn print_top_line(
        buf: &mut String,
        border: Border<char>,
        color: Border<StaticColor>,
        splits: &[usize],
        split_char: char,
        width: usize,
    ) {
        if !border.has_top() {
            return;
        }

        let mut used_color: Option<StaticColor> = None;

        if border.has_left() {
            if let Some(color) = color.left_top_corner {
                used_color = Some(color);
                buf.push_str(color.get_prefix());
            }

            let c = border.left_top_corner.unwrap_or(' ');
            buf.push(c);
        }

        if let Some(color) = color.top {
            match used_color {
                Some(used) => {
                    if used != color {
                        buf.push_str(used.get_suffix());
                        buf.push_str(color.get_prefix());
                    }
                }
                None => {
                    buf.push_str(color.get_prefix());
                    used_color = Some(color);
                }
            }
        }

        let c = border.top.unwrap_or(' ');
        if splits.is_empty() {
            buf.extend(repeat(c).take(width));
        } else {
            let mut splits = splits;
            for i in 0..width {
                if splits.first() == Some(&i) {
                    buf.push(split_char);
                    splits = &splits[1..];
                } else {
                    buf.push(c);
                }
            }
        }

        if border.has_right() {
            if let Some(color) = color.right_top_corner {
                match used_color {
                    Some(used) => {
                        if used != color {
                            buf.push_str(used.get_suffix());
                            buf.push_str(color.get_prefix());
                        }
                    }
                    None => {
                        buf.push_str(color.get_prefix());
                        used_color = Some(color);
                    }
                }
            }

            let c = border.right_top_corner.unwrap_or(' ');
            buf.push(c);
        }

        if let Some(used) = used_color {
            buf.push_str(used.get_suffix());
        }

        buf.push('\n');
    }

    fn print_bottom_line(
        buf: &mut String,
        border: Border<char>,
        color: Border<StaticColor>,
        width: usize,
    ) {
        if !border.has_bottom() {
            return;
        }

        let mut used_color: Option<StaticColor> = None;

        if border.has_left() {
            if let Some(color) = color.left_bottom_corner {
                used_color = Some(color);
                buf.push_str(color.get_prefix());
            }

            let c = border.left_bottom_corner.unwrap_or(' ');
            buf.push(c);
        }

        if let Some(color) = color.bottom {
            match used_color {
                Some(used) => {
                    if used != color {
                        buf.push_str(used.get_suffix());
                        buf.push_str(color.get_prefix());
                    }
                }
                None => {
                    buf.push_str(color.get_prefix());
                    used_color = Some(color);
                }
            }
        }

        let c = border.bottom.unwrap_or(' ');
        buf.extend(repeat(c).take(width));

        if border.has_right() {
            if let Some(color) = color.right_bottom_corner {
                match used_color {
                    Some(used) => {
                        if used != color {
                            buf.push_str(used.get_suffix());
                            buf.push_str(color.get_prefix());
                        }
                    }
                    None => {
                        buf.push_str(color.get_prefix());
                        used_color = Some(color);
                    }
                }
            }

            let c = border.right_bottom_corner.unwrap_or(' ');
            buf.push(c);
        }

        if let Some(used) = used_color {
            buf.push_str(used.get_suffix());
        }

        buf.push('\n');
    }

    fn create_border<T>(borders: Borders<T>) -> Border<T> {
        Border {
            top: borders.top,
            bottom: borders.bottom,
            left: borders.left,
            right: borders.right,
            left_top_corner: borders.top_left,
            left_bottom_corner: borders.bottom_left,
            right_top_corner: borders.top_right,
            right_bottom_corner: borders.bottom_right,
        }
    }

    fn config_borders(borders: &mut Borders<char>, ctx: &PrintContext) {
        // set top_left
        {
            if ctx.kv && ctx.kv_is_first {
                borders.top_left = borders.top_intersection;
            }

            if ctx.kv && !ctx.kv_is_first {
                borders.top_left = borders.intersection;
            }

            if ctx.kv && ctx.list && !ctx.list_is_first {
                borders.top_left = borders.left_intersection;
            }

            if ctx.is_first_col && !ctx.is_first_row {
                borders.top_left = borders.left_intersection;
            }

            if ctx.lean_top {
                borders.top_left = borders.top_intersection;
            }

            if ctx.top_left {
                borders.top_left = borders.left_intersection;
            }

            if ctx.top_intersection {
                borders.top_left = borders.intersection;
            }
        }

        if ctx.is_last_col && !ctx.is_first_row {
            borders.top_right = borders.right_intersection;
        }

        if !ctx.is_first_col && ctx.is_last_row {
            borders.bottom_left = borders.bottom_intersection;
        }

        if !ctx.is_last_row || ctx.no_bottom {
            cfg_no_bottom_borders(borders);
        }

        if ctx.no_right {
            cfg_no_right_borders(borders);
        }
    }

    struct ConfigCell(PrintContext);

    impl<R, D> TableOption<R, D, ColoredConfig> for ConfigCell {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            {
                // we set a horizontal lines to borders to not complicate logic with cleaning it

                let mut borders = *cfg.get_borders();
                if let Some(line) = cfg.get_horizontal_line(0) {
                    borders.top = line.main;
                    borders.top_left = line.left;
                    borders.top_right = line.right;
                }

                if let Some(line) = cfg.get_horizontal_line(1) {
                    borders.bottom = line.main;
                    borders.bottom_left = line.left;
                    borders.bottom_right = line.right;
                }

                cfg.clear_theme();
                cfg.set_borders(borders);
            }

            let mut ctx = self.0;

            let has_vertical = cfg.get_borders().has_left();
            if !ctx.intersections_horizontal.is_empty() && has_vertical {
                let mut splits = short_splits(&mut ctx.intersections_horizontal, ctx.size.width);
                squash_splits(&mut splits);

                let c = cfg.get_borders().bottom_intersection.unwrap_or(' ');
                cfg_set_top_chars(cfg, &splits, c)
            }

            let has_horizontal = cfg.get_borders().has_top();
            if !ctx.intersections_vertical.is_empty() && has_horizontal {
                let mut splits = short_splits(&mut ctx.intersections_vertical, ctx.size.height);
                squash_splits(&mut splits);

                let c = cfg.get_borders().right_intersection.unwrap_or(' ');
                cfg_set_left_chars(cfg, &splits, c)
            }

            let mut borders = *cfg.get_borders();

            // set top_left
            {
                if ctx.kv && ctx.kv_is_first {
                    borders.top_left = borders.top_intersection;
                }

                if ctx.kv && !ctx.kv_is_first {
                    borders.top_left = borders.intersection;
                }

                if ctx.kv && ctx.list && !ctx.list_is_first {
                    borders.top_left = borders.left_intersection;
                }

                if ctx.is_first_col && !ctx.is_first_row {
                    borders.top_left = borders.left_intersection;
                }

                if ctx.lean_top {
                    borders.top_left = borders.top_intersection;
                }

                if ctx.top_left {
                    borders.top_left = borders.left_intersection;
                }

                if ctx.top_intersection {
                    borders.top_left = borders.intersection;
                }
            }

            if ctx.is_last_col && !ctx.is_first_row {
                borders.top_right = borders.right_intersection;
            }

            if !ctx.is_first_col && ctx.is_last_row {
                borders.bottom_left = borders.bottom_intersection;
            }

            if !ctx.is_last_row || ctx.no_bottom {
                cfg_no_bottom_borders(&mut borders);
            }

            if ctx.no_right {
                cfg_no_right_borders(&mut borders);
            }

            cfg.set_borders(borders);
        }
    }

    fn cfg_no_bottom_borders(borders: &mut Borders<char>) {
        borders.bottom = None;
        borders.bottom_intersection = None;
        borders.bottom_left = None;
        borders.bottom_right = None;
        borders.horizontal = None;
    }

    fn cfg_no_right_borders(borders: &mut Borders<char>) {
        borders.right = None;
        borders.right_intersection = None;
        borders.top_right = None;
        borders.bottom_right = None;
        borders.vertical = None;
    }

    fn cfg_set_top_chars(cfg: &mut ColoredConfig, list: &[usize], c: char) {
        for &split in list {
            let offset = split;
            cfg.set_horizontal_char((0, 0), c, Offset::Begin(offset));
        }
    }

    fn cfg_set_left_chars(cfg: &mut ColoredConfig, list: &[usize], c: char) {
        for &offset in list {
            cfg.set_vertical_char((0, 0), c, Offset::Begin(offset));
        }
    }

    struct NoTopBorders;

    impl<R, D> TableOption<R, D, ColoredConfig> for NoTopBorders {
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

    impl<R, D> TableOption<R, D, ColoredConfig> for NoBottomBorders {
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

    impl<R, D> TableOption<R, D, ColoredConfig> for NoRightBorders {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
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

    impl<R, D> TableOption<R, D, ColoredConfig> for TopLeftChangeTopIntersection {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.top_left = borders.top_intersection;

            cfg.set_borders(borders);
        }
    }

    struct TopLeftChangeIntersection;

    impl<R, D> TableOption<R, D, ColoredConfig> for TopLeftChangeIntersection {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.top_left = borders.intersection;

            cfg.set_borders(borders);
        }
    }

    struct TopLeftChangeToLeft;

    impl<R, D> TableOption<R, D, ColoredConfig> for TopLeftChangeToLeft {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.top_left = borders.left_intersection;

            cfg.set_borders(borders);
        }
    }

    struct TopRightChangeToRight;

    impl<R, D> TableOption<R, D, ColoredConfig> for TopRightChangeToRight {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.top_right = borders.right_intersection;

            cfg.set_borders(borders);
        }
    }

    struct BottomLeftChangeSplit;

    impl<R, D> TableOption<R, D, ColoredConfig> for BottomLeftChangeSplit {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.bottom_left = borders.left_intersection;

            cfg.set_borders(borders);
        }
    }

    struct BottomLeftChangeSplitToIntersection;

    impl<R, D> TableOption<R, D, ColoredConfig> for BottomLeftChangeSplitToIntersection {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.bottom_left = borders.intersection;

            cfg.set_borders(borders);
        }
    }

    struct BottomRightChangeToRight;

    impl<R, D> TableOption<R, D, ColoredConfig> for BottomRightChangeToRight {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            let mut borders = *cfg.get_borders();
            borders.bottom_right = borders.right_intersection;

            cfg.set_borders(borders);
        }
    }

    struct BottomLeftChangeToBottomIntersection;

    impl<R, D> TableOption<R, D, ColoredConfig> for BottomLeftChangeToBottomIntersection {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
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
        for<'a> D: Dimension + Estimate<&'a R, ColoredConfig>,
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

    impl<R, D> TableOption<R, D, ColoredConfig> for SetTopChars<'_> {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            for &split in self.0 {
                let offset = split;
                cfg.set_horizontal_char((0, 0), self.1, Offset::Begin(offset));
            }
        }
    }

    struct SetLeftChars<'a>(&'a [usize], char);

    impl<R, D> TableOption<R, D, ColoredConfig> for SetLeftChars<'_> {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            for &offset in self.0 {
                cfg.set_vertical_char((0, 0), self.1, Offset::Begin(offset));
            }
        }
    }

    struct GetTopIntersection(char);

    impl<R, D> TableOption<R, D, ColoredConfig> for &mut GetTopIntersection {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            self.0 = cfg.get_borders().top_intersection.unwrap_or(' ');
        }
    }

    struct GetBottomIntersection(char);

    impl<R, D> TableOption<R, D, ColoredConfig> for &mut GetBottomIntersection {
        fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
            self.0 = cfg.get_borders().bottom_intersection.unwrap_or(' ');
        }
    }

    #[derive(Debug, Default)]
    struct Dimensions {
        all: HashMap<usize, Dim>,
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
    struct ArrayDimensions {
        max: Dim,
        index: HashMap<usize, usize>,
    }

    fn collect_table_dimensions(val: &TableValue, cfg: &CompactMultilineConfig) -> Dimensions {
        let mut buf = Dimensions::default();
        let (dim, _) = __collect_table_dims(&mut buf, val, cfg, 0);
        let _ = buf.all.insert(0, dim);
        buf
    }

    fn __collect_table_dims(
        buf: &mut Dimensions,
        val: &TableValue,
        cfg: &CompactMultilineConfig,
        pos: usize,
    ) -> (Dim, usize) {
        match val {
            TableValue::Cell(text) => (str_dimension(text, cfg), 0),
            TableValue::Row(list) => {
                if list.is_empty() {
                    return (empty_dimension(cfg), 0);
                }

                let mut index = ArrayDimensions {
                    max: Dim::default(),
                    index: HashMap::with_capacity(list.len()),
                };

                let mut total_width = 0;

                let mut count_elements = list.len();
                let mut val_pos = pos + 1;
                for (i, value) in list.iter().enumerate() {
                    let (dim, elements) = __collect_table_dims(buf, value, cfg, val_pos);
                    count_elements += elements;

                    total_width += dim.width;

                    index.max.width = max(index.max.width, dim.width);
                    index.max.height = max(index.max.height, dim.height);

                    let _ = buf.all.insert(val_pos, dim);

                    let _ = index.index.insert(i, val_pos);

                    val_pos += 1 + elements;
                }

                let max_height = index.max.height;

                let _ = buf.arrays.insert(pos, index);

                let has_vertical = cfg.get_borders().has_left();
                total_width += has_vertical as usize * (list.len() - 1);

                (Dim::new(total_width, max_height), count_elements)
            }
            TableValue::Column(list) => {
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

                    total_height += dim.height;

                    index.max.width = max(index.max.width, dim.width);
                    index.max.height = max(index.max.height, dim.height);

                    let _ = buf.all.insert(val_pos, dim);

                    let _ = index.index.insert(i, val_pos);

                    val_pos += 1 + elements;
                }

                let max_width = index.max.width;

                let _ = buf.arrays.insert(pos, index);

                let has_horizontal = cfg.get_borders().has_top();
                total_height += has_horizontal as usize * (list.len() - 1);

                (Dim::new(max_width, total_height), count_elements)
            }
        }
    }

    fn empty_dimension(cfg: &CompactMultilineConfig) -> Dim {
        Dim::new(get_padding_horizontal(cfg), 1 + get_padding_vertical(cfg))
    }

    fn str_dimension(text: &str, cfg: &CompactMultilineConfig) -> Dim {
        let (count_lines, width) = string_dimension(text);
        let w = width + get_padding_horizontal(cfg);
        let h = count_lines + get_padding_vertical(cfg);
        Dim::new(w, h)
    }

    fn get_padding_horizontal(cfg: &CompactMultilineConfig) -> usize {
        let pad = cfg.get_padding();
        pad.left.size + pad.right.size
    }

    fn get_padding_vertical(cfg: &CompactMultilineConfig) -> usize {
        let pad = cfg.get_padding();
        pad.top.size + pad.bottom.size
    }

    fn split_value(value: usize, by: usize) -> (usize, usize) {
        let val = value / by;
        let rest = value - val * by;
        (val, rest)
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

        let _ = splits.drain(..out.len());

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

        let _ = splits.drain(..out.len());

        if splits.is_empty() {
            return (false, out);
        }

        if pos <= width {
            splits[0] -= width - pos;
            if splits[0] > 0 {
                splits[0] -= 1;
            } else {
                let _ = splits.remove(0);
                return (true, out);
            }
        }

        (false, out)
    }

    fn squash_splits(splits: &mut [usize]) {
        splits.iter_mut().enumerate().for_each(|(i, s)| *s += i);
    }

    fn set_margin(table: &str, margin: Sides<Indent>, color: Sides<Option<StaticColor>>) -> String {
        if table.is_empty() {
            return String::new();
        }

        let mut buf = String::new();
        let width = string_width_multiline(table);
        let top_color = color.top;
        let bottom_color = color.bottom;
        let left_color = color.left;
        let right_color = color.right;
        for _ in 0..margin.top.size {
            print_chars(&mut buf, margin.left.fill, left_color, margin.left.size);
            print_chars(&mut buf, margin.top.fill, top_color, width);
            print_chars(&mut buf, margin.right.fill, right_color, margin.right.size);
            buf.push('\n');
        }

        for line in get_lines(table) {
            print_chars(&mut buf, margin.left.fill, left_color, margin.left.size);
            buf.push_str(&line);
            print_chars(&mut buf, margin.right.fill, right_color, margin.right.size);
            buf.push('\n');
        }

        for _ in 0..margin.bottom.size {
            print_chars(&mut buf, margin.left.fill, left_color, margin.left.size);
            print_chars(&mut buf, margin.bottom.fill, bottom_color, width);
            print_chars(&mut buf, margin.right.fill, right_color, margin.right.size);
            buf.push('\n');
        }

        let _ = buf.remove(buf.len() - 1);

        buf
    }

    fn convert_border_colors(pad_color: Sides<StaticColor>) -> Sides<Option<StaticColor>> {
        Sides::new(
            (!pad_color.left.is_empty()).then(|| pad_color.left),
            (!pad_color.right.is_empty()).then(|| pad_color.right),
            (!pad_color.top.is_empty()).then(|| pad_color.top),
            (!pad_color.bottom.is_empty()).then(|| pad_color.bottom),
        )
    }
}
