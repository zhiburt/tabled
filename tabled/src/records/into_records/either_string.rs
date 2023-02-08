
pub enum EitherString<T> {
    Owned(String),
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
