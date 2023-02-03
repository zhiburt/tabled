pub use crate::grid::config::GridConfig;

/// A trait which is responsilbe for configuration of a [`Table`].
pub trait TableOption<R, D> {
    /// The function modifies a [`Grid`] object.
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D);
}

impl<T, R, D> TableOption<R, D> for &mut T
where
    T: TableOption<R, D> + ?Sized,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        T::change(self, records, cfg, dimension);
    }
}

// todo: we can create 2 dimmerent TableOptions
// 1 will do actuall calculation and will do NOCache while other will do cache
//
// So Width::wrap will be something Wrap(Vec::new()) and will have some methods right in it.