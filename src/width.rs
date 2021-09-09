use crate::CellOption;
use papergrid::{Entity, Grid, Settings};
use unicode_segmentation::UnicodeSegmentation;

enum Wrap<S> {
    Truncate(S),
    Wrap,
}

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
pub struct MaxWidth<S>(usize, Wrap<S>);

impl<S> MaxWidth<S>
where
    S: AsRef<str>,
{
    pub fn truncating(width: usize, suffix: S) -> Self {
        Self(width, Wrap::Truncate(suffix))
    }
}

impl MaxWidth<&'static str> {
    pub fn wrapping(width: usize) -> Self {
        Self(width, Wrap::Wrap)
    }
}

impl<S: AsRef<str>> CellOption for MaxWidth<S> {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let width = self.0;

        let content = grid.get_cell_content(row, column);

        match &self.1 {
            Wrap::Truncate(filler) => {
                let striped_content = strip(content, width);
                let old_content_length = content.len();
                let new_content_length = striped_content.len();
                if new_content_length != old_content_length {
                    let content = format!("{}{}", striped_content, filler.as_ref());
                    grid.set(Entity::Cell(row, column), Settings::new().text(content))
                }
            }
            Wrap::Wrap => {
                let split_content = split(content, width);
                let old_content_length = content.len();
                let new_content_length = split_content.len();
                if new_content_length != old_content_length {
                    let content = format!("{}", split_content);
                    grid.set(Entity::Cell(row, column), Settings::new().text(content))
                }
            }
        }
    }
}

fn strip(s: &str, width: usize) -> String {
    #[cfg(not(feature = "color"))]
    {
        s.chars().take(width).collect::<String>()
    }
    #[cfg(feature = "color")]
    {
        let max_width = std::cmp::min(s.chars().count(), width);
        ansi_cut::AnsiCut::cut(&s, ..max_width).to_string()
    }
}

fn split(s: &str, width: usize) -> String {
    // TODO: consider colors here.
    s.graphemes(true)
        .collect::<Vec<&str>>()
        .chunks(width)
        .map(|chunk| chunk.concat())
        .collect::<Vec<String>>()
        .join("\n")
}
