# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.5] - 2021-06-19

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
