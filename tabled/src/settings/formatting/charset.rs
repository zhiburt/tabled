use std::borrow::Cow;

use crate::{
    grid::config::{Entity, Position},
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    settings::{CellOption, TableOption},
};

/// A structure to handle special chars.
///
/// # Example
///
#[cfg_attr(feature = "assert", doc = "```")]
#[cfg_attr(not(feature = "assert"), doc = "```ignore")]
/// use std::iter::FromIterator;
///
/// use tabled::{
///     Table, builder::Builder,
///     settings::formatting::Charset,
///     assert::assert_table,
/// };
///
/// let win_text = "Some text which was created on windows \r\nyes they use these '\\r\\n'";
/// let linux_text = "Some text which was created on linux \nyes they use this '\\n'";
///
/// let mut table = Table::from_iter([
///     ["windows", "linux"],
///     [win_text, linux_text],
/// ]);
/// table.with(Charset::new().clean());
///
/// assert_table!(
///     table,
///     "+-----------------------------------------+---------------------------------------+"
///     "| windows                                 | linux                                 |"
///     "+-----------------------------------------+---------------------------------------+"
///     "| Some text which was created on windows  | Some text which was created on linux  |"
///     "| yes they use these '\\r\\n'               | yes they use this '\\n'                |"
///     "+-----------------------------------------+---------------------------------------+"
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Charset {
    clean: bool,
    tab_size: Option<usize>,
}

impl Charset {
    /// Returns default [`Charset`] structure,
    /// which does do anything unless no method is specified.
    pub fn new() -> Self {
        Self {
            clean: false,
            tab_size: None,
        }
    }

    /// Returns [`CleanCharset`] which removes all `\t` and `\r` occurrences.
    ///
    /// Notice that tab is just removed rather then being replaced with spaces.
    /// You might be better call [`TabSize`] first if you not expect such behavior.
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "assert", doc = "```")]
    #[cfg_attr(not(feature = "assert"), doc = "```ignore")]
    /// use tabled::{
    ///     Table, settings::formatting::Charset,
    ///     assert::assert_table,
    /// };
    ///
    /// let text = "Some\ttext\t\twith \\tabs";
    ///
    /// let mut table = Table::new([text]);
    /// table.with(Charset::new().clean());
    ///
    /// assert_table!(
    ///     table,
    ///     "+--------------------+"
    ///     "| &str               |"
    ///     "+--------------------+"
    ///     "| Sometextwith \\tabs |"
    ///     "+--------------------+"
    /// );
    /// ```
    ///
    /// [`TabSize`]: crate::settings::formatting::TabSize
    pub fn clean(mut self) -> Self {
        self.clean = true;
        self
    }

    /// Replace `\t` with `size` spaces in the same pass that strips other control characters.
    ///
    /// By default, `\t` is dropped (no replacement). Setting a tab size makes a single
    /// `Charset::clean().tab_size(N)` call equivalent to `TabSize::new(N)` followed by
    /// `Charset::clean()`, but with one allocation and one pass per cell.
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "assert", doc = "```")]
    #[cfg_attr(not(feature = "assert"), doc = "```ignore")]
    /// use tabled::{
    ///     Table,
    ///     settings::formatting::Charset,
    ///     assert::assert_table,
    /// };
    ///
    /// let text = "Some\ttext";
    ///
    /// let mut table = Table::new([text]);
    /// table.with(Charset::new().tab_size(4));
    ///
    /// assert_table!(
    ///     table,
    ///    "+--------------+"
    ///    "| &str         |"
    ///    "+--------------+"
    ///    "| Some    text |"
    ///    "+--------------+"
    /// );
    /// ```
    pub fn tab_size(mut self, size: usize) -> Self {
        self.tab_size = Some(size);
        self
    }

    /// Removes all symbols which may break the layout such as `\t`, `\r` and more.
    ///
    /// Notice that tab is just removed rather then being replaced with spaces.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::settings::formatting::Charset;
    ///
    /// assert_eq!(
    ///     Charset::charset_clean("Some\ttext\t\twith \\tabs\r\nSome"),
    ///     "Some    text        with \\tabs\nSome"
    /// )
    /// ```
    pub fn charset_clean(s: &str) -> Cow<'_, str> {
        Cow::Owned(charset_clean(s, 4))
    }
}

impl<R, D, C> TableOption<R, C, D> for Charset
where
    R: Records + ExactRecords + RecordsMut<String> + PeekableRecords,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        // TODO: Add a grid iterator which produces POS to squash these for loops

        if !self.clean && self.tab_size.is_none() {
            return;
        }

        for row in 0..records.count_rows() {
            for col in 0..records.count_columns() {
                let pos = Position::new(row, col);
                let text = records.get_text(pos);

                let text = match self {
                    Charset {
                        clean: true,
                        tab_size: None,
                    } => charset_clean(text, 0),
                    Charset {
                        clean: true,
                        tab_size: Some(tab),
                    } => charset_clean(text, tab),
                    Charset {
                        clean: false,
                        tab_size: Some(tab),
                    } => charset_tab(text, tab),
                    Charset {
                        clean: false,
                        tab_size: None,
                    } => unreachable!(),
                };

                records.set(pos, text);
            }
        }
    }
}

impl<R, C> CellOption<R, C> for Charset
where
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, _: &mut C, entity: Entity) {
        if !self.clean && self.tab_size.is_none() {
            return;
        }

        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for pos in entity.iter(count_rows, count_cols) {
            let text = records.get_text(pos);

            let text = match self {
                Charset {
                    clean: true,
                    tab_size: None,
                } => charset_clean(text, 0),
                Charset {
                    clean: true,
                    tab_size: Some(tab),
                } => charset_clean(text, tab),
                Charset {
                    clean: false,
                    tab_size: Some(tab),
                } => charset_tab(text, tab),
                Charset {
                    clean: false,
                    tab_size: None,
                } => unreachable!(),
            };

            records.set(pos, text);
        }
    }
}

fn charset_clean(text: &str, tab_size: usize) -> String {
    // It's enough for covering '\t' and '\r'
    // as well as a list of other unwanted escapes.
    let mut out = String::with_capacity(text.len());

    for c in text.chars() {
        match c {
            '\t' => {
                for _ in 0..tab_size {
                    out.push(' ');
                }
            }
            '\n' => out.push(c),
            c if c < ' ' => {}
            _ => out.push(c),
        }
    }

    out
}

fn charset_tab(text: &str, tab_size: usize) -> String {
    let mut out = String::with_capacity(text.len());

    for c in text.chars() {
        match c {
            '\t' => {
                for _ in 0..tab_size {
                    out.push(' ');
                }
            }
            _ => out.push(c),
        }
    }

    out
}
