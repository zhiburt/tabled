use crate::config::Position;

/// The representation of data, rows and columns of a grid.
pub trait PeekableRecords {
    /// Returns a text of a cell by an index.
    fn get_text(&self, pos: Position) -> &str;

    /// Returns a line of a text of a cell by an index.
    fn get_line(&self, pos: Position, line: usize) -> &str {
        self.get_text(pos).lines().nth(line).unwrap()
    }

    /// Returns an amount of lines of a text of a cell by an index.
    fn count_lines(&self, pos: Position) -> usize {
        self.get_text(pos).lines().count()
    }

    /// Returns a width of a text of a cell by an index.
    fn get_width(&self, pos: Position) -> usize {
        crate::util::string::string_width_multiline(self.get_text(pos))
    }

    /// Returns a width of line of a text of a cell by an index.
    fn get_line_width(&self, pos: Position, line: usize) -> usize {
        crate::util::string::string_width(self.get_line(pos, line))
    }
}

impl<R> PeekableRecords for &R
where
    R: PeekableRecords,
{
    fn get_text(&self, pos: Position) -> &str {
        R::get_text(self, pos)
    }

    fn get_line(&self, pos: Position, line: usize) -> &str {
        R::get_line(self, pos, line)
    }

    fn count_lines(&self, pos: Position) -> usize {
        R::count_lines(self, pos)
    }

    fn get_width(&self, pos: Position) -> usize {
        R::get_width(self, pos)
    }

    fn get_line_width(&self, pos: Position, line: usize) -> usize {
        R::get_line_width(self, pos, line)
    }
}
