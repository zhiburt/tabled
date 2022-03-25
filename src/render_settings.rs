use papergrid::{Entity, Grid, Settings};

use crate::CellOption;

#[derive(Debug, Default, Clone)]
pub struct RenderSettings {
    alignment_policy: Option<AlignmentStrategy>,
    trim_policy: Option<TrimStrategy>,
}

impl RenderSettings {
    pub fn alignement(mut self, policy: AlignmentStrategy) -> Self {
        self.alignment_policy = Some(policy);
        self
    }

    pub fn trim(mut self, policy: TrimStrategy) -> Self {
        self.trim_policy = Some(policy);
        self
    }
}

#[derive(Debug, Clone)]
pub enum AlignmentStrategy {
    PerCell,
    PerLine,
}

#[derive(Debug, Clone)]
pub enum TrimStrategy {
    Vertical,
    Horizontal,
    Both,
    None,
}

impl CellOption for RenderSettings {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let mut formatting = grid.style(&Entity::Cell(row, column)).formatting.clone();

        if let Some(policy) = &self.alignment_policy {
            match policy {
                AlignmentStrategy::PerCell => formatting.allow_lines_alignement = false,
                AlignmentStrategy::PerLine => formatting.allow_lines_alignement = true,
            }
        }

        if let Some(policy) = &self.trim_policy {
            match policy {
                TrimStrategy::Vertical => {
                    formatting.vertical_trim = true;
                }
                TrimStrategy::Horizontal => {
                    formatting.horizontal_trim = true;
                }
                TrimStrategy::Both => {
                    formatting.vertical_trim = true;
                    formatting.horizontal_trim = true;
                }
                TrimStrategy::None => {
                    formatting.vertical_trim = false;
                    formatting.horizontal_trim = false;
                }
            }
        }

        grid.set(
            &Entity::Cell(row, column),
            Settings::new().formatting(formatting),
        )
    }
}
