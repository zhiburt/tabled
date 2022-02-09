use crate::CellOption;
use papergrid::{Entity, Grid, Settings};

/// Using MaxWidth you can set a max width of an object on a [Grid].
///
/// ## Example
///
/// ```
/// use tabled::{Full, MaxWidth, Modify, Style, Table};
///
/// let data = [
///     "123456789",
///     "qwertyuiop[]",
///     "[[[[[[[[[[[[[[[[[",
/// ];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Full).with(MaxWidth::truncating(5).suffix("...")));
/// ```
///
/// While working with colors you must setup `colors` feature.
pub struct MaxWidth;

impl MaxWidth {
    pub fn truncating(width: usize) -> Truncate<&'static str> {
        Truncate::new(width)
    }

    pub fn wrapping(width: usize) -> Wrap {
        Wrap::new(width)
    }
}

pub struct Truncate<S> {
    width: usize,
    suffix: S,
}

impl Truncate<&'static str> {
    pub fn new(width: usize) -> Self {
        Self { width, suffix: "" }
    }
}

impl<T> Truncate<T> {
    pub fn suffix<S>(self, suffix: S) -> Truncate<S> {
        Truncate {
            width: self.width,
            suffix,
        }
    }
}

impl<S> CellOption for Truncate<S>
where
    S: AsRef<str>,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let striped_content = strip(content, self.width);
        if striped_content.len() < content.len() {
            let new_content = format!("{}{}", striped_content, self.suffix.as_ref());
            grid.set(
                &Entity::Cell(row, column),
                Settings::new().text(new_content),
            )
        }
    }
}

pub struct Wrap {
    width: usize,
    keep_words: bool,
}

impl Wrap {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            keep_words: false,
        }
    }

    /// Set the wrap's keep words.
    pub fn keep_words(mut self) -> Self {
        self.keep_words = true;
        self
    }
}

impl CellOption for Wrap {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let wrapped_content = if !self.keep_words {
            split(content, self.width)
        } else {
            split_keeping_words(content, self.width)
        };
        grid.set(
            &Entity::Cell(row, column),
            Settings::new().text(wrapped_content),
        )
    }
}

pub(crate) fn strip(s: &str, width: usize) -> String {
    #[cfg(not(feature = "color"))]
    {
        s.chars().take(width).collect::<String>()
    }
    #[cfg(feature = "color")]
    {
        let width = to_byte_length(s, width);
        ansi_str::AnsiStr::ansi_cut(s, ..width)
    }
}

pub(crate) fn split(s: &str, width: usize) -> String {
    #[cfg(not(feature = "color"))]
    {
        s.chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && i % width == 0 {
                    Some('\n')
                } else {
                    None
                }
                .into_iter()
                .chain(std::iter::once(c))
            })
            .collect::<String>()
    }
    #[cfg(feature = "color")]
    {
        if width == 0 {
            s.to_string()
        } else {
            chunks(s, width).join("\n")
        }
    }
}

#[cfg(not(feature = "color"))]
fn split_keeping_words(s: &str, width: usize) -> String {
    let mut buf = String::new();
    let mut i = 0;
    for c in s.chars() {
        if i != 0 && i % width == 0 {
            let prev_c = buf.chars().last().unwrap();
            let is_splitting_word = !prev_c.is_whitespace() && !c.is_whitespace();
            if is_splitting_word {
                let pos = buf.chars().rev().position(|c| c.is_whitespace());
                match pos {
                    Some(pos) => {
                        if pos < width {
                            // it's a part of a word which we is ok to move to the next line;
                            // we know that there will be enough space for this part + next character.
                            //
                            // todo: test about this next char space
                            let range_len = buf
                                .chars()
                                .rev()
                                .take(pos)
                                .map(|c| c.len_utf8())
                                .sum::<usize>();
                            buf.insert(buf.len() - range_len, '\n');
                            i = range_len;
                        } else {
                            // The words is too long to be moved,
                            // we can't move it any way so just leave everything as it is
                            buf.push('\n');
                        }
                    }
                    None => {
                        // We don't find a whitespace
                        // so its a long word so we can do nothing about it
                        buf.push('\n');
                    }
                }
            } else {
                // This place doesn't separate a word
                // So we just do a general split.
                buf.push('\n');
            }
        }

        buf.push(c);

        i += 1;
    }

    buf
}

#[cfg(feature = "color")]
fn split_keeping_words(s: &str, width: usize) -> String {
    use ansi_str::AnsiStr;

    let mut buf = String::new();
    let mut s = s.to_string();
    while !s.is_empty() {
        let width = to_byte_length(&s, width);
        let (mut lhs, mut rhs) = s.ansi_split_at(width);

        let lhs_stripped = lhs.ansi_strip();
        let left_ends_with_letter = lhs_stripped
            .chars()
            .last()
            .map_or(false, |c| !c.is_whitespace());
        let right_starts_with_letter = rhs
            .ansi_strip()
            .chars()
            .next()
            .map_or(false, |c| !c.is_whitespace());
        let is_splitting_word = left_ends_with_letter && right_starts_with_letter;
        if is_splitting_word {
            let pos = lhs_stripped.chars().rev().position(|c| c.is_whitespace());
            match pos {
                Some(pos) => {
                    if pos < width {
                        // it's a part of a word which we is ok to move to the next line;
                        // we know that there will be enough space for this part + next character.
                        //
                        // todo: test about this next char space
                        let range_len = lhs_stripped
                            .chars()
                            .rev()
                            .take(pos)
                            .map(|c| c.len_utf8())
                            .sum::<usize>();

                        let move_part = lhs.ansi_get(lhs_stripped.len() - range_len..).unwrap();
                        lhs = lhs.ansi_get(..lhs_stripped.len() - range_len).unwrap();
                        rhs = move_part + &rhs;

                        buf.push_str(&lhs);
                        buf.push('\n');
                    } else {
                        // The words is too long to be moved,
                        // we can't move it any way so just leave everything as it is
                        buf.push_str(&lhs);
                        buf.push('\n');
                    }
                }
                None => {
                    // We don't find a whitespace
                    // so its a long word so we can do nothing about it
                    buf.push_str(&lhs);
                    buf.push('\n');
                }
            }
        } else {
            buf.push_str(&lhs);
            buf.push('\n');
        }

        s = rhs;
    }

    buf
}

#[cfg(feature = "color")]
fn to_byte_length(s: &str, width: usize) -> usize {
    s.chars().take(width).map(|c| c.len_utf8()).sum::<usize>()
}

#[cfg(feature = "color")]
fn chunks(s: &str, width: usize) -> Vec<String> {
    use ansi_str::AnsiStr;

    let mut v = Vec::new();
    let mut s = s.to_string();
    while !s.is_empty() {
        let width = to_byte_length(&s, width);
        let (lhs, rhs) = s.ansi_split_at(width);
        s = rhs;
        v.push(lhs);
    }

    v
}
