use crate::settings::TableOption;

/// This is a struct wrapper for a lambda which changes config.
#[derive(Debug)]
pub struct FormatConfig<F>(pub(crate) F);

impl<F, R, D, C> TableOption<R, D, C> for FormatConfig<F>
where
    F: FnMut(&mut C),
{
    fn change(mut self, _: &mut R, cfg: &mut C, _: &mut D) {
        (self.0)(cfg);
    }
}
