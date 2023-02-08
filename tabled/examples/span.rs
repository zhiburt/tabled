use tabled::{
    settings::{
        alignment::Alignment,
        object::{Cell, Segment},
        span::Span,
        style::{CorrectSpans, Style},
        Modify,
    },
    Table,
};

fn main() {
    let data = [["just 1 column"; 5]; 5];

    let h_span = |r, c, span| Modify::new(Cell(r, c)).with(Span::column(span));
    let v_span = |r, c, span| Modify::new(Cell(r, c)).with(Span::row(span));

    let table = Table::new(data)
        .with(h_span(0, 0, 5).with("span all 5 columns"))
        .with(h_span(1, 0, 4).with("span 4 columns"))
        .with(h_span(2, 0, 2).with("span 2 columns"))
        .with(v_span(2, 4, 4).with("just 1 column\nspan\n4\ncolumns"))
        .with(v_span(3, 1, 2).with("span 2 columns\nspan\n2\ncolumns"))
        .with(v_span(2, 3, 3).with("just 1 column\nspan\n3\ncolumns"))
        .with(h_span(3, 1, 2))
        .with(Style::modern())
        .with(CorrectSpans)
        .with(Alignment::center_vertical())
        .to_string();

    println!("{}", table);
}
