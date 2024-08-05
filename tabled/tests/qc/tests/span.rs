use quickcheck::Arbitrary;
use quickcheck_macros::quickcheck;

use tabled::{
    builder::Builder,
    grid::util::string::{get_line_width, get_text_width},
    settings::{Modify, Span, Style},
    Table,
};

#[quickcheck]
fn qc_tget_string_widthable_is_consistent_with_hspan_and_vspan(table_structure: TableStructure) {
    let mut table = create_table(table_structure.rows);
    set_theme(&mut table, table_structure.theme);
    set_span_hspan(&mut table, &table_structure.row_span);
    set_span_vspan(&mut table, &table_structure.col_span);

    let output = table.to_string();

    if let Some(line) = output.lines().next() {
        assert_eq!(get_line_width(line), get_text_width(&output));
    }
}

#[quickcheck]
fn qc_table_is_consistent_with_hspan(table_structure: TableStructure) {
    let mut table = create_table(table_structure.rows);
    set_theme(&mut table, table_structure.theme);
    set_span_hspan(&mut table, &table_structure.row_span);

    let output = table.to_string();

    if let Some(line) = output.lines().next() {
        assert_eq!(get_line_width(line), get_text_width(&output));
    }
}

#[quickcheck]
fn qc_table_is_consistent_with_vspan(table_structure: TableStructure) {
    let mut table = create_table(table_structure.rows);
    set_theme(&mut table, table_structure.theme);
    set_span_vspan(&mut table, &table_structure.col_span);

    let output = table.to_string();

    if let Some(line) = output.lines().next() {
        assert_eq!(get_line_width(line), get_text_width(&output));
    }
}

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
fn test_data_span_test() {
    let table_structure = TableStructure {
        rows: vec![
            vec![
                309, 340, 156, 295, 207, 231, 329, 294, 240, 343, 167, 299, 1,
            ],
            vec![944, 274, 1, 384, 693, 186, 358],
        ],
        theme: ThemeFixture::Modern,
        row_span: vec![0, 2],
        col_span: vec![10, 3],
    };

    let rows = table_structure.clone().rows;
    let theme = table_structure.theme;

    let mut table = create_table(rows);
    set_theme(&mut table, theme);
    table.with(
        Modify::new((1, 2))
            .with(Span::column(0))
            .with(Span::row(10)),
    );

    let output = table.to_string();

    if let Some(line) = output.lines().next() {
        assert_eq!(get_line_width(line), get_string_width(&output));
    }
}

fn set_span_hspan(table: &mut Table, list: &[u8]) {
    let mut i = 0;
    for r in 0..table.count_rows() {
        for c in 0..table.count_columns() {
            if list.len() <= i {
                return;
            }

            let span = list[i];
            table.with(Modify::new((r, c)).with(Span::column(span as usize)));

            i += 1;
        }
    }
}

fn set_span_vspan(table: &mut Table, list: &[u8]) {
    let mut i = 0;
    for r in 0..table.count_rows() {
        for c in 0..table.count_columns() {
            if list.len() <= i {
                return;
            }

            let span = list[i];
            table.with(Modify::new((r, c)).with(Span::row(span as usize)));

            i += 1;
        }
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

fn create_table(rows: Vec<Vec<u32>>) -> Table {
    let mut builder = Builder::default();
    for r in rows.iter() {
        builder.push_record(r.iter().map(|e| e.to_string()).collect::<Vec<String>>());
    }

    builder.build()
}
