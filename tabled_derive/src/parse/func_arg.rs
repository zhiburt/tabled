use syn::{spanned::Spanned, Error, Expr, Lit, Member, Result};

#[derive(Debug)]
pub enum FuncArg {
    SelfRef,
    SelfProperty(String),
    Byte(u8),
    Char(char),
    Bool(bool),
    Uint(usize),
    Int(isize),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
}

impl FuncArg {
    pub fn parse_expr(expr: &Expr) -> Result<Self> {
        parse_func_arg(expr)
    }
}

fn parse_func_arg(expr: &Expr) -> Result<FuncArg> {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Str(val) => Ok(FuncArg::String(val.value())),
            Lit::ByteStr(val) => Ok(FuncArg::Bytes(val.value())),
            Lit::Byte(val) => Ok(FuncArg::Byte(val.value())),
            Lit::Char(val) => Ok(FuncArg::Char(val.value())),
            Lit::Bool(val) => Ok(FuncArg::Bool(val.value())),
            Lit::Float(val) => val.base10_parse::<f64>().map(FuncArg::Float),
            Lit::Int(val) => {
                if val.base10_digits().starts_with('-') {
                    val.base10_parse::<isize>().map(FuncArg::Int)
                } else {
                    val.base10_parse::<usize>().map(FuncArg::Uint)
                }
            }
            Lit::Verbatim(val) => Err(Error::new(val.span(), "unsupported argument")),
        },
        Expr::Path(path) => {
            let indent = path.path.get_ident().map(|indent| indent.to_string());
            if matches!(indent.as_deref(), Some("self" | "Self")) {
                Ok(FuncArg::SelfRef)
            } else {
                Err(Error::new(path.span(), "unsupported argument"))
            }
        }
        Expr::Field(field) => match &field.member {
            Member::Named(ident) => Ok(FuncArg::SelfProperty(ident.to_string())),
            Member::Unnamed(index) => Ok(FuncArg::SelfProperty(index.index.to_string())),
        },
        expr => Err(Error::new(expr.span(), "unsupported argument")),
    }
}
