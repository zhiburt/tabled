use crate::{
    grid::config::ColoredConfig,
    grid::records::{ExactRecords, Records, RecordsMut, Resizable},
    settings::TableOption,
};

use super::Panel;

/// Footer renders a [`Panel`] at the bottom.
/// See [`Panel`].
#[derive(Debug)]
pub struct Footer<S>(S);

impl<S> Footer<S> {
    /// Creates a new object.
    pub fn new(text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(text)
    }
}

impl<S, R, D> TableOption<R, D, ColoredConfig> for Footer<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + Resizable + RecordsMut<String>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dimension: &mut D) {
        Panel::horizontal(records.count_rows(), self.0.as_ref()).change(records, cfg, dimension);
    }
}
