mod buf_records;
mod limit_column_records;
mod limit_row_records;
mod truncate_records;

pub use buf_records::{BufRecords, BufRecords2};
pub use limit_column_records::ColumnLimitRecords;
pub use limit_row_records::RowLimitRecords;
pub use truncate_records::{TruncatedRecords, Width};

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
