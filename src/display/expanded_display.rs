use crate::Tabled;

/// ExpandedDisplay display data in a 'expanded display mode' from postgress.
/// It may be usefull for a large data sets with a lot of fields.
///
/// See 'Examples' in https://www.postgresql.org/docs/current/app-psql.html.
///
/// It escapes strings to resolve a multi-line ones.
/// Because of that `colors` may not be rendered.
pub struct ExpandedDisplay {
    format_record_splitter: Option<fn(usize) -> String>,
    format_value: Option<Box<dyn Fn(&str) -> String>>,
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
            format_record_splitter: None,
            format_value: None,
        }
    }

    /// Sets a line format which will be used to split records.
    ///
    /// Default formating is "-[ RECORD {} ]-".
    ///
    /// At least one '\n' char will be printed at the end regardless if you set it or not.
    pub fn header_template(&mut self, f: fn(usize) -> String) -> &mut Self {
        self.format_record_splitter = Some(f);
        self
    }

    /// Sets a value formatter.
    ///
    /// This method overrides others formatters like [ExpandedDisplay::truncate] and [ExpandedDisplay::wrap].
    pub fn formatter(&mut self, f: impl Fn(&str) -> String + 'static) -> &mut Self {
        self.format_value = Some(Box::new(f));
        self
    }

    /// Sets max width of value.
    /// The rest will be trunceted.
    pub fn truncate(&mut self, max: usize, tail: impl AsRef<str>) -> &mut Self {
        let tail = tail.as_ref().to_string();
        self.format_value = Some(Box::new(move |s| {
            let mut s = s
                .chars()
                .take(max)
                .collect::<String>()
                .escape_debug()
                .to_string();

            if s.chars().count() >= max {
                s.push_str(&tail);
            }

            s
        }));
        self
    }

    /// Sets max width of value,
    /// when limit is reached next chars will be placed on the next line.
    pub fn wrap(&mut self, max: usize) -> &mut Self {
        self.format_value = Some(Box::new(move |s| {
            s.chars()
                .enumerate()
                .flat_map(|(i, c)| {
                    if i != 0 && i % max == 0 {
                        Some('\n')
                    } else {
                        None
                    }
                    .into_iter()
                    .chain(std::iter::once(c))
                })
                .collect::<String>()
        }));
        self
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

            let values = record
                .iter()
                .map(|value| match &self.format_value {
                    Some(f) => (f)(value),
                    None => value.to_string(),
                })
                .collect::<Vec<_>>();

            match self.format_record_splitter {
                Some(f_header) => {
                    let header = (f_header)(i);
                    writeln!(f, "{}", header)?;
                }
                None => {
                    let record_max_width = values
                        .iter()
                        .map(|value| value.lines().map(|l| l.chars().count()).max())
                        .max()
                        .unwrap_or_default()
                        .unwrap_or_default();

                    writeln!(
                        f,
                        "-[ RECORD {} ]{:-<width$}",
                        i,
                        "-",
                        width = record_max_width + 2 // 2 is is space and '|' char which we use in our formatting
                    )?;
                }
            }

            for (value, field) in values.iter().zip(fields.iter()) {
                write_record_line(f, field, value, max_field_width)?;
            }
        }

        Ok(())
    }
}

fn write_record_line(
    f: &mut std::fmt::Formatter<'_>,
    field: &str,
    value: &str,
    max_field_width: usize,
) -> std::fmt::Result {
    if value.is_empty() {
        writeln!(f, "{:width$} | {}", field, value, width = max_field_width)?;
        return Ok(());
    }

    for (i, line) in value.lines().enumerate() {
        let field = if i == 0 { field } else { "" };
        writeln!(f, "{:width$} | {}", field, line, width = max_field_width)?;
    }
    Ok(())
}
