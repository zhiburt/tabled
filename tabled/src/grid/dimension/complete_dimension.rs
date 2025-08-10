use crate::grid::{
    config::{ColoredConfig, Entity, SpannedConfig},
    dimension::{Dimension, Estimate, PeekableGridDimension},
    records::{vec_records::Cell, IntoRecords, Records},
};

/// CompleteDimension is a [`Dimension`] implementation for a [`Table`]
///
/// [`Table`]: crate::Table
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
// todo; change to vec ....
pub struct CompleteDimension {
    width: Option<Vec<usize>>,
    height: Option<Vec<usize>>,
}

impl CompleteDimension {
    /// Checks whether is the dimensions is set.
    pub fn is_complete(&self) -> bool {
        self.width.is_some() && self.height.is_some()
    }

    /// Checks whether is nothing was set.
    pub fn is_empty(&self) -> bool {
        self.width.is_none() && self.height.is_none()
    }

    /// Set column widths.
    ///
    /// In general the method is only considered to be useful to a [`TableOption`].
    ///
    /// BE CAREFUL WITH THIS METHOD as it supposed that the content is not bigger than the provided widths.
    ///
    /// [`TableOption`]: crate::settings::TableOption
    pub fn set_widths(&mut self, columns: Vec<usize>) {
        self.width = Some(columns);
    }

    /// Set rows heights.
    ///
    /// In general the method is only considered to be useful to a [`TableOption`].
    ///
    /// BE CAREFUL WITH THIS METHOD as it supposed that the content is not bigger than the provided heights.
    ///
    /// [`TableOption`]: crate::settings::TableOption
    pub fn set_heights(&mut self, rows: Vec<usize>) {
        self.height = Some(rows);
    }

    /// Force width estimation.
    pub fn clear_width(&mut self) {
        self.width = None;
    }

    /// Force height estimation.
    pub fn clear_height(&mut self) {
        self.height = None;
    }

    /// Force width and height estimation.
    pub fn clear(&mut self) {
        self.width = None;
        self.height = None;
    }

    /// Copies a reference from self.
    pub fn reastimate(&mut self, hint: Option<Entity>) {
        dims_reastimate_likely(self, hint);
    }

    /// Return inner width list.
    pub fn get_widths(&self) -> Option<&[usize]> {
        self.width.as_deref()
    }

    /// Return inner heights list.
    pub fn get_heights(&self) -> Option<&[usize]> {
        self.height.as_deref()
    }
}

impl Dimension for CompleteDimension {
    fn get_width(&self, column: usize) -> usize {
        let width = self
            .width
            .as_ref()
            .expect("It must always be Some at this point");

        width[column]
    }

    fn get_height(&self, row: usize) -> usize {
        let height = self
            .height
            .as_ref()
            .expect("It must always be Some at this point");

        height[row]
    }
}

impl<R> Estimate<R, SpannedConfig> for CompleteDimension
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: Cell,
{
    fn estimate(&mut self, records: R, cfg: &SpannedConfig) {
        match (self.width.is_some(), self.height.is_some()) {
            (true, true) => {}
            (true, false) => {
                self.height = Some(PeekableGridDimension::height(records, cfg));
            }
            (false, true) => {
                self.width = Some(PeekableGridDimension::width(records, cfg));
            }
            (false, false) => {
                let (width, height) = PeekableGridDimension::dimension(records, cfg);

                self.width = Some(width);
                self.height = Some(height);
            }
        }
    }
}

impl<R> Estimate<R, ColoredConfig> for CompleteDimension
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: Cell,
{
    fn estimate(&mut self, records: R, cfg: &ColoredConfig) {
        Estimate::estimate(self, records, cfg.as_ref())
    }
}

fn dims_reastimate_likely(dims: &mut CompleteDimension, hint: Option<Entity>) {
    let hint = match hint {
        Some(hint) => hint,
        None => return,
    };

    match hint {
        Entity::Global | Entity::Cell(_, _) => {
            dims.clear_width();
            dims.clear_height()
        }
        Entity::Column(_) => {
            dims.clear_width();
        }
        Entity::Row(_) => dims.clear_height(),
    }
}
