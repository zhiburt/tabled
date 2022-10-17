[<img alt="github" src="https://img.shields.io/badge/github-zhiburt/tabled-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/zhiburt/tabled/)
[<img alt="crates.io" src="https://img.shields.io/crates/v/tabled.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tabled)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tabled-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tabled)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/zhiburt/tabled/Continuous%20integration/master?style=for-the-badge" height="20">](https://github.com/zhiburt/tabled/actions)
[<img alt="coverage" src="https://img.shields.io/coveralls/github/zhiburt/tabled/master?style=for-the-badge" height="20">](https://coveralls.io/github/zhiburt/tabled)
[<img alt="dependency status" src="https://deps.rs/repo/github/zhiburt/tabled/status.svg?style=for-the-badge" height="20">](https://deps.rs/repo/github/zhiburt/tabled)

# tabled

An easy to use library for pretty printing tables of Rust `struct`s and `enum`s.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/preview-show.gif">
  <img alt="Preview" src="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/preview-show-light.gif">
</picture>

## Table of Contents

- [Usage](#usage)
- [Settings](#settings)
  - [Style](#style)
    - [Themes](#themes)
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
  - [Alignment](#alignment)
  - [Format](#format)
  - [Padding](#padding)
    - [Padding Color](#padding-color)
  - [Margin](#margin)
    - [Margin Color](#margin-color)
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
- [Derive](#derive)
  - [Override a column name](#override-a-column-name)
  - [Hide a column](#hide-a-column)
  - [Set column order](#set-column-order)
  - [Format fields](#format-fields)
  - [Format headers](#format-headers)
  - [Inline](#inline)
- [Dynamic table](#dynamic-table)
  - [Build index](#build-index)
- [Features](#features)
  - [Color](#color)
  - [Tuple combination](#tuple-combination)
  - [Object](#object)
  - [Macros](#macros)
    - [Col and Row](#col-and-row)
- [Views](#views)
  - [Expanded display](#expanded-display)
- [Formats](#formats)
  - [`json` format](#json-format)
- [Notes](#notes)
  - [ANSI escape codes](#ansi-escape-codes)
  - [Emoji](#emoji)
  - [Semver](#semver)
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
        name: "Rust",
        designed_by: "Graydon Hoare",
        invented_year: 2010
    },
    Language{
        name: "Go",
        designed_by: "Rob Pike",
        invented_year: 2009
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
use tabled::TableIteratorExt;

let numbers = [1, 2, 3];
let table = numbers.table();
    
println!("{:#^10}", table);
```

## Settings

This section lists the set of settings you can apply to your table.
Most of the settings are used by `.with` method of `Table`.

You can find a list of show cases in **[examples folder](/examples/README.md)**.

### Style

#### Themes

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
let style = tabled::Style::modern().off_horizontal();
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
use tabled::style::{Style, HorizontalLine, VerticalLine};

let style = Style::modern()
    .off_horizontal()
    .off_vertical()
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
use tabled::{object::Rows, Border, Modify, Style, TableIteratorExt};

let data = [["123", "456"], ["789", "000"]];

let table = data
    .table()
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
use tabled::{Table, BorderText};

let mut table = Table::new(["Hello World"]);
table.with(BorderText::new(0, "+-.table"));

assert_eq!(
    table.to_string(),
    "+-.table------+\n\
     | &str        |\n\
     +-------------+\n\
     | Hello World |\n\
     +-------------+"
);
```

Sometimes though it's not convinient to set a string.
But rather necessary to set a custom char.

You can use `BorderChar` to achieve this.

```rust
use tabled::{
    object::Columns,
    style::{BorderChar, Offset, Style},
    Modify, Table,
};

fn main() {
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
}
```

#### Colorize borders

You can set a colors of all borders using `Color`.

```rust
use tabled::color::Color;

let color = Color::try_from(" ".magenta().to_string()).unwrap();

table.with(color)
```

You can also set a color border of intividial cell by using `BorderColored`.

```rust
use tabled::{Modify, style::{Symbol, BorderColored}, object::Columns};

// set a top border of each cell in second column to red '=' character.
let b = Symbol::ansi("═".red().to_string()).unwrap();

table.with(Modify::new(Columns::single(2)).with(BorderColored::default().top(c)))
```

### Alignment

You can set a horizontal and vertical alignment for any `Object` (e.g `Columns`, `Rows`).

```rust
use tabled::{TableIteratorExt, Modify, Alignment, object::Segment};

let mut table = data.table();
table
    .with(Modify::new(Segment::all()).with(Alignment::left()).with(Alignment::top()));
```

### Format

The `Format` function provides an interface for a modification of cells.

```rust
use tabled::{Table, Modify, format::Format, object::{Rows, Columns}};

let mut table = Table::new(&data);
table
    .with(Modify::new(Rows::first()).with(Format::new(|s| format!("Head {}", s))))
    .with(Modify::new(Columns::new(1..=2)).with(Format::new(|s| format!("<< {} >>", s))));
```

It's also possible to use functions with signature `Fn(&str) -> String` as a formatter.

```rust
use tabled::{Table, Modify, object::{Rows, Columns}};

let mut table = Table::new(&data);
table
    .with(Modify::new(Columns::single(3)).with(|s: &str| format!("<< {} >>", s)))
    .with(Modify::new(Rows::first()).with(str::to_lowercase));
```

IMPORTANT: you may need to specify the type in your lambda otherwise the compiler may be disagreed to work :)

### Padding

The `Padding` structure provides an interface for a left, right, top and bottom padding of cells.

```rust
use tabled::{Table, Modify, Padding, object::Cell};

let mut table = Table::new(&data);
table.with(Modify::new(Cell(0, 3)).with(Padding::new(1, 1, 0, 2)));

// It's possible to set a fill char for padding.
let mut table = Table::new(&data)
table.with(Modify::new(Cell(0, 3)).with(Padding::new(1, 1, 0, 2).set_fill('>', '<', '^', 'V')));
```

#### Padding Color

You can set a color for padding characters.

BE AWARE: It only works with `color` feature.

```rust
use std::convert::TryFrom;
use owo_colors::OwoColorize;
use tabled::{
    color::Color, object::Segment, padding_color::PaddingColor, Modify, Padding, Table,
};

let mut table = Table::new(&data);

let on_red = Color::try_from(' '.on_red().to_string()).unwrap();
let padding = Modify::new(Segment::all())
    .with(Padding::new(1, 1, 0, 2))
    .with(PaddingColor::new(on_red.clone(), on_red.clone(), on_red.clone(), on_red));

table.with(padding);
```

### Margin

`Margin` sets extra space around the border (top, bottom, left, right).

```rust
use tabled::{Table, Margin};

let mut table = Table::new(&data);
table.with(Margin::new(3, 4, 1, 2).set_fill('>', '<', 'v', '^'));
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

BE AWARE: It only works with `color` feature.

```rust
use std::convert::TryFrom;
use owo_colors::OwoColorize;
use tabled::{color::Color, margin_color::MarginColor, Margin, Table};

let on_red = Color::try_from(' '.on_red().to_string()).unwrap();

let mut table = Table::new(&data);
table
    .with(Margin::new(3, 4, 1, 2))
    .with(MarginColor::new(on_red.clone(), on_red.clone(), on_red.clone(), on_red));
```


### Width

Using the following structures you can configure a width of a table and a single cell.
But be aware that it doesn't often consider `Padding`.

The functions preserves the text color.

#### Truncate

`Truncate` sets a maximum width of a cell by truncating its content.

```rust
use tabled::{TableIteratorExt, Modify, Width, object::Rows};

let mut table = data.table();

// Truncating content to 10 chars in case it's bigger than that
// in a first row.
table.with(Modify::new(Rows::first()).with(Width::truncate(10)));

// Truncating content to 7 chars and puts a suffix '...' after it
// in all rows except a first.
table.with(Modify::new(Rows::new(1..)).with(Width::truncate(10).suffix("...")));
```

`Trucate` also can be used to set a maximum width of a whole table.

```rust
use tabled::{TableIteratorExt, Width};

let mut table = data.table();

// Tries to set table width to 22, in case it's bigger than that.
table.with(Width::truncate(22));
```

It can be used in combination with `MinWidth` to set an exact table size.

#### Wrapping

`Wrap` sets a maximum width of a cell by wrapping its content to new lines.

```rust
use tabled::{TableIteratorExt, Modify, Width, object::Rows};

let mut table = data.table();

// Wrap content to 10 chars in case it's bigger than that
// in a first row.
table.with(Modify::new(Rows::first()).with(Width::wrap(10)));

// Use a strategy where we try to keep words not splited (where possible).
table.with(Modify::new(Rows::new(1..)).with(Width::wrap(10).keep_words()));
```

`Wrap` also can be used to set a maximum width of a whole table.

```rust
use tabled::{TableIteratorExt, Width};

let mut table = data.table();

// Tries to set table width to 22, in case it's bigger than that.
table.with(Width::wrap(22));
```

It can be used in combination with `MinWidth` to set an exact table size.

#### Increaase width

`MinWidth` sets a minimal width of an object.

```rust
use tabled::{TableIteratorExt, Modify, Width, object::Rows};

let mut table = data.table();

// increase the space used by cells in all rows except the header to be at least 10
table.with(Modify::new(Rows::new(1..)).with(Width::increase(10)));
```

`MinWidth` also can be used to set a minimum width of a whole table.

```rust
use tabled::{TableIteratorExt, Width};

let mut table = data.table();

// increase width of a table in case it was lower than 10.
table.with(Width::increase(10));
```

It can be used in combination with `Truncate` and `Wrap` to set an exact table size.

#### Justify

You can set a constant width for all columns using `Justify`.

```rust
use tabled::{TableIteratorExt, Width};

let mut table = data.table();
table.with(Width::justify(10));
```

#### Priority

You can tweak `Truncate`, `Wrap`, `MinWidth` logic by setting a priority by which a trim/inc be done.

```rust
use tabled::{TableIteratorExt, Width, width::PriorityMax};

let mut table = data.table();
table.with(Width::truncate(10).priority::<PriorityMax>());
```

#### Percent

By default you use `usize` int to set width settings,
but you could do it also with `tabled::width::Percent`.

```rust
use tabled::width::{TableIteratorExt, Percent, Width};

let mut table = data.table();
table.with(Width::wrap(Percent(75)));
```

### Height

You can increase a table or a specific cell height using `Height` motifier.

#### Height increase

```rust
use tabled::{TableIteratorExt, Height, Modify, Rows};

let mut table = data.table();

// increase height of a table in case it was lower than 10.
table.with(Height::increase(10));

// increase height of cells in the last row on a table in case if some of them has it lower than 10.
table.with(Modify::new(Rows::last()).with(Height::increase(10)));
```

#### Height limit

```rust
use tabled::{TableIteratorExt, Height, Modify, Rows};

let mut table = data.table();

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
use tabled::Rotate;

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
use tabled::{TableIteratorExt, Disable};

let mut table = data.table();
table
    .with(Disable::Row(..1))
    .with(Disable::Column(3..4));
```

### Extract

You can `Extract` segments of a table to focus on a reduced number of rows and columns.

```rust
use tabled::{Table, Extract};

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
use tabled::{Table, Extract, Style};

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
use tabled::{Table, Panel};

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
use tabled::{Table, Panel};

let mut table = Table::new(&data);
table
    .with(Panel::vertical(2).text("A panel on 2nd row"))
    .with(Panel::horizontal(0).text("A panel on 1st column"));
```

### Merge

It's possible to create `"Panel"`s by combining the duplicates using `Merge`.

```rust
use tabled::{merge::Merge, TableIteratorExt};

let data = [['A', 'B', 'B'], ['A', 'W', 'E'], ['Z', 'Z', 'Z']];

let mut table = data.table();
table
    .with(Merge::horizontal())
    .with(Merge::vertical());

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
use tabled::Concat;

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
    object::{Cell, Columns, Object, Rows},
    Border, Highlight, Style, TableIteratorExt,
};

let data = vec![
    ["A", "B", "C"],
    ["D", "E", "F"]
];

let mut table = data.table();
table
    .with(Style::modern())
    .with(Highlight::new(
        Rows::first().and(Columns::single(2).and(Cell(1, 1))),
        Border::filled('*'),
    ));
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
use tabled::{object::Cell, object::Segment, Alignment, Modify, Span, TableIteratorExt};

let data = vec![
    ["A", "B", "C"],
    ["D", "E", "F"],
];

let mut table = data.table();
table
    .with(Modify::new(Cell(0, 0)).with(Span::column(3)))
    .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
    .with(Modify::new(Segment::all()).with(Alignment::center()));

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
use tabled::{object::Cell, object::Segment, Alignment, Modify, Span, TableIteratorExt};

let data = vec![
    ["A", "B", "C"],
    ["D", "E", "F"],
];

let mut table = data.table();
table
    .with(Modify::new(Cell(0, 1)).with(Span::row(3)))
    .with(Modify::new(Segment::all()).with(Alignment::center()));

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

It's also possible to change function argument to be `&self`,
using `#[tabled(display_with("some_function", args))]`

```rust
use tabled::Tabled;

#[derive(Tabled)]
pub struct MyRecord {
    pub id: i64,
    #[tabled(display_with("Self::display_valid", args))]
    pub valid: Option<bool>
}

impl MyRecord {
    fn display_valid(&self) -> String {
        match self.valid {
            Some(s) => format!("is valid thing = {}", s),
            None => format!("is not valid"),
        }
    }
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

## Dynamic table

Sometimes you can't say what type of data you are going to deal with (like parsing `csv`).
In such cases it may be handy to build table dynamically.

```rust
use tabled::{builder::Builder, Style};

let song = r#"
And the cat's in the cradle and the silver spoon
Little boy blue and the man on the moon
When you comin' home dad?
I don't know when, but we'll get together then son
You know we'll have a good time then
"#;

let mut builder = Builder::default();
let mut max_words = 0;
for line in song.lines() {
    if line.is_empty() {
        continue;
    }

    let words: Vec<_> = line.split_terminator(' ').collect();
    max_words = std::cmp::max(max_words, words.len());
    builder.add_record(words);
}

let columns = (0..max_words).map(|i| i.to_string()).collect::<Vec<_>>();
builder.set_columns(columns);

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

### Build index

You can change a table layout by `Builder`.

```rust
// previos example
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
use tabled::{builder::Builder, Style};

let mut builder = Builder::default();
builder
    .set_columns(["Index", "Language", "Status"])
    .add_record(["1", "English", "In progress"])
    .add_record(["2", "Deutsch", "Not ready"]);

let mut builder = builder.index();
builder.set_index(1).set_name(None);

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

## Features

### Color

The library doesn't bind you in usage of any color library but to be able to work correctly with color input you should
add the `color` feature of `tabled` to your `Cargo.toml`

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
use tabled::{object::Segment, Alignment, ModifyObject, Style, Table, Tabled};

#[derive(Tabled)]
struct Developer(#[tabled(rename = "name")] &'static str);

#[derive(Tabled)]
enum Domain {
    Security,
    Embeded,
    Frontend,
    Unknown,
}

let data = vec![
    (Developer("Terri Kshlerin"), Domain::Embeded),
    (Developer("Catalina Dicki"), Domain::Security),
    (Developer("Jennie Schmeler"), Domain::Frontend),
    (Developer("Maxim Zhiburt"), Domain::Unknown),
];

let table = Table::new(data)
    .with(Style::psql())
    .with(Segment::all().modify().with(Alignment::center()))
    .to_string();

assert_eq!(
    table,
    concat!(
        "      name       | Security | Embeded | Frontend | Unknown \n",
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
use tabled::object::{Object, Segment, Cell, Rows, Columns};
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

## Views

`Tabled` supports not only Table view!

### Expanded display

You can use `ExpanedDisplay` if your data structure has a lot of fields.

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

## Notes

### ANSI escape codes

By default `tabled` doesn't handle [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code).
By default such things as hyperlinks, blinking and others things which can be achieved via ANSI codes might not work correctly.

To enable this support, add the `color` feature to your `Cargo.toml`

```toml
tabled = { version = "*", features = ["color"] }
```

### Emoji

The library support emojies out of the box but be aware that some of the terminals and editors may not render them as you would expect.

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

### Semver

> When you need to release a breaking change—any breaking change—you do it in a major version. Period. No excuses.

We still do it.
We often do break change on minor version bump.
So you probably shall not depend on minor version (like `0.7`).
It's likely better to depend on constant version e.g. `0.8.0`

### Comparison

Nowadays there's a few libraries for pretty tables.
Some may wonder why `tabled` is better or worse than others libraries?

I hope `tabled` does it's job good, but at the end of the day you probably need to decide it yourself.
If you have any ideas for an enhancement or have a question about `tabled` please file an issue.

Bellow you will find a list of crates which do simmilar things or do something which `tabled` doesn't.
You can find performance Comparison benchmarks for some of them [here](https://github.com/zhiburt/tabled/tree/comparision-libs-bench). 

The description is taken from the author's quotes.

* *[`cli-table`](https://github.com/devashishdxt/cli-table/) tends to keep the compile time and crate size low and support all the platforms. It has an optional `csv` support.*

* *[`comfy-table`](https://github.com/Nukesor/comfy-table) focuses on providing a minimalistic, but rock-solid library for building text-based tables with focus on safety and dynamic-length content arrangement.*

* *[`term-table-rs`](https://github.com/RyanBluth/term-table-rs) main focus is on a good set of tools for rendering CLI tables, while allowing users to bring their own tools for things like colors. It has an ability to have different number of columns in each row of the table.*

Please if you feel about some crate being worth menthioned open an issue.
