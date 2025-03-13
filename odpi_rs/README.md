# odpi_rs - a thin wrapper over Oracle Database Programming Interface for C

Relations between odpic-sys, odpi_rs and oracle crates.

* The odpic-sys crate provides unsafe C functions and types of ODPI-C.
* The odpi_rs crate provides safe intarfaces based on odpic-sys.  
  Its methods correspond one-to-one with ODPI-C functions as possible.
* The oracle crate provides developer-friendly API.

## Goals and non-goals

**Goals**

* Supports all ODPI-C features.
* Provides both sync and async API.

**Non-Goals**

* Provides developer-friendly API including [async iterators].

## Status

In development. Not all features are supported.
Some methods will be changed. Don't use this in production.

## Features flags

The following features can be enable in `Cargo.toml`.

Feature | Descrpition
---|---
`tokio` | Enable async API based on [tokio]
`async-std` | Enable async API based on [async-std]
`smol` | Enable async API based on [smol]

When one of async features is enabled, [ODPI-C functions which may be blocked by network round trips][round_trips]
run in a separate thread provided by the async runtime.

When none of async features are enabled, async API are converted to sync API with the help of the [`maybe_async`] crate.

[Rust]: https://www.rust-lang.org/
[ODPI-C]: https://oracle.github.io/odpi/
[Oracle database]: https://www.oracle.com/database/index.html
[tokio]: https://tokio.rs/
[async-std]: https://async.rs/
[smol]: https://github.com/smol-rs/smol
[round_trips]: https://odpi-c.readthedocs.io/en/latest/user_guide/round_trips.html
[`maybe_async`]: https://docs.rs/maybe-async/latest/maybe_async/
[async iterators]: https://rust-lang.github.io/async-book/part-guide/streams.html
