# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [v1.0.0] - 2025-05-27

### Added

- Added `Debug` implementations to hashers.
- Added `Clone` implementations to hashers.

### Changed

- Updated the edition from 2015 to 2021.

### Removed

- Removed the `byteorder` dependency.
- Removed the `BuildHasherDefault` type.
  - This type existed because `core::hash::BuildHasherDefault` did not have a const constructor.
  - As of 1.85 `core::hash::BuildHasherDefault` has a const constructor.

## [v0.3.1] - 2022-08-09

### Fixed

- fixed downstream clippy warnings around `murmur3::Hasher`

## [v0.3.0] - 2022-04-29

### Changed

- [breaking-change] Made `Hasher` a subtrait of `core::hash::Hasher`, and
  renamed `finish` to `finish32` to avoid conflicting
- Relaxed some restrictions on `BuildHasherDefault`

### Removed

- [breaking-change] Removed `Hash` in favour of `core::hash::Hash`
- [breaking-change] Removed `BuildHasher` in favour of `core::hash::BuildHasher`

## [v0.2.1] - 2021-04-19

### Added

- implement `Clone`, `PartialEq`, `Eq` for `BuildDefaultHasher`

## [v0.2.0] - 2021-04-19

### Changed

- [breaking-change] `const-fn` feature has been removed
- [breaking-change] MSRV policy has been changed to "only latest stable is guaranteed to be supported"

## [v0.1.2] - 2019-11-14

### Updated

- Included MSRV in the documentation; also added examples for 2015 and 2018
  editions.

## [v0.1.1] - 2019-11-14

### Added

- Implementations for tuples

## v0.1.0 - 2018-04-23

Initial release

[Unreleased]: https://github.com/rust-embedded-community/hash32/compare/v1.0.0...HEAD
[v1.0.0]: https://github.com/rust-embedded-community/hash32/compare/v0.3.1...v1.0.0
[v0.3.1]: https://github.com/rust-embedded-community/hash32/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/rust-embedded-community/hash32/compare/v0.2.1...v0.3.0
[v0.2.1]: https://github.com/rust-embedded-community/hash32/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/rust-embedded-community/hash32/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/rust-embedded-community/hash32/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/rust-embedded-community/hash32/compare/v0.1.0...v0.1.1
