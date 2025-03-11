use tabled::{settings::style::Style, tables::CompactTable};
use testing_table::assert_table;

fn main() {
    let data = [
        ["Debian", "1.1.1.1", "true"],
        ["Arch", "127.1.1.1", "true"],
        ["Manjaro", "Arch", "true"],
    ];

    let table = CompactTable::from(data).with(Style::modern());
    let mut buf = [0; 1024 * 10];
    let mut w = Writer::new(&mut buf);

    table.fmt(&mut w).unwrap();

    assert_table!(w.as_str(),
        "┌─────────┬───────────┬──────┐"
        "│ Debian  │ 1.1.1.1   │ true │"
        "│─────────┼───────────┼──────│"
        "│ Arch    │ 127.1.1.1 │ true │"
        "│─────────┼───────────┼──────│"
        "│ Manjaro │ Arch      │ true │"
        "└─────────┴───────────┴──────┘"
    );
}

struct Writer<'a> {
    buf: &'a mut [u8],
    cursor: usize,
}

impl<'a> Writer<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, cursor: 0 }
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[0..self.cursor]).unwrap()
    }
}

impl core::fmt::Write for Writer<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let cap = self.buf.len();

        for (i, &b) in self.buf[self.cursor..cap]
            .iter_mut()
            .zip(s.as_bytes().iter())
        {
            *i = b;
        }

        self.cursor = usize::min(cap, self.cursor + s.as_bytes().len());

        Ok(())
    }
}
