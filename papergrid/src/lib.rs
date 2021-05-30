use std::{
    borrow::Cow,
    cmp::max,
    collections::HashMap,
    fmt::{self, write, Display},
    io, iter,
};

pub struct Grid {
    size: (usize, usize),
    rows_styles: Vec<RowStyle>,
    styles: HashMap<Entity, Settings>,
    cells: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct RowStyle {
    top_line: LineStyle,
    bottom_line: LineStyle,
    inner: LineStyle,
}

#[derive(Debug, Clone, Default)]
pub struct LineStyle {
    main: Option<char>,
    intersection: Option<char>,
    left_intersection: Option<char>,
    right_intersection: Option<char>,
}

impl LineStyle {
    fn is_empty(&self) -> bool {
        self.main.is_none()
            && self.intersection.is_none()
            && self.left_intersection.is_none()
            && self.right_intersection.is_none()
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
enum Entity {
    Global,
    Column(usize),
    Row(usize),
    Cell(usize, usize),
}

#[derive(Debug, Clone)]
struct Settings {
    ident: Ident,
    alignment: Alignment,
}

#[derive(Debug, Clone)]
struct Ident {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

#[derive(Debug, Clone)]
pub enum Alignment {
    Center,
    Left,
    Right,
}

impl Alignment {
    fn align(&self, text: &str, length: usize) -> String {
        match self {
            Alignment::Center => format!("{: ^1$}", text, length),
            Alignment::Left => format!("{: <1$}", text, length),
            Alignment::Right => format!("{: >1$}", text, length),
        }
    }
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut styles = HashMap::new();
        styles.insert(
            Entity::Global,
            Settings {
                alignment: Alignment::Left,
                ident: Ident {
                    bottom: 0,
                    left: 0,
                    right: 0,
                    top: 0,
                },
            },
        );

        let rows_styles = iter::repeat(RowStyle {
            inner: LineStyle {
                main: Some('-'),
                intersection: Some('|'),
                left_intersection: Some('|'),
                right_intersection: Some('|'),
            },
            bottom_line: LineStyle {
                main: Some('-'),
                intersection: Some('+'),
                left_intersection: Some('+'),
                right_intersection: Some('+'),
            },
            top_line: LineStyle {
                main: Some('-'),
                intersection: Some('+'),
                left_intersection: Some('+'),
                right_intersection: Some('+'),
            },
        })
        .take(rows)
        .collect();

        Grid {
            size: (rows, columns),
            cells: vec![vec![String::new(); columns]; rows],
            rows_styles,
            styles,
        }
    }

    pub fn cell(&mut self, i: usize, j: usize) -> &mut String {
        self.cells.get_mut(i).unwrap().get_mut(j).unwrap()
    }

    pub fn count_rows(&self) -> usize {
        self.size.0
    }

    pub fn count_columns(&self) -> usize {
        self.size.1
    }

    fn columns_width(&self) -> Vec<usize> {
        (0..self.count_columns())
            .map(|column| self.column_width(column))
            .collect()
    }

    // the function suppose you provide a correct column index
    fn column_width(&self, column: usize) -> usize {
        let mut width = 0;
        for row in 0..self.count_rows() {
            let style = self.style(row, column);
            let cell = &self.cells[row][column];
            let cell_width = string_width(cell) + style.ident.left + style.ident.right;
            width = max(width, cell_width);
        }

        width
    }

    fn rows_height(&self) -> Vec<usize> {
        (0..self.count_rows())
            .map(|row| self.row_height(row))
            .collect()
    }

    // the function suppose you provide a correct column index
    fn row_height(&self, row: usize) -> usize {
        let mut height = 0;
        for column in 0..self.count_columns() {
            let style = self.style(row, column);
            let cell = &self.cells[row][column];
            let cell_height = cell.lines().count() + style.ident.top + style.ident.bottom;
            height = max(height, cell_height);
        }

        height
    }

    fn style(&self, row: usize, column: usize) -> Settings {
        let v = [
            self.styles.get(&Entity::Cell(row, column)),
            self.styles.get(&Entity::Column(column)),
            self.styles.get(&Entity::Row(row)),
            self.styles.get(&Entity::Global),
        ];

        for styles in &v {
            if let Some(style) = styles {
                return (*style).clone();
            }
        }

        unreachable!("there's a global settings guaranted in the map")
    }

    fn build_cells<'a>(&'a self) -> Vec<Vec<Vec<String>>> {
        let columns_widths = self.columns_width();
        let rows_height = self.rows_height();

        let count_rows = self.count_rows();
        let count_columns = self.count_columns();
        let mut rows = Vec::with_capacity(count_rows);
        for row in 0..count_rows {
            let mut cells = Vec::with_capacity(count_columns);

            for column in 0..count_columns {
                let style = self.style(row, column);

                let text = build_cell(
                    &self.cells[row][column],
                    style,
                    columns_widths[column],
                    rows_height[row],
                );

                cells.push(text);
            }

            rows.push(cells);
        }

        rows
    }

    fn build_row(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        row: &[Vec<String>],
        border: &LineStyle,
    ) -> fmt::Result {
        let height = row.iter().map(|cell| cell.len()).max().unwrap_or_else(|| 0);
        for i in 0..height {
            write_option(f, border.left_intersection)?;

            for (y, cell) in row.iter().enumerate() {
                if y != 0 {
                    write_option(f, border.intersection)?;
                }

                write!(f, "{}", cell[i])?;
            }

            write_option(f, border.right_intersection)?;

            writeln!(f)?;
        }

        Ok(())
    }
}

// I like old solution with Full/Frame/Off

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // fixme: calculate it correctly
        let columns_width = self.columns_width();
        let rows = self.build_cells();

        for (i, row) in rows.iter().enumerate() {
            let border = self
                .rows_styles
                .get(i)
                .expect("it's expected that grid has N styles where N is an amount of rows");

            if i == 0 {
                build_line(f, &columns_width, &border.top_line)?;
            }

            self.build_row(f, &row, &border.inner)?;
            build_line(f, &columns_width, &border.bottom_line)?;
        }

        Ok(())
    }
}

fn build_cell(text: &str, style: Settings, column_w: usize, row_h: usize) -> Vec<String> {
    let width = column_w - style.ident.left - style.ident.right;
    let height = row_h - style.ident.top - style.ident.bottom;
    let text = split_text(text, width, height);

    let aligned_text = text
        .into_iter()
        .map(|line| style.alignment.align(&line, width));

    let aligned_text = aligned_text.map(|line| {
        format!(
            "{}{}{}",
            " ".repeat(style.ident.left),
            line,
            " ".repeat(style.ident.right)
        )
    });

    let mut complete_text =
        Vec::with_capacity(aligned_text.len() + style.ident.top + style.ident.bottom);
    complete_text.extend(iter::repeat(" ".repeat(column_w)).take(style.ident.top));
    complete_text.extend(aligned_text);
    complete_text.extend(iter::repeat(" ".repeat(column_w)).take(style.ident.bottom));

    complete_text
}

fn build_line(
    f: &mut std::fmt::Formatter<'_>,
    cells_width: &[usize],
    border: &LineStyle,
) -> fmt::Result {
    write_option(f, border.left_intersection)?;

    for (i, w) in cells_width.iter().enumerate() {
        write_option(f, border.main.map(|s| s.to_string().repeat(*w)))?;

        if i != cells_width.len() - 1 {
            write_option(f, border.intersection)?;
        }
    }

    write_option(f, border.right_intersection)?;
    
    if !border.is_empty() {
        writeln!(f)?;
    }

    Ok(())
}

fn write_option<D: Display>(f: &mut std::fmt::Formatter<'_>, text: Option<D>) -> fmt::Result {
    match text {
        Some(text) => write!(f, "{}", text),
        None => Ok(()),
    }
}

fn split_text(text: &str, width: usize, height: usize) -> Vec<Cow<str>> {
    let mut lines = textwrap::wrap(text, width);
    while lines.len() < height {
        lines.push(str::repeat(" ", width).into())
    }

    lines
}

fn string_width(text: &str) -> usize {
    text.lines()
        .map(|line| line.chars().filter(|c| !c.is_control()).count())
        .max()
        .unwrap_or_else(|| 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_1x1_test() {
        let mut grid = Grid::new(1, 1);
        *grid.cell(0, 0) = "asd".to_string();
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+\n\
             |asd|\n\
             +---+\n"
        )
    }

    #[test]
    fn grid_2x2_test() {
        let mut grid = Grid::new(2, 2);
        *grid.cell(0, 0) = "asd".to_string();
        *grid.cell(0, 1) = "asd".to_string();
        *grid.cell(1, 0) = "asd".to_string();
        *grid.cell(1, 1) = "asd".to_string();
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+---+\n\
             |asd|asd|\n\
             +---+---+\n\
             |asd|asd|\n\
             +---+---+\n"
        )
    }

    #[test]
    fn grid_2x2_without_vertical_border_test() {
        let mut grid = Grid::new(2, 2);
        *grid.cell(0, 0) = "asd".to_string();
        *grid.cell(0, 1) = "asd".to_string();
        *grid.cell(1, 0) = "asd".to_string();
        *grid.cell(1, 1) = "asd".to_string();
        let mut first_line = grid.rows_styles.get_mut(0).unwrap();
        first_line.top_line = LineStyle::default();
        first_line.bottom_line = LineStyle::default();
        first_line.inner = LineStyle {
            intersection: Some(' '),
            left_intersection: None,
            right_intersection: None,
            main: None,
        };
        drop(first_line);

        let mut second_line = grid.rows_styles.get_mut(1).unwrap();
        second_line.top_line = LineStyle::default();
        second_line.bottom_line = LineStyle::default();
        second_line.inner = LineStyle {
            intersection: Some(' '),
            left_intersection: None,
            right_intersection: None,
            main: None,
        };

        let str = grid.to_string();
        assert_eq!(
            str,
            "asd asd\n\
             asd asd\n"
        )
    }

    #[test]
    fn grid_2x2_custom_vertical_border_test() {
        let mut grid = Grid::new(2, 2);
        *grid.cell(0, 0) = "asd".to_string();
        *grid.cell(0, 1) = "asd".to_string();
        *grid.cell(1, 0) = "asd".to_string();
        *grid.cell(1, 1) = "asd".to_string();

        // todo: public interface
        let mut first_line = grid.rows_styles.get_mut(0).unwrap();
        first_line.top_line = LineStyle::default();
        first_line.bottom_line = LineStyle::default();
        first_line.inner = LineStyle {
            intersection: Some('@'),
            left_intersection: Some('$'),
            right_intersection: Some('%'),
            main: None,
        };
        drop(first_line);

        let mut second_line = grid.rows_styles.get_mut(1).unwrap();
        second_line.top_line = LineStyle::default();
        second_line.bottom_line = LineStyle::default();
        second_line.inner = LineStyle {
            intersection: Some('^'),
            left_intersection: Some('#'),
            right_intersection: Some('!'),
            main: None,
        };

        let str = grid.to_string();
        assert_eq!(
            str,
            "$asd@asd%\n\
             #asd^asd!\n"
        )
    }

    #[test]
    fn grid_2x2_vertical_resize_test() {
        let mut grid = Grid::new(2, 2);
        assert_eq!(string_width("asd  "), 5);
        *grid.cell(0, 0) = "asd".to_string();
        *grid.cell(0, 1) = "asd".to_string();
        *grid.cell(1, 0) = "asd".to_string();
        *grid.cell(1, 1) = "asd     ".to_string();
        // grid.with_column(0, Ident::Left(20));
        // grid.with_cell((1 ,1), Border('-'));
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+--------+\n\
             |asd|asd     |\n\
             +---+--------+\n\
             |asd|asd     |\n\
             +---+--------+\n"
        )
    }

    #[test]
    fn grid_3x2_test() {
        let mut grid = Grid::new(3, 2);
        *grid.cell(0, 0) = "asd".to_string();
        *grid.cell(0, 1) = "asd".to_string();
        *grid.cell(1, 0) = "asd".to_string();
        *grid.cell(1, 1) = "asd".to_string();
        *grid.cell(2, 0) = "asd".to_string();
        *grid.cell(2, 1) = "asd".to_string();
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+---+\n\
             |asd|asd|\n\
             +---+---+\n\
             |asd|asd|\n\
             +---+---+\n\
             |asd|asd|\n\
             +---+---+\n"
        )
    }
}
