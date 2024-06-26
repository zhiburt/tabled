[<img alt="github" src="https://img.shields.io/badge/github-zhiburt/tabled-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/zhiburt/tabled/)
[<img alt="crates.io" src="https://img.shields.io/crates/v/tabled.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tabled)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tabled-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tabled)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/zhiburt/tabled/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/zhiburt/tabled/actions)
[<img alt="coverage" src="https://img.shields.io/coveralls/github/zhiburt/tabled/master?style=for-the-badge" height="20">](https://coveralls.io/github/zhiburt/tabled)
[<img alt="dependency status" src="https://deps.rs/repo/github/zhiburt/tabled/status.svg?style=for-the-badge" height="20">](https://deps.rs/repo/github/zhiburt/tabled)

# <a href="#"> <img alt="logo" align="center" src="https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg" href="/" width=65> </a> <span> tabled </span>

An easy to use library for pretty printing tables of Rust `struct`s and `enum`s.

You can do a lot of things with the library.\
If it doesn't do something which you feel it should or it's not clear how to, please file an issue.

This README contains a lot of information but it might be not complete,\
you can find more examples in an **[examples](/tabled/examples/)** folder.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/preview-show.gif">
  <img alt="Preview" src="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/preview-show-light.gif">
</picture>

## Table of Contents

- [Usage](#usage)
- [Settings](#settings)
  - [Style](#style)
    - [Styles](#styles)
      - [ascii](#ascii)
      - [modern](#modern)
      - [sharp](#sharp)
      - [rounded](#rounded)
      - [extended](#extended)
      - [psql](#psql)
      - [markdown](#markdown)
      - [re\_structured\_text](#re_structured_text)
      - [dots](#dots)
      - [ascii\_rounded](#ascii_rounded)
      - [blank](#blank)
      - [empty](#empty)
    - [Customization](#style-customization)
    - [Cell Border](#cell-border)
    - [Text on borders](#text-on-borders)
    - [Colorize borders](#colorize-borders)
    - [Theme](#theme)
      - [Colorize content](#colorize-content)
      - [Column names](#colorize-content)
  - [Alignment](#alignment)
  - [Format](#format)
  - [Padding](#padding)
  - [Margin](#margin)
    - [Color](#margin-color)
  - [Shadow](#shadow)
  - [Width](#width)
    - [Truncate](#truncate)
    - [Wrapping](#wrapping)
    - [Increase width](#increase-width)
    - [Justify](#justify)
    - [Priority](#priority)
    - [Percent](#percent)
  - [Height](#height)
    - [Height Increase](#height-increase)
    - [Height Limit](#height-limit)
  - [Rotate](#rotate)
  - [Disable](#disable)
  - [Extract](#extract)
  - [Header and Footer and Panel](#header-and-footer-and-panel)
  - [Merge](#merge)
  - [Concat](#concat)
  - [Highlight](#highlight)
  - [Span](#span)
    - [Horizontal span](#horizontal-span)
    - [Vertical span](#vertical-span)
  - [Split](#split)
  - [Duplicate](#duplicate)
- [Derive](#derive)
  - [Override a column name](#override-a-column-name)
  - [Hide a column](#hide-a-column)
  - [Set column order](#set-column-order)
  - [Format fields](#format-fields)
  - [Format headers](#format-headers)
  - [Inline](#inline)
- [Table types](#table-types)
  - [`Table`](#table)
  - [`IterTable`](#itertable)
  - [`CompactTable`](#compacttable)
  - [`PoolTable`](#pooltable)
  - [`ExtendedTable`](#extendedtable)
- [Tips and Tricks](#tips-and-tricks)
  - [`std::fmt::*` options](#std::fmt::*-options)
  - [Tuple combination](#tuple-combination)
  - [Object](#object)
  - [Builder](#builder)
  - [Macros](#macros)
    - [`col` and `row`](#col-and-row)
    - [`static_table`](#static_table)
- [Features](#features)
- [Formats](#formats)
  - [`json` format](#json-format)
  - [`ron` format](#ron-format)
  - [`csv` format](#csv-format)
  - [`toml` format](#toml-format)
  - [`html` format](#html-format)
- [Notes](#notes)
  - [Charset](#charset)
  - [ANSI escape codes](#ansi-escape-codes)
  - [Emoji](#emoji)
  - [Semver](#semver)
  - [MSRV](#msrv)
  - [Comparison](#comparison)

## Usage

To print a list of structs or enums as a table, there are 2 ways.

* Using a builder method, to build a table step by step
* Implement a `Tabled` trait for your type (or annotate your type with a derive macro) and use a iterator of this type.

A builder method gets handy, when a data schema is unknown,\
while a typed struct is useful in cases where we know the data structure beforehand.

Notice that there are a lot of [*mods*](#settings) available for your tables,
as well as helpers such as [*derive macros*](#derive) and [*proc macros*](#macros).

Below are shown both of these methods.\
The example below is demonstrates a derive method.

```rust
use tabled::{Tabled, Table};

#[derive(Tabled)]
struct Language {
    name: &'static str,
    designed_by: &'static str,
    invented_year: usize,
}

let languages = vec![
    Language{
        name: "C",
        designed_by: "Dennis Ritchie",
        invented_year: 1972
    },
    Language{
        name: "Go",
        designed_by: "Rob Pike",
        invented_year: 2009
    },
    Language{
        name: "Rust",
        designed_by: "Graydon Hoare",
        invented_year: 2010
    },
];

let table = Table::new(languages).to_string();

let expected = "+------+----------------+---------------+\n\
                | name | designed_by    | invented_year |\n\
                +------+----------------+---------------+\n\
                | C    | Dennis Ritchie | 1972          |\n\
                +------+----------------+---------------+\n\
                | Go   | Rob Pike       | 2009          |\n\
                +------+----------------+---------------+\n\
                | Rust | Graydon Hoare  | 2010          |\n\
                +------+----------------+---------------+";

assert_eq!(table, expected);
```

The next example shows a builder example.

```rust
use tabled::{builder::Builder, settings::Style};

let lyrics = r#"
    And the cat's in the cradle and the silver spoon
    Little boy blue and the man on the moon
    When you comin' home dad?
    I don't know when, but we'll get together then son
    You know we'll have a good time then
"#;

let mut builder = Builder::default();
for line in lyrics.lines() {
    let line = line.trim();
    if line.is_empty() {
        continue;
    }

    let words: Vec<_> = line.split_terminator(' ').collect();
    builder.push_record(words);
}

let columns = (0..builder.count_columns()).map(|i| i.to_string());
builder.insert_record(0, columns);

let mut table = builder.build();
table.with(Style::ascii_rounded());

let expected = concat!(
    ".------------------------------------------------------------------------------------.\n",
    "| 0      | 1     | 2      | 3     | 4    | 5      | 6    | 7        | 8      | 9     |\n",
    "| And    | the   | cat's  | in    | the  | cradle | and  | the      | silver | spoon |\n",
    "| Little | boy   | blue   | and   | the  | man    | on   | the      | moon   |       |\n",
    "| When   | you   | comin' | home  | dad? |        |      |          |        |       |\n",
    "| I      | don't | know   | when, | but  | we'll  | get  | together | then   | son   |\n",
    "| You    | know  | we'll  | have  | a    | good   | time | then     |        |       |\n",
    "'------------------------------------------------------------------------------------'",
);

assert_eq!(table, expected);
```

## Settings

This section lists the set of settings you can apply to your table.
Most of the settings are leveraged by `Table::with` and `Table::modify`.

### Style

#### Styles

There are a list of ready to use styles.
Each style can be customized.
A custom style also can be created from scratch.

A style can be used like this.

```rust
use tabled::{Table, Style};

let mut table = Table::new(&data);
table.with(Style::psql());
```

Below is a rendered list of the preconfigured styles.

If you think that there's some valuable style to be added,
please open an issue.

##### ascii

```text
+------+----------------+---------------+
| name | designed_by    | invented_year |
+------+----------------+---------------+
| C    | Dennis Ritchie | 1972          |
+------+----------------+---------------+
| Rust | Graydon Hoare  | 2010          |
+------+----------------+---------------+
| Go   | Rob Pike       | 2009          |
+------+----------------+---------------+
```

##### modern

```text
┌──────┬────────────────┬───────────────┐
│ name │ designed_by    │ invented_year │
├──────┼────────────────┼───────────────┤
│ C    │ Dennis Ritchie │ 1972          │
├──────┼────────────────┼───────────────┤
│ Rust │ Graydon Hoare  │ 2010          │
├──────┼────────────────┼───────────────┤
│ Go   │ Rob Pike       │ 2009          │
└──────┴────────────────┴───────────────┘
```

##### sharp

```text
┌──────┬────────────────┬───────────────┐
│ name │ designed_by    │ invented_year │
├──────┼────────────────┼───────────────┤
│ C    │ Dennis Ritchie │ 1972          │
│ Rust │ Graydon Hoare  │ 2010          │
│ Go   │ Rob Pike       │ 2009          │
└──────┴────────────────┴───────────────┘
```

##### rounded

```text
╭──────┬────────────────┬───────────────╮
│ name │ designed_by    │ invented_year │
├──────┼────────────────┼───────────────┤
│ C    │ Dennis Ritchie │ 1972          │
│ Rust │ Graydon Hoare  │ 2010          │
│ Go   │ Rob Pike       │ 2009          │
╰──────┴────────────────┴───────────────╯
```

##### extended

```text
╔══════╦════════════════╦═══════════════╗
║ name ║ designed_by    ║ invented_year ║
╠══════╬════════════════╬═══════════════╣
║ C    ║ Dennis Ritchie ║ 1972          ║
╠══════╬════════════════╬═══════════════╣
║ Rust ║ Graydon Hoare  ║ 2010          ║
╠══════╬════════════════╬═══════════════╣
║ Go   ║ Rob Pike       ║ 2009          ║
╚══════╩════════════════╩═══════════════╝
```

##### psql

```text
 name | designed_by    | invented_year 
------+----------------+---------------
 C    | Dennis Ritchie | 1972          
 Rust | Graydon Hoare  | 2010          
 Go   | Rob Pike       | 2009          
```

##### markdown

```text
| name | designed_by    | invented_year |
|------|----------------|---------------|
| C    | Dennis Ritchie | 1972          |
| Rust | Graydon Hoare  | 2010          |
| Go   | Rob Pike       | 2009          |
```

##### re_structured_text

```text
====== ================ ===============
 name   designed_by     invented_year 
====== ================ ===============
 C      Dennis Ritchie   1972          
 Rust   Graydon Hoare    2010          
 Go     Rob Pike         2009          
====== ================ ===============
```

##### dots

```text
.........................................
: name : designed_by    : invented_year :
:......:................:...............:
: C    : Dennis Ritchie : 1972          :
: Rust : Graydon Hoare  : 2010          :
: Go   : Rob Pike       : 2009          :
:......:................:...............:
```

##### ascii_rounded

```text
.---------------------------------------.
| name | designed_by    | invented_year |
| C    | Dennis Ritchie | 1972          |
| Rust | Graydon Hoare  | 2010          |
| Go   | Rob Pike       | 2009          |
'---------------------------------------'
```

##### blank

```text
 name   designed_by      invented_year 
 C      Dennis Ritchie   1972          
 Rust   Graydon Hoare    2010          
 Go     Rob Pike         2009                 
```

##### empty

```text
name designed_by    invented_year
C    Dennis Ritchie 1972         
Rust Graydon Hoare  2010         
Go   Rob Pike       2009         
```

#### Style customization

You can modify existing styles to fit your needs.
Notice that all modifications are done at compile time.

Check the [documentation](https://docs.rs/tabled/latest/tabled/settings/struct.Style.html) for
more customization options.

```rust
use tabled::settings::{Style, HorizontalLine, VerticalLine};

let style = Style::modern()
    .horizontals([(1, HorizontalLine::inherit(Style::modern()).horizontal('═'))])
    .verticals([(1, VerticalLine::inherit(Style::modern()))])
    .remove_horizontal()
    .remove_vertical();
```

The style will look like the following for the first example.

```rust
┌──────┬───────────────────────────────┐
│ name │ designed_by     invented_year │
├══════┼═══════════════════════════════┤
│ C    │ Dennis Ritchie  1972          │
│ Go   │ Rob Pike        2009          │
│ Rust │ Graydon Hoare   2010          │
└──────┴───────────────────────────────┘
```

Also you can build your own one from scratch, and doing so is not always possible at compile time,
so you may use `Theme` object to do that.

Notice that the `Theme` is quite powerful by itself, you can check it in the [documentation](https://docs.rs/tabled/latest/tabled/settings/struct.Theme.html).

```rust
use tabled::grid::config::{Border, HorizontalLine};
use tabled::settings::Theme;

let mut style = Theme::default();
style.set_lines_horizontal(HashMap::from_iter([(1, HorizontalLine::full('-', '-', '+', '+'))]));
style.set_border_frame(Border::filled('+'));
```

The style will look like the following.

```rust
+++++++++++++++++++++++++++++++++++++++
+ name  designed_by     invented_year +
+-------------------------------------+
+ C     Dennis Ritchie  1972          +
+ Go    Rob Pike        2009          +
+ Rust  Graydon Hoare   2010          +
+++++++++++++++++++++++++++++++++++++++
```

#### Cell Border

Sometimes `tabled::Style` settings are not enough.
Sometimes it's nesessary to change a border of a particular cell.

For this purpose you can use `Border`.

```rust
use tabled::{settings::{object::Rows, Border, Style}, Table};

let data = [["123", "456"], ["789", "000"]];

let table = Table::new(data)
    .with(Style::ascii())
    .modify(Rows::first(), Border::new().set_top('x'))
    .to_string();

assert_eq!(
    table,
    "+xxxxx+xxxxx+\n\
     | 0   | 1   |\n\
     +-----+-----+\n\
     | 123 | 456 |\n\
     +-----+-----+\n\
     | 789 | 000 |\n\
     +-----+-----+"
);
```

#### Text on borders

You can set a string to a horizontal border line.

```rust
use tabled::{settings::style::LineText, Table};
use tabled::settings::object::Rows;

let mut table = Table::new(["Hello World"]);
table.with(LineText::new("+-.table", Rows::first()));

assert_eq!(
    table.to_string(),
    "+-.table------+\n\
     | &str        |\n\
     +-------------+\n\
     | Hello World |\n\
     +-------------+"
);
```

Sometimes though it's not convenient to set a string.
But rather necessary to set a custom char.

You can use `LineChar` to achieve this.

```rust
use tabled::{
    settings::{
        object::Columns,
        style::{LineChar, Offset},
        Modify, Style,
    },
    Table,
};

let table = Table::new([["Hello", "World", "!"]])
    .with(Style::markdown())
    .with(
        Modify::new(Columns::new(..))
            .with(LineChar::horizontal(':', Offset::Begin(0)))
            .with(LineChar::horizontal(':', Offset::End(0))),
    )
    .to_string();

assert_eq!(
    table,
    "| 0     | 1     | 2 |\n\
     |:-----:|:-----:|:-:|\n\
     | Hello | World | ! |"
);
```

#### Colorize borders

You can set the colors of all borders using `Color`.

```rust
use tabled::settings::{style::BorderColor, Color};

table.with(BorderColor::new().set_top(Color::FG_GREEN));
```

You can also set a color border of an individual cell by using `BorderColored`.

```rust
use tabled::settings::{object::Columns, style::BorderColor, Color};

table.modify(Columns::single(2), BorderColor::new().set_top(Color::FG_GREEN));
```

#### Theme

It was considered that having a few atomic settings is better rather then have one but feature full.

But `tabled::settings::themes::*` diverges a bit from this idea.
It contains a list of settings which do bigger changes to the table.

The first one is `Theme` itself.
You can change layout, set style, colorize, config borders and even reverse the table with it.

```rust
use tabled::settings::{
    object::{Columns, Object},
    Alignment, Style, Theme,
};

let mut style = Theme::from_style(Style::ascii_rounded());
style.remove_border_horizontal();
style.remove_border_vertical();
style.align_columns(Alignment::left());
style.set_footer(true);

table.with(style);
table.modify(Columns::new(1..).not(Columns::last()), Alignment::center());
table.modify(Columns::last(), Alignment::right());
```

You'll see the following output when run against the first example.

```text
.---------------------------------------------------------------------------.
| name                  C             Go          Rust                 name |
| designed_by     Dennis Ritchie   Rob Pike   Graydon Hoare     designed_by |
| invented_year        1972          2009         2010        invented_year |
'---------------------------------------------------------------------------'
```

##### Colorize content

You can colorize the content by the pattern or a specific cell.

```rust
use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    settings::{object::Rows, style::Style, themes::Colorization, Color},
};

let data = vec![
    vec!["Word", "Translation", "Lang"],
    vec!["World", "le monde", "FR"],
    vec!["World", "Welt", "DE"],
];

let color_col1 = Color::BG_GREEN | Color::FG_BLACK;
let color_col2 = Color::BG_MAGENTA | Color::FG_BLACK;
let color_col3 = Color::BG_YELLOW | Color::FG_BLACK;
let color_head = Color::BG_WHITE | Color::FG_BLACK;
let color_head_text = Color::BG_BLUE | Color::FG_BLACK;

let mut table = Builder::from_iter(data).build();
table
    .with(Style::empty())
    .with(Colorization::columns([color_col1, color_col2, color_col3]))
    .with(Colorization::exact([color_head], Rows::first()))
    .modify(Rows::first(), color_head_text);

println!("{table}");
```

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zhiburt/tabled/assets/20165848/5dcff779-0aa5-499e-bb08-4373459c9db6">
  <img alt="Preview" src="https://github.com/zhiburt/tabled/assets/20165848/d6f9a936-9d33-4d0b-acfc-9ef875d59cc3">

</picture>

##### Column names

You can move the header right to the borders.

```rust
use tabled::{
    builder::Builder,
    settings::{style::Style, themes::ColumnNames},
};

let data = vec![
    vec![String::from("header 0"), String::from("header 1")],
    vec![String::from("Hello"), String::from("World")],
    vec![String::from("Bonjour"), String::from("le monde")],
    vec![String::from("Hallo"), String::from("Welt")],
];

let mut table = Builder::from(data).build();
table.with(Style::modern()).with(ColumnNames::default());

println!("{table}");
```

```text
┌header 0─┬header 1──┐
│ Hello   │ World    │
├─────────┼──────────┤
│ Bonjour │ le monde │
├─────────┼──────────┤
│ Hallo   │ Welt     │
└─────────┴──────────┘
```

### Alignment

You can set a horizontal and vertical alignment for any `Object` (e.g `Columns`, `Rows`).

```rust
use tabled::{
    settings::{object::Segment, Alignment, Settings},
    Table,
};

let data = [("Text", "Multiline\ntext"), ("text", "text")];
let mut table = Table::new(data);
table.modify(
    Segment::all(),
    Settings::new(Alignment::right(), Alignment::bottom()),
);

println!("{table}");
```

The output would be.

```text
+------+-----------+
| &str |      &str |
+------+-----------+
|      | Multiline |
| Text | text      |
+------+-----------+
| text |      text |
+------+-----------+
```

### Format

The `Format` function provides an interface for modification of cells.

```rust
use tabled::{
    settings::{format::Format, object::Rows},
    Table,
};

let data = vec![[0; 10]; 9];
let mut table = Table::new(data);
table.modify(
    Rows::new(..),
    Format::positioned(|_, (row, col)| ((row + 1) * (col + 1)).to_string()),
);

println!("{table}");
```

The result you must get will be.

```text
+----+----+----+----+----+----+----+----+----+-----+
| 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  | 9  | 10  |
+----+----+----+----+----+----+----+----+----+-----+
| 2  | 4  | 6  | 8  | 10 | 12 | 14 | 16 | 18 | 20  |
+----+----+----+----+----+----+----+----+----+-----+
| 3  | 6  | 9  | 12 | 15 | 18 | 21 | 24 | 27 | 30  |
+----+----+----+----+----+----+----+----+----+-----+
| 4  | 8  | 12 | 16 | 20 | 24 | 28 | 32 | 36 | 40  |
+----+----+----+----+----+----+----+----+----+-----+
| 5  | 10 | 15 | 20 | 25 | 30 | 35 | 40 | 45 | 50  |
+----+----+----+----+----+----+----+----+----+-----+
| 6  | 12 | 18 | 24 | 30 | 36 | 42 | 48 | 54 | 60  |
+----+----+----+----+----+----+----+----+----+-----+
| 7  | 14 | 21 | 28 | 35 | 42 | 49 | 56 | 63 | 70  |
+----+----+----+----+----+----+----+----+----+-----+
| 8  | 16 | 24 | 32 | 40 | 48 | 56 | 64 | 72 | 80  |
+----+----+----+----+----+----+----+----+----+-----+
| 9  | 18 | 27 | 36 | 45 | 54 | 63 | 72 | 81 | 90  |
+----+----+----+----+----+----+----+----+----+-----+
| 10 | 20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 | 100 |
+----+----+----+----+----+----+----+----+----+-----+
```

### Padding

The `Padding` structure provides an interface for left, right, top and bottom padding of cells.
You can set indent size and color of the padding.

```rust
use tabled::settings::{
    object::{Columns, Object, Rows},
    Color, Padding,
};

// Set a padding size for first column
table.modify(Columns::first().not((0, 0)), Padding::new(0, 10, 0, 0));

// Set a padding for a last column (except first row)
table.modify(Columns::last().not(Rows::first()), Padding::new(1, 1, 0, 0).fill('[', ']', ' ', ' '));

// Set a padding for a first row
table.modify(
    Rows::first(),
    Padding::new(2, 2, 0, 2).fill(' ', ' ', ' ', ' ').colorize(
        Color::BG_BLUE,
        Color::BG_BLUE,
        Color::BG_BLUE,
        Color::BG_BLUE,
    ),
);
```

Applying the last change to the first example will result in the following.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zhiburt/tabled/assets/20165848/2df75093-90c4-4b26-a2be-33d45391ace3">
  <img alt="Preview" src="https://github.com/zhiburt/tabled/assets/20165848/7ec1f377-362b-469a-94e2-6e8599a1bd6d">
</picture>

### Margin

`Margin` sets extra space around the table (top, bottom, left, right).
As for `Padding` you can set indent, size and color of the extra space.

```rust
use tabled::settings::Margin;

table.with(Margin::new(3, 4, 1, 2).fill('>', '<', 'v', '^'));
```

If you run it for a first example you'll get.

```text
vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
>>>+------+----------------+---------------+<<<<
>>>| name | designed_by    | invented_year |<<<<
>>>+------+----------------+---------------+<<<<
>>>| C    | Dennis Ritchie | 1972          |<<<<
>>>+------+----------------+---------------+<<<<
>>>| Go   | Rob Pike       | 2009          |<<<<
>>>+------+----------------+---------------+<<<<
>>>| Rust | Graydon Hoare  | 2010          |<<<<
>>>+------+----------------+---------------+<<<<
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

#### Margin Color

You can set a color for the characters.

```rust
use tabled::settings::{Margin, Color};

table.with(Margin::new(3, 4, 1, 2).fill('>', '<', 'v', '^').colorize(
    Color::BG_BRIGHT_BLUE,
    Color::BG_BRIGHT_CYAN,
    Color::BG_BLUE,
    Color::BG_RED,
));
```

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zhiburt/tabled/assets/20165848/324114ce-42e3-4aaa-be5c-18abf7e835b3">
  <img alt="Preview" src="https://github.com/zhiburt/tabled/assets/20165848/5e280686-a8f5-4f1f-a1f6-3c97e1dd1fe3">
</picture>

### Shadow

`Shadow` can be used to set a 'shadow' like margin.

```rust
use tabled::{settings::{Style, Shadow}, Table};

let data = vec![["A", "B", "C"]];
let table = Table::new(data)
    .with(Style::modern())
    .with(Shadow::new(1))
    .to_string();

println!("{}", table);
```

An output could look like the following.

```text
┌───┬───┬───┐ 
│ 0 │ 1 │ 2 │▒
├───┼───┼───┤▒
│ A │ B │ C │▒
└───┴───┴───┘▒
 ▒▒▒▒▒▒▒▒▒▒▒▒▒
```

### Width

Using the following structures you can configure the width of a table and a single cell.

BEWARE that `Width` controls only content, so it can't make things smaller then a certain minimum.
BEWARE that it DOES NOT consider `Padding` when adjusting the width.

The functions preserves `ansi` color sequences (when `ansi` feature is on).

Below is an example of setting an exact table width.

```rust
use tabled::{
    settings::{
        peaker::{PriorityMax, PriorityMin},
        Settings, Width,
    },
    Table,
};

fn gen_table(string_size: usize, width: usize) -> String {
    let data = vec![(string_size.to_string(), "x".repeat(string_size))];

    let mut table = Table::new(data);
    table.with(Settings::new(
        Width::wrap(width).priority::<PriorityMax>(),
        Width::increase(width).priority::<PriorityMin>(),
    ));

    table.to_string()
}

let table = gen_table(50, 40);
println!("{table}");

let table = gen_table(20, 40);
println!("{table}");
```

The result must be seen as following.

```text
+--------+-----------------------------+
| String | String                      |
+--------+-----------------------------+
| 50     | xxxxxxxxxxxxxxxxxxxxxxxxxxx |
|        | xxxxxxxxxxxxxxxxxxxxxxx     |
+--------+-----------------------------+
+---------------+----------------------+
| String        | String               |
+---------------+----------------------+
| 20            | xxxxxxxxxxxxxxxxxxxx |
+---------------+----------------------+
```

#### Truncate

`Truncate` sets a maximum width of a cell by truncating its content.

```rust
use tabled::settings::{Width, object::Rows};

// Truncating content to 10 chars in case it's bigger than that
// in a first row.
table.modify(Rows::first(), Width::truncate(10));

// Truncating content to 7 chars and puts a suffix '...' after it
// in all rows except a first.
table.modify(Rows::new(1..), Width::truncate(10).suffix("..."));
```

`Truncate` can be used to set a maximum width of a whole table.

```rust
use tabled::settings::Width;

// Tries to set table width to 22, in case it's bigger than that.
table.with(Width::truncate(22));
```

It can be used in combination with `MinWidth` to set an exact table size.

#### Wrapping

`Wrap` sets a maximum width of a cell by wrapping its content to new lines.

```rust
use tabled::settings::{Width, object::Rows};

// Wrap content to 10 chars in case it's bigger than that
// in a first row.
table.modify(Rows::first().with(Width::wrap(10)));

// Use a strategy where we try not to keep words split (where possible).
table.modify(Rows::new(1..).with(Width::wrap(10).keep_words()));
```

`Wrap` can be used to set a maximum width of a whole table.

```rust
use tabled::settings::Width;

// Tries to set table width to 22, in case it's bigger than that.
table.with(Width::wrap(22));
```

It can be used in combination with `MinWidth` to set an exact table size.

#### Increase width

`MinWidth` sets a minimal width of an object.

```rust
use tabled::settings::{Width, object::Rows};

// increase the space used by cells in all rows except the header to be at least 10
table.modify(Rows::new(1..), Width::increase(10));
```

`MinWidth` also can be used to set a minimum width of a whole table.

```rust
use tabled::settings::Width;

// increase width of a table in case it was lower than 10.
table.with(Width::increase(10));
```

It can be used in combination with `Truncate` and `Wrap` to set an exact table size.

#### Justify

You can set a constant width for all columns using `Justify`.

```rust
use tabled::settings::Width;

table.with(Width::justify(10));
```

#### Priority

You can tweak `Truncate`, `Wrap`, `MinWidth` logic by setting a priority by which a trim/inc be done.

```rust
use tabled::settings::{Width, peaker::PriorityMax};

table.with(Width::truncate(10).priority::<PriorityMax>());
```

#### Percent

By default you use `usize` int to set width settings,
but you could do it also with `tabled::width::Percent`.

```rust
use tabled::settings::{Width, measurement::Percent};

table.with(Width::wrap(Percent(75)));
```

### Height

You can increase a table or a specific cell height using the `Height` modifier.

Beware that `Height` controls only content,
so it can't make things smaller then a certain minimum.

Below is an example of setting an exact table height and width.

```rust
use std::iter::FromIterator;
use tabled::{
    settings::{Height, Settings, Width},
    Table,
};

fn gen_data(width: usize, height: usize) -> Vec<Vec<String>> {
    let dims = format!("{}x{}", width, height);
    let string = vec!["x".repeat(width); height].join("\n");

    vec![
        vec![String::from("N"), String::from("string")],
        vec![dims, string],
    ]
}

fn gen_table(data: Vec<Vec<String>>, width: usize, height: usize) -> String {
    let mut table = Table::from_iter(data);

    table.with(
        Settings::empty()
            .with(Width::truncate(width))
            .with(Width::increase(width))
            .with(Height::increase(height))
            .with(Height::limit(height)),
    );

    table.to_string()
}

println!("{}", gen_table(gen_data(40, 10), 30, 8));
println!("{}", gen_table(gen_data(40, 4), 80, 12));
```

```text
+-------+--------------------+
| N     | string             |
|       |                    |
|       |                    |
+-------+--------------------+
| 40x10 | xxxxxxxxxxxxxxxxxx |
|       |                    |
+-------+--------------------+
+-----------------------------------+------------------------------------------+
| N                                 | string                                   |
|                                   |                                          |
|                                   |                                          |
+-----------------------------------+------------------------------------------+
| 40x4                              | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx |
|                                   | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx |
|                                   | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx |
|                                   | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx |
|                                   |                                          |
|                                   |                                          |
+-----------------------------------+------------------------------------------+
```

#### Height increase

Increasing the height of a cell of a whole table could be done by `Height::increase`.

```rust
use tabled::settings::{Height, object::Rows};

// increase height of a table in case it was lower than 10.
table.with(Height::increase(10));

// increase height of cells in the last row on a table in case if some of them has it lower than 10.
table.modify(Rows::last(), Height::increase(10));
```

#### Height limit

Truncation of the height of a cell of a whole table could be done by `Height::limit`.

```rust
use tabled::settings::{Height, object::Rows};

// decrease height of a table to 10 in case it was bigger than that.
table.with(Height::limit(10));

// decrease height of cells in the last row on a table to 10 in case if some of them has it bigger than that.
table.modify(Rows::last(), Height::limit(10));
```

### Rotate

You can rotate a table using `tabled::Rotate`.

Imagine you have a table already which output may look like this.

```text
┌────┬──────────────┬───────────────────────────┐
│ id │ distribution │ link                      │
├────┼──────────────┼───────────────────────────┤
│ 0  │ Fedora       │ https://getfedora.org/    │
├────┼──────────────┼───────────────────────────┤
│ 2  │ OpenSUSE     │ https://www.opensuse.org/ │
├────┼──────────────┼───────────────────────────┤
│ 3  │ Endeavouros  │ https://endeavouros.com/  │
└────┴──────────────┴───────────────────────────┘
```

Now we will add the following modificator and the output will be rotated;

```rust
use tabled::settings::Rotate;

table.with(Rotate::Left);
```

```text
┌──────────────┬────────────────────────┬───────────────────────────┬──────────────────────────┐
│ link         │ https://getfedora.org/ │ https://www.opensuse.org/ │ https://endeavouros.com/ │
├──────────────┼────────────────────────┼───────────────────────────┼──────────────────────────┤
│ distribution │ Fedora                 │ OpenSUSE                  │ Endeavouros              │
├──────────────┼────────────────────────┼───────────────────────────┼──────────────────────────┤
│ id           │ 0                      │ 2                         │ 3                        │
└──────────────┴────────────────────────┴───────────────────────────┴──────────────────────────┘
```

### Disable

You can remove certain rows or columns from the table by `Disable`.

```rust
use tabled::settings::{
    object::{Columns, Rows},
    Disable,
};

table
    .with(Disable::row(Rows::first()))
    .with(Disable::column(Columns::single(2)));
```

If the above example be applied for a first example in a file it would look like this.

```text
+------+----------------+
| C    | Dennis Ritchie |
+------+----------------+
| Go   | Rob Pike       |
+------+----------------+
| Rust | Graydon Hoare  |
+------+----------------+
```

### Extract

You can `Extract` data segments of a table to focus on it closely.

```rust
use tabled::settings::Extract;

table.with(Extract::segment(1..3, 1..));
```

```text
+-------+-------------+-----------+
|  i32  |    &str     |   bool    |
+-------+-------------+-----------+         +-------------+-----------+
| : 0 : | : Grodno :  | : true :  |         | : Grodno :  | : true :  |
+-------+-------------+-----------+    =    +-------------+-----------+
| : 1 : |  : Minsk :  | : true :  |         |  : Minsk :  | : true :  |
+-------+-------------+-----------+         +-------------+-----------+
| : 2 : | : Hamburg : | : false : |
+-------+-------------+-----------+
| : 3 : |  : Brest :  | : true :  |
+-------+-------------+-----------+
```

### Header and Footer and Panel

You can add a `Header` and `Footer` to display some information.

```rust
use tabled::settings::Panel;

let count_elements = table.count_rows();

table
    .with(Panel::vertical(0, "A vertical panel").width(1))
    .with(Panel::header("Tabled Name"))
    .with(Panel::footer(format!("{} elements", count_elements)));
```

When applied to the main example of this file it will result in the following output.

```text
+---+------+----------------+---------------+
| Tabled Name                               |
+---+------+----------------+---------------+
| A | name | designed_by    | invented_year |
|   |      |                |               |
| v |      |                |               |
| e |      |                |               |
+ r +------+----------------+---------------+
| t | C    | Dennis Ritchie | 1972          |
| i |      |                |               |
| c |      |                |               |
+ a +------+----------------+---------------+
| l | Go   | Rob Pike       | 2009          |
|   |      |                |               |
| p |      |                |               |
+ a +------+----------------+---------------+
| n | Rust | Graydon Hoare  | 2010          |
| e |      |                |               |
| l |      |                |               |
+---+------+----------------+---------------+
| 4 elements                                |
+---+------+----------------+---------------+
```

### Merge

It's possible to create `"Panel"`s by combining the duplicates using `Merge`.

```rust
use tabled::{settings::merge::Merge, Table};

let data = [['A', 'B', 'B'], ['A', 'W', 'E'], ['Z', 'Z', 'Z']];

let mut table = Table::new(data);
table.with(Merge::horizontal()).with(Merge::vertical());

println!("{}", table);
```

```
+---+---+---+
| 0 | 1 | 2 |
+---+---+---+
| A | B     |
+   +---+---+
|   | W | E |
+---+---+---+
| Z         |
+---+---+---+
```

### Concat

You can concatenate 2 tables using `Concat`.
It will stick 2 tables together either vertically or horizontally.

The example below shows the result of horizontal concat of the primary table of this file.

```rust
use tabled::settings::Concat;

table.with(Concat::horizontal(table.clone()));
```

The result.

```text
+------+----------------+---------------+------+----------------+---------------+
| name | designed_by    | invented_year | name | designed_by    | invented_year |
+------+----------------+---------------+------+----------------+---------------+
| C    | Dennis Ritchie | 1972          | C    | Dennis Ritchie | 1972          |
+------+----------------+---------------+------+----------------+---------------+
| Go   | Rob Pike       | 2009          | Go   | Rob Pike       | 2009          |
+------+----------------+---------------+------+----------------+---------------+
| Rust | Graydon Hoare  | 2010          | Rust | Graydon Hoare  | 2010          |
+------+----------------+---------------+------+----------------+---------------+
```

The example below show the result of vertical concat of the primary table of this file.

```rust
use tabled::settings::Concat;

table.with(Concat::vertical(table.clone()));
```

The result.

```text
+------+----------------+---------------+
| name | designed_by    | invented_year |
+------+----------------+---------------+
| C    | Dennis Ritchie | 1972          |
+------+----------------+---------------+
| Go   | Rob Pike       | 2009          |
+------+----------------+---------------+
| Rust | Graydon Hoare  | 2010          |
+------+----------------+---------------+
| name | designed_by    | invented_year |
+------+----------------+---------------+
| C    | Dennis Ritchie | 1972          |
+------+----------------+---------------+
| Go   | Rob Pike       | 2009          |
+------+----------------+---------------+
| Rust | Graydon Hoare  | 2010          |
+------+----------------+---------------+
```

### Highlight

`Highlight` can be used to change the borders of target region.
Here's an example.

```rust
use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        Border, Highlight, Style,
    },
    Table,
};

let data = vec![["A", "B", "C"], ["D", "E", "F"]];

let mut table = Table::new(data);
table.with(Style::modern());
table.with(Highlight::border(
    Rows::first().and(Columns::single(2).and((1, 1))),
    Border::filled('*'),
));

println!("{}", table);
```

The resulting table would be the following.

```text
*************
* 0 │ 1 │ 2 *
*****───┼───*
│ A * B │ C *
├───*****───*
│ D │ E * F *
└───┴───*****
```

### Span

It's possible to set a horizontal(column) span and vertical(row) span to a cell. For certain look and feel, this might cause visual artifacts on  table borders (see #399). This can be fixed by using tabled::settings::style::BorderSpanCorrection.

#### Horizontal span

```rust
use tabled::{
    settings::{Alignment, Span},
    Table,
};

let data = vec![["A", "B", "C"], ["D", "E", "F"]];

let mut table = Table::new(data);
table
    .modify((0, 0), Span::column(3))
    .modify((1, 0), Span::column(2))
    .with(Alignment::center());

println!("{}", table);
```

```text
+---+---+---+
|     0     |
+---+---+---+
|   A   | C |
+---+---+---+
| D | E | F |
+---+---+---+
```

#### Vertical span

```rust
use tabled::{
    settings::{Alignment, Span},
    Table,
};

let data = vec![["A", "B", "C"], ["D", "E", "F"]];

let mut table = Table::new(data);
table
    .modify((0, 1), Span::row(3))
    .with(Alignment::center())
    .with(Alignment::center_vertical());

println!("{}", table);
```

```text
+---+---+---+
| 0 |   | 2 |
+---+   +---+
| A | 1 | C |
+---+   +---+
| D |   | F |
+---+---+---+
```

### Split

You can `Split` a table on a row or column to redistribute the cells beyond that point
into a new shape with the provided point acting as the new, upper boundary in the direction selected.

Adding this to a first example will result in the next table.

```rust
use tabled::settings::{Style, split::Split};

table.with(Style::modern());
table.with(Split::column(2).concat());
```

The result of the running example will be as follows.

```text
┌───────────────┬────────────────┐
│ name          │ designed_by    │
├───────────────┼────────────────┤
│ C             │ Dennis Ritchie │
├───────────────┼────────────────┤
│ Go            │ Rob Pike       │
├───────────────┼────────────────┤
│ Rust          │ Graydon Hoare  │
├───────────────┼────────────────┤
│ invented_year │                │
├───────────────┼────────────────┤
│ 1972          │                │
├───────────────┼────────────────┤
│ 2009          │                │
├───────────────┼────────────────┤
│ 2010          │                │
└───────────────┴────────────────┘
```

#### Duplicate

It's possible to duplicate a given set of cell to another set.

```rust
use tabled::{Table, settings::{Dup, object::Rows}};

let mut table = Table::new(data);

// copy lastfirst line to the last line (last line gets erased).
table.with(Dup::new(Rows::last(), Rows::first()));
```

## Derive

To be able to use a `Tabled` macros each field must implement `std::fmt::Display`
otherwise it will not work.

The following example will cause a error.

```rust,compile_fail
use tabled::Tabled;
#[derive(Tabled)]
struct SomeType {
    field1: SomeOtherType,
}

struct SomeOtherType;
```

The `Tabled` macro is available when `derive` feature in turned on.
And it is by default.

Most of the default types implement the corresponding traits too.

### Override a column name

You can use a `#[tabled(rename = "")]` attribute to override a column name.

```rust
use tabled::Tabled;

#[derive(Tabled)]
struct Person {
    #[tabled(rename = "Name")]
    first_name: &'static str,
    #[tabled(rename = "Surname")]
    last_name: &'static str,
}
```

### Format headers

Beside `#[tabled(rename = "")]` you can change a format of a column name using
`#[tabled(rename_all = "UPPERCASE")]`.

```rust
use tabled::Tabled;

#[derive(Tabled)]
#[tabled(rename_all = "CamelCase")]
struct Person {
    id: u8,
    number: &'static str,
    name: &'static str,
    #[tabled(rename_all = "snake_case")]
    middle_name: &'static str,
}
```

### Hide a column

You can mark filds as hidden in which case they will be ignored and not be present on a sheet.

A similar effect could be achieved by the means of a `Disable` setting.

```rust
use tabled::Tabled;

#[derive(Tabled)]
struct Person {
   id: u8,
   #[tabled(skip)]
   number: &'static str,
   name: &'static str,
}
```

### Set column order

You can change the order in which they will be displayed in table.

```rust
use tabled::Tabled;

#[derive(Tabled)]
struct Person {
   id: u8,
   #[tabled(order = 0)]
   number: &'static str,
   #[tabled(order = 1)]
   name: &'static str,
}
```

### Format fields

As was said already, using `#[derive(Tabled)]` is possible only when all fields implement a `Display` trait.
However, this may be often not the case for example when a field uses the `Option` type. There's 2 common ways how to solve this:

- Implement `Tabled` trait manually for a type.
- Wrap `Option` to something like `DisplayedOption<T>(Option<T>)` and implement a Display trait for it.

Alternatively, you can use the `#[tabled(display_with = "func")]` attribute for the field to specify a display function.

```rust
use tabled::Tabled;

#[derive(Tabled)]
pub struct MyRecord {
    pub id: i64,
    #[tabled(display_with = "display_option")]
    pub valid: Option<bool>
}

fn display_option(o: &Option<bool>) -> String {
    match o {
        Some(s) => format!("is valid thing = {}", s),
        None => format!("is not valid"),
    }
}
```

You can send an argument to a function like this (it also possible to use `&self`),
using `#[tabled(display_with("some_function", "arg1", 2, self))]`

```rust
use tabled::Tabled;

#[derive(Tabled)]
pub struct MyRecord {
    pub id: i64,
    #[tabled(display_with("Self::display_valid", self, 1))]
    pub valid: Option<bool>
}

impl MyRecord {
    fn display_valid(&self, arg: usize) -> String {
        match self.valid {
            Some(s) => format!("is valid thing = {} {}", s, arg),
            None => format!("is not valid {}", arg),
        }
    }
}
```

To reduce boilerplate code, one can also achieve this using the `format` attribute within `#[derive(Tabled)]`.

```rust
use tabled::Tabled;

#[derive(Tabled)]
pub struct Motorcycle {
    weight: usize,
    #[tabled(format = "{} cc")]
    cc: usize,
}
```

In the above example, the cc field will be formatted using the specified format string "{} cc", where {} is replaced with the value of cc.

Just like with `display_with` attribute, you can pass arguments for more complex formatting scenarios:

```rust
use tabled::Tabled;

#[derive(Tabled)]
pub struct Motorcycle {
    weight: usize,
    #[tabled(format = "{}/{} cc/kg", self.cc, self.weight)]
    cc: usize,
}
```

In this case, the cc field will be formatted using the format string "{}/{} cc/kg", and {} will be replaced with the values of cc and weight, respectively.

### Inline

It's possible to inline internal data if it implements the `Tabled` trait using `#[tabled(inline)]`.
You can also set a prefix which will be used for all inlined elements by `#[tabled(inline("prefix>>"))]`.

```rust
use tabled::Tabled;

#[derive(Tabled)]
struct Person {
    id: u8,
    name: &'static str,
    #[tabled(inline)]
    ed: Education,
}

#[derive(Tabled)]
struct Education {
    uni: &'static str,
    graduated: bool,
}
```

And it works for enums as well.

```rust
use tabled::Tabled;

#[derive(Tabled)]
enum Vehicle {
    #[tabled(inline("Auto::"))]
    Auto {
        model: &'static str,
        engine: &'static str,
    },
    #[tabled(inline)]
    Bikecycle(
        &'static str,
        #[tabled(inline)] Bike,
    ),
}

#[derive(Tabled)]
struct Bike {
    brand: &'static str,
    price: f32,
}
```

## Table types

`tabled` has a few representations of tables. Some differ in view, and some differ in implementation details.

There are situations when you might better use one but not another.
But sometimes some can be used interchangeably.

Below you'll find a short list of existing ones. You can find descriptive information about each in the documentation.

### `Table`

Main table of the library.
It's implemenentation requires that all data be stored on heap.

### `IterTable`

It's similar to main `Table`, it's only difference is that it does not require a the whole buffer.
It only requires a buffer for 1 row at a time.

It might be useful when you can't fit all your data in memory.

### `CompactTable`

Similar to `IterTable` but it might not require any buffer.
It also has capability for a sniffing logic, where we estimate data dimension on a small selection of data.

It might be useful in a very constrain environments.
It is the only table which supports `no-std`.

### `PoolTable`

Unlike `Table` it does not nessarily require columns to be aligned.
It provides capabilities for a completely and utterly diverse table layout.

Example

```rust
use tabled::{
    settings::{Alignment, Style},
    tables::PoolTable,
};

let characters = [
    "Naruto Uzumaki",
    "Kakashi Hatake",
    "Minato Namikaze",
    "Jiraiya",
    "Orochimaru",
    "Itachi Uchiha",
];

let data = characters.chunks(2);

let table = PoolTable::new(data)
    .with(Style::dots())
    .with(Alignment::center())
    .to_string();

println!("{table}");
```

The output would look like the following.

```
...................................
: Naruto Uzumaki : Kakashi Hatake :
:................:................:
:  Minato Namikaze   :  Jiraiya   :
:....................:............:
:  Orochimaru   :  Itachi Uchiha  :
:...............:.................:
```

### `ExtendedTable`

You can use `ExtendedTable` if your data structure has a lot of fields.

Here's an example.

```rust
use tabled::{tables::ExtendedTable, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: &'static str,
    is_active: bool,
    is_cool: bool,
}

let data = [
    Distribution {
        name: "Manjaro",
        is_cool: true,
        is_active: true,
    },
    Distribution {
        name: "Debian",
        is_cool: true,
        is_active: true,
    },
    Distribution {
        name: "Debian",
        is_cool: true,
        is_active: true,
    },
];

let table = ExtendedTable::new(&data);

println!("{}", table);
```

You'll see the following.

```text
-[ RECORD 0 ]------
name      | Manjaro
is_active | true
is_cool   | true
-[ RECORD 1 ]------
name      | Debian
is_active | true
is_cool   | true
-[ RECORD 2 ]------
name      | Debian
is_active | true
is_cool   | true
```

## Tips and Tricks

### `std::fmt::*` options

You use formatting(`std::fmt::*`) options to apply certain settings.

```rust
use tabled::Table;

let numbers = [1, 2, 3];
let table = Table::new(numbers);

println!("{:#^10}", table);
```

The result will be as follows.

```text
#+-----+##
#| i32 |##
#+-----+##
#|  1  |##
#+-----+##
#|  2  |##
#+-----+##
#|  3  |##
#+-----+##
```

### ANSI

The library doesn't bind you in the usage of any color library but to be able to work correctly with colored input (with ANSI sequences), and avoid [miscalculation of string width](https://github.com/zhiburt/tabled/issues/26)
because of embedded ansi sequences, you should add the `ansi` feature to your `Cargo.toml`:

```toml
tabled = { version = "*", features = ["ansi"] }
```

Then you can use colored strings as values and table will be properly rendered.

Tuning our favorite example will result in the following:

```rust
use tabled::{format::Format, object::Columns, Style, Table};
use owo_colors::OwoColorize;

let mut table = Table::new(&data);
table
    .with(Style::psql())
    .modify(Columns::single(0), Format::new(|s| s.red().to_string()))
    .modify(Columns::single(1), Format::new(|s| s.blue().to_string()))
    .modify(Columns::new(2..), Format::new(|s| s.green().to_string()));
```

![carbon-2](https://user-images.githubusercontent.com/20165848/120526301-b95efc80-c3e1-11eb-8779-0ec48894463b.png)

### Tuple combination

You also can combine objects which implement `Tabled` by means of tuples, you will get a combined columns of them.

```rust
use tabled::{
    settings::{Alignment, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct Developer(#[tabled(rename = "name")] &'static str);

#[derive(Tabled)]
enum Domain {
    Security,
    Embedded,
    Frontend,
    Unknown,
}

let data = vec![
    (Developer("Terri Kshlerin"), Domain::Embedded),
    (Developer("Catalina Dicki"), Domain::Security),
    (Developer("Jennie Schmeler"), Domain::Frontend),
    (Developer("Maxim Zhiburt"), Domain::Unknown),
];

let table = Table::new(data)
    .with(Style::psql())
    .with(Alignment::center())
    .to_string();

assert_eq!(
    table,
    concat!(
        "      name       | Security | Embedded | Frontend | Unknown \n",
        "-----------------+----------+---------+----------+---------\n",
        " Terri Kshlerin  |          |    +    |          |         \n",
        " Catalina Dicki  |    +     |         |          |         \n",
        " Jennie Schmeler |          |         |    +     |         \n",
        "  Maxim Zhiburt  |          |         |          |    +    ",
    )
);
```

### Object

You can apply settings to a subgroup of cells using `and` and `not` methods for an object.

```rust
use tabled::settings::object::{Object, Segment, Cell, Rows, Columns};
Segment::all().not(Rows::first()); // select all cells except header.
Columns::first().and(Columns::last()); // select cells from first and last columns.
Rows::first().and(Columns::single(0)).not(Cell(0, 0)); // select the header and first column except the (0, 0) cell.
```

Also you can target a column via its name using `ByColumnName`.

```rust
use tabled::{location::ByColumnName, Alignment, Modify};

table.with(Modify::new(ByColumnName::new("name")).with(Alignment::center()));
```

### Builder

`Builder` is a powerful tool you shall be aware of.

For example you can use `Builder::index` to make a particular column an index,
which will stay on the left.

```rust
use tabled::{builder::Builder, settings::Style};

let mut builder = Builder::default();
builder.push_record(["Index", "Language", "Status"]);
builder.push_record(["1", "English", "In progress"]);
builder.push_record(["2", "Deutsch", "Not ready"]);

let builder = builder.index().column(1).name(None);

let mut table = builder.build();
table.with(Style::rounded());

println!("{}", table);
```

```text
╭─────────┬───────┬─────────────╮
│         │ Index │ Status      │
├─────────┼───────┼─────────────┤
│ English │ 1     │ In progress │
│ Deutsch │ 2     │ Not ready   │
╰─────────┴───────┴─────────────╯
```

For example you can use `transpose()` method to change the layout.

```rust
// A dynamic table example
// ...

let mut builder = builder.index().transpose();
```

```text
.-------------------------------------------------.
|   | 0      | 1      | 2      | 3        | 4     |
| 0 | And    | Little | When   | I        | You   |
| 1 | the    | boy    | you    | don't    | know  |
| 2 | cat's  | blue   | comin' | know     | we'll |
| 3 | in     | and    | home   | when,    | have  |
| 4 | the    | the    | dad?   | but      | a     |
| 5 | cradle | man    |        | we'll    | good  |
| 6 | and    | on     |        | get      | time  |
| 7 | the    | the    |        | together | then  |
| 8 | silver | moon   |        | then     |       |
| 9 | spoon  |        |        | son      |       |
'-------------------------------------------------'
```

### Macros

Utilities for dynamic `Table` displays.

#### Col and Row

`col!` creates a single column table, with a given set of cells.
`row!` creates a single row table, with a given set of cells.

Combine `col!` and `row!` to create flexible table visualizations.

```rust
use tabled::{col, row, settings::Style};

let mut table = row![
    col!["table 0", "0", "1", "2"],
    col!["table 1", "world"],
    col!["table 2"],
];
table.with(Style::modern_rounded());

println!("{table}");
```

The output you're going to see running it.

```text
╭─────────────┬─────────────┬─────────────╮
│ +---------+ │ +---------+ │ +---------+ │
│ | table 0 | │ | table 1 | │ | table 2 | │
│ +---------+ │ +---------+ │ +---------+ │
│ | 0       | │ | world   | │             │
│ +---------+ │ +---------+ │             │
│ | 1       | │             │             │
│ +---------+ │             │             │
│ | 2       | │             │             │
│ +---------+ │             │             │
╰─────────────┴─────────────┴─────────────╯
```

#### `static_table`

It's possible to construct a table at compile time, via [`static_table`](/static_table/README.md).
You'd need to include a different crate to use it.

```toml
static_table = "*"
```

```rust
let table = static_table::static_table!(
    [
        ["x", "y", "op", "result"],
        ["1", '2', '*', '2'],
        ["2", '2', '*', '4']
    ],
    THEME = "ROUNDED",
);

assert_eq!(
    table,
    "╭───┬───┬────┬────────╮\n\
     │ x │ y │ op │ result │\n\
     ├───┼───┼────┼────────┤\n\
     │ 1 │ 2 │ *  │ 2      │\n\
     │ 2 │ 2 │ *  │ 4      │\n\
     ╰───┴───┴────┴────────╯",
);
```

Notice that you can even use it in documentation.

```rust
/// Multiply 2 integers together.
///
#[doc = static_table::static_table!([
    ["x", "y", "result"],
    ["1", '0', '0'],
    ["1", '2', '2'],
    ["2", '2', '4']
])]
pub fn mul(left: usize, right: usize) -> usize {
    left + right
}
```

It will look as follows.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zhiburt/tabled/assets/20165848/704c285b-f8ab-481b-a5bf-130406aad7d5">
  <img alt="Preview" src="https://github.com/zhiburt/tabled/assets/20165848/164a82db-8ce7-4366-b21d-c679bea8d9fe">
</picture>

## Features

The library has a list of features.

- `std`     - Used by default. If not its considered `no_std` with a limited set of functionality.
- `derive`  - Used by default. Adds support for `Tabled` derive macro.
- `ansi`    - A support for ANSI sequences.
- `macros`  - A support for `row!`, `col!` macro.

## Formats

You can convert some formats to a `Table` using a utility library.

### `json` format

You can convert arbitrary `json` to a `Table` using [`json_to_table`](/json_to_table/README.md) library.
See the **[example](/json_to_table/README.md)**.

### `ron` format

You can convert arbitrary `ron` to a `Table` using [`ron_to_table`](/ron_to_table/README.md) library.
See the **[example](/ron_to_table/README.md)**.

### `csv` format

You can convert arbitrary `csv` to a `Table` using [`csv_to_table`](/csv_to_table/README.md) library.
See the **[example](/csv_to_table/README.md)**.

### `toml` format

You can convert arbitrary `toml` to a `Table` using [`toml_to_table`](/toml_to_table/README.md) library.
See the **[example](/toml_to_table/README.md)**.

### `html` format

You can convert a `Table` into `HTML` `<table>` using [`table_to_html`](/table_to_html/README.md) library.
See the **[example](/json_to_table/README.md)**.


## Notes

### Charset

Since version `0.11` we no longer have special treatment for symbols which WILL break your terminal output such as
`\t` and `\r`.
So if your content might contain them you shall either handle it yourself,
or call `tabled::settings::formatting::Charset::clean` and `tabled::settings::formatting::Tabsize`.

### ANSI escape codes

By default `tabled` doesn't handle [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code).
By default such things as hyperlinks, blinking and others things which can be achieved via ANSI codes might not work correctly.

To enable this support, add the `ansi` feature to your `Cargo.toml`

```toml
tabled = { version = "*", features = ["ansi"] }
```

### Emoji

The library support emojies out of the box (but sometimes `ansi` feature is required).
Be aware that some of the terminals and editors may not render them as you would expect.

Let's add emojies to an example from a [Usage](#Usage) section.

```rust
let languages = vec![
    Language {
        name: "C 💕",
        designed_by: "Dennis Ritchie",
        invented_year: 1972,
    },
    Language {
        name: "Rust 👍",
        designed_by: "Graydon Hoare",
        invented_year: 2010,
    },
    Language {
        name: "Go 🧋",
        designed_by: "Rob Pike",
        invented_year: 2009,
    },
];
```

The resultant table will look like the following.
As you can see Github tricks a bit a return table, but `GNOME terminal` and `Alacritty` terminal handle it correctly.

```text
+---------+----------------+---------------+
| name    | designed_by    | invented_year |
+---------+----------------+---------------+
| C 💕    | Dennis Ritchie | 1972          |
+---------+----------------+---------------+
| Rust 👍 | Graydon Hoare  | 2010          |
+---------+----------------+---------------+
| Go 🧋   | Rob Pike       | 2009          |
+---------+----------------+---------------+
```

### Terminal size

It's a frequent case where it's necessary to align a table to a terminal width or height.
You can achieve that by using `Width` and `Height`.
You can peak a strategy by which a column/row truncation/widening will be done by using `Priority`.

This example uses `terminal_size` crate to determine ones size, but it's possible to use anything.

```rust
use tabled::{
    builder::Builder,
    settings::{peaker::PriorityMax, Height, Settings, Width},
    Table,
};
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};

fn get_terminal_size() -> (usize, usize) {
    let (TerminalWidth(width), TerminalHeight(height)) =
        terminal_size().expect("failed to obtain a terminal size");

    (width as usize, height as usize)
}

let (width, height) = get_terminal_size();

let data = [
    ["0.2.1", "2021-06-23", "true", "#[header(inline)] attribute"],
    ["0.2.0", "2021-06-19", "false", "API changes"],
    ["0.1.4", "2021-06-07", "false", "display_with attribute"],
];

let table_settings = Settings::default()
    .with(Width::wrap(width).priority::<PriorityMax>())
    .with(Width::increase(width))
    .with(Height::limit(height))
    .with(Height::increase(height));

let mut table = Table::from_iter(data);
table.with(table_settings);

println!("{table}");
```

### Semver

> When you need to release a breaking change—any breaking change—you do it in a major version. Period. No excuses.

We still do it.
We often do break change on minor version bump.
So you probably shall not depend on minor version (like `0.7`).
It's likely better to depend on constant version e.g. `=0.8.0`

### MSRV

Breaking MSRV considered to be a breaking change; but see [semver-note](#semver)

### Comparison

Nowadays there's a few libraries for pretty tables.
Some may wonder why `tabled` is better or worse than others libraries?

I hope `tabled` does it's job well, but at the end of the day you probably need to decide for yourself.
If you have any ideas for an enhancement or have a question about `tabled` please file an issue.

Below you will find a list of crates which do similar things or do something which `tabled` doesn't.

You can find performance comparison benchmarks [here](https://github.com/zhiburt/tabled/tree/master/tabled/benches/lib_comp).

The description is taken from the author's quotes.

* *[`cli-table`](https://github.com/devashishdxt/cli-table/) tends to keep the compile time and crate size low and support all the platforms. It has an optional `csv` support.*

* *[`comfy-table`](https://github.com/Nukesor/comfy-table) focuses on providing a minimalistic, but rock-solid library for building text-based tables with focus on safety and dynamic-length content arrangement.*

* *[`term-table-rs`](https://github.com/RyanBluth/term-table-rs) main focus is on a good set of tools for rendering CLI tables, while allowing users to bring their own tools for things like colors. It has an ability to have different number of columns in each row of the table.*

Please open an issue if you feel another crate is worth mentioning.
