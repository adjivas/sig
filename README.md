# Signal

[![Crate][crate-badge]][crate] [![docs-badge][]][docs] [![license-badge][]][license] [![travis-badge][]][travis] [![circle-badge][]][circle]

Supports POSIX. Untested on Windows (Not tested).

### Todo:
Solve issues:
* [Support Windows](https://github.com/adjivas/sig/issues/5)
* [Consider using sigaction() instead of signal()](https://github.com/adjivas/sig/issues/3)
* [Signal handlers should be unsafe functions](https://github.com/adjivas/sig/issues/2)

#### How to build:
```shell
git clone https://github.com/adjivas/sig.git signal && cd signal
cargo build
```

#### How to use:
* cargo run --example getpid
* cargo run --example usr1

#### Directory-Tree:
```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
|__ examples
|   |__ usr1.rs
|   \__ getpid.rs
\__ src
    |__ ffi.rs
    |__ lib.rs
    \__ macros.rs
```

#### License:
**Sig**'s code in this repo is licensed under either of:

uses the [APACHE][license-mit] and [MIT][license-apache] license.
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0).
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).

  [license-apache]: https://github.com/adjivas/sig/blob/master/LICENSE-APACHE
  [license-mit]: https://github.com/adjivas/sig/blob/master/LICENSE-MIT

  at your option.

#### Contribution:

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[crate-badge]: https://img.shields.io/crates/v/sig.svg?style=flat-square
[crate]: https://crates.io/crates/sig
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://adjivas.github.io/sig/sig
[license-badge]: https://img.shields.io/crates/l/cublas.svg?style=flat-square
[license]: https://github.com/adjivas/sig/blob/master/README.md#license
[travis-badge]: https://travis-ci.org/adjivas/sig.svg?style=flat-square
[travis]: https://travis-ci.org/adjivas/sig
[circle-badge]: https://circleci.com/gh/adjivas/sig/tree/master.svg?style=svg
[circle]: https://circleci.com/gh/adjivas/sig/tree/master
