/// A trait which is responsible for configuration of a [`Table`].
///
/// [`Table`]: crate::Table
pub trait TableOption<R, D, C> {
    /// The function allows modification of records and a grid configuration.
    fn change(&mut self, records: &mut R, cfg: &mut C, dimension: &mut D);
}

impl<T, R, D, C> TableOption<R, D, C> for &mut T
where
    T: TableOption<R, D, C> + ?Sized,
{
    fn change(&mut self, records: &mut R, cfg: &mut C, dimension: &mut D) {
        T::change(self, records, cfg, dimension);
    }
}

impl<T, R, D, C> TableOption<R, D, C> for [T]
where
    T: TableOption<R, D, C>,
{
    fn change(&mut self, records: &mut R, cfg: &mut C, dimension: &mut D) {
        for opt in self {
            opt.change(records, cfg, dimension)
        }
    }
}

#[cfg(feature = "std")]
impl<T, R, D, C> TableOption<R, D, C> for Vec<T>
where
    T: TableOption<R, D, C>,
{
    fn change(&mut self, records: &mut R, cfg: &mut C, dimension: &mut D) {
        for opt in self {
            opt.change(records, cfg, dimension)
        }
    }
}
