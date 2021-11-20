#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Border, Entity, Grid, Settings};

pub struct Highlight {
    target: Target,
    border: Border,
}

impl Highlight {
    pub fn frame(border: Border) -> Self {
        Self::new(Target::Frame, border)
    }

    pub fn cell(row: usize, column: usize, border: Border) -> Self {
        Self::new(Target::Cell{row, column}, border)
    }
    
    pub fn row(row: usize, border: Border) -> Self {
        Self::new(Target::Row{from: row, to: row+1}, border)
    }

    pub fn column(column: usize, border: Border) -> Self {
        Self::new(Target::Column{from: column, to: column+1}, border)
    }

    fn new(target: Target, border: Border) -> Self { Self { target, border } }
}

pub enum Target {
    Cell {
        row: usize,
        column: usize,
    },
    Row{
        from: usize,
        to: usize,
    },
    Column {
        from: usize,
        to: usize,
    },
    Frame,
}

impl TableOption for Highlight {
    fn change(&mut self, grid: &mut Grid) {
        let settings = Settings::default().border(self.border.clone()).border_restriction(false);
        match &self.target {
            &Target::Cell { row, column } => {
                grid.set(&Entity::Cell(row, column), settings);
            },
            &Target::Row { from, .. } => {
                grid.set(&Entity::Row(from), settings);
            },
            &Target::Column { from, .. } => {
                grid.set(&Entity::Column(from), settings);
            },
            Target::Frame => {
                grid.set(&Entity::Global, settings);
            },
        }
    }
}