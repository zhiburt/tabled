use crate::Tabled;

/// ExpandedDisplay display data in a 'expanded display mode' from postgress.
/// It may be usefull for a large data sets with a lot of fields.
///
/// See 'Examples' in https://www.postgresql.org/docs/current/app-psql.html.
///
/// It escapes strings to resolve a multi-line ones.
/// Because of that `colors` may not be rendered.
pub struct ExpandedDisplay {
    fields: Vec<String>,
    records: Vec<Vec<String>>,
}

impl ExpandedDisplay {
    /// Creates a new instance of ExpandedDisplay
    pub fn new<T: Tabled>(iter: impl IntoIterator<Item = T>) -> Self {
        let data = iter.into_iter().map(|i| i.fields()).collect();
        let header = T::headers();

        Self {
            records: data,
            fields: header,
        }
    }
}

impl std::fmt::Display for ExpandedDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // It's possible that field|header can be a multiline string so
        // we escape it and trim \" chars.
        let fields = self
            .fields
            .iter()
            .map(|f| {
                let escaped = format!("{:?}", f);
                escaped
                    .chars()
                    .skip(1)
                    .take(escaped.len() - 1 - 1)
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        let max_field_width = fields
            .iter()
            .map(|f| f.chars().count())
            .max()
            .unwrap_or_default();

        for (i, record) in self.records.iter().enumerate() {
            assert_eq!(record.len(), fields.len());

            writeln!(f, "-[ RECORD {} ]-", i)?;
            for (value, field) in record.iter().zip(fields.iter()) {
                writeln!(f, "{:width$} | {:?}", field, value, width = max_field_width)?;
            }
        }

        Ok(())
    }
}
