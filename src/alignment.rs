use papergrid::{Alignment, Entity, Grid, Settings};

use crate::TableOption;

#[derive(Debug)]
pub struct HorizontalAlignment {
    alignment: Alignment,
    object: AlignmentObject,
}

impl HorizontalAlignment {
    pub fn new(alignment: Alignment, object: AlignmentObject) -> Self {
        Self { alignment, object }
    }
}

#[derive(Debug)]
pub enum AlignmentObject {
    Header,
    Data,
    Full,
}

impl TableOption for HorizontalAlignment {
    fn change(&self, grid: &mut Grid) {
        match self.object {
            AlignmentObject::Data => {
                for row in 1..grid.count_rows() {
                    grid.set(
                        Entity::Row(row),
                        Settings::new().alignment(self.alignment.clone()),
                    )
                }
            }
            AlignmentObject::Header => grid.set(
                Entity::Row(0),
                Settings::new().alignment(self.alignment.clone()),
            ),
            AlignmentObject::Full => grid.set(
                Entity::Global,
                Settings::new().alignment(self.alignment.clone()),
            ),
        }
    }
}
