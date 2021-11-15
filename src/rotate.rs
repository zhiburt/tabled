#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid};

/// Rotate can be used to rotate a table by 90 degrees.
#[derive(Debug)]
pub enum Rotate {
    Left,
    Right,
    Top,
    Bottom,
}

impl TableOption for Rotate {
    fn change(&mut self, grid: &mut Grid) {
        match self {
            Self::Left => {
                let mut new = Grid::new(grid.count_columns(), grid.count_rows());
                for row in 0..grid.count_rows() {
                    for (lhs_column, rhs_column) in
                        (0..grid.count_columns()).zip((0..grid.count_columns()).rev())
                    {
                        {
                            let border = grid.get_border(row, lhs_column);
                            if border.left.is_some() && !new.is_vertical_split_set(row) {
                                new.add_vertical_split(row)
                            }
    
                            if border.right.is_some() && !new.is_vertical_split_set(row+1) {
                                new.add_vertical_split(row+1)
                            }
    
                            if border.top.is_some() && !new.is_horizontal_split_set(rhs_column) {
                                new.add_horizontal_split(rhs_column)
                            }
    
                            if border.bottom.is_some() && !new.is_horizontal_split_set(rhs_column+1) {
                                new.add_horizontal_split(rhs_column+1)
                            }
                        }

                        let settings = grid.get_settings(row, lhs_column).span(1);
                        new.set(&Entity::Cell(rhs_column, row), settings)
                    }
                }

                *grid = new;
            }
            Self::Right => {
                let mut new = Grid::new(grid.count_columns(), grid.count_rows());
                let mut last_row = grid.count_rows();
                for row in 0..grid.count_rows() {
                    last_row -= 1;
                    for column in 0..grid.count_columns() {
                        {
                            let border = grid.get_border(row, column);
    
                            if border.left.is_some() && !new.is_vertical_split_set(last_row) {
                                new.add_vertical_split(last_row)
                            }
    
                            if border.right.is_some() && !new.is_vertical_split_set(last_row+1) {
                                new.add_vertical_split(last_row+1)
                            }
    
                            if border.top.is_some() && !new.is_horizontal_split_set(column) {
                                new.add_horizontal_split(column)
                            }
    
                            if border.bottom.is_some() && !new.is_horizontal_split_set(column+1) {
                                new.add_horizontal_split(column+1)
                            }
                        }

                        let border = grid.get_settings(row, column).span(0);
                        new.set(&Entity::Cell(column, last_row), border);
                    }
                }

                *grid = new;
            }
            Self::Bottom => {
                let mut new = Grid::new(grid.count_rows(), grid.count_columns());
                for column in 0..grid.count_columns() {
                    for row in 0..grid.count_rows() {
                        {
                            let last_row = grid.count_rows() - 1 - row;
                            let border = grid.get_border(last_row, column);
                            if border.left.is_some() && !new.is_vertical_split_set(column) {
                                new.add_vertical_split(column)
                            }
    
                            if border.right.is_some() && !new.is_vertical_split_set(column+1) {
                                new.add_vertical_split(column+1)
                            }
    
                            if border.top.is_some() && !new.is_horizontal_split_set(last_row) {
                                new.add_horizontal_split(last_row)
                            }
    
                            if border.bottom.is_some() && !new.is_horizontal_split_set(last_row+1) {
                                new.add_horizontal_split(last_row+1)
                            }
                        }

                        let last_row = grid.count_rows() - 1 - row;
                        let border = grid.get_settings(row, column).span(0);
                        new.set(&Entity::Cell(last_row, column), border)
                    }
                }

                *grid = new;
            }
            Self::Top => Self::Bottom.change(grid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let table = || Table::new([(123, 456, 789), (234, 567, 891)]);

        assert_eq!(
            table()
                .with(Rotate::Left)
                .with(Rotate::Left)
                .with(Rotate::Left)
                .with(Rotate::Left)
                .to_string(),
            table().to_string()
        );
        assert_eq!(
            table()
                .with(Rotate::Right)
                .with(Rotate::Right)
                .with(Rotate::Right)
                .with(Rotate::Right)
                .to_string(),
            table().to_string()
        );
        assert_eq!(
            table().with(Rotate::Right).with(Rotate::Left).to_string(),
            table().to_string()
        );
        assert_eq!(
            table().with(Rotate::Left).with(Rotate::Right).to_string(),
            table().to_string()
        );
        assert_eq!(
            table().with(Rotate::Bottom).with(Rotate::Top).to_string(),
            table().to_string()
        );
        assert_eq!(
            table()
                .with(Rotate::Bottom)
                .with(Rotate::Bottom)
                .to_string(),
            table().to_string()
        );
        assert_eq!(
            table().with(Rotate::Top).with(Rotate::Top).to_string(),
            table().to_string()
        );
    }

    #[test]
    fn test_3x3_box() {
        let table = Table::new([(123, 456, 789), (234, 567, 891)]);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+\n\
             | i32 | i32 | i32 |\n\
             +-----+-----+-----+\n\
             | 123 | 456 | 789 |\n\
             +-----+-----+-----+\n\
             | 234 | 567 | 891 |\n\
             +-----+-----+-----+\n"
        );

        let table = table.with(Rotate::Left);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+\n\
             | i32 | 789 | 891 |\n\
             +-----+-----+-----+\n\
             | i32 | 456 | 567 |\n\
             +-----+-----+-----+\n\
             | i32 | 123 | 234 |\n\
             +-----+-----+-----+\n"
        );

        let table = table.with(Rotate::Right).with(Rotate::Right);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+\n\
             | 234 | 123 | i32 |\n\
             +-----+-----+-----+\n\
             | 567 | 456 | i32 |\n\
             +-----+-----+-----+\n\
             | 891 | 789 | i32 |\n\
             +-----+-----+-----+\n"
        );
    }

    #[test]
    fn test_left_rotate() {
        let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+\n\
             | i32 | i32 | i32 |\n\
             +-----+-----+-----+\n\
             | 123 | 456 | 789 |\n\
             +-----+-----+-----+\n\
             | 234 | 567 | 891 |\n\
             +-----+-----+-----+\n\
             | 111 | 222 | 333 |\n\
             +-----+-----+-----+\n"
        );

        let table = table.with(Rotate::Left);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+-----+\n\
             | i32 | 789 | 891 | 333 |\n\
             +-----+-----+-----+-----+\n\
             | i32 | 456 | 567 | 222 |\n\
             +-----+-----+-----+-----+\n\
             | i32 | 123 | 234 | 111 |\n\
             +-----+-----+-----+-----+\n"
        );
    }

    #[test]
    fn test_right_rotate() {
        let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+\n\
             | i32 | i32 | i32 |\n\
             +-----+-----+-----+\n\
             | 123 | 456 | 789 |\n\
             +-----+-----+-----+\n\
             | 234 | 567 | 891 |\n\
             +-----+-----+-----+\n\
             | 111 | 222 | 333 |\n\
             +-----+-----+-----+\n"
        );

        let table = table.with(Rotate::Right);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+-----+\n\
             | 111 | 234 | 123 | i32 |\n\
             +-----+-----+-----+-----+\n\
             | 222 | 567 | 456 | i32 |\n\
             +-----+-----+-----+-----+\n\
             | 333 | 891 | 789 | i32 |\n\
             +-----+-----+-----+-----+\n"
        );
    }

    #[test]
    fn test_bottom_rotate() {
        let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+\n\
             | i32 | i32 | i32 |\n\
             +-----+-----+-----+\n\
             | 123 | 456 | 789 |\n\
             +-----+-----+-----+\n\
             | 234 | 567 | 891 |\n\
             +-----+-----+-----+\n\
             | 111 | 222 | 333 |\n\
             +-----+-----+-----+\n"
        );

        let table = table.with(Rotate::Bottom);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+\n\
             | 111 | 222 | 333 |\n\
             +-----+-----+-----+\n\
             | 234 | 567 | 891 |\n\
             +-----+-----+-----+\n\
             | 123 | 456 | 789 |\n\
             +-----+-----+-----+\n\
             | i32 | i32 | i32 |\n\
             +-----+-----+-----+\n"
        );
    }

    #[test]
    fn test_top_rotate() {
        let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+\n\
             | i32 | i32 | i32 |\n\
             +-----+-----+-----+\n\
             | 123 | 456 | 789 |\n\
             +-----+-----+-----+\n\
             | 234 | 567 | 891 |\n\
             +-----+-----+-----+\n\
             | 111 | 222 | 333 |\n\
             +-----+-----+-----+\n"
        );

        let table = table.with(Rotate::Top);
        assert_eq!(
            table.to_string(),
            "+-----+-----+-----+\n\
             | 111 | 222 | 333 |\n\
             +-----+-----+-----+\n\
             | 234 | 567 | 891 |\n\
             +-----+-----+-----+\n\
             | 123 | 456 | 789 |\n\
             +-----+-----+-----+\n\
             | i32 | i32 | i32 |\n\
             +-----+-----+-----+\n"
        );
    }
}
