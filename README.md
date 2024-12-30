![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg?style=for-the-badge)
[![Crates.io version](https://img.shields.io/crates/v/atomiq?style=for-the-badge)](https://crates.io/crates/atomiq)
[![Docs.rs](https://img.shields.io/docsrs/atomiq?style=for-the-badge)](https://docs.rs/atomiq)
![Size](https://img.shields.io/crates/size/atomiq?style=for-the-badge)
[![License](https://img.shields.io/github/license/TheChilliPL/atomiq?style=for-the-badge)](https://github.com/TheChilliPL/atomiq/blob/main/LICENSE)


# atomiq

Convenient tool for atomics in Rust.

Inspired by the `atomig` crate, but with a more flexible design.

## Features

- Common atomic struct `Atomic<T>`.
- Traits like `Atomizable` with a derive macro for easy implementation.
- Standard library/core implementation.
- [Loom][loom] implementation for testing (`loom` crate feature).
- Atomic option type.

[loom]: https://docs.rs/loom
