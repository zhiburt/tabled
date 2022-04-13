# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

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
- Constants in `style` module now marked deprecated in regard of const functions as more ergonomic choise.

### Fixed

- Fix rendering of single column table with full style (with horizontal split lines)

## [0.4.2] - 2021-12-30

### Added

- `Builder` structure for dynamic creation of table

### Fixed

- It turns out that in version [0.4.0] `tabled_derive` was inproperly bumped version. So it broke version [0.3].
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

- Fix `fmt::Display` requirments on hidden fields in inline mode
- README.md fix by [@panarch](https://github.com/panarch)
- Refactorings

## [0.3.0] - 2021-09-10

### Added

- `ExpandedDisplay` a different view of data structures. It eases viewing structures with a lot a fields. Proposed by [@sd2k](https://github.com/sd2k)
- `MaxWidth::wrapping` a wrapping mechanism. Setting it will make text wrap to next line after reaching limit.

### Changed

- `MaxWidth` interface changed in regard to support `wrapping`. Now old `MaxWidth` logic can be called by `MaxWidth::trucating`.

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
- Add blank `Tabled` implementaton for String
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

- Improved performance in papergrid; Now it makes 100 allocs on basic example where priviously 400!

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
