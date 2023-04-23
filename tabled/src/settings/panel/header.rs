use crate::{
    grid::config::ColoredConfig,
    grid::records::{ExactRecords, Records, RecordsMut, Resizable},
    settings::TableOption,
};

use super::Panel;

/// Header inserts a [`Panel`] at the top.
/// See [`Panel`].
#[derive(Debug)]
pub struct Header<S>(S);

impl<S> Header<S> {
    /// Creates a new object.
    pub fn new(text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(text)
    }
}

impl<S, R, D> TableOption<R, D, ColoredConfig> for Header<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + Resizable + RecordsMut<String>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dimension: &mut D) {
        Panel::horizontal(0, self.0.as_ref()).change(records, cfg, dimension);
    }
}
