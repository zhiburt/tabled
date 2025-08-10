use crate::{
    grid::{
        config::Position,
        records::{ExactRecords, PeekableRecords, Records},
    },
    settings::object::{EntityOnce, Object},
};

/// The structure is an implementation of [`Location`] to search for cells with a given content.
///
/// [`Location`]: crate::settings::location::Location
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByValue<O, F> {
    search: O,
    f: F,
}

impl<O, F> ByValue<O, F> {
    /// Constructs a new object of the structure.
    pub fn new(search: O, f: F) -> Self
    where
        F: Fn(&str, &str) -> bool,
    {
        Self { search, f }
    }
}

impl<O, F, R> Object<R> for ByValue<O, F>
where
    O: Object<R>,
    F: Fn(&str, &str) -> bool,
    R: Records + PeekableRecords + ExactRecords,
{
    type Iter = EntityOnce;

    fn cells(&self, records: &R) -> Self::Iter {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();
        let cells = self.search.cells(records);

        let mut last: Option<&str> = None;
        let mut last_pos: Option<Position> = None;

        for e in cells.into_iter() {
            for pos in e.iter(count_rows, count_cols) {
                let text = records.get_text(pos);
                match last {
                    Some(last_value) => {
                        let take = (self.f)(text, last_value);
                        if take {
                            last = Some(text);
                            last_pos = Some(pos);
                        }
                    }
                    None => {
                        last = Some(text);
                        last_pos = Some(pos);
                    }
                }
            }
        }

        let pos = last_pos.map(|pos| pos.into());

        EntityOnce::new(pos)
    }
}
