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
///     .with(Modify::new(Full).with(MaxWidth::truncating(5, "...")));
/// ```
///
/// While working with colors you must setup `colors` feature.
pub struct MaxWidth<S> {
    width: usize,
    wrap: Wrap<S>,
}

enum Wrap<S> {
    Truncate(S),
    Wrap,
}

impl<S> MaxWidth<S>
where
    S: AsRef<str>,
{
    pub fn truncating(width: usize, suffix: S) -> Self {
        Self {
            width,
            wrap: Wrap::Truncate(suffix),
        }
    }
}

impl MaxWidth<&'static str> {
    pub fn wrapping(width: usize) -> Self {
        Self {
            width,
            wrap: Wrap::Wrap,
        }
    }
}

impl<S: AsRef<str>> CellOption for MaxWidth<S> {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        match &self.wrap {
            Wrap::Truncate(filler) => {
                let striped_content = strip(content, self.width);
                if striped_content.len() < content.len() {
                    let new_content = format!("{}{}", striped_content, filler.as_ref());
                    grid.set(&Entity::Cell(row, column), Settings::new().text(new_content))
                }
            }
            Wrap::Wrap => {
                let wrapped_content = split(content, self.width);
                if wrapped_content.len() != content.len() {
                    grid.set(
                        &Entity::Cell(row, column),
                        Settings::new().text(wrapped_content),
                    )
                }
            }
        }
    }
}

pub(crate) fn strip(s: &str, width: usize) -> String {
    #[cfg(not(feature = "color"))]
    {
        s.chars().take(width).collect::<String>()
    }
    #[cfg(feature = "color")]
    {
        let max_width = std::cmp::min(s.chars().count(), width);
        ansi_cut::AnsiCut::cut(&s, ..max_width)
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
            ansi_cut::chunks(s, width).join("\n")
        }
    }
}
