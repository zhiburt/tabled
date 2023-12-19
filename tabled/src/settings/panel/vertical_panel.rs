use crate::{
    grid::config::{ColoredConfig, SpannedConfig},
    grid::records::{ExactRecords, Records, RecordsMut, Resizable},
    settings::TableOption,
};

/// A vertical/row span from 0 to a count columns.
#[derive(Debug)]
pub struct VerticalPanel<S> {
    text: S,
    col: usize,
}

impl<S> VerticalPanel<S> {
    /// Creates a new vertical panel.
    pub fn new(col: usize, text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self { text, col }
    }

    /// Split the set text to a certain width, so it fits within it.
    pub fn width(self, width: usize) -> VerticalPanel<String>
    where
        S: AsRef<str>,
    {
        let mut text = String::new();

        if width > 0 {
            text = split_string_by_width(self.text.as_ref(), width);
        }

        VerticalPanel {
            text,
            col: self.col,
        }
    }
}

impl<S, R, D> TableOption<R, ColoredConfig, D> for VerticalPanel<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + Resizable + RecordsMut<String>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        if self.col > count_cols {
            return;
        }

        let is_intersect_horizontal_span = (0..=records.count_rows())
            .any(|row| cfg.is_cell_covered_by_column_span((row, self.col)));

        if is_intersect_horizontal_span {
            return;
        }

        move_columns_aside(records, self.col);
        move_column_spans(cfg, self.col);

        let text = self.text.as_ref().to_owned();
        records.set((0, self.col), text);

        cfg.set_row_span((0, self.col), count_rows);
    }
}

fn move_columns_aside<R: Records + Resizable>(records: &mut R, column: usize) {
    records.push_column();

    let count_columns = records.count_columns();
    let shift_count = count_columns - column;
    for i in 1..shift_count {
        let col = count_columns - i;
        records.swap_column(col, col - 1);
    }
}

fn move_column_spans(cfg: &mut SpannedConfig, target_column: usize) {
    for ((row, col), span) in cfg.get_column_spans() {
        if col < target_column {
            continue;
        }

        cfg.set_column_span((row, col), 1);
        cfg.set_column_span((row, col + 1), span);
    }

    for ((row, col), span) in cfg.get_row_spans() {
        if col < target_column {
            continue;
        }

        cfg.set_row_span((row, col), 1);
        cfg.set_row_span((row, col + 1), span);
    }
}

fn split_string_by_width(str: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let (lhs, rhs) = crate::util::string::split_str(str, width);
    if rhs.is_empty() {
        return lhs.into_owned();
    }

    let mut buf = lhs.into_owned();
    let mut next = rhs.into_owned();
    while !next.is_empty() {
        let (lhs, rhs) = crate::util::string::split_str(&next, width);
        buf.push('\n');
        buf.push_str(&lhs);
        next = rhs.into_owned();
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string_by_width() {
        assert_eq!(split_string_by_width("123456789", 3), "123\n456\n789");
        assert_eq!(split_string_by_width("123456789", 2), "12\n34\n56\n78\n9");
        assert_eq!(
            split_string_by_width("123456789", 1),
            "1\n2\n3\n4\n5\n6\n7\n8\n9"
        );
        assert_eq!(split_string_by_width("123456789", 0), "");

        assert_eq!(
            split_string_by_width("\u{1b}[31;100m😳😳🏳️\u{1b}[39m\u{1b}[49m😳🏳️", 3),
            {
                #[cfg(feature = "ansi")]
                {
                    "\u{1b}[31m\u{1b}[100m😳\u{1b}[39m\u{1b}[49m�\n\u{1b}[31m\u{1b}[100m🏳\u{fe0f}\u{1b}[39m\u{1b}[49m😳\n🏳\u{fe0f}"
                }
                #[cfg(not(feature = "ansi"))]
                {
                    "\u{1b}[31\n;10\n0m�\n😳🏳\n\u{fe0f}\u{1b}[39\nm\u{1b}[4\n9m�\n🏳\u{fe0f}"
                }
            }
        );
    }
}
