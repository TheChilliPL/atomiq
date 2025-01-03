# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Dates in this file are in `YYYY-MM-DD (HH:MM)` format in [Europe/Warsaw] timezone[^1].

[Europe/Warsaw]: https://en.wikipedia.org/wiki/Europe/Warsaw
[^1]: Standard Time: CET/UTC+1, Daylight Saving Time: CEST/UTC+2

## [Unreleased]

### Added

- `CancellationToken` type that can be used to cancel async operations.

## [0.2.1] - 2025-01-02 14:37

### Added

- New `CancellationToken` struct that can be used to cancel async operations.

## [0.2.0] - 2024-12-30 23:01

### Added

- New `Atomic` struct that can be used as a drop-in replacement for
  `std::sync::atomic::Atomic*` types.
- New `Atomizable` trait that is implemented for all types that can be
  put into `Atomic`.
- New `Atom` trait that is implemented only for atomizable primitives.
- Easy way to make simple structs atomizable by using `#[derive(Atomizable)]`.
- Public prelude for easier usage.
- Exposing `Arc` if `alloc` feature is enabled.

### Changed

- [x] Various atomic types are conditionally implemented for each architecture
  based on the `#[cfg(target_has_atomic)]` attribute.

### Removed

- [x] Old `XAtomic` traits. [BREAKING]
- [x] Support for multiple implementations at once in order to simplify the
  codebase and usage. [BREAKING]

## [0.1.1] - 2024-12-28 14:20

### Added

- Examples in documentation.

### Changed

- Improved documentation.

### Removed

- Unused `log` dependency.

## [0.1.0] - 2024-12-27 03:56

### Added

- First release.
- Common atomic traits: `SimpleAtomic`, `Atomic`, `BoolAtomic`, `IntAtomic`.
- Support for core and Loom implementations with `compat-core` and `compat-loom` crate features.
- Default implementation with `default-core` and `default-loom` crate features.
- Atomic option type.

[unreleased]: https://github.com/TheChilliPL/atomiq/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/TheChilliPL/atomiq/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/TheChilliPL/atomiq/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/TheChilliPL/atomiq/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/TheChilliPL/atomiq/releases/tag/v0.1.0