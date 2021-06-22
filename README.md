[![Build Status](https://github.com/zhiburt/tabled/actions/workflows/ci.yml/badge.svg?style=for-the-badge)](https://github.com/zhiburt/tabled/actions)
[![Coverage Status](https://img.shields.io/coveralls/github/zhiburt/tabled/master)](https://coveralls.io/repos/github/zhiburt/tabled/badge.svg?branch=branch)
[![Crate](https://img.shields.io/crates/v/tabled)](https://crates.io/crates/tabled)
[![docs.rs](https://img.shields.io/docsrs/tabled?color=blue)](https://docs.rs/tabled/0.1.1/tabled/)
[![license](https://img.shields.io/crates/l/tabled)](./LICENSE.txt)
[![dependency status](https://deps.rs/repo/github/zhiburt/tabled/status.svg)](https://deps.rs/repo/github/zhiburt/tabled)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fzhiburt%2Ftabled.svg?type=small)](https://app.fossa.com/projects/git%2Bgithub.com%2Fzhiburt%2Ftabled?ref=badge_small)
# tabled

An easy to use library for pretty printing tables of Rust `struct`s and `enum`s.

# Agenda

* [Usage](#Usage)
    * [Derive information](#Derive-information)
* [Style](#Style)
    * [Styles](#Styles)
        * [Default](#Default)
        * [Psql](#Psql)
        * [GithubMarkdown](#GithubMarkdown)
        * [Pseudo](#Pseudo)
        * [PseudoClean](#PseudoClean)
        * [Noborder](#Noborder)
    * [Custom Style](#Custom-Style)
    * [Alignment](#Alignment)
    * [Format](#Format)
    * [Indent](#Indent)
    * [Max width](#Max-width)
    * [Disable](#Disable)
    * [Color](#Color)
* [Features](#Features)
    * [Column name override](#Column-name-override)
    * [Hide a column](#Hide-a-column)
    * [Custom field formatting](#Custom-field-formatting)
    * [Tuple combination](#Tuple-combination)
    * [Object](#Object)
* [Notes](#Notes)
   * [Emoji](#Emoji)

# Usage

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

## Derive information

To be able to use a `Tabled` macro each field should implement `std::fmt::Display`
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

Most of the default types implements the trait out of the box.

```rust
use tabled::Table;
let some_numbers = [1, 2, 3];
let table = Table::new(&some_numbers);
```

# Style

## Styles

A list of ready to use styles.
Styles can be chosen by passing a `Style` argument option.

```rust
let table = Table::new(&data).with(Style::psql());
```

### Default

```,
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

### Psql

```
 name |  designed_by   | invented_year 
------+----------------+---------------
  C   | Dennis Ritchie |     1972      
 Rust | Graydon Hoare  |     2010      
  Go  |    Rob Pike    |     2009      
```

### GithubMarkdown

```
| name |  designed_by   | invented_year |
|------+----------------+---------------|
|  C   | Dennis Ritchie |     1972      |
| Rust | Graydon Hoare  |     2010      |
|  Go  |    Rob Pike    |     2009      |
```

### Pseudo

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

### PseudoClean

```
┌──────┬────────────────┬───────────────┐
│ name │  designed_by   │ invented_year │
├──────┼────────────────┼───────────────┤
│  C   │ Dennis Ritchie │     1972      │
│ Rust │ Graydon Hoare  │     2010      │
│  Go  │    Rob Pike    │     2009      │
└──────┴────────────────┴───────────────┘
```

### Noborder

```
 name    designed_by     invented_year 
  C     Dennis Ritchie       1972      
  Rust   Graydon Hoare       2010      
  Go       Rob Pike          2009      
```

## Custom Style

You can modify existing styles to fits your needs.

```rust
let style = tabled::Style::noborder()
                .frame_bottom(Some(Line::short('*', ' '')))
                .split(Some(Line::short(' ', ' ')))
                .inner(' ');

let table = Table::new(&data).with(style);
```

## Alignment

You can set a horizontal and vertical alignment for a `Header`, `Column`, `Row` or `Full` set of cells.

```rust
Table::new(&data)
    .with(Modify::new(Full)
        .with(Alignment::left())
        .with(Alignment::top())
    );
```

## Format

The `Format` function provides an interface for a modification of cells.

```rust
Table::new(&data)
    .with(Style::psql()),
    .with(Modify::new(Column(..)).with(Format(|s| format!("<< {} >>", s))))
    .with(Modify::new(Row(..1)).with(Format(|s| format!("Head {}", s))));
```

It's also possible to use functions with signature `Fn(&str) -> String` as a formatter.

```rust
Table::new(&data)
    .with(Style::psql()),
    .with(Modify::new(Column(..)).with(|s: &str| format!("<< {} >>", s)))
    .with(Modify::new(Row(..1)).with(str::to_lowercase));
```

IMPORTANT: you may need to specify type in your lambda otherwise compiler may be disagreed to work :)

## Indent

The `Indent` type provides an interface for a left, right, top and bottom indent of cells.

```rust
Table::new(&data).with(Modify::new(Row(1..)).with(Indent::new(1, 1, 0, 2)));
```

## Max width

Using `MaxWidth` type its possible to set a max width of an object.

```rust
Table::new(&data).with(Modify::new(Row(1..)).with(MaxWidth(10, "...")));
```

NOTE: The implementation is not stable in terms of handling colors and emojies 

## Disable

You can remove certain rows or columns from the table.

```rust
Table::new(&data)
    .with(Disable::Row(..1))
    .with(Disable::Column(3..4));
```

## Color

The library doesn't bind you in usage of any color library but to be able to work corectly with color input you should provide a `--features color`.

```rust
Table::new(&data)
    .with(Style::psql())
    .with(Modify::new(Column(..1)).with(Format(|s| s.red().to_string())))
    .with(Modify::new(Column(1..2)).with(Format(|s| s.blue().to_string())))
    .with(Modify::new(Column(2..)).with(Format(|s| s.green().to_string())));
```

![carbon-2](https://user-images.githubusercontent.com/20165848/120526301-b95efc80-c3e1-11eb-8779-0ec48894463b.png)

# Features

## Column name override

You can use a `#[header("")]` attribute to override a column name.

```rust
#[derive(Tabled)]
struct Person {
    #[header("Name")]
    first_name: &'static str,
    #[header("Surname")]
    last_name: &'static str,
}
```

## Hide a column

You can mark filds as hidden in which case they fill be ignored and not be present on a sheet.

A similar affect could be achived by the means of a `Disable` setting.

```rust
struct Person {
   #[header(hidden = true)]
   id: u8,
   #[header("field 2", hidden)]
   number: &'static str,
   name: &'static str,
}
```

## Custom field formatting

`#[derive(Tabled)]` is possible only when all fields implement a `Display` trait.

However, this may be often not the case for example when a field uses the `Option` type.

There's 2 common ways how to solve this:

* Implement `Tabled` trait manually for a type.
* Wrap `Option` to something like DisplayedOption<T>(Option<T>) and implement a Display trait for it.

Or to use an attribute `#[field(display_with = "func")]` for the field. To use it you must provide a function name in a `display_with` parameter.
   
```rust
fn display_option(o: &Option<bool>) -> String {
    match o {
        Some(s) => format!("is valid thing = {}", s), 
        None => format!("is not valid"),
    }
}

#[derive(Tabled)]
pub struct MyRecord {
    pub id: i64,
    #[field(display_with="display_option")]
    pub valid: Option<bool>
}
```

## Tuple combination

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
struct Developer(#[header("name")] &'static str);

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

## Object

You can peak your target for settings using `and` and `not` methods for an object.

```rust
Full.not(Row(..1)) // peak all cells except header
Head.and(Column(..1)).not(Cell(0, 0)) // peak a header and first column except a (0, 0) cell
```

## Notes

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
