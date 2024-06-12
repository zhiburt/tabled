# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.15.0] - 2023-12-20

### Added

- Added `Table::modify` as an option to `Modify` usage, which make cleaner code.
- Added more features to `themes::Theme`.
- Added a new setting `Reverse` to reverse the table.
- Added vertical support for `LineText`.
- Added `ByContet` locator.
- Added `ByCondition` locator.
- Added `Locator` factory for `Location`s.
- Added tabled features to all subprojects to be able to reduce binary size.

# Changed

- Move `Style` to const implementation (it involved changes to `Border`/`BorderColor` and more related subjects).
- Added a basic implementation for a render.
- Added a new `Style` (by [@Brijeshkrishna](https://github.com/Brijeshkrishna)).
- Refactored a `Builder` methods (by [@CouldBeFree](https://github.com/CouldBeFree))
- Changed `ColumnNames` interface.
- Changed `LineText` interface.
- Changed `IntoRecords` interface.
- Reordered `TableOption` interface.
- Renamed `BorderText` to `LineText`.
- Renamed `BorderChar` to `LineChar`.
- Renamed `RawStyle` to `themes::Theme`.
- Renamed `Locator` to `Location`.
- Renamed `papergrid::Color` trait to `ANSIFmt`.
- Renamed `papergrid::StaticColor` trait to `ANSIStr`.
- Renamed `papergrid::ColorBuf` trait to `ANSIBuf`.
- Renamed `color` feature to `ansi`.

# Fixed

- Fix `IndexBuilder::is_empty` function (by @[pjhades](https://github.com/pjhades)).
- Fix a clippy warning in `tabled_derive`.
- Fix spelling mistakes (by [@oliashish](https://github.com/oliashish)).
- Fix spelling mistakes (by [@Kobzol](https://github.com/Kobzol)).

## [0.13.0] - 2023-07-24

### Added

- Added `settings::Dup` to toplicate content.
- Added `settings::themes::ColumnNames` to set text on border (adjusted to cells).
- Added `Xor` implementation for `Color`.
- Added `Colorization` to set colors of table by a pattern.

## [0.14.0] - 2023-08-04

### Added

- Added `TableOption::hint_change` method as an optimization vector.

# Changed

- `ColumnNames` interface was changed.

# Fixed

- `ColumnNames` alignment issue.

## [0.13.0] - 2023-07-24

### Added

- Added `settings::Dup` to toplicate content.
- Added `settings::themes::ColumnNames` to set text on border (adjusted to cells).
- Added `Xor` implementation for `Color`.
- Added `Colorization` to set colors of table by a pattern.

## [0.12.2] - 2023-06-11

### Fixed

- Fixed wrapping algorithm (`tabled::settings::Wrap::keep_words`) for `color` feature.


## [0.12.1] - 2023-06-02

### Changed

- Improved `Wrap` by reducing ansi usage (at the end of a line in some cases).

### Fixed

- Fixed ansi issue related to `ansi-str` by bumping it.

## [0.12.0] - 2023-04-23

### Added

### Changed

- Made a list of changes to `tabled` .
- Made a list of changes to `papergrid` interface.

### Fixed

- Fixed `PoolTable` vertical intersection issue.

## [0.11.0] - 2023-04-12

### Added

- Created `static_table` a macro to build tables at compile time.
- Created `ron_to_table` format conversion library.
- Added `IterTable` a table with a different backend logic.
- Added `CompactTable` a table with a different backend logic.
- Added `PoolTable` a table with a different backend logic.
- Added `tabled(display_with("function", arg1, arg2))` custom arguments support.
- Added `Split` setting for `tabled::Table` (by [@IsaacCloos](https://github.com/IsaacCloos)).
- Added more property based tests (by [@arunma](https://github.com/arunma))
- Added dependabot integration (by [@danieleades](https://github.com/danieleades))
- Added MSRV checks to CI (by [@danieleades](https://github.com/danieleades))
- Documentation improvements (by [@IsaacCloos](https://github.com/IsaacCloos)).
- Documentation improvements (by [@jondot](https://github.com/jondot)).

### Changed

- Made a different interface for `table_to_html`.
- Made a list of changes to `tabled` interface.
- Made a list of changes to `papergrid` interface.
- Made a list of changes to `json_to_table` interface.
- Comparison benchmarks were moved to the `master` branch.

### Fixed

- Fixed `Tabled` macros path issue (by [@zjp-CN](https://github.com/zjp-CN)).
- Fixed header issue in `tabled_to_html` (by [@MRoci](https://github.com/MRoci)).
- Fixed `Rotate::Left/Right` issue.
- Fixed a few issues in `json_to_table`.
- Fixed `Tabled` derive macro issue with `skip`+`order` usage.
- Fixed `Width::wrap().keep_words()` issue.
- Fixed a few clippy warnings (by [@danieleades](https://github.com/danieleades))
- Fixed typos (by Elric Milon).
- Fixed typos (by [@fn-bruce](https://github.com/fn-bruce))
- Fixed typos (by [@fa993](https://github.com/fa993))
- Added missing linence files by [@michel-slm](https://github.com/michel-slm)

## [0.10.0] - 2022-10-18

### Added

- Added `Shadow` configuration to create a margin which would look like a 'shadow'.
- Added `table_to_html` crate to build an `HTML` from a `Table`.
- Added default list of colors to `Color` such as `Color::RED`, `Color::BLUE` etc.
- Added a new style `Style::sharp` by [@wfxr](https://github.com/wfxr).

### Changed

- Bumped `ansi-str` in hope it being more effective.

### Fixed

- docs.rs build issue.

## [0.9.0] - 2022-09-30

### Added

- Macros `row!` and `col!` by [@IsaacCloos](https://github.com/IsaacCloos).
- Added `Panel::vertical` to create a vertically spanned panels.
- Added `Span::row` to create vertically spanned cells.
- Added `Merge` to combine cells with the same content together via `Span`.
- Added `ByColumnName` locator to target columns via name and use it as `Object`.
- Added `VerticalLine` to set custom vertical lines.
- Added `Height` structure to control table/cell height.
- Added `BorderChar` to set a char by an arbitrary offset on a split horizonta/vertical lines.
- Added support for `fmt::*` settings when called with `format`/`println`.
- Created `json_to_table` crate to convert json into table.
- `papergrid` has got a few new functions.

### Changed

- `papergrid` was restructured to be more generic in regard of underlying data types.
- `Table::with` function now doesn't consume the table but called by reference `&mut`.
- impl `TableOption` for `Alignment`.
- impl `TableOption` for `Padding`.
- impl `CellOption` for `String`.
- `Wrap` logic was changed specifically with `color` feature.
- `Wrap` now recognizes hyperlinks (by [@Dan Davison](https://github.com/dandavison)).
- `Tabled` trait now returns `std::borrow::Cow` instead of `String`.

### Fixed

- Fixed issues in `Width::wrap` logic. (one found by [@Dan Davison](https://github.com/dandavison)).


## [0.8.0] - 2022-07-22

### Added

- Created `#[tabled(order = 2)]` attribute to support reordering of fields.
- Created `#[tabled(rename_all = "UPPERCASE")]` attribute to rename columns.
- Created `#[tabled(display_width("some_func", args))]` attribute to being able to call functions with `&self` argument.
- Created `derive` feature to hide `Table` macro behind it.
- Created `Object::intersect` method.
- Created `Object::inverse` method.
- Created `width::Percent` to set width not in absolute values.
- Created `PaddingColor` to colorize `Padding`.
- Created `MarginColor` to colorize `Margin`.
- Added `Priority` strategy for `Width`.
- Added `Style::correct_spans` functions to correct borders when used with spans..
- Added `HighlightColored` a version of `Highlight` which supports coloring.
- Added `ModifyObject` trait to be like `Modify::new`.
- Added `Color`, `BorderColored` and `RawStyleColored` to colorize borders more effectively.
- Added `Style::lines` setter so you it's possible to override not only header.

### Changed

- Performance was improved overall.
- Removed a trailing `\n` from `fmt::Display` output.
- Changed default horizontal alignment from `Center` to `Left`. (generally because it's more effitient in basic case)
- Changed a definition of `CellChange` trait.
- Changed `Modify`; it must be more effitient when used without `Object` methods.
- Changed public methods of `Builder` to use `&mut self` intestead of `self`.
- Changed logic of `Wrap::keep_words`.
- Changed logic of `Truncate::suffix`.
- Removed `Style::header_*` methods.
- Renamed `MaxWidth` to `Width`.
- Renamed `CustomStyle` to `Style`.
- Renamed `StyleConfig` to `RawStyle`.
- Renamed `Style::*_off` methods to `Style::off_*`.

### Fixed

- Fixed a list of issues with coloring
- Fixed `Style` usage, some methods were not influence `Table` in some cases.
- Fixed `\t` handling in `Width` functions.
- Improved documentation.
- Refactorings.

## [0.7.0] - 2022-05-16

### Added

- Created a tabled logo which is used on docs.rs. 
- Added a `Justify` type which sets all columns to the same width.
- `Table::builder` method which can be used to modify the default layout of table built from `Tabled`.
- Added `IndexBuilder` which can be used to transpose the table.
- Added support for `Border` coloring.
- Added `Style::frame` function which returns border.
- Added `Columns::first`, `Columns::last` functions.
- `Sub`, `Add` implementations for `FirstRow` and `LastRow`.
- Added `Style::rounded` style.

### Changed

- Default height set to 0.
- Renamed `TopBorderText` to `BorderText`.
- Removed `object` module from public export.
- Deprecate `object::Full` in regard of `object::Segment::all()`.
- Improved documentation.
- Refactorings.

### Fixed

- Fix `Style` rendering issues.
- Fix rendering when `Panel` and `Span`s are used.
- Fix `Style` rendering of single row and single column tables.

## [0.6.1] - 2022-04-14

### Fixed

- Fix `MinStyle` logic when used with `Span`.
- Fix `MinStyle`, `MaxStyle` logic when used with zero `Span` table.

## [0.6.0] - 2022-04-04

### Added

- Added `MinWidth` type, which changes cell widths in case it's necessary.
- `MinWidth` and `MaxWidth` can be used in combination in order to set a table width.
- Added `Span` support. It cab be used to set a column span.
- Added `Extract` type by @[IsaacCloos](https://github.com/IsaacCloos). It can be used to extract a segment from a table.
- Added an option to set padding character by @[kozmod](https://github.com/kozmod).
- `Margin` by @[kozmod](https://github.com/kozmod). Can be used to set a indent of the whole table.
- Added a support for custom target in `Highlight`.
- `Border` can be used directly to set a cell border.
- A list of format settings: `TrimStrategy`, `AlignmentStrategy`. 
- `\t` processing.
- Added a list of examples.

### Changed

- `#[field]` `#[header]` in `Tabled` macro was renamed to `#[tabled]`.
- A default behaviour of alignment was changed.
- `Indent` renamed to `Padding` by @[kozmod](https://github.com/kozmod).
- A deprecated style constants were removed.
- `object` module was removed from module prelude.
- `Format` functions were refactored. So the interface was changed.
- `Alignment::center_horizontal` was renamed to `Alignment::center`.

### Fixed

- Fix `Style` rendering issues.
- Fix `Panel` rendering issues.
- Fix `Span` rendering issues.

## [0.5.0] - 2022-02-10

### Added

- `CustomStyle` type which now handles modifications of styles.
- `TopBorderText` type which can be used to write a text on a bottom border. 
- `Wrap::keep_words` oprtion in order to not split words in a middle while doing a wrap.
- Add more default styles `Style::dots()`, `Style::re_structured_text()`, `Style::extended()`

### Changed

- `MaxWidth` interface.
- Constants in `style` module now marked deprecated in regard of const functions as more ergonomic choice.

### Fixed

- Fix rendering of single column table with full style (with horizontal split lines)

## [0.4.2] - 2021-12-30

### Added

- `Builder` structure for dynamic creation of table

### Fixed

- It turns out that in version [0.4.0] `tabled_derive` was improperly bumped version. So it broke version [0.3].
  To fix it it was necessary to yank versions [0.4.0] and [0.4.1] and yank tabled_derive [0.1.9].
  It was pointed out by [@Disasm](https://github.com/Disasm).

## [0.4.0] - 2021-12-30

### Added

- `Style` constants like `Style::ASCII`, `Style::PSEUDO`,
- `Highlight` modificator which does highlight of any combination of cells 
- `TableIteratorExt` trait by [@24seconds](https://github.com/24seconds)
- `Concat` modificator by [@senk8](https://github.com/senk8)
- A `Table::shape` method which returns table's size 

### Changed

- `Tabled` trait now requires a `LENGTH` constant.
- `Style` functions were deprecated. 
- Allow border changes for any cell.

### Fixed

- Fix `fmt::Display` requirements on hidden fields in inline mode
- README.md fix by [@panarch](https://github.com/panarch)
- Refactorings

## [0.3.0] - 2021-09-10

### Added

- `ExpandedDisplay` a different view of data structures. It eases viewing structures with a lot a fields. Proposed by [@sd2k](https://github.com/sd2k)
- `MaxWidth::wrapping` a wrapping mechanism. Setting it will make text wrap to next line after reaching limit.

### Changed

- `MaxWidth` interface changed in regard to support `wrapping`. Now old `MaxWidth` logic can be called by `MaxWidth::truncating`.

### Fixed

- Fix an issue that setting `Alignment` was changing `Indent` settings.

## [0.2.3] - 2021-09-06

### Added

- `Rotate` option for grid to rotate grid over 90 degrees.
- `FormatWithIndex` modifier for cells
- `FormatFrom` modifier for cells

### Changed

- Refactoring in `tabled_derive`

### Fixed

- Improve documentation by [@CGMossa](https://github.com/CGMossa)

## [0.2.2] - 2021-07-14

### Added

- Add `Header/Footer` option for grid. 
- Add path (`::`) support in `display_with` attribute.
- Add `Tabled` implementation for constant arrays. 
- Add blank implementation of `TableOption` for `&TableOption` 

## [0.2.1] - 2021-06-23

### Added

- Add `MaxWidth` option for cells
- Add `#[header(inline)]` attribute to inline internal data structures which implement `Tabled` trait
- Add blank `Tabled` implementation for String
- Add `#[header(inline)]` example

### Changed

- Use `ansi-cut` instead of `console` to truncate string
- Switch to `github CI` instead of `travis.ci` because free credit limit was reached

### Fixed

- A sublte refactoring in `tabled_derive`

## [0.2.0] - 2021-06-19

### Added

- Add `Table` type instead of `table!` macros
- Consider lambdas Fn(&str) -> String as format options
- Add basic usage example

### Changed

- Removed `table!` macros.

### Fixed

- Improved performance in papergrid; Now it makes 100 allocs on basic example where previously 400!

## [0.1.4] - 2021-06-07

### Added

- Add a vertical indent support in `Alignment` setting
- Add `Indent` setting for a grid
- Add a support for an attribute `#[field(display_with = "function_name")]` for custom display of a struct fields

### Changed

- `Alignment` interface

### Fixed

- Spelling and grammara mistakes #10 in README.md. Reported by [@atcol](https://github.com/atcol) 
- Panic on emojies #9. Reported by [@nicoulaj](https://github.com/nicoulaj) 

## [0.1.3] - 2021-06-05

### Added

- Add a `Disable` setting for removing rows/column out of the grid
- `Object` combination via `and()`, `not()` methods for targeting more thoroughly
- Modification of default `Style`s
- Add `#[header(hidden)]` attribute to hide variants and fields
