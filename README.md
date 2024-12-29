![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

# atomiq

Convenient tool for atomics in Rust.

Inspired by the `atomig` crate, but with a more flexible design.

## Features

- Common atomic traits and types.
- Standard library/core implementation (default `compat-core` create feature)
- [Loom][loom] implementation for testing (`compat-loom` crate feature)
- Avoiding generics by providing default implementation (either `default-core` (default) or
`default-loom` crate features)
- Atomic option type (requires default implementation)

[loom]: https://docs.rs/loom

License: MPL-2
