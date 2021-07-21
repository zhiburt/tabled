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
///     .with(Modify::new(Full).with(MaxWidth(5, "...")));
/// ```
///
/// While working with colors you must setup `colors` feature.
pub struct MaxWidth<S>(pub usize, pub S)
where
    S: AsRef<str>;

impl<S: AsRef<str>> CellOption for MaxWidth<S> {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let width = self.0;
        let filler = self.1.as_ref();

        let content = grid.get_cell_content(row, column);
        let striped_content = strip(content, width);

        let old_content_length = content.len();
        let new_content_length = striped_content.len();

        if new_content_length != old_content_length {
            let content = format!("{}{}", striped_content, filler);
            grid.set(Entity::Cell(row, column), Settings::new().text(content))
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
