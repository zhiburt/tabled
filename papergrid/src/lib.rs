pub struct Grid {
    size: (usize, usize),
    cells: Vec<Cell>,
}

#[derive(Clone)]
pub struct Cell {
    content: String,
    alignment: Alignment,
    border: Border,
    ident: Ident,
    span_row: usize,
    span_column: usize,
}

#[derive(Clone)]
struct Border {
    top: String,
    bottom: String,
    left: String,
    right: String,
    corner: String,
}

#[derive(Clone)]
struct Ident {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Center,
    Left,
    Right,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        Grid {
            size: (rows, columns),
            cells: vec![Cell::new(); rows * columns],
        }
    }

    pub fn cell(&mut self, i: usize, j: usize) -> &mut Cell {
        let index = self.count_columns() * i + j;
        self.cells.get_mut(index).unwrap()
    }

    pub fn count_rows(&self) -> usize {
        self.size.0
    }

    pub fn count_columns(&self) -> usize {
        self.size.1
    }
}

fn rows<T>(slice: &[T], count_rows: usize, count_columns: usize) -> Vec<&[T]> {
    (0..count_rows)
        .map(|row_index| {
            let row_start = count_columns * row_index;
            &slice[row_start..row_start + count_columns]
        })
        .collect()
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = rows(&self.cells, self.size.0, self.size.1);

        let grid = create_formatters(&rows);
        let grid = size(&grid);
        let grid = resolve(&grid);

        if grid.is_empty() {
            write!(f, "")
        } else {
            writeln!(f, "{}", combine(&grid))
        }
    }
}

fn create_formatters(rows: &[&[Cell]]) -> Vec<Vec<CellFormatter>> {
    let count_columns = rows.get(0).map_or(0, |first_row| first_row.len());
    let count_rows = rows.len();

    let column_spaned = (0..count_rows)
        .flat_map(|row| {
            (0..count_columns)
                .flat_map(|cell| {
                    let span_column = rows[row][cell].span_column;
                    if span_column > 0 {
                        Some(
                            (row + 1..row + 1 + span_column)
                                .map(|row| (row, cell))
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    let row_spaned = (0..count_rows)
        .flat_map(|row| {
            (0..count_columns)
                .flat_map(|cell| {
                    let span_row = rows[row][cell].span_row;
                    if span_row > 0 {
                        Some(
                            (cell + 1..cell + 1 + span_row)
                                .map(|cell| (row, cell))
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    (0..count_rows)
        .map(|row| {
            (0..count_columns)
                .map(|cell| {
                    if row_spaned.contains(&(row, cell)) {
                        return CellFormatter::Empty(Orientation::Horizontal);
                    } else if column_spaned.contains(&(row, cell)) {
                        return CellFormatter::Empty(Orientation::Vertical);
                    }

                    let mut formatter = BoxFormatter::new(rows[row][cell].clone()).boxed();

                    if cell != 0 {
                        formatter = formatter.un_left().un_left_connection();
                    }

                    if row != 0 {
                        formatter = formatter.un_top();
                    }

                    CellFormatter::Boxed(formatter)
                })
                .collect()
        })
        .collect()
}

fn size(rows: &[Vec<CellFormatter>]) -> Vec<Vec<CellFormatter>> {
    let mut rows = rows.to_vec();
    if rows.is_empty() {
        return rows;
    }

    rows.iter_mut().for_each(|row| {
        if row.iter().filter(|cell| cell.span_column() == 0).count() < 2 {
            return;
        }

        let height = row
            .iter()
            .filter(|cell| cell.span_column() == 0)
            .map(|cell| cell.height())
            .max()
            .unwrap();

        row.iter_mut()
            .filter(|cell| cell.span_column() == 0)
            .for_each(|cell| cell.set_height(height));
    });

    let parent = |rows: &[CellFormatter], mut cell: usize| {
        while rows[cell].is_empty() {
            cell -= 1
        }
        cell
    };
    let count_cells = |rows: &[Vec<CellFormatter>], cell: usize| {
        rows.iter()
            .filter(|row| !row[cell].is_vertical_transparent())
            .count()
    };
    let column_height = |rows: &[Vec<CellFormatter>], cell: usize| {
        rows.iter()
            .map(|row| {
                if row[cell].is_horizontal_transparent() {
                    row[parent(row, cell)].height()
                } else {
                    row[cell].height()
                }
            })
            .sum::<usize>()
    };
    let column_length = |rows: &[Vec<CellFormatter>], cell: usize| {
        column_height(rows, cell) + count_cells(rows, cell) - 1
    };

    let cells_len = rows.first().map_or(0, |f| f.len());
    let (index, height) = (0..cells_len)
        .map(|cell| column_length(&rows, cell))
        .enumerate()
        .max_by_key(|&(_, height)| height)
        .map_or((0, 0), |w| w);

    let golden_cells = rows
        .iter()
        .map(|row| !row[index].is_vertical_transparent() && row[index].span_column() == 0)
        .collect::<Vec<bool>>();

    (0..cells_len).for_each(|cell| {
        let rest = height - column_length(&rows, cell);

        (0..rows.len())
            .filter(|&row| !rows[row][cell].is_empty())
            .filter(|&row| !(golden_cells[row] && rows[row][cell].span_column() == 0))
            .cycle()
            .take(rest)
            .collect::<Vec<_>>()
            .iter()
            .for_each(|&row| {
                let h = rows[row][cell].height() + 1;
                rows[row][cell].set_height(h);
            });
    });

    rows
}

fn resolve(rows: &[Vec<CellFormatter>]) -> Vec<Vec<CellFormatter>> {
    let mut rows = rows.to_vec();
    if rows.is_empty() {
        return rows;
    }

    let count_columns = rows.first().map_or(0, |r| r.len());

    (0..count_columns).for_each(|cell| {
        let cell_weight = rows
            .iter()
            .filter(|row| row[cell].span_row() == 0)
            .map(|row| row[cell].weight())
            .max()
            .map_or(0, |w| w);

        rows.iter_mut()
            .filter(|row| row[cell].span_row() == 0)
            .for_each(|row| row[cell].set_weight(cell_weight));
    });

    let parent = |rows: &[Vec<CellFormatter>], mut row: usize, column: usize| {
        while rows[row][column].is_empty() {
            row -= 1
        }
        row
    };
    let count_cells = |rows: &[Vec<CellFormatter>], row: usize| {
        rows[row]
            .iter()
            .filter(|cell| !cell.is_horizontal_transparent())
            .count()
    };
    let row_weight = |rows: &[Vec<CellFormatter>], row: usize| {
        rows[row]
            .iter()
            .enumerate()
            .map(|(column, cell)| {
                if cell.is_vertical_transparent() {
                    rows[parent(rows, row, column)][column].weight()
                } else {
                    cell.weight()
                }
            })
            .sum::<usize>()
    };
    let row_length = |rows: &[Vec<CellFormatter>], row: usize| {
        row_weight(rows, row) + count_cells(rows, row) - 1
    };

    let (index, weight) = (0..rows.len())
        .map(|row| row_length(&rows, row))
        .enumerate()
        .max_by_key(|&(_, weight)| weight)
        .map_or((0, 0), |w| w);

    let golden_cells = rows[index]
        .iter()
        .map(|cell| !cell.is_horizontal_transparent() && cell.span_row() == 0)
        .collect::<Vec<bool>>();

    (0..rows.len()).for_each(|row| {
        let rest = weight - row_length(&rows, row);
        if rest == 0 {
            return;
        }

        let row = &mut rows[row];
        (0..row.len())
            .filter(|&cell| !row[cell].is_horizontal_transparent())
            .filter(|&cell| !(golden_cells[cell] && row[cell].span_row() == 0))
            .cycle()
            .take(rest)
            .collect::<Vec<_>>()
            .iter()
            .for_each(|&cell| {
                let w = row[cell].weight() + 1;
                row[cell].set_weight(w);
            });
    });

    rows
}

impl Cell {
    fn new() -> Self {
        Cell {
            alignment: Alignment::Center,
            content: String::new(),
            border: Border {
                top: "-".to_owned(),
                bottom: "-".to_owned(),
                left: "|".to_owned(),
                right: "|".to_owned(),
                corner: "+".to_owned(),
            },
            ident: Ident {
                top: 0,
                bottom: 0,
                left: 0,
                right: 0,
            },
            span_row: 0,
            span_column: 0,
        }
    }

    pub fn set_content(&mut self, s: &str) -> &mut Self {
        self.content = s.to_owned();
        self
    }

    pub fn set_corner(&mut self, s: &str) -> &mut Self {
        self.border.corner = s.to_owned();
        self
    }

    pub fn set_alignment(&mut self, a: Alignment) -> &mut Self {
        self.alignment = a;
        self
    }

    pub fn set_vertical_ident(&mut self, size: usize) -> &mut Self {
        self.ident.top = size;
        self.ident.bottom = size;
        self
    }

    pub fn set_horizontal_ident(&mut self, size: usize) -> &mut Self {
        self.ident.left = size;
        self.ident.right = size;
        self
    }

    pub fn set_row_span(&mut self, size: usize) -> &mut Self {
        self.span_row = size;
        self
    }

    pub fn set_column_span(&mut self, size: usize) -> &mut Self {
        self.span_column = size;
        self
    }

    fn height(&self) -> usize {
        self.content.lines().count()
    }

    fn weight(&self) -> usize {
        self.content
            .lines()
            .map(|l| l.len())
            .max()
            .map_or(0, |max| max)
    }
}

#[derive(Clone)]
struct BoxFormatter {
    cell: Cell,
    left: Option<()>,
    right: Option<()>,
    top: Option<()>,
    bottom: Option<()>,
    left_connection: Option<()>,
    right_connection: Option<()>,
    weight: usize,
    height: usize,
}

impl BoxFormatter {
    fn new(cell: Cell) -> Self {
        BoxFormatter {
            cell: cell,
            left: None,
            right: None,
            top: None,
            bottom: None,
            left_connection: None,
            right_connection: None,
            weight: 0,
            height: 0,
        }
    }

    fn un_left(mut self) -> Self {
        self.left = None;
        self
    }

    fn un_left_connection(mut self) -> Self {
        self.left_connection = None;
        self
    }

    fn un_top(mut self) -> Self {
        self.top = None;
        self
    }

    fn boxed(mut self) -> Self {
        self.left = Some(());
        self.right = Some(());
        self.top = Some(());
        self.bottom = Some(());
        self.right_connection = Some(());
        self.left_connection = Some(());
        self
    }

    fn height(mut self, h: usize) -> Self {
        self.height = h;
        self
    }

    fn format(&self) -> String {
        let c = &self.cell;
        let weight = if self.weight == 0 {
            c.content
                .lines()
                .map(|l| l.chars().count())
                .max()
                .map_or(0, |max| max)
        } else {
            self.weight
        };

        let mut content = c.content.clone();
        let count_lines = c.content.chars().filter(|&c| c == '\n').count();

        if self.height > count_lines {
            content.push_str(&"\n".repeat(self.height - count_lines))
        }

        content.push_str(&"\n".repeat(c.ident.bottom));
        content.insert_str(0, &"\n".repeat(c.ident.top));

        let left_ident = " ".repeat(c.ident.left);
        let right_ident = " ".repeat(c.ident.right);

        let left_border = self.left.map_or("", |_| &c.border.left);
        let right_border = self.right.map_or("", |_| &c.border.right);

        let mut lines = content
            .lines()
            .map(|l| align(l, c.alignment, weight))
            .map(|l| format!("{}{}{}", left_ident, l, right_ident))
            .map(|l| {
                format!(
                    "{left:}{}{right:}",
                    l,
                    left = left_border,
                    right = right_border,
                )
            })
            .collect::<Vec<String>>();

        let lhs = self.left_connection.map_or("", |_| &c.border.corner);
        let rhs = self.right_connection.map_or("", |_| &c.border.corner);

        let weight = weight + c.ident.left + c.ident.right;

        if self.top.is_some() {
            let line = lhs.to_owned() + &c.border.top.repeat(weight) + rhs;
            lines.insert(0, line);
        }
        if self.bottom.is_some() {
            let line = lhs.to_owned() + &c.border.bottom.repeat(weight) + rhs;
            lines.push(line);
        }

        lines.join("\n")
    }
}

#[derive(Clone)]
enum CellFormatter {
    Boxed(BoxFormatter),
    Empty(Orientation),
}

#[derive(Clone)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl CellFormatter {
    fn is_empty(&self) -> bool {
        match self {
            CellFormatter::Empty(..) => true,
            _ => false,
        }
    }

    fn is_vertical_transparent(&self) -> bool {
        match self {
            CellFormatter::Empty(Orientation::Vertical) => true,
            _ => false,
        }
    }

    fn is_horizontal_transparent(&self) -> bool {
        match self {
            CellFormatter::Empty(Orientation::Horizontal) => true,
            _ => false,
        }
    }

    fn weight(&self) -> usize {
        match self {
            CellFormatter::Boxed(f) => {
                let w = if f.weight != 0 {
                    f.weight
                } else {
                    f.cell.weight()
                };

                w + f.cell.ident.left + f.cell.ident.right
            }
            _ => 0,
        }
    }

    fn span_row(&self) -> usize {
        match self {
            CellFormatter::Boxed(f) => f.cell.span_row,
            _ => 0,
        }
    }

    fn span_column(&self) -> usize {
        match self {
            CellFormatter::Boxed(f) => f.cell.span_column,
            _ => 0,
        }
    }

    fn height(&self) -> usize {
        match self {
            CellFormatter::Boxed(f) => {
                let h = if f.height != 0 {
                    f.height
                } else {
                    f.cell.height()
                };
                h + f.cell.ident.top + f.cell.ident.bottom
            }
            _ => 0,
        }
    }

    fn set_weight(&mut self, w: usize) {
        match self {
            CellFormatter::Boxed(f) => f.weight = w - f.cell.ident.left - f.cell.ident.right,
            _ => (),
        }
    }

    fn set_height(&mut self, h: usize) {
        match self {
            CellFormatter::Boxed(f) => f.height = h - f.cell.ident.top - f.cell.ident.bottom,
            _ => (),
        }
    }

    fn format(&self) -> String {
        match self {
            CellFormatter::Boxed(cell) => cell.format(),
            _ => String::from(""),
        }
    }
}

fn align(text: &str, a: Alignment, length: usize) -> String {
    match a {
        Alignment::Center => format!("{: ^1$}", text, length),
        Alignment::Left => format!("{: <1$}", text, length),
        Alignment::Right => format!("{: >1$}", text, length),
    }
}

fn concat_lines(a: &str, b: &str) -> String {
    assert_eq!(a.lines().count(), b.lines().count());
    a.lines()
        .zip(b.lines())
        .map(|(a, b)| a.to_owned() + b)
        .collect::<Vec<String>>()
        .join("\n")
}

fn write_by_line(a: &str, b: &str, line_gap: usize) -> String {
    if b.is_empty() {
        return a.to_owned();
    }

    let mut i = a.lines().skip(line_gap).collect::<Vec<_>>().join("\n");
    if i.is_empty() {
        i = "\n".repeat(b.lines().count());
    }

    let d = 1 + i.lines().count() - b.lines().count();
    let bb = b.to_owned() + &"\n".repeat(d);

    let added = concat_lines(&i, &bb);

    a.lines()
        .take(line_gap)
        .chain(added.lines())
        .collect::<Vec<&str>>()
        .join("\n")
}

fn combine(rows: &[Vec<CellFormatter>]) -> String {
    let mut check = vec![0; rows.len()];
    let row_len = rows.get(0).map_or(0, |r| r.len());
    let rows_len = rows
        .iter()
        .map(|row| {
            row.iter()
                .find(|f| f.span_column() == 0 && !f.is_empty())
                .map_or(
                    row.iter().next().map_or(0, |c| c.format().lines().count()),
                    |c| c.format().lines().count(),
                )
        })
        .collect::<Vec<usize>>();

    combine_fn(rows, &rows_len, &mut check, row_len)
}

fn combine_fn(
    rows: &[Vec<CellFormatter>],
    rows_len: &[usize],
    check: &mut [usize],
    border: usize,
) -> String {
    let mut buffer = String::new();
    let mut line_gap = 0;
    rows.iter().enumerate().for_each(|(row_index, row)| {
        row.iter()
            .enumerate()
            .take(border)
            .for_each(|(cell_index, cell)| {
                if check[row_index] <= cell_index {
                    if cell.span_column() > 0 {
                        let sub_s = combine_fn(
                            &rows[row_index + 1..],
                            &rows_len[row_index + 1..],
                            &mut check[row_index + 1..],
                            cell_index,
                        );
                        let next_gap = buffer.lines().count();
                        buffer = write_by_line(&buffer, &sub_s, next_gap);
                    }

                    buffer = write_by_line(&buffer, &cell.format(), line_gap);

                    check[row_index] += 1;
                }
            });

        line_gap += rows_len[row_index];
    });

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Might this behavior should be changed
    fn cell_formating_empty() {
        let mut cell = Cell::new();
        cell.set_content("").set_corner("-");

        let expected = concat!("--\n", "--");

        assert_eq!(expected, BoxFormatter::new(cell).boxed().format());
    }

    #[test]
    fn cell_formating_single() {
        let mut cell = Cell::new();
        cell.set_content("hello world").set_corner("-");

        let expected = concat!("-------------\n", "|hello world|\n", "-------------");

        assert_eq!(expected, BoxFormatter::new(cell).boxed().format());
    }

    #[test]
    fn cell_formating_multiline() {
        let mut cell = Cell::new();
        cell.set_content("hello\nworld").set_corner("-");

        let expected = concat!("-------\n", "|hello|\n", "|world|\n", "-------");

        assert_eq!(expected, BoxFormatter::new(cell).boxed().format());
    }

    #[test]
    fn cell_formating_multilane_forced() {
        let mut cell = Cell::new();
        cell.set_content("hello").set_corner("-");

        let expected = concat!("-------\n", "|hello|\n", "|     |\n", "-------");

        assert_eq!(expected, BoxFormatter::new(cell).boxed().height(2).format());
    }

    #[test]
    fn empty_cell_formating_with_height_2() {
        let mut cell = Cell::new();
        cell.set_content("").set_corner("-");

        let expected = concat!("--\n", "||\n", "||\n", "--");
        let formated_cell = BoxFormatter::new(cell).boxed().height(2).format();

        assert_eq!(expected, formated_cell);
    }

    #[test]
    fn empty_cell_formating_with_height_1() {
        let mut cell = Cell::new();
        cell.set_content("").set_corner("-");

        let expected = concat!("--\n", "||\n", "--");
        let formated_cell = BoxFormatter::new(cell).boxed().height(1).format();

        assert_eq!(expected, formated_cell);
    }

    #[test]
    fn cell_formating_with_height_2() {
        let mut cell = Cell::new();
        cell.set_content("text").set_corner("-");

        let expected = concat!("------\n", "|text|\n", "|    |\n", "------");
        let formated_cell = BoxFormatter::new(cell).boxed().height(2).format();

        assert_eq!(expected, formated_cell);
    }

    #[test]
    fn cell_new_line_formating_with_height_2() {
        let mut cell = Cell::new();
        cell.set_content("\n").set_corner("-");

        let expected = concat!("--\n", "||\n", "||\n", "--");
        let formated_cell = BoxFormatter::new(cell).boxed().height(2).format();

        assert_eq!(expected, formated_cell);
    }
}
