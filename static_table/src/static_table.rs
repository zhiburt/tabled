use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use quote::ToTokens;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{self, Brace},
    ExprLit, Ident, Lit, LitInt, LitStr, Result, Token,
};
use tabled::{
    builder::Builder,
    settings::{themes::BorderCorrection, Alignment, Margin, Modify, Padding, Span, Style},
    Table,
};

struct MatrixRow {
    #[allow(dead_code)]
    bracket_token: token::Bracket,
    elems: MatrixRowElements,
}

enum MatrixRowElements {
    List(Punctuated<ExprVal, Token![,]>),
    Static {
        elem: ExprVal,
        #[allow(dead_code)]
        semi_token: Token![;],
        len: LitInt,
    },
}

impl Parse for MatrixRow {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let bracket_token = bracketed!(content in input);

        if content.peek2(Token![;]) {
            return Ok(Self {
                bracket_token,
                elems: MatrixRowElements::Static {
                    elem: content.parse()?,
                    semi_token: content.parse()?,
                    len: content.parse()?,
                },
            });
        }

        let mut elems = Punctuated::new();
        while !content.is_empty() {
            let val = content.parse()?;
            elems.push_value(val);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            elems.push_punct(punct);
        }

        Ok(Self {
            bracket_token,
            elems: MatrixRowElements::List(elems),
        })
    }
}

enum ExprVal {
    Lit(ExprLit),
    Scope {
        #[allow(dead_code)]
        brace_token: token::Brace,
        expr: Option<ScopeVal>,
    },
}

enum ScopeVal {
    Expr(ExprLit),
    List(Punctuated<ExprLit, Token![,]>),
    Sized {
        elem: ExprLit,
        #[allow(dead_code)]
        semi_token: Token![;],
        len: LitInt,
    },
}

impl Parse for ExprVal {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Brace) {
            let content;
            let brace_token = braced!(content in input);

            if content.is_empty() {
                return Ok(ExprVal::Scope {
                    brace_token,
                    expr: None,
                });
            }

            if content.peek2(Token![;]) {
                return Ok(ExprVal::Scope {
                    brace_token,
                    expr: Some(ScopeVal::Sized {
                        elem: content.parse()?,
                        semi_token: content.parse()?,
                        len: content.parse()?,
                    }),
                });
            }

            let elem: ExprLit = content.parse()?;

            if content.is_empty() {
                return Ok(ExprVal::Scope {
                    brace_token,
                    expr: Some(ScopeVal::Expr(elem)),
                });
            }

            let mut elems = Punctuated::new();
            elems.push(elem);

            while !content.is_empty() {
                let punct: Token![,] = content.parse()?;
                elems.push_punct(punct);

                if content.is_empty() {
                    // trailing comma
                    break;
                }

                let val: ExprLit = content.parse()?;
                elems.push_value(val);
            }

            return Ok(ExprVal::Scope {
                brace_token,
                expr: Some(ScopeVal::List(elems)),
            });
        }

        Ok(Self::Lit(input.parse()?))
    }
}

struct MatrixInput {
    #[allow(dead_code)]
    bracket_token: token::Bracket,
    data: MatrixData,
}

enum MatrixData {
    List(Punctuated<MatrixRow, Token![,]>),
    Static {
        elem: MatrixRow,
        #[allow(dead_code)]
        semi_token: Token![;],
        len: LitInt,
    },
}

impl Parse for MatrixInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let bracket_token = bracketed!(content in input);

        if content.is_empty() {
            return Ok(Self {
                bracket_token,
                data: MatrixData::List(Punctuated::new()),
            });
        }

        let elem = content.parse()?;

        if content.peek(Token![;]) {
            return Ok(MatrixInput {
                bracket_token,
                data: MatrixData::Static {
                    elem,
                    semi_token: content.parse()?,
                    len: content.parse()?,
                },
            });
        }

        let mut elems = Punctuated::new();
        elems.push(elem);

        while !content.is_empty() {
            let punct: Token![,] = content.parse()?;
            elems.push_punct(punct);

            if content.is_empty() {
                // trailing comma
                break;
            }

            let val = content.parse()?;
            elems.push_value(val);
        }

        Ok(MatrixInput {
            bracket_token,
            data: MatrixData::List(elems),
        })
    }
}

struct KeyValue<V> {
    key: Ident,
    #[allow(dead_code)]
    token: Token!(=),
    value: V,
}

impl<V: Parse> Parse for KeyValue<V> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            key: input.parse()?,
            token: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub(crate) struct TableStruct {
    matrix: MatrixInput,
    comma_token: Option<Token![,]>,
    settings: Punctuated<KeyValue<LitStr>, Token!(,)>,
}

impl Parse for TableStruct {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let matrix = input.parse()?;
        let mut comma_token = None;
        let mut settings = Punctuated::new();

        if input.peek(Token![,]) {
            comma_token = Some(input.parse()?);
            while !input.is_empty() {
                let val = input.parse()?;
                settings.push_value(val);
                if input.is_empty() {
                    break;
                }
                let punct = input.parse()?;
                settings.push_punct(punct);
            }
        }

        Ok(Self {
            matrix,
            comma_token,
            settings,
        })
    }
}

struct Pad<T> {
    left: T,
    #[allow(dead_code)]
    comma1_tk: Token!(,),
    right: T,
    #[allow(dead_code)]
    comma2_tk: Token!(,),
    top: T,
    #[allow(dead_code)]
    comma3_tk: Token!(,),
    bottom: T,
}

impl<T: Parse> Parse for Pad<T> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            left: input.parse()?,
            comma1_tk: input.parse()?,
            right: input.parse()?,
            comma2_tk: input.parse()?,
            top: input.parse()?,
            comma3_tk: input.parse()?,
            bottom: input.parse()?,
        })
    }
}

fn expr_lit_to_string(expr_lit: &ExprLit) -> Result<String> {
    match &expr_lit.lit {
        Lit::Str(val) => Ok(val.value()),
        Lit::ByteStr(val) => Ok(format!("{:?}", val.value())),
        Lit::Int(val) => Ok(val.base10_digits().to_string()),
        Lit::Float(val) => Ok(val.base10_digits().to_string()),
        Lit::Char(val) => Ok(val.value().to_string()),
        Lit::Byte(val) => Ok(val.value().to_string()),
        Lit::Bool(val) => Ok(val.value().to_string()),
        Lit::Verbatim(val) => Ok(val.to_token_stream().to_string()),
        _ => Err(syn::Error::new_spanned(
            expr_lit,
            "Unsupported literal type",
        )),
    }
}

fn expr_val_to_list(expr_val: &ExprVal) -> Result<Vec<String>> {
    match expr_val {
        ExprVal::Lit(lit) => Ok(vec![expr_lit_to_string(lit)?]),
        ExprVal::Scope { expr, .. } => match expr {
            Some(val) => match val {
                ScopeVal::Expr(lit) => Ok(vec![expr_lit_to_string(lit)?]),
                ScopeVal::List(list) => {
                    let mut data = Vec::with_capacity(list.len());
                    for val in list {
                        data.push(expr_lit_to_string(val)?);
                    }

                    Ok(data)
                }
                ScopeVal::Sized { elem, len, .. } => {
                    let len = len.base10_parse::<usize>()?;
                    let mut data = vec![String::new(); len];
                    if len > 0 {
                        data[0] = expr_lit_to_string(elem)?;
                    }

                    Ok(data)
                }
            },
            None => Ok(vec![String::new()]),
        },
    }
}

fn collect_matrix(matrix: &MatrixInput) -> Result<Vec<Vec<String>>> {
    match &matrix.data {
        MatrixData::List(list) => {
            let mut data = vec![];
            for row in list {
                let row = collect_row(&row.elems)?;
                data.push(row)
            }

            Ok(data)
        }
        MatrixData::Static { elem, len, .. } => {
            let data = collect_row(&elem.elems)?;
            let len = len.base10_parse::<usize>()?;

            Ok(vec![data; len])
        }
    }
}

fn collect_row(elems: &MatrixRowElements) -> Result<Vec<String>> {
    let mut row = vec![];

    match elems {
        MatrixRowElements::List(list) => {
            for val in list {
                let vals = expr_val_to_list(val)?;
                row.extend(vals);
            }
        }
        MatrixRowElements::Static { elem, len, .. } => {
            let len = len.base10_parse::<usize>()?;
            let elem = expr_val_to_list(elem)?;
            let iter = std::iter::repeat_n(elem, len).flatten();

            row.extend(iter);
        }
    }

    Ok(row)
}

fn collect_vspan(matrix: &MatrixInput) -> Result<HashMap<(usize, usize), usize>> {
    let mut spans = HashMap::new();

    match &matrix.data {
        MatrixData::List(list) => {
            for (row, e) in list.iter().enumerate() {
                match &e.elems {
                    MatrixRowElements::List(list) => {
                        let mut i = 0;
                        for e in list {
                            match e {
                                ExprVal::Lit(_) => i += 1,
                                ExprVal::Scope { expr, .. } => match expr {
                                    Some(val) => match val {
                                        ScopeVal::Expr(_) => i += 1,
                                        ScopeVal::List(list) => i += list.len(),
                                        ScopeVal::Sized { len, .. } => {
                                            let len = len.base10_parse::<usize>()?;
                                            if len > 0 {
                                                spans.insert((row, i), len);
                                                i += len;
                                            }
                                        }
                                    },
                                    None => i += 1,
                                },
                            }
                        }
                    }
                    MatrixRowElements::Static { elem, len, .. } => {
                        let arr_len = len.base10_parse::<usize>()?;
                        match elem {
                            ExprVal::Lit(_) => {}
                            ExprVal::Scope { expr, .. } => {
                                if let Some(val) = expr {
                                    match val {
                                        ScopeVal::Expr(_) => {}
                                        ScopeVal::List(_) => {}
                                        ScopeVal::Sized { len, .. } => {
                                            let len = len.base10_parse::<usize>()?;
                                            if len > 0 {
                                                let iter =
                                                    (0..arr_len).map(|i| ((row, i * len), len));
                                                spans.extend(iter);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        MatrixData::Static { elem, len, .. } => {
            let count_rows = len.base10_parse::<usize>()?;

            match &elem.elems {
                MatrixRowElements::List(list) => {
                    let mut i = 0;
                    for e in list {
                        match e {
                            ExprVal::Lit(_) => i += 1,
                            ExprVal::Scope { expr, .. } => match expr {
                                Some(val) => match val {
                                    ScopeVal::Expr(_) => i += 1,
                                    ScopeVal::List(list) => i += list.len(),
                                    ScopeVal::Sized { len, .. } => {
                                        let len = len.base10_parse::<usize>()?;
                                        if len > 0 {
                                            spans
                                                .extend((0..count_rows).map(|row| ((row, i), len)));
                                            i += len;
                                        }
                                    }
                                },
                                None => i += 1,
                            },
                        }
                    }
                }
                MatrixRowElements::Static { .. } => {}
            }
        }
    }

    Ok(spans)
}

fn collect_hspan(matrix: &MatrixInput) -> Result<HashMap<(usize, usize), usize>> {
    let mut filled = HashSet::new();
    let mut empties = HashSet::new();
    match &matrix.data {
        MatrixData::List(list) => {
            for (row, e) in list.iter().enumerate() {
                match &e.elems {
                    MatrixRowElements::List(list) => {
                        let mut col = 0;
                        for e in list {
                            match e {
                                ExprVal::Lit(_) => col += 1,
                                ExprVal::Scope { expr, .. } => match expr {
                                    Some(val) => match val {
                                        ScopeVal::List(list) => col += list.len(),
                                        ScopeVal::Expr(_) => {
                                            filled.insert((row, col));
                                            col += 1;
                                        }
                                        ScopeVal::Sized { len, .. } => {
                                            filled.insert((row, col));
                                            let len = len.base10_parse::<usize>()?;
                                            col += len;
                                        }
                                    },
                                    None => {
                                        empties.insert((row, col));
                                        col += 1;
                                    }
                                },
                            }
                        }
                    }
                    MatrixRowElements::Static { elem, len, .. } => {
                        let arr_len = len.base10_parse::<usize>()?;

                        match elem {
                            ExprVal::Lit(_) => {}
                            ExprVal::Scope { expr, .. } => match expr {
                                Some(val) => match val {
                                    ScopeVal::List(_) => {}
                                    ScopeVal::Expr(_) => {
                                        filled.extend((0..arr_len).map(|col| (row, col)));
                                    }
                                    ScopeVal::Sized { len, .. } => {
                                        let len = len.base10_parse::<usize>()?;
                                        filled.extend((0..arr_len).map(|col| (row, col * len)));
                                    }
                                },
                                None => {
                                    empties.extend((0..arr_len).map(|col| (row, col)));
                                }
                            },
                        }
                    }
                }
            }
        }
        MatrixData::Static { .. } => {}
    }

    let mut spans = HashMap::new();
    for (row, col) in filled {
        let mut size = 0;
        for row in row + 1.. {
            if empties.contains(&(row, col)) {
                size += 1;
            } else {
                break;
            }
        }

        if size > 0 {
            spans.insert((row, col), size + 1);
        }
    }

    Ok(spans)
}

// todo: export the constants from crate so they could be highlighted by language servers.
// Yet this is unstable to do.
fn is_supported_theme(name: &str) -> bool {
    matches!(
        name,
        "EMPTY"
            | "BLANK"
            | "ASCII"
            | "ASCII_ROUNDED"
            | "DOTS"
            | "MODERN"
            | "SHARP"
            | "ROUNDED"
            | "EXTENDED"
            | "RE_STRUCTURED_TEXT"
            | "MARKDOWN"
            | "PSQL"
    )
}

fn apply_theme(table: &mut Table, name: &str) {
    match name {
        "EMPTY" => table.with(Style::empty()),
        "BLANK" => table.with(Style::blank()),
        "ASCII" => table.with(Style::ascii()),
        "ASCII_ROUNDED" => table.with(Style::ascii_rounded()),
        "DOTS" => table.with(Style::dots()),
        "MODERN" => table.with(Style::modern()),
        "SHARP" => table.with(Style::sharp()),
        "ROUNDED" => table.with(Style::rounded()),
        "EXTENDED" => table.with(Style::extended()),
        "RE_STRUCTURED_TEXT" => table.with(Style::re_structured_text()),
        "MARKDOWN" => table.with(Style::markdown()),
        "PSQL" => table.with(Style::psql()),
        _ => unreachable!(),
    };
}

fn build_padding(pad: Pad<LitInt>) -> syn::Result<Padding> {
    let left = pad.left.base10_parse::<usize>()?;
    let right = pad.right.base10_parse::<usize>()?;
    let top = pad.top.base10_parse::<usize>()?;
    let bottom = pad.bottom.base10_parse::<usize>()?;

    Ok(Padding::new(left, right, top, bottom))
}

fn build_margin(pad: Pad<LitInt>) -> syn::Result<Margin> {
    let left = pad.left.base10_parse::<usize>()?;
    let right = pad.right.base10_parse::<usize>()?;
    let top = pad.top.base10_parse::<usize>()?;
    let bottom = pad.bottom.base10_parse::<usize>()?;

    Ok(Margin::new(left, right, top, bottom))
}

fn panic_not_supported_theme(ident: &LitStr) {
    proc_macro_error2::abort!(
        ident,
        "The given settings is not supported";
        note="custom themes are yet not supported";
        help = r#"Supported themes are [EMPTY, BLANK, ASCII, ASCII_ROUNDED, DOTS, MODERN, SHARP, ROUNDED, EXTENDED, RE_STRUCTURED_TEXT, MARKDOWN, PSQL]"#
    )
}

fn panic_not_supported_alignment(ident: &LitStr) {
    proc_macro_error2::abort!(
        ident,
        "The given settings is not supported";
        help = r#"Supported alignment are [LEFT, RIGHT, CENTER, CENTER_VERTICAL, TOP, BOTTOM]"#
    )
}

#[allow(dead_code)]
fn panic_not_supported_bool(ident: &LitStr) {
    proc_macro_error2::abort!(
        ident,
        "Unexpected bool value";
        help = r#"Expected to get [TRUE, FALSE]"#
    )
}

fn panic_not_supported_settings(ident: &Ident) {
    proc_macro_error2::abort!(
        ident,
        "The given settings is not supported";
        help = r#"Supported list is [THEME, PADDING, MARGIN]"#
    )
}

pub(crate) fn build_table(table_st: &TableStruct) -> Result<String> {
    let mut table = create_table(&table_st.matrix)?;

    if table_st.comma_token.is_some() {
        apply_settings(&mut table, &table_st.settings)?;
    }

    let has_spans = table.get_config().has_column_spans() || table.get_config().has_row_spans();
    if has_spans {
        table.with(BorderCorrection::span());
    }

    Ok(table.to_string())
}

fn apply_settings(
    table: &mut Table,
    settings: &Punctuated<KeyValue<LitStr>, Token![,]>,
) -> Result<()> {
    for kv in settings {
        config_table(table, kv)?;
    }

    Ok(())
}

fn is_supported_alignment(name: &str) -> bool {
    matches!(
        name,
        "LEFT" | "RIGHT" | "CENTER" | "CENTER_VERTICAL" | "TOP" | "BOTTOM"
    )
}

fn apply_alignment(table: &mut Table, name: &str) {
    match name {
        "LEFT" => table.with(Alignment::left()),
        "RIGHT" => table.with(Alignment::right()),
        "CENTER" => table.with(Alignment::center()),
        "CENTER_VERTICAL" => table.with(Alignment::center_vertical()),
        "TOP" => table.with(Alignment::top()),
        "BOTTOM" => table.with(Alignment::bottom()),
        _ => unreachable!(),
    };
}

fn config_table(table: &mut Table, kv: &KeyValue<LitStr>) -> Result<()> {
    if kv.key == "THEME" {
        let theme = kv.value.value();
        if !is_supported_theme(&theme) {
            panic_not_supported_theme(&kv.value);
        }

        apply_theme(table, &theme);
    } else if kv.key == "PADDING" {
        let padding = kv.value.parse().and_then(build_padding)?;
        table.with(padding);
    } else if kv.key == "MARGIN" {
        let margin = kv.value.parse().and_then(build_margin)?;
        table.with(margin);
    } else if kv.key == "ALIGNMENT" {
        let alignment = kv.value.value();
        if !is_supported_alignment(&alignment) {
            panic_not_supported_alignment(&kv.value);
        }

        apply_alignment(table, &alignment);
    } else {
        panic_not_supported_settings(&kv.key);
    }

    Ok(())
}

fn create_table(mat: &MatrixInput) -> Result<Table> {
    let data = collect_matrix(mat)?;
    let vspan = collect_vspan(mat)?;
    let hspan = collect_hspan(mat)?;

    let builder = Builder::from_iter(data);
    let mut table = builder.build();

    for (pos, span) in vspan {
        table.with(Modify::new(pos).with(Span::column(span as isize)));
    }

    for (pos, span) in hspan {
        table.with(Modify::new(pos).with(Span::row(span as isize)));
    }

    Ok(table)
}
