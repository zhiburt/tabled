use crate::CellOption;
use papergrid::{Entity, Grid, Settings};

/// Format a structure which modifies a `Grid`
pub struct MaxWidth<S>(pub usize, pub S)
where
    S: AsRef<str>;

impl<S: AsRef<str>> CellOption for MaxWidth<S> {
    fn change_cell(&self, grid: &mut Grid, row: usize, column: usize) {
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

fn strip(s: &str, w: usize) -> String {
    #[cfg(not(feature = "color"))]
    {
        s.chars().take(w).collect::<String>()
    }
    #[cfg(feature = "color")]
    {
        console::truncate_str(s, w, "").to_string()
    }
}
