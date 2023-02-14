use static_table::static_table;

#[test]
fn static_table_with_theme_rounded() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "ROUNDED");
    let expected = "╭───┬───┬─────╮\n\
                          │ 1 │ 2 │ 123 │\n\
                          ├───┼───┼─────┤\n\
                          │ 1 │ 2 │ 123 │\n\
                          │ 1 │ 2 │ 123 │\n\
                          ╰───┴───┴─────╯";
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_modern() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "MODERN");
    let expected = "┌───┬───┬─────┐\n\
                          │ 1 │ 2 │ 123 │\n\
                          ├───┼───┼─────┤\n\
                          │ 1 │ 2 │ 123 │\n\
                          ├───┼───┼─────┤\n\
                          │ 1 │ 2 │ 123 │\n\
                          └───┴───┴─────┘";
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_extended() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "EXTENDED");
    let expected = "╔═══╦═══╦═════╗\n\
                          ║ 1 ║ 2 ║ 123 ║\n\
                          ╠═══╬═══╬═════╣\n\
                          ║ 1 ║ 2 ║ 123 ║\n\
                          ╠═══╬═══╬═════╣\n\
                          ║ 1 ║ 2 ║ 123 ║\n\
                          ╚═══╩═══╩═════╝";
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_sharp() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "SHARP");
    let expected = "┌───┬───┬─────┐\n\
                          │ 1 │ 2 │ 123 │\n\
                          ├───┼───┼─────┤\n\
                          │ 1 │ 2 │ 123 │\n\
                          │ 1 │ 2 │ 123 │\n\
                          └───┴───┴─────┘";
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_ascii() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "ASCII");
    let expected = "+---+---+-----+\n\
                          | 1 | 2 | 123 |\n\
                          +---+---+-----+\n\
                          | 1 | 2 | 123 |\n\
                          +---+---+-----+\n\
                          | 1 | 2 | 123 |\n\
                          +---+---+-----+";
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_ascii_rounded() {
    let table = static_table!(
        [[1, 2, 123], [1, 2, 123], [1, 2, 123]],
        THEME = "ASCII_ROUNDED"
    );
    let expected = ".-------------.\n\
                          | 1 | 2 | 123 |\n\
                          | 1 | 2 | 123 |\n\
                          | 1 | 2 | 123 |\n\
                          '-------------'";
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_ascii_dots() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "DOTS");
    let expected = "...............\n\
                          : 1 : 2 : 123 :\n\
                          :...:...:.....:\n\
                          : 1 : 2 : 123 :\n\
                          :...:...:.....:\n\
                          : 1 : 2 : 123 :\n\
                          :...:...:.....:";
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_psql() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "PSQL");
    let expected = concat!(
        " 1 | 2 | 123 \n",
        "---+---+-----\n",
        " 1 | 2 | 123 \n",
        " 1 | 2 | 123 "
    );
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_markdown() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "MARKDOWN");
    let expected = "| 1 | 2 | 123 |\n\
                          |---|---|-----|\n\
                          | 1 | 2 | 123 |\n\
                          | 1 | 2 | 123 |";
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_re_structured_text() {
    let table = static_table!(
        [[1, 2, 123], [1, 2, 123], [1, 2, 123]],
        THEME = "RE_STRUCTURED_TEXT"
    );
    let expected = concat!(
        "=== === =====\n",
        " 1   2   123 \n",
        "=== === =====\n",
        " 1   2   123 \n",
        " 1   2   123 \n",
        "=== === ====="
    );
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_blank() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "BLANK");
    let expected = concat!(" 1   2   123 \n", " 1   2   123 \n", " 1   2   123 ");
    assert_eq!(table, expected);
}

#[test]
fn static_table_with_theme_empty() {
    let table = static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "EMPTY");
    let expected = concat!(" 1  2  123 \n", " 1  2  123 \n", " 1  2  123 ");
    assert_eq!(table, expected);
}
