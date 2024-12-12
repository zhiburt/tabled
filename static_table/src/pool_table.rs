use quote::ToTokens;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{self},
    ExprLit, Ident, Lit, LitInt, LitStr, Result, Token,
};
use tabled::{
    settings::{Alignment, Margin, Padding, Style},
    tables::{PoolTable, TableValue},
};

struct MatrixRow {
    #[allow(dead_code)]
    bracket_token: token::Bracket,
    elems: MatrixRowElements,
}

enum MatrixRowElements {
    List(Punctuated<ExprLit, Token![,]>),
    Static {
        elem: ExprLit,
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
        _ => Err(syn::Error::new(expr_lit.span(), "unsupported literal type")),
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
    match elems {
        MatrixRowElements::List(list) => {
            let mut row = Vec::with_capacity(list.len());
            for val in list {
                let val = expr_lit_to_string(val)?;
                row.push(val);
            }

            Ok(row)
        }
        MatrixRowElements::Static { elem, len, .. } => {
            let len = len.base10_parse::<usize>()?;
            let elem = expr_lit_to_string(elem)?;
            let row = vec![elem; len];

            Ok(row)
        }
    }
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

fn apply_theme(table: &mut PoolTable, name: &str) {
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
        note="custom themes are yet not supported";
        help = r#"Supported alignment are [LEFT, RIGHT, CENTER, CENTER_VERTICAL, TOP, BOTTOM]"#
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

    Ok(table.to_string())
}

fn apply_settings(
    table: &mut PoolTable,
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

fn apply_alignment(table: &mut PoolTable, name: &str) {
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

fn config_table(table: &mut PoolTable, kv: &KeyValue<LitStr>) -> Result<()> {
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

fn create_table(mat: &MatrixInput) -> Result<PoolTable> {
    let data = collect_matrix(mat)?;
    let value = matrix_to_table_value(data);
    Ok(PoolTable::from(value))
}

fn matrix_to_table_value(m: Vec<Vec<String>>) -> TableValue {
    TableValue::Column(
        m.into_iter()
            .map(|row| TableValue::Row(row.into_iter().map(TableValue::Cell).collect()))
            .collect(),
    )
}
