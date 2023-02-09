use crate::{grid::config::GridConfig, settings::TableOption};

/// This is a struct wrapper for a lambda which changes config.
#[derive(Debug)]
pub struct FormatConfig<F>(pub F)
where
    F: FnMut(&mut GridConfig);

impl<F, R, D> TableOption<R, D> for FormatConfig<F>
where
    F: FnMut(&mut GridConfig),
{
    fn change(&mut self, _: &mut R, cfg: &mut GridConfig, _: &mut D) {
        (self.0)(cfg);
    }
}
