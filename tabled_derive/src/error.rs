use proc_macro2::Span;

#[derive(Debug, Clone)]
pub enum Error {
    Syn(syn::Error),
    Custom {
        span: Span,
        error: String,
        help: Option<String>,
    },
}

impl Error {
    pub fn new<E>(error: E, span: Span, help: Option<String>) -> Self
    where
        E: Into<String>,
    {
        let error = error.into();
        Self::Custom { error, help, span }
    }

    pub fn message<E>(error: E) -> Self
    where
        E: Into<String>,
    {
        let error = error.into();

        Self::Custom {
            error,
            help: None,
            span: Span::call_site(),
        }
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Error::Syn(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Syn(err) => err.fmt(f),
            Error::Custom { error, .. } => {
                write!(f, "a custom error: {error}")
            }
        }
    }
}

impl std::error::Error for Error {}

pub fn abort(err: Error) -> ! {
    match err {
        Error::Syn(err) => {
            proc_macro_error::abort! {err.span(), "{}", err}
        }
        Error::Custom { span, error, help } => match help {
            Some(help) => {
                proc_macro_error::abort! {span, "{}",  error; help="{}", help}
            }
            None => {
                proc_macro_error::abort! {span, "{}",  error}
            }
        },
    }
}
