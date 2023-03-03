use quickcheck::Arbitrary;
use quickcheck_macros::quickcheck;
use tabled::{
    builder::Builder,
    grid::util::string::{string_width, string_width_multiline},
    settings::style::Style,
    Table,
};

#[quickcheck]
fn qc_table_is_consistent(data: Vec<Vec<isize>>) -> bool {
    let mut builder = Builder::default();
    for row in data {
        let row = row.into_iter().map(|i| i.to_string()).collect::<Vec<_>>();
        builder.push_record(row);
    }

    let table = builder.build().to_string();

    let lines = table.lines().collect::<Vec<_>>();
    let lines_has_the_same_length = lines
        .iter()
        .map(|line| string_width(line))
        .all(|line_width| line_width == lines[0].len());
    lines_has_the_same_length
}

#[quickcheck]
fn qc_table_is_consistent_with_borders(table_structure: TableStructure) {
    let rows = table_structure.rows;
    let theme = table_structure.theme;

    let builder = Builder::from_iter(rows);

    let mut table = builder.build();
    set_theme(&mut table, theme);
    let output = table.to_string();

    if let Some(line) = output.lines().next() {
        assert_eq!(string_width(line), string_width_multiline(&output));
    }
}

#[derive(Clone, Debug)]
struct TableStructure {
    pub rows: Vec<Line>,
    pub theme: ThemeFixture,
}

type Line = Vec<String>;

#[derive(Clone, Debug)]
pub enum ThemeFixture {
    Empty,
    Blank,
    Ascii,
    Psql,
    Markdown,
    Modern,
    Sharp,
    Rounded,
    Extended,
    Dots,
    RestructuredText,
    AsciiRounded,
}
impl Arbitrary for TableStructure {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self {
            rows: Arbitrary::arbitrary(g),
            theme: ThemeFixture::arbitrary(g),
        }
    }
}

impl Arbitrary for ThemeFixture {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        use ThemeFixture::*;
        g.choose(&[
            Empty,
            Blank,
            Ascii,
            Psql,
            Markdown,
            Modern,
            Sharp,
            Rounded,
            Extended,
            Dots,
            RestructuredText,
            AsciiRounded,
        ])
        .unwrap()
        .to_owned()
    }
}

fn set_theme(table: &mut Table, theme: ThemeFixture) {
    use ThemeFixture::*;
    match theme {
        Empty => {
            table.with(Style::empty());
        }
        Blank => {
            table.with(Style::blank());
        }
        Ascii => {
            table.with(Style::ascii());
        }
        Psql => {
            table.with(Style::psql());
        }
        Markdown => {
            table.with(Style::markdown());
        }
        Modern => {
            table.with(Style::modern());
        }
        Sharp => {
            table.with(Style::sharp());
        }
        Rounded => {
            table.with(Style::rounded());
        }
        Extended => {
            table.with(Style::extended());
        }
        Dots => {
            table.with(Style::dots());
        }
        RestructuredText => {
            table.with(Style::re_structured_text());
        }
        AsciiRounded => {
            table.with(Style::ascii_rounded());
        }
    }
}
