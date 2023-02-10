//! A module with a utility enum [`EitherString`].

/// Either allocated string or some type which can be used as a string.
#[derive(Debug)]
pub enum EitherString<T> {
    /// Allocated string.
    Owned(String),
    /// Something which can be used as a string.
    Some(T),
}

impl<T> AsRef<str> for EitherString<T>
where
    T: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        match self {
            EitherString::Owned(s) => s.as_ref(),
            EitherString::Some(s) => s.as_ref(),
        }
    }
}
