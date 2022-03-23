[![Build Status](https://github.com/zhiburt/tabled/actions/workflows/ci.yml/badge.svg?style=for-the-badge)](https://github.com/zhiburt/tabled/actions)
[![Coverage Status](https://img.shields.io/coveralls/github/zhiburt/tabled/master)](https://coveralls.io/github/zhiburt/tabled)
[![Crate](https://img.shields.io/crates/v/tabled)](https://crates.io/crates/tabled)
[![docs.rs](https://img.shields.io/docsrs/tabled?color=blue)](https://docs.rs/tabled)
[![license](https://img.shields.io/crates/l/tabled)](./LICENSE.txt)
[![dependency status](https://deps.rs/repo/github/zhiburt/tabled/status.svg)](https://deps.rs/repo/github/zhiburt/tabled)

# tabled

An easy to use library for pretty printing tables of Rust `struct`s and `enum`s.

## Table of Contents

* [Usage](#Usage)
* [Settings](#Settings)
    * [Style](#Style)
        * [Themes](#Themes)
            * [ASCII](#ASCII)
            * [Psql](#Psql)
            * [Github Markdown](#Github-Markdown)
            * [Modern](#Modern)
            * [ReStructuredText](#ReStructuredText)
            * [Extended](#Extended)
            * [Dots](#Dots)
            * [Blank](#Blank)
            * [Custom](#Custom)
        * [Cell Border](#Cell-Border)
        * [Text in a top border](#Text-in-a-top-border)
    * [Alignment](#Alignment)
    * [Format](#Format)
        * [Modifiers](#Modifiers)
    * [Padding](#Padding)
    * [Max width](#Max-width)
    * [Rotate](#Rotate)
    * [Disable](#Disable)
    * [Extract](#Extract)
        * [Refinish](#Refinishing)
    * [Header and Footer](#Header-and-Footer)
    * [Concat](#Concat)
    * [Highlight](#Highlight)
    * [Column span](#Column-span)
* [Derive](#Derive)
    * [Column name override](#Column-name-override)
    * [Hide a column](#Hide-a-column)
    * [Custom field formatting](#Custom-field-formatting)
    * [Inline](#Inline)
* [Features](#Features)
    * [Tuple combination](#Tuple-combination)
    * [Color](#Color)
    * [Object](#Object)
* [Views](#Views)
    * [Expanded Display](#Expanded-Display)
* [Notes](#Notes)
   * [ANSI escape codes](#ANSI-escape-codes) 
   * [Dynamic table](#Dynamic-table)
   * [Emoji](#Emoji)

## Usage

To print a list of structs or enums as a table your types should implement the the `Tabled` trait or derive with a `#[derive(Tabled)]` macro.

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
                | name |  designed_by   | invented_year |\n\
                +------+----------------+---------------+\n\
                |  C   | Dennis Ritchie |     1972      |\n\
                +------+----------------+---------------+\n\
                | Rust | Graydon Hoare  |     2010      |\n\
                +------+----------------+---------------+\n\
                |  Go  |    Rob Pike    |     2009      |\n\
                +------+----------------+---------------+\n";

assert_eq!(table, expected);
```

Most of the default types implement the trait out of the box.

```rust
use tabled::TableIteratorExt;
let some_numbers = [1, 2, 3];
let table = some_numbers.table();
```

## Settings

In this section is listened a set of settings you can apply for your table.

### Style

#### Themes

There are a list of ready to use styles.
Each style can be castomized.

A custom style also can be created from scratch.

A style can be used by passing it to `.with` method of `Table`.

```rust
use tabled::{Table, Style};

let table = Table::new(&data).with(Style::psql());
```

Bellow rendered a list of pre configured styles.

If you think that there's some valuable style to be added,
Please open an issue.

##### ASCII

```
+------+----------------+---------------+
| name |  designed_by   | invented_year |
+------+----------------+---------------+
|  C   | Dennis Ritchie |     1972      |
+------+----------------+---------------+
| Rust | Graydon Hoare  |     2010      |
+------+----------------+---------------+
|  Go  |    Rob Pike    |     2009      |
+------+----------------+---------------+
```

##### Psql

```
 name |  designed_by   | invented_year 
------+----------------+---------------
  C   | Dennis Ritchie |     1972      
 Rust | Graydon Hoare  |     2010      
  Go  |    Rob Pike    |     2009      
```

##### Github Markdown

```
| name |  designed_by   | invented_year |
|------+----------------+---------------|
|  C   | Dennis Ritchie |     1972      |
| Rust | Graydon Hoare  |     2010      |
|  Go  |    Rob Pike    |     2009      |
```

##### Modern

```
┌──────┬────────────────┬───────────────┐
│ name │  designed_by   │ invented_year │
├──────┼────────────────┼───────────────┤
│  C   │ Dennis Ritchie │     1972      │
├──────┼────────────────┼───────────────┤
│ Rust │ Graydon Hoare  │     2010      │
├──────┼────────────────┼───────────────┤
│  Go  │    Rob Pike    │     2009      │
└──────┴────────────────┴───────────────┘
```

##### ReStructuredText

```
====== ================ ===============
 name    designed_by     invented_year 
====== ================ ===============
  C     Dennis Ritchie       1972      
 Rust   Graydon Hoare        2010      
  Go       Rob Pike          2009      
====== ================ ===============
```

##### Extended

```
╔══════╦════════════════╦═══════════════╗
║ name ║  designed_by   ║ invented_year ║
╠══════╬════════════════╬═══════════════╣
║  C   ║ Dennis Ritchie ║     1972      ║
╠══════╬════════════════╬═══════════════╣
║ Rust ║ Graydon Hoare  ║     2010      ║
╠══════╬════════════════╬═══════════════╣
║  Go  ║    Rob Pike    ║     2009      ║
╚══════╩════════════════╩═══════════════╝
```

##### Dots

```
.........................................
: name :  designed_by   : invented_year :
:......:................:...............:
:  C   : Dennis Ritchie :     1972      :
: Rust : Graydon Hoare  :     2010      :
:  Go  :    Rob Pike    :     2009      :
:......:................:...............:
```

##### Blank

```
 name    designed_by     invented_year 
  C     Dennis Ritchie       1972      
  Rust   Graydon Hoare       2010      
  Go       Rob Pike          2009      
```

##### Custom

You can modify existing styles to fit your needs.

```rust
let style = tabled::Style::modern().header_off().horizontal_off();
```

The style will look like the following.

```rust
┌──────┬────────────────┬───────────────┐
│ name │  designed_by   │ invented_year │
│  C   │ Dennis Ritchie │     1972      │
│ Rust │ Graydon Hoare  │     2010      │
│  Go  │    Rob Pike    │     2009      │
└──────┴────────────────┴───────────────┘
```

You can find more methods which are available in the [documentation](https://docs.rs/tabled/latest/tabled/style/struct.CustomStyle.html)

#### Cell Border

Sometimes `tabled::Style` settings are not enough.
Sometimes it's nesessary to change a border of a particular cell.

For this porpouse you can use `Border`.

```rust
use tabled::{TableIteratorExt, Modify, Border, object::Rows};

let data = [["123", "456"], ["789", "000"]];

let table = data.table()
    .with(Style::ascii())
    .with(Modify::new(Rows::single(0)).with(Border::default().top('x')));

let expected = "+xxxxx+xxxxx+\n\
                |  0  |  1  |\n\
                +-----+-----+\n\
                | 123 | 456 |\n\
                +-----+-----+\n\
                | 789 | 000 |\n\
                +-----+-----+\n";

assert_eq!(table.to_string(), expected);
```

#### Text in a top border

You can write a custom text at the top border if it's present in a style.

```rust
use tabled::{Table, style::TopBorderText};

let table = Table::new(["Hello World"])
    .with(TopBorderText::new("+-.table"));

assert_eq!(
    table.to_string(),
    "+-.table------+\n\
     |    &str     |\n\
     +-------------+\n\
     | Hello World |\n\
     +-------------+\n"
);
```

### Alignment

You can set a horizontal and vertical alignment for a `Header`, `Column`, `Row` or `Full` set of cells.

```rust
use tabled::{TableIteratorExt, Modify, Alignment, object::Full};

data.table()
    .with(Modify::new(Full).with(Alignment::left()).with(Alignment::top()));
```

### Format

The `Format` function provides an interface for a modification of cells.

```rust
use tabled::{Table, Modify, Format, object::{Rows, Columns}};

Table::new(&data)
    .with(Modify::new(Rows::first()).with(Format(|s| format!("Head {}", s))));
    .with(Modify::new(Columns::new(1..=2)).with(Format(|s| format!("<< {} >>", s))))
```

It's also possible to use functions with signature `Fn(&str) -> String` as a formatter.

```rust
use tabled::{Table, Modify, object::{Rows, Columns}};

Table::new(&data)
    .with(Modify::new(Columns::single(3)).with(|s: &str| format!("<< {} >>", s)))
    .with(Modify::new(Rows::first()).with(str::to_lowercase));
```

> IMPORTANT: you may need to specify type in your lambda

#### Modifiers

- `FormatFrom` - Uses `Vec` elements as new content.
- `FormatWithIndex` - Like `Format` but with `row` and `column` index in lambda.
- `Trim` - Removes leading and trailing whitespace from range of cells 

### Padding

The `Padding` structure provides an interface for a left, right, top and bottom padding of cells.

```rust
use tabled::{Table, Modify, Padding, object::Cell};

Table::new(&data)
    .with(Modify::new(Cell(0, 3)).with(Padding::new(1, 1, 0, 2)));

// It's possible to set a fill char for padding.
Table::new(&data)
    .with(Modify::new(Cell(0, 3)).with(Padding::new(1, 1, 0, 2).set_fill('>', '<', '^', 'V')));
```

### Max width

Using `MaxWidth` type its possible to set a max width of an object.
While tinkering content we don't forget about its color.

```rust
use tabled::{TableIteratorExt, Modify, MaxWidth, object::Rows};

// Truncating content to 10 chars in all rows except a header.
data.table()
    .with(Modify::new(Rows::new(1..)).with(MaxWidth::truncating(10, "...")));

// Wrapping content by new lines after 10 chars in a last row.
data.table()
    .with(Modify::new(Rows::last()).with(MaxWidth::wrapping(10)));
```

### Rotate

You can rotate table using `tabled::Rotate`.

Imagine you have a table already. And the output may look like this.

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
table.with(Rotate::Left)
```

```text
┌──────────────┬────────────────────────┬───────────────────────────┬──────────────────────────┐
│     link     │ https://getfedora.org/ │ https://www.opensuse.org/ │ https://endeavouros.com/ │
├──────────────┼────────────────────────┼───────────────────────────┼──────────────────────────┤
│ destribution │         Fedora         │         OpenSUSE          │       Endeavouros        │
├──────────────┼────────────────────────┼───────────────────────────┼──────────────────────────┤
│      id      │           0            │             2             │            3             │
└──────────────┴────────────────────────┴───────────────────────────┴──────────────────────────┘
```

### Disable

You can remove certain rows or columns from the table.

```rust
use tabled::{TableIteratorExt, Disable};

data.table()
    .with(Disable::Row(..1))
    .with(Disable::Column(3..4));
```

### Extract

You can `Extract` segments of a table to focus on a reduced number of rows and columns.

```rust
use tabled::{Table, Extract};

let rows = 1..3;
let columns = 1..;
Table::new(&data)
    .with(Extract::segment(rows, columns));
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

let rows = 1..3;
let columns = 1..;
Table::new(&data)
    .with(Extract::segment(rows, columns))
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


### Header and Footer

You can add a `Header` and `Footer` to display some information.

```rust
use tabled::{Table, Header, Footer};

Table::new(&data)
    .with(Header("Tabled Name"))
    .with(Footer(format!("{} elements", data.len())))
```

A look will differ from a style you choose.
But it's how it may look like.

```text
┌────────────────────────────────────────────────────────────┐
│                       Tabled Name                          │
├────────────────────────────────────────────────────────────┤
                            ...
├───────┼──────────────┼─────────┼───────────────────────────┤
│                        3 elements                          │
└────────────────────────────────────────────────────────────┘
```

You can also add a full row on any line using `tabled::Panel`.

### Concat

You can concatanate 2 tables using `Concat`.
It will stick 2 tables together either vertically or horizontally.

```rust
let t1: Table = ...;
let t2: Table = ...;

let t3: Table = t1.with(Concat::vertical(t2));
```

#### Highlight

`Highlight` can be used to change a borders of target sector.
Here's an example.

```rust
use tabled::{
    object::{Columns, Object, Rows},
    style::{Border, Style},
    Highlight, TableIteratorExt,
};

let data = vec![
    ["A", "B", "C"],
    ["D", "E", "F"]
];

let table = data.table()
    .with(Style::modern())
    .with(Highlight::new(
        Rows::first().and(Columns::single(2).and(Cell(1, 1))),
        Border::filled('*'),
    ));
```

The printed table would be like the following.

```text
*************
* 0 │ 1 │ 2 *
*****───┼───*
│ A * B │ C *
├───*****───*
│ D │ E * F *
└───┴───*****
```

#### Column span

It's possible to have a horizontal (column) span of a cell.

The code example and the resulting table.

```rust
use tabled::{object::Cell, Modify, Span, TableIteratorExt};

fn main() {
    let data = vec![
        ["A", "B", "C"],
        ["D", "E", "F"],
    ];

    let table = data
        .table()
        .with(Modify::new(Cell(0, 0)).with(Span::column(3)))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)));

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

### Column name override

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

A similar affect could be achived by the means of a `Disable` setting.

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

### Custom field formatting

`#[derive(Tabled)]` is possible only when all fields implement a `Display` trait.

However, this may be often not the case for example when a field uses the `Option` type.

There's 2 common ways how to solve this:

* Implement `Tabled` trait manually for a type.
* Wrap `Option` to something like `DisplayedOption<T>(Option<T>)` and implement a Display trait for it.

Or to use an attribute `#[tabled(display_with = "func")]` for the field. To use it you must provide a function name in a `display_with` parameter.
   
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

### Inline
   
It's possible to inline internal data if it implements `Tabled` trait.
Use `#[tabled(inline)]` for it.
Also you can set a prefix which will be used for all inlined elements use `#[tabled(inline("prefix>>"))]` for it.

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
    Bikecycle(#[tabled(rename = "name")] &'static str, #[tabled(inline)] Bike),
}
        
#[derive(Tabled)]
struct Bike {
    brand: &'static str,
    price: f32,
}
```

## Features

### Color

The library doesn't bind you in usage of any color library but to be able to work corectly with color input you should provide a `--features color`.

```rust
use tabled::{Table, Modify, Style, Format, object::Columns};

Table::new(&data)
    .with(Style::psql())
    .with(Modify::new(Columns::single(0)).with(Format(|s| s.red().to_string())))
    .with(Modify::new(Columns::single(1)).with(Format(|s| s.blue().to_string())))
    .with(Modify::new(Columns::new(2..)).with(Format(|s| s.green().to_string())));
```

![carbon-2](https://user-images.githubusercontent.com/20165848/120526301-b95efc80-c3e1-11eb-8779-0ec48894463b.png)
   
### Tuple combination

You also can combine objets which implements `Tabled` by means of tuples, you will get a combined columns of them.

```rust
use tabled::{Tabled, Table, Style};

#[derive(Tabled)]
enum Domain {
    Security,
    Embeded,
    Frontend,
    Unknown,
}

#[derive(Tabled)]
struct Developer(#[tabled("name")] &'static str);

let data = vec![
    (Developer("Terri Kshlerin"), Domain::Embeded),
    (Developer("Catalina Dicki"), Domain::Security),
    (Developer("Jennie Schmeler"), Domain::Frontend),
    (Developer("Maxim Zhiburt"), Domain::Unknown),
];

let table = Table::new(data).with(Style::psql()).to_string();

assert_eq!(
    table,
    concat!(
        "      name       | Security | Embeded | Frontend | Unknown \n",
        "-----------------+----------+---------+----------+---------\n",
        " Terri Kshlerin  |          |    +    |          |         \n",
        " Catalina Dicki  |    +     |         |          |         \n",
        " Jennie Schmeler |          |         |    +     |         \n",
        "  Maxim Zhiburt  |          |         |          |    +    \n"
    )
);
```

### Object

You can peak your target for settings using `and` and `not` methods for an object.

```rust
use tabled::object::{Full, Cell, Rows, Columns};

Full.not(Rows::first()) // peak all cells except header
Rows::first().and(Columns::single(0)).not(Cell(0, 0)) // peak a header and first column except a (0, 0) cell
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

fn main() {
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
}
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

## Notes

### ANSI escape codes

By default `tabled` doesn't handle [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code).
By default such things as hyperlinks, blinking and others things which can be achived via ANSI codes might not work correctly.

`tabled` support it by setting a `color` feature.

```toml
tabled = { version = "*", features = ["color"] }
```

### Dynamic table

It might be hard to build a table using `Tabled` trait, if you have a data set which structure is determined at runtime.
In such situation you can use a `Builder`.

```rust
use tabled::{builder::Builder, Style};

fn main() {
    let table = Builder::default()
        .set_header(["Index", "Language"])
        .add_row(["1", "English"])
        .add_row(["2", "Deutsch"])
        .build()
        .with(Style::psql());

    println!("{}", table);
}
```

### Emoji
   
The library support emojies out of the box but be aware that some of the terminals and editors may not render them as you would expect.
   
Let's add emojies to an example from a [Usage](##Usage) section.

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
 
 As you can see Github triks a bit a return table, but `GNOME terminal` and `Alacritty` terminal handles it correctly.
   
 ```rust
+---------+----------------+---------------+
|  name   |  designed_by   | invented_year |
+---------+----------------+---------------+
|  C 💕   | Dennis Ritchie |     1972      |
+---------+----------------+---------------+
| Rust 👍 | Graydon Hoare  |     2010      |
+---------+----------------+---------------+
|  Go 🧋  |    Rob Pike    |     2009      |
+---------+----------------+---------------+
```
