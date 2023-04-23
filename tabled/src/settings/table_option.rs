/// A trait which is responsible for configuration of a [`Table`].
///
/// [`Table`]: crate::Table
pub trait TableOption<R, D, C> {
    /// The function allows modification of records and a grid configuration.
    fn change(self, records: &mut R, cfg: &mut C, dimension: &mut D);
}

impl<T, R, D, C> TableOption<R, D, C> for &[T]
where
    for<'a> &'a T: TableOption<R, D, C>,
{
    fn change(self, records: &mut R, cfg: &mut C, dimension: &mut D) {
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
    fn change(self, records: &mut R, cfg: &mut C, dimension: &mut D) {
        for opt in self {
            opt.change(records, cfg, dimension)
        }
    }
}
