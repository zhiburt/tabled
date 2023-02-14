#![allow(clippy::uninlined_format_args)]

extern crate proc_macro;

use proc_macro_error::proc_macro_error;
use quote::{quote, IdentFragment, ToTokens};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{self, Brace},
    Expr, ExprCall, ExprLit, Ident, LitInt, Result, Token,
};
use tabled::settings::Modify;

#[allow(dead_code)]
struct MatrixRow {
    bracket_token: token::Bracket,
    elems: MatrixRowElements,
}

#[allow(dead_code)]
enum MatrixRowElements {
    List(Punctuated<ExprVal, Token![,]>),
    Static {
        elem: ExprVal,
        semi_token: Token![;],
        len: LitInt,
    },
}

impl Parse for MatrixRow {
    fn parse(input: ParseStream) -> Result<Self> {
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

#[allow(dead_code)]
enum ExprVal {
    Lit(ExprLit),
    Scope {
        brace_token: token::Brace,
        level: usize,
        expr: Option<ScopeVal>,
    },
}

#[allow(dead_code)]
enum ScopeVal {
    Expr(ExprLit),
    List(Punctuated<ExprLit, Token![,]>),
    Sized {
        elem: ExprLit,
        semi_token: Token![;],
        len: LitInt,
    },
}

impl Parse for ExprVal {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Brace) {
            let mut content;
            let brace_token = braced!(content in input);

            let mut level = 1;
            while content.peek(Brace) {
                let _ = braced!(content in content);
                level += 1;
            }

            if content.is_empty() {
                return Ok(ExprVal::Scope {
                    brace_token,
                    level,
                    expr: None,
                });
            }

            if content.peek2(Token![;]) {
                return Ok(ExprVal::Scope {
                    brace_token,
                    level,
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
                    level,
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
                level,
                expr: Some(ScopeVal::List(elems)),
            });
        }

        Ok(Self::Lit(input.parse()?))
    }
}

#[allow(dead_code)]
struct MatrixInput {
    bracket_token: token::Bracket,
    data: MatrixData,
}

#[allow(dead_code)]
enum MatrixData {
    List(Punctuated<MatrixRow, Token![,]>),
    Static {
        elem: MatrixRow,
        semi_token: Token![;],
        len: LitInt,
    },
}

impl Parse for MatrixInput {
    fn parse(input: ParseStream) -> Result<Self> {
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

struct TableStruct {
    matrix: MatrixInput,
    comma_token: Option<Token![,]>,
    theme: Option<Ident>,
}

impl Parse for TableStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let matrix = input.parse()?;
        let mut theme = None;
        let mut comma_token = None;

        if input.peek(Token![,]) {
            let tok = input.parse()?;
            if !input.is_empty() {
                let val = input.parse()?;
                theme = Some(val);
                comma_token = Some(tok);
            }
        }

        Ok(Self {
            matrix,
            comma_token,
            theme,
        })
    }
}

fn expr_lit_to_string(expr_lit: &ExprLit) -> String {
    match &expr_lit.lit {
        syn::Lit::Str(s) => s.value(),
        syn::Lit::ByteStr(s) => format!("{:?}", s.value()),
        lit => lit.to_token_stream().to_string(),
    }
}

fn expr_val_to_list(expr_val: &ExprVal) -> Vec<String> {
    match expr_val {
        ExprVal::Lit(lit) => vec![expr_lit_to_string(lit)],
        ExprVal::Scope { expr, .. } => match expr {
            Some(val) => match val {
                ScopeVal::Expr(lit) => vec![expr_lit_to_string(lit)],
                ScopeVal::List(list) => list.into_iter().map(expr_lit_to_string).collect(),
                ScopeVal::Sized { elem, len, .. } => {
                    let len = len.base10_parse::<usize>().unwrap();
                    let mut data = vec![String::new(); len];
                    if len > 0 {
                        data[0] = expr_lit_to_string(elem);
                    }

                    data
                }
            },
            None => vec![String::new()],
        },
    }
}

fn collect_matrix(table: &TableStruct) -> Vec<Vec<String>> {
    match &table.matrix.data {
        MatrixData::List(list) => list
            .into_iter()
            .map(|arr| match &arr.elems {
                MatrixRowElements::List(list) => list
                    .into_iter()
                    .flat_map(expr_val_to_list)
                    .collect::<Vec<_>>(),
                MatrixRowElements::Static { elem, len, .. } => {
                    let len = len.base10_parse::<usize>().unwrap();
                    let elem = expr_val_to_list(elem);
                    std::iter::repeat(elem)
                        .take(len)
                        .flatten()
                        .collect::<Vec<_>>()
                }
            })
            .collect::<Vec<_>>(),
        MatrixData::Static { elem, len, .. } => {
            let data = match &elem.elems {
                MatrixRowElements::List(list) => list
                    .into_iter()
                    .flat_map(expr_val_to_list)
                    .collect::<Vec<_>>(),
                MatrixRowElements::Static { elem, len, .. } => {
                    let len = len.base10_parse::<usize>().unwrap();
                    let elem = expr_val_to_list(elem);
                    std::iter::repeat(elem)
                        .take(len)
                        .flatten()
                        .collect::<Vec<_>>()
                }
            };
            let len = len.base10_parse::<usize>().unwrap();

            vec![data; len]
        }
    }
}

fn collect_vspan(table: &TableStruct) -> HashMap<(usize, usize), usize> {
    let mut spans = HashMap::new();

    match &table.matrix.data {
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
                                            let len = len.base10_parse::<usize>().unwrap();
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
                        let arr_len = len.base10_parse::<usize>().unwrap();
                        match elem {
                            ExprVal::Lit(_) => {}
                            ExprVal::Scope { expr, .. } => match expr {
                                Some(val) => match val {
                                    ScopeVal::Expr(_) => {}
                                    ScopeVal::List(_) => {}
                                    ScopeVal::Sized { len, .. } => {
                                        let len = len.base10_parse::<usize>().unwrap();
                                        if len > 0 {
                                            let iter = (0..arr_len).map(|i| ((row, i * len), len));
                                            spans.extend(iter);
                                        }
                                    }
                                },
                                None => {}
                            },
                        }
                    }
                }
            }
        }
        MatrixData::Static { elem, len, .. } => {
            let count_rows = len.base10_parse::<usize>().unwrap();

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
                                        let len = len.base10_parse::<usize>().unwrap();
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

    spans
}

fn collect_hspan(table: &TableStruct) -> HashMap<(usize, usize), usize> {
    let mut filled = HashSet::new();
    let mut empties = HashSet::new();
    match &table.matrix.data {
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
                                            let len = len.base10_parse::<usize>().unwrap();
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
                        let arr_len = len.base10_parse::<usize>().unwrap();

                        match elem {
                            ExprVal::Lit(_) => {}
                            ExprVal::Scope { expr, .. } => match expr {
                                Some(val) => match val {
                                    ScopeVal::List(_) => {}
                                    ScopeVal::Expr(_) => {
                                        filled.extend((0..arr_len).map(|col| (row, col)));
                                    }
                                    ScopeVal::Sized { len, .. } => {
                                        let len = len.base10_parse::<usize>().unwrap();
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

    spans
}

// todo: export the constants from crate so they could be highlighted by language servers.
// Yet this is unstable to do.
fn is_supported_theme(name: &str) -> bool {
    matches!(
        name,
        "STYLE_EMPTY"
            | "STYLE_BLANK"
            | "STYLE_ASCII"
            | "STYLE_ASCII_ROUNDED"
            | "STYLE_DOTS"
            | "STYLE_MODERN"
            | "STYLE_SHARP"
            | "STYLE_ROUNDED"
            | "STYLE_EXTENDED"
            | "STYLE_RE_STRUCTURED_TEXT"
            | "STYLE_MARKDOWN"
            | "STYLE_PSQL"
    )
}
fn apply_theme(table: &mut tabled::Table, name: &str) {
    match name {
        "STYLE_EMPTY" => table.with(tabled::settings::style::Style::empty()),
        "STYLE_BLANK" => table.with(tabled::settings::style::Style::blank()),
        "STYLE_ASCII" => table.with(tabled::settings::style::Style::ascii()),
        "STYLE_ASCII_ROUNDED" => table.with(tabled::settings::style::Style::ascii_rounded()),
        "STYLE_DOTS" => table.with(tabled::settings::style::Style::dots()),
        "STYLE_MODERN" => table.with(tabled::settings::style::Style::modern()),
        "STYLE_SHARP" => table.with(tabled::settings::style::Style::sharp()),
        "STYLE_ROUNDED" => table.with(tabled::settings::style::Style::rounded()),
        "STYLE_EXTENDED" => table.with(tabled::settings::style::Style::extended()),
        "STYLE_RE_STRUCTURED_TEXT" => {
            table.with(tabled::settings::style::Style::re_structured_text())
        }
        "STYLE_MARKDOWN" => table.with(tabled::settings::style::Style::markdown()),
        "STYLE_PSQL" => table.with(tabled::settings::style::Style::psql()),
        _ => unreachable!(),
    };
}

#[proc_macro]
#[proc_macro_error]
pub fn static_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mat = parse_macro_input!(input as TableStruct);

    let vspan = collect_vspan(&mat);
    let hspan = collect_hspan(&mat);
    let data = collect_matrix(&mat);

    let builder = tabled::builder::Builder::from_iter(data);
    let mut table = builder.build();

    if let Some(ident) = mat.theme {
        let theme = ident.to_string();
        if !is_supported_theme(&theme) {
            panic_not_supported_theme(ident);
        }

        apply_theme(&mut table, &theme);
    }

    for (pos, span) in vspan {
        table.with(Modify::new(pos).with(tabled::settings::span::ColumnSpan::new(span)));
    }

    for (pos, span) in hspan {
        table.with(Modify::new(pos).with(tabled::settings::span::RowSpan::new(span)));
    }

    let table = table.to_string();

    let out = quote! {
        #table
    };

    out.into()
}

fn panic_not_supported_theme(ident: Ident) {
    proc_macro_error::abort!(
        ident,
        "The given settings is not supported";
        note="custom themes are yet not supported";
        help = r#"Supported theames are [STYLE_EMPTY, STYLE_BLANK, STYLE_ASCII, STYLE_ASCII_ROUNDED, STYLE_DOTS, STYLE_MODERN, STYLE_SHARP, STYLE_ROUNDED, STYLE_EXTENDED, STYLE_RE_STRUCTURED_TEXT, STYLE_MARKDOWN, STYLE_PSQL]"#
    )
}
