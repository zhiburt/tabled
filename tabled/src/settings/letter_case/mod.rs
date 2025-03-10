use std::fmt::Write;

use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    settings::{CellOption, TableOption},
};

/// LetterCase changes a letter case of text word by word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LetterCase {
    /// Case: `lower case`
    Lower,
    /// Case: `UPPER CASE`
    Upper,
    /// Case: `Title Case`
    Title,
}

impl<R, D, C> TableOption<R, C, D> for LetterCase
where
    R: Records + ExactRecords + RecordsMut<String> + PeekableRecords,
{
    fn change(self, records: &mut R, cfg: &mut C, _: &mut D) {
        CellOption::change(self, records, cfg, Entity::Global);
    }
}

impl<R, C> CellOption<R, C> for LetterCase
where
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, _: &mut C, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();
        for pos in entity.iter(count_rows, count_cols) {
            let text = records.get_text(pos);
            let text = change_letter_case(text, self);
            records.set(pos, text);
        }
    }
}

#[cfg(not(feature = "ansi"))]
fn change_letter_case(text: &str, case: LetterCase) -> String {
    let mut buf = String::new();
    for (i, word) in text.split(' ').enumerate() {
        if i > 0 {
            buf.push(' ');
        }

        let word = convert_word_case(word, case);
        buf.push_str(&word);
    }

    buf
}

#[cfg(feature = "ansi")]
fn change_letter_case(text: &str, case: LetterCase) -> String {
    let mut buf = String::new();
    for (i, word) in ansi_str::AnsiStr::ansi_split(text, " ").enumerate() {
        if i > 0 {
            buf.push(' ');
        }

        let word = convert_word_case_colored(&word, case);
        buf.push_str(&word);
    }

    buf
}

#[cfg(feature = "ansi")]
fn convert_word_case_colored(text: &str, case: LetterCase) -> String {
    let mut buf = String::new();
    let mut word_started = false;

    if !ansi_str::AnsiStr::ansi_has_any(text) {
        return convert_word_case(text, case);
    }

    for b in ansi_str::get_blocks(text) {
        let _ = write!(&mut buf, "{}", b.style().start());

        let text = b.text();
        for c in text.chars() {
            match case {
                LetterCase::Lower => {
                    let _ = write!(&mut buf, "{}", c.to_lowercase());
                }
                LetterCase::Upper => {
                    let _ = write!(&mut buf, "{}", c.to_uppercase());
                }
                LetterCase::Title => {
                    if c == ' ' {
                        buf.push(c);
                        word_started = false;
                    } else {
                        if word_started {
                            let _ = write!(&mut buf, "{}", c.to_lowercase());
                        } else {
                            let _ = write!(&mut buf, "{}", c.to_uppercase());
                            word_started = true;
                        }
                    }
                }
            }
        }

        let _ = write!(&mut buf, "{}", b.style().end());
    }

    buf
}

fn convert_word_case(text: &str, case: LetterCase) -> String {
    match case {
        LetterCase::Lower => text.to_lowercase(),
        LetterCase::Upper => text.to_uppercase(),
        LetterCase::Title => {
            let mut buf = String::new();
            let mut chars = text.chars();

            if let Some(c) = chars.next() {
                let _ = write!(&mut buf, "{}", c.to_uppercase());
            }

            for c in chars {
                let _ = write!(&mut buf, "{}", c.to_lowercase());
            }

            buf.to_string()
        }
    }
}
