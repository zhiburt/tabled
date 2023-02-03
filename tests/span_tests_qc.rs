use quickcheck::Arbitrary;
use tabled::{builder::Builder, object::Cell, Modify, Span, Style};

#[derive(Clone, Debug)]
struct TableStructure {
    pub rows: Vec<Line>,
    pub theme: ThemeFixture,
    pub row_span: Vec<u8>,
    pub col_span: Vec<u8>,
}

type Line = Vec<u32>;

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
            row_span: Arbitrary::arbitrary(g),
            col_span: Arbitrary::arbitrary(g),
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

#[test]
#[ignore = "Test fails"]
fn test_data_span_test() {
    let table_structure = TableStructure {
        rows: vec![
            vec![
                309, 340, 156, 295, 207, 231, 329, 294, 240, 343, 167, 299, 1,
            ],
            vec![944, 274, 1, 384, 693, 186, 358],
        ],
        theme: Modern,
        row_span: vec![0, 2],
        col_span: vec![10, 3],
    };

    let rows = table_structure.clone().rows;
    let theme = table_structure.clone().theme;
    let col_count = rows
        .iter()
        .map(|r| r.len())
        .reduce(usize::max)
        .unwrap_or_default();

    let mut builder = Builder::default();
    for r in rows.iter() {
        builder.add_record(r.iter().map(|e| e.to_string()).collect::<Vec<String>>());
    }

    builder.set_columns((1..col_count + 1).map(|head| head.to_string()));
    let mut table = builder.build();

    table.with(
        Modify::new(Cell(1, 2))
            .with(Span::row(0))
            .with(Span::column(10)),
    );
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

    let output = table.to_string();

    let table_lines: Vec<String> = output.lines().map(|l| l.into()).collect();
    let line_width =
        tabled::papergrid::util::string_width(table_lines.first().unwrap_or(&"".to_owned()));

    assert!(table_lines
        .iter()
        .all(|l| tabled::papergrid::util::string_width(l) == line_width));
}

#[quickcheck_macros::quickcheck]
#[ignore = "Test fails"]
fn qc_table_is_consistent_with_spans(table_structure: TableStructure) {
    let rows = table_structure.clone().rows;
    let theme = table_structure.clone().theme;
    let row_count = rows.len();
    let col_count = rows
        .iter()
        .map(|r| r.len())
        .reduce(usize::max)
        .unwrap_or_default();

    let mut row_span = table_structure.row_span;
    let mut col_span = table_structure.col_span;

    let mut builder = Builder::default();
    for r in rows.iter() {
        builder.add_record(r.iter().map(|e| e.to_string()).collect::<Vec<String>>());
    }

    builder.set_columns((1..col_count + 1).map(|head| head.to_string()));
    let mut table = builder.build();

    for rr in 1..row_count {
        if row_span.is_empty() && col_span.is_empty() {
            break;
        }
        for cc in 1..col_count {
            table.with(
                Modify::new(Cell(rr, cc))
                    .with(Span::row(row_span.pop().unwrap_or(1) as usize))
                    .with(Span::column(col_span.pop().unwrap_or(1) as usize)),
            );
        }
    }

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

    let output = table.to_string();

    let table_lines: Vec<String> = output.lines().map(|l| l.into()).collect();
    let line_width =
        tabled::papergrid::util::string_width(table_lines.first().unwrap_or(&"".to_owned()));
    assert!(table_lines
        .iter()
        .all(|l| tabled::papergrid::util::string_width(l) == line_width));
}
