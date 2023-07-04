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
  - [Dynamic table](#dynamic-table)
    - [Build index](#build-index)
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
    - [Customization](#customization)
    - [Cell Border](#cell-border)
    - [Text on borders](#text-on-borders)
    - [Colorize borders](#colorize-borders)
  - [Theme](#theme)
    - [Colorize content](#colorize-content)
    - [Column names](#colorize-content)
  - [Alignment](#alignment)
  - [Format](#format)
  - [Padding](#padding)
    - [Padding Color](#padding-color)
  - [Margin](#margin)
    - [Margin Color](#margin-color)
  - [Shadow](#shadow)
  - [Width](#width)
    - [Truncate](#truncate)
    - [Wrapping](#wrapping)
    - [Increaase width](#increaase-width)
    - [Justify](#justify)
    - [Priority](#priority)
    - [Percent](#percent)
  - [Height](#height)
    - [Height Increase](#height-increase)
    - [Height Limit](#height-limit)
  - [Rotate](#rotate)
  - [Disable](#disable)
  - [Extract](#extract)
    - [Refinishing](#refinishing)
  - [Header and Footer and Panel](#header-and-footer-and-panel)
  - [Merge](#merge)
  - [Concat](#concat)
  - [Highlight](#highlight)
  - [Span](#span)
    - [Horizontal span](#horizontal-span)
    - [Vertical span](#vertical-span)
  - [Split](#split)
    - [Directions](#directions)
    - [Behaviors](#behaviors)
    - [Displays](#displays)
  - [Duplicate](#duplicate)
- [Derive](#derive)
  - [Override a column name](#override-a-column-name)
  - [Hide a column](#hide-a-column)
  - [Set column order](#set-column-order)
  - [Format fields](#format-fields)
  - [Format headers](#format-headers)
  - [Inline](#inline)
- [Features](#features)
  - [Color](#color)
  - [Tuple combination](#tuple-combination)
  - [Object](#object)
  - [Macros](#macros)
    - [`col` and `row`](#col-and-row)
    - [`static_table`](#static_table)
- [Table types](#table-types)
  - [`Table`](#table)
  - [`IterTable`](#itertable)
  - [`CompactTable`](#compacttable)
  - [`PoolTable`](#pooltable)
  - [`ExpandedDisplay`](#expanded-display)
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

To print a list of structs or enums as a table your types should implement the the `Tabled` trait or derive it with a `#[derive(Tabled)]` macro.
Most of the default types implement the trait out of the box.

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
                | Rust | Graydon Hoare  | 2010          |\n\
                +------+----------------+---------------+\n\
                | Go   | Rob Pike       | 2009          |\n\
                +------+----------------+---------------+";

assert_eq!(table, expected);
```

You can also use some of the formatting(`std::fmt::*`) options.

```rust
use tabled::Table;

let numbers = [1, 2, 3];
let table = Table::new(numbers);
    
println!("{:#^10}", table);
```

### Dynamic table

Sometimes you can't say what type of data you are going to deal with (like parsing `csv`).
In such cases it may be handy to build table dynamically (step by step).

```rust
use tabled::{builder::Builder, settings::Style};

let song = r#"
    And the cat's in the cradle and the silver spoon
    Little boy blue and the man on the moon
    When you comin' home dad?
    I don't know when, but we'll get together then son
    You know we'll have a good time then
"#;

let mut builder = Builder::default();
for line in song.lines() {
    if line.is_empty() {
        continue;
    }

    let words: Vec<_> = line.trim().split_terminator(' ').collect();
    builder.push_record(words);
}

let columns = (0..builder.count_columns()).map(|i| i.to_string());
builder.set_header(columns);

let mut table = builder.build();
table.with(Style::ascii_rounded());

println!("{}", table);
```

```text
.------------------------------------------------------------------------------------.
| 0      | 1     | 2      | 3     | 4    | 5      | 6    | 7        | 8      | 9     |
| And    | the   | cat's  | in    | the  | cradle | and  | the      | silver | spoon |
| Little | boy   | blue   | and   | the  | man    | on   | the      | moon   |       |
| When   | you   | comin' | home  | dad? |        |      |          |        |       |
| I      | don't | know   | when, | but  | we'll  | get  | together | then   | son   |
| You    | know  | we'll  | have  | a    | good   | time | then     |        |       |
'------------------------------------------------------------------------------------'
```

#### Build index

You can change a table layout by `Builder`.

```rust
// previous example
// ...

let mut builder = builder.index();
builder.transpose();
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

You can use `Builder::index` to make a particular column an index, which will stay on the left.

```rust
use tabled::{builder::Builder, settings::Style};

let mut builder = Builder::default();
builder
    .set_header(["Index", "Language", "Status"])
    .push_record(["1", "English", "In progress"])
    .push_record(["2", "Deutsch", "Not ready"]);

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

## Settings

This section lists the set of settings you can apply to your table.
Most of the settings are used by `.with` method of `Table`.

You can find a list of show cases in **[examples folder](/tabled/examples/README.md)**.

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

#### Customization

You can modify existing styles to fit your needs.

```rust
let style = tabled::settings::Style::modern().remove_horizontal();
```

The style will look like the following.

```rust
┌──────┬────────────────┬───────────────┐
│ name │ designed_by    │ invented_year │
│ C    │ Dennis Ritchie │ 1972          │
│ Rust │ Graydon Hoare  │ 2010          │
│ Go   │ Rob Pike       │ 2009          │
└──────┴────────────────┴───────────────┘
```

You can change the existing styles.

```rust
use tabled::settings::style::{HorizontalLine, Style, VerticalLine};

let style = Style::modern()
    .remove_horizontals()
    .remove_verticals()
    .horizontals([HorizontalLine::new(1, Style::modern().get_horizontal())
        .main(Some('═'))
        .intersection(None)])
    .verticals([VerticalLine::new(1, Style::modern().get_vertical())]);
```

The style will look like the following.

```rust
┌──────┬───────────────────────────────┐
│ name │ designed_by     invented_year │
├══════┼═══════════════════════════════┤
│ C    │ Dennis Ritchie  1972          │
│ Rust │ Graydon Hoare   2010          │
│ Go   │ Rob Pike        2009          │
└──────┴───────────────────────────────┘
```

Check the [documentation](https://docs.rs/tabled/latest/tabled/style/struct.Style.html) for
more customization options.

#### Cell Border

Sometimes `tabled::Style` settings are not enough.
Sometimes it's nesessary to change a border of a particular cell.

For this purpose you can use `Border`.

```rust
use tabled::{settings::{object::Rows, Border, Modify, Style}, Table};

let data = [["123", "456"], ["789", "000"]];
    
let table = Table::new(data)
    .with(Style::ascii())
    .with(Modify::new(Rows::first()).with(Border::default().top('x')))
    .to_string();
    
let expected = "+xxxxx+xxxxx+\n\
                | 0   | 1   |\n\
                +-----+-----+\n\
                | 123 | 456 |\n\
                +-----+-----+\n\
                | 789 | 000 |\n\
                +-----+-----+";
    
assert_eq!(table, expected);
```

#### Text on borders

You can set a string to a horizontal border line.

```rust
use tabled::{settings::style::BorderText, Table};

let mut table = Table::new(["Hello World"]);
table.with(BorderText::new("+-.table").horizontal(0));

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

You can use `BorderChar` to achieve this.

```rust
use tabled::{
    settings::{
        object::Columns,
        style::{BorderChar, Offset},
        Modify, Style,
    },
    Table,
};

let table = Table::new([["Hello", "World", "!"]])
    .with(Style::markdown())
    .with(
        Modify::new(Columns::new(..))
            .with(BorderChar::horizontal(':', Offset::Begin(0)))
            .with(BorderChar::horizontal(':', Offset::End(0))),
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

You can set a colors of all borders using `Color`.

```rust
use tabled::settings::{Color, style::BorderColor};

table.with(BorderColor::default().top(Color::FG_GREEN)))
```

You can also set a color border of intividial cell by using `BorderColored`.

```rust
use tabled::settings::{Modify, style::BorderColor, Color, object::Columns};

table.with(Modify::new(Columns::single(2)).with(BorderColor::default().top(Color::FG_GREEN)))
```

### Theme

#### Colorize content

You can colorize the content by the pattern or a specific cell.

```rust
use tabled::{
    builder::Builder,
    settings::{object::Rows, style::Style, themes::Colorization, Color, Modify},
};

let data = vec![
    vec![String::from("header 0"), String::from("header 1")],
    vec![String::from("Hello"), String::from("World")],
    vec![String::from("Bonjour"), String::from("le monde")],
    vec![String::from("Hallo"), String::from("Welt")],
];

let color1 = Color::BG_WHITE | Color::FG_BLACK;
let color2 = Color::BG_GREEN | Color::FG_BLACK;
let color3 = Color::BG_MAGENTA | Color::FG_BLACK;
let color4 = Color::BG_BLUE | Color::FG_BLACK;

let mut table = Builder::from(data).build();
table
    .with(Style::empty())
    .with(Colorization::columns([color2, color3]))
    .with(Colorization::exact([color1], Rows::first()))
    .with(Modify::new(Rows::first()).with(color4));
```

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zhiburt/tabled/assets/20165848/7e9b139c-a0fa-470d-9095-a36d1f01b55a">
  <img alt="Preview" src="https://github.com/zhiburt/tabled/assets/20165848/2096a1a2-aaee-42f1-b00f-ca6f9317cd29">
</picture>

#### Column names

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
    settings::{Modify, Alignment, object::Segment}
    Table,
};

let mut table = Table::new(data);
table
    .with(Modify::new(Segment::all()).with(Alignment::left()).with(Alignment::top()));
```

### Format

The `Format` function provides an interface for a modification of cells.

```rust
use tabled::{
    Table,
    settings::{Modify, format::Format, object::{Rows, Columns}},
};

let mut table = Table::new(&data);
table
    .with(Modify::new(Rows::first()).with(Format::new(|s| format!("Head {}", s))))
    .with(Modify::new(Columns::new(1..=2)).with(Format::new(|s| format!("<< {} >>", s))));
```

### Padding

The `Padding` structure provides an interface for a left, right, top and bottom padding of cells.

```rust
use tabled::{
    Table,
    settings::{Modify, Padding, object::Cell}
};

let mut table = Table::new(&data);
table.with(Modify::new(Cell(0, 3)).with(Padding::new(1, 1, 0, 2)));

// It's possible to set a fill char for padding.
let mut table = Table::new(&data)
table.with(Modify::new(Cell(0, 3)).with(Padding::new(1, 1, 0, 2).fill('>', '<', '^', 'V')));
```

#### Padding Color

You can set a color for padding characters.

BE AWARE: It only works with `color` feature.

```rust
use tabled::{
    settings::{Color, Padding},
    Table,
};

let mut table = Table::new(&data);

table.with(Padding::new(1, 1, 0, 2).colorize(
    Color::FG_BLUE,
    Color::FG_BLUE,
    Color::FG_BLUE,
    Color::FG_BLUE,
));
```

### Margin

`Margin` sets extra space around the border (top, bottom, left, right).

```rust
use tabled::{Table, settings::Margin};

let mut table = Table::new(&data);
table.with(Margin::new(3, 4, 1, 2).fill('>', '<', 'v', '^'));
```

An output would depend on the `data`. But it could look like the following.

```text
vvvvvvvvvvvvvvvvvvvvvvvvvvvvv
>>>┌─────────┬──────────┐<<<<
>>>│ feature │ released │<<<<
>>>│ margin  │ 0.6.0    │<<<<
>>>└─────────┴──────────┘<<<<
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

#### Margin Color

You can set a color for padding characters.

```rust
use tabled::{
    settings::{Color, Margin},
    Table,
};

let mut table = Table::new(data);
table
    .with(Margin::new(3, 4, 1, 2))
    .with(Margin::new(1, 1, 1, 1).colorize( Color::BG_RED, Color::BG_RED, Color::BG_RED, Color::BG_RED));
```

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

Using the following structures you can configure a width of a table and a single cell.
But be aware that it doesn't often consider `Padding`.

The functions preserves the text color.

#### Truncate

`Truncate` sets a maximum width of a cell by truncating its content.

```rust
use tabled::{Table, settings::{Modify, Width, object::Rows}};

let mut table = Table::new(data);

// Truncating content to 10 chars in case it's bigger than that
// in a first row.
table.with(Modify::new(Rows::first()).with(Width::truncate(10)));

// Truncating content to 7 chars and puts a suffix '...' after it
// in all rows except a first.
table.with(Modify::new(Rows::new(1..)).with(Width::truncate(10).suffix("...")));
```

`Truncate` also can be used to set a maximum width of a whole table.

```rust
use tabled::{Table, settings::Width};

let mut table = Table::new(data);

// Tries to set table width to 22, in case it's bigger than that.
table.with(Width::truncate(22));
```

It can be used in combination with `MinWidth` to set an exact table size.

#### Wrapping

`Wrap` sets a maximum width of a cell by wrapping its content to new lines.

```rust
use tabled::{Table, settings::{Modify, Width, object::Rows}};

let mut table = Table::new(data);

// Wrap content to 10 chars in case it's bigger than that
// in a first row.
table.with(Modify::new(Rows::first()).with(Width::wrap(10)));

// Use a strategy where we try to keep words not splited (where possible).
table.with(Modify::new(Rows::new(1..)).with(Width::wrap(10).keep_words()));
```

`Wrap` also can be used to set a maximum width of a whole table.

```rust
use tabled::{Table, settings::Width};

let mut table = Table::new(data);

// Tries to set table width to 22, in case it's bigger than that.
table.with(Width::wrap(22));
```

It can be used in combination with `MinWidth` to set an exact table size.

#### Increaase width

`MinWidth` sets a minimal width of an object.

```rust
use tabled::{Table, settings::{Modify, Width, object::Rows}};

let mut table = Table::new(data);

// increase the space used by cells in all rows except the header to be at least 10
table.with(Modify::new(Rows::new(1..)).with(Width::increase(10)));
```

`MinWidth` also can be used to set a minimum width of a whole table.

```rust
use tabled::{Table, settings::Width};

let mut table = Table::new(data);

// increase width of a table in case it was lower than 10.
table.with(Width::increase(10));
```

It can be used in combination with `Truncate` and `Wrap` to set an exact table size.

#### Justify

You can set a constant width for all columns using `Justify`.

```rust
use tabled::{Table, settings::Width};

let mut table = Table::new(data);
table.with(Width::justify(10));
```

#### Priority

You can tweak `Truncate`, `Wrap`, `MinWidth` logic by setting a priority by which a trim/inc be done.

```rust
use tabled::{Table, settings::{Width, peaker::PriorityMax}};

let mut table = Table::new(data);
table.with(Width::truncate(10).priority::<PriorityMax>());
```

#### Percent

By default you use `usize` int to set width settings,
but you could do it also with `tabled::width::Percent`.

```rust
use tabled::width::{Table, settings::{measurement::Percent, Width}};

let mut table = Table::new(data);
table.with(Width::wrap(Percent(75)));
```

### Height

You can increase a table or a specific cell height using `Height` motifier.

#### Height increase

```rust
use tabled::{Table, settings::{Height, Modify, object::Rows}};

let mut table = Table::new(data);

// increase height of a table in case it was lower than 10.
table.with(Height::increase(10));

// increase height of cells in the last row on a table in case if some of them has it lower than 10.
table.with(Modify::new(Rows::last()).with(Height::increase(10)));
```

#### Height limit

```rust
use tabled::{Table, settings::{Height, Modify, object::Rows}};

let mut table = Table::new(data);

// decrease height of a table to 10 in case it was bigger than that.
table.with(Height::limit(10));

// decrease height of cells in the last row on a table to 10 in case if some of them has it bigger than that.
table.with(Modify::new(Rows::last()).with(Height::limit(10)));
```

### Rotate

You can rotate table using `tabled::Rotate`.

Imagine you have a table already which output may look like this.

```text
┌────┬──────────────┬───────────────────────────┐
│ id │ destribution │ link                      │
├────┼──────────────┼───────────────────────────┤
│ 0  │ Fedora       │ https://getfedora.org/    │
├────┼──────────────┼───────────────────────────┤
│ 2  │ OpenSUSE     │ https://www.opensuse.org/ │
├────┼──────────────┼───────────────────────────┤
│ 3  │ Endeavouros  │ https://endeavouros.com/  │
└────┴──────────────┴───────────────────────────┘
```

Now we will add the following modificator and the output will be;

```rust
use tabled::settings::Rotate;

table.with(Rotate::Left)
```

```text
┌──────────────┬────────────────────────┬───────────────────────────┬──────────────────────────┐
│ link         │ https://getfedora.org/ │ https://www.opensuse.org/ │ https://endeavouros.com/ │
├──────────────┼────────────────────────┼───────────────────────────┼──────────────────────────┤
│ destribution │ Fedora                 │ OpenSUSE                  │ Endeavouros              │
├──────────────┼────────────────────────┼───────────────────────────┼──────────────────────────┤
│ id           │ 0                      │ 2                         │ 3                        │
└──────────────┴────────────────────────┴───────────────────────────┴──────────────────────────┘
```

### Disable

You can remove certain rows or columns from the table.

```rust
use tabled::{Table, settings::Disable};

let mut table = Table::new(data);
table
    .with(Disable::Row(..1))
    .with(Disable::Column(3..4));
```

### Extract

You can `Extract` segments of a table to focus on a reduced number of rows and columns.

```rust
use tabled::{Table, settings::Extract};

let mut table = Table::new(&data);
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

#### Refinishing

For styles with unique corner and edge textures it is possible to reapply a table style once a `Table` extract has been created.

```rust
use tabled::{Table, settings::{Extract, Style}};

let mut table = Table::new(&data);
table
    .with(Extract::segment(1..3, 1..))
    .with(Style::modern());
```

```text
Raw extract
┼───────────────────────────┼──────────────────┼──────────────┤
│ The Dark Side of the Moon │ 01 March 1973    │ Unparalleled │
┼───────────────────────────┼──────────────────┼──────────────┤
│ Rumours                   │ 04 February 1977 │ Outstanding  │
┼───────────────────────────┼──────────────────┼──────────────┤

Refinished extract
┌───────────────────────────┬──────────────────┬───────────────┐
│ The Dark Side of the Moon │ 01 March 1973    │ Unparalleled  │
├───────────────────────────┼──────────────────┼───────────────┤
│ Rumours                   │ 04 February 1977 │  Outstanding  │
└───────────────────────────┴──────────────────┴───────────────┘
```

### Header and Footer and Panel

You can add a `Header` and `Footer` to display some information.

```rust
use tabled::{Table, settings::Panel};

let mut table = Table::new(&data);
table
    .with(Panel::header("Tabled Name"))
    .with(Panel::footer(format!("{} elements", data.len())))
```

The look will depend on the style you choose
but it may look something like this:

```text
┌────────────────────────────────────────────────────────────┐
│                       Tabled Name                          │
├────────────────────────────────────────────────────────────┤
                            ...
├───────┼──────────────┼─────────┼───────────────────────────┤
│                        3 elements                          │
└────────────────────────────────────────────────────────────┘
```

You can also add a full row/column using `tabled::Panel`.

```rust
use tabled::{Table, settings::Panel};

let mut table = Table::new(&data);
table
    .with(Panel::vertical(2).text("A panel on 2nd row"))
    .with(Panel::horizontal(0).text("A panel on 1st column"));
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

You can concatanate 2 tables using `Concat`.
It will stick 2 tables together either vertically or horizontally.

```rust
use tabled::settings::Concat;

// let t1: Table = ...;
// let t2: Table = ...;

// vertical concat
t1.with(Concat::vertical(t2));

// horizontal concat
t1.with(Concat::horizontal(t2));
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
table.with(Highlight::new(
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

It's possible to set a horizontal(column) span and vertical(row) span to a cell.

#### Horizontal span

```rust
use tabled::{
    settings::{Alignment, Modify, Span},
    Table,
};

let data = vec![["A", "B", "C"], ["D", "E", "F"]];

let mut table = Table::new(data);
table
    .with(Modify::new((0, 0)).with(Span::column(3)))
    .with(Modify::new((1, 0)).with(Span::column(2)))
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
    settings::{Alignment, Modify, Span},
    Table,
};

let data = vec![["A", "B", "C"], ["D", "E", "F"]];

let mut table = Table::new(data);
table
    .with(Modify::new((0, 1)).with(Span::row(3)))
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
into a new shape with the provided point acting as the new, upper boundry in the direction selected.

#### Directions

Direction functions are the entry point for the `Split` setting.

There are two directions available: `column` and `row`.

```rust
use std::iter::FromIterator;
use tabled::{Table, settings::split::Split};

let mut table = Table::from_iter(['a'..='z']);
table.with(Split::column(12));
table.with(Split::row(2));
```

```text
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │ y │ z │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ y │ z │   │   │   │   │   │   │   │   │   │   │<- y and z act as anchors to new empty cells
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘   to conform to the new shape
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ a │ y │ b │ z │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤<- Display::Clean removes empty cells that would be anchors otherwise
│ m │   │ n │   │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
      ^anchors^
```


#### Behaviors

Behaviors determine how cells attempt to conform to the new tables shape.

There are two behaviors available: `zip` and `concat`. 

`zip` is the default behavior.

```rust
use tabled::{Table, settings::split::Split};

let mut table = Table::new(&data);

table.with(Split::column(2).concat());
table.with(Split::column(2).zip());
```

```text
                                                +---+---+
                                                | a | b |
                                                +---+---+
+---+---+---+---+---+                           | f | g |
| a | b | c | d | e | Split::column(2).concat() +---+---+
+---+---+---+---+---+           =>              | c | d |
| f | g | h | i | j |                           +---+---+
+---+---+---+---+---+                           | h | i |
                                                +---+---+
                                                | e |   |
                                                +---+---+
                                                | j |   |
                                                +---+---+

                  sect 3                        +---+---+
 sect 1   sect 2 (anchors)                      | a | b |
  /   \   /   \   /   \                         +---+---+
+---+---+---+---+---+                           | c | d |
| a | b | c | d | e |  Split::column(2).zip()   +---+---+
+---+---+---+---+---+           =>              | e |   |
| f | g | h | i | j |                           +---+---+
+---+---+---+---+---+                           | f | g |
                                                +---+---+
                                                | h | i |
                                                +---+---+
                                                | j |   |
                                                +---+---+
```

#### Displays

Display functions give the user the choice to `retain` or `clean` empty sections in a `Split` table result.

- `retain` does not filter any existing or newly added cells when conforming to a new shape.

- `clean` filters out empty columns/rows from the output and prevents empty cells from acting as anchors to newly inserted cells.

`clean` is the default `Display`.

```rust
use std::iter::FromIterator;
use tabled::{
    settings::{split::Split, style::Style},
    Table,
};
let mut table = Table::from_iter(['a'..='z']);

table.with(Split::column(25)).with(Style::modern());
table.clone().with(Split::column(1).concat().retain());
table.clone().with(Split::column(1).concat()); // .clean() is not necessary as it is the default display property 
```

```text
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │ y │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ z │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │<- lots of extra cells generated
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┐
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │ y │ z │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐ ^ cells retained during concatenation
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │ y │ z │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘<- cells removed during concatenation
```

#### Duplicate

It's possible to duplicate a given set of cell to another set.

```rust
use tabled::{Table, settings::{Dup, object::Rows}};

let mut table = Table::new(data);

// copy last line to the first line (first line gets erased).
table.with(Dup::new(Rows::first(), Rows::last()));
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

The `Tabled` macros available when `derive` feature in turned on.
And it is by default.

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

You can mark filds as hidden in which case they fill be ignored and not be present on a sheet.

A similar affect could be achieved by the means of a `Disable` setting.

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

## Features

### Color

The library doesn't bind you in usage of any color library but to be able to work correctly with color input, and avoid [miscalculation of string width](https://github.com/zhiburt/tabled/issues/26)
because of embedded ansi sequences, you should add the `color` feature of `tabled` to your `Cargo.toml`:

```toml
tabled = { version = "*", features = ["color"] } 
```

Then you can use colored strings as values and table dimension will be properly estimated.

```rust
use owo_colors::OwoColorize;
// ...
let mut builder = tabled::builder::Builder::default();
builder.push_record(vec!["green".green(), "red".red()])
let mut table = builder.build();
```

Another example:

```rust
use tabled::{format::Format, object::Columns, Modify, Style, Table};

let mut table = Table::new(&data);
table
    .with(Style::psql())
    .with(Modify::new(Columns::single(0)).with(Format::new(|s| s.red().to_string())))
    .with(Modify::new(Columns::single(1)).with(Format::new(|s| s.blue().to_string())))
    .with(Modify::new(Columns::new(2..)).with(Format::new(|s| s.green().to_string())));
```

![carbon-2](https://user-images.githubusercontent.com/20165848/120526301-b95efc80-c3e1-11eb-8779-0ec48894463b.png)

### Tuple combination

You also can combine objects which implements `Tabled` by means of tuples, you will get a combined columns of them.

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
use tabled::{locator::ByColumnName, Alignment, Modify};

table.with(Modify::new(ByColumnName::new("name")).with(Alignment::center()));
```

### Macros

Utilities for dynamic `Table` displays.

#### Col and Row

Combine `col!` and `row!` to create flexible table visualizations.

```rust
row![table1, table2];
```

```text
+-------------------------------------------+---------------------------------------------+
| .---------------------------------------. | ┌────────────────────┬─────┬──────────────┐ |
| | name             | age | is_validated | | │ name               │ age │ is_validated │ |
| | Jon Doe          | 255 | false        | | ├────────────────────┼─────┼──────────────┤ |
| | Mark Nelson      | 13  | true         | | │ Jack Black         │ 51  │ false        │ |
| | Terminal Monitor | 0   | false        | | ├────────────────────┼─────┼──────────────┤ |
| | Adam Blend       | 17  | true         | | │ Michelle Goldstein │ 44  │ true         │ |
| '---------------------------------------' | └────────────────────┴─────┴──────────────┘ |
+-------------------------------------------+---------------------------------------------+
```

```rust
col![table1, table2];
```

```text
+---------------------------------------------+
| .---------------------------------------.   |
| | name             | age | is_validated |   |
| | Jon Doe          | 255 | false        |   |
| | Mark Nelson      | 13  | true         |   |
| | Terminal Monitor | 0   | false        |   |
| | Adam Blend       | 17  | true         |   |
| '---------------------------------------'   |
+---------------------------------------------+
| ┌────────────────────┬─────┬──────────────┐ |
| │ name               │ age │ is_validated │ |
| ├────────────────────┼─────┼──────────────┤ |
| │ Jack Black         │ 51  │ false        │ |
| ├────────────────────┼─────┼──────────────┤ |
| │ Michelle Goldstein │ 44  │ true         │ |
| └────────────────────┴─────┴──────────────┘ |
+---------------------------------------------+
```

```rust
row![table1; 3];
```

```text
+-------------------------------------------+-------------------------------------------+-------------------------------------------+
| .---------------------------------------. | .---------------------------------------. | .---------------------------------------. |
| | name             | age | is_validated | | | name             | age | is_validated | | | name             | age | is_validated | |
| | Jon Doe          | 255 | false        | | | Jon Doe          | 255 | false        | | | Jon Doe          | 255 | false        | |
| | Mark Nelson      | 13  | true         | | | Mark Nelson      | 13  | true         | | | Mark Nelson      | 13  | true         | |
| | Terminal Monitor | 0   | false        | | | Terminal Monitor | 0   | false        | | | Terminal Monitor | 0   | false        | |
| | Adam Blend       | 17  | true         | | | Adam Blend       | 17  | true         | | | Adam Blend       | 17  | true         | |
| '---------------------------------------' | '---------------------------------------' | '---------------------------------------' |
+-------------------------------------------+-------------------------------------------+-------------------------------------------+
```

```rust
col![
    row![table_a, table_b], 
    table_c
]
```

```text
+----------------------------------------------------------------------------------+
| +--------------------------------+---------------------------------------------+ |
| | +-------+-----+--------------+ | ┌────────────────────┬─────┬──────────────┐ | |
| | | name  | age | is_validated | | │ name               │ age │ is_validated │ | |
| | +-------+-----+--------------+ | ├────────────────────┼─────┼──────────────┤ | |
| | | Sam   | 31  | true         | | │ Jack Black         │ 51  │ false        │ | |
| | +-------+-----+--------------+ | ├────────────────────┼─────┼──────────────┤ | |
| | | Sarah | 26  | true         | | │ Michelle Goldstein │ 44  │ true         │ | |
| | +-------+-----+--------------+ | └────────────────────┴─────┴──────────────┘ | |
| +--------------------------------+---------------------------------------------+ |
+----------------------------------------------------------------------------------+
| .---------------------------------------.                                        |
| | name             | age | is_validated |                                        |
| | Jon Doe          | 255 | false        |                                        |
| | Mark Nelson      | 13  | true         |                                        |
| | Terminal Monitor | 0   | false        |                                        |
| | Adam Blend       | 17  | true         |                                        |
| '---------------------------------------'                                        |
+----------------------------------------------------------------------------------+
```

#### `static_table`

It's possible to construct a table at compile time, via [`static_table`](/static_table/README.md).
You'd need to include a different crate to use it.

Notice that you can even use it in documentation.

```rust
/// Multiply 2 integers together.
/// 
/// ```
#[doc = static_table::static_table!([
    ["a", "b", "result"],
    ["1", '2', '3'],
    ["2", '2', '4']
])]
/// ```
pub fn mul(left: usize, right: usize) -> usize {
    left + right
}
```


## Table types

`tabled` has a few representations of tables some differs from it's view some from it's implememtation details.

There are situations when you might better use one but not another.
But sometimes some can be used interchangable.

Bellow you'll find a short list of existing ones. You can find a descriptive information about each at the documentation. 

### `Table`

Main table of the library.
It's implemenentation requires that all data be stored on heap.

### `IterTable`

It's simmilar to main `Table`, it's only difference is that it does not require a the whole buffer.
It only requires a buffer for 1 row at a time.

It might be usefull when you can't fit all your data in memory.

### `CompactTable`

Simmular to `IterTable` but it might not require any buffer.
It also has capability for a sniffing logic, where we estimate data dimension on a small selection of data.

It might be usefull in a very constrain environments.
It is the only table which supports `no-std`.

### `PoolTable`

Unlike `Table` it does not nessarily requires columns be aligned.
It provides capabilities for a completely uterly diverse table layout.

Example

```rust
use tabled::{
    settings::{Alignment, Style},
    tables::PoolTable,
};

fn main() {
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
}

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

### `ExpandedDisplay`

You can use `ExpandedDisplay` if your data structure has a lot of fields.

Here's an example.

```rust
use tabled::{display::ExpandedDisplay, Tabled};

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

let table = ExpandedDisplay::new(&data);

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

## Formats

You can convert some formats to a `Table`.

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

To enable this support, add the `color` feature to your `Cargo.toml`

```toml
tabled = { version = "*", features = ["color"] }
```

### Emoji

The library support emojies out of the box (include `color` feature)
but be aware that some of the terminals and editors may not render them as you would expect.

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
As you can see Github tricks a bit a return table, but `GNOME terminal` and `Alacritty` terminal handles it correctly.

 ```rust
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

It's a friquent case where it's nessary to align a table to a terminal width or height.
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

fn build_table() -> Table {
    let data = [
        ["0.2.1", "2021-06-23", "true", "#[header(inline)] attribute"],
        ["0.2.0", "2021-06-19", "false", "API changes"],
        ["0.1.4", "2021-06-07", "false", "display_with attribute"],
    ];

    Builder::from_iter(data).build()
}

fn get_terminal_size() -> (usize, usize) {
    let (TerminalWidth(width), TerminalHeight(height)) =
        terminal_size().expect("failed to obtain a terminal size");

    (width as usize, height as usize)
}

fn main() {
    let (width, height) = get_terminal_size();

    let term_size_settings = Settings::default()
        .with(Width::wrap(width).priority::<PriorityMax>())
        .with(Width::increase(width))
        .with(Height::limit(height))
        .with(Height::increase(height));

    let mut table = build_table();
    table.with(term_size_settings);

    println!("{table}");
}
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

I hope `tabled` does it's job good, but at the end of the day you probably need to decide it yourself.
If you have any ideas for an enhancement or have a question about `tabled` please file an issue.

Bellow you will find a list of crates which do similar things or do something which `tabled` doesn't.

You can find performance comparison benchmarks [here](https://github.com/zhiburt/tabled/tree/master/tabled/benches/lib_comp).

The description is taken from the author's quotes.

* *[`cli-table`](https://github.com/devashishdxt/cli-table/) tends to keep the compile time and crate size low and support all the platforms. It has an optional `csv` support.*

* *[`comfy-table`](https://github.com/Nukesor/comfy-table) focuses on providing a minimalistic, but rock-solid library for building text-based tables with focus on safety and dynamic-length content arrangement.*

* *[`term-table-rs`](https://github.com/RyanBluth/term-table-rs) main focus is on a good set of tools for rendering CLI tables, while allowing users to bring their own tools for things like colors. It has an ability to have different number of columns in each row of the table.*

Please if you feel about some crate being worth menthioned open an issue.
