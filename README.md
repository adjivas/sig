# Signal

[![Crate][crate-badge]][crate] [![docs-badge][]][docs] [![license-badge][]][license] [![travis-badge][]][travis] [![circle-badge][]][circle]

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
*sig*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license][license].

[crate-badge]: https://img.shields.io/badge/crates.io-v0.1.0-orange.svg?style=flat-square
[crate]: https://crates.io/crates/sig
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://adjivas.github.io/sig/sig
[license-badge]: http://img.shields.io/badge/license-GPLv3-blue.svg?style=flat-square
[license]: https://github.com/adjivas/sig/blob/master/LICENSE
[travis-badge]: https://travis-ci.org/adjivas/sig.svg?style=flat-square
[travis]: https://travis-ci.org/adjivas/sig
[circle-badge]: https://circleci.com/gh/adjivas/sig/tree/master.svg?style=svg
[circle]: https://circleci.com/gh/adjivas/sig/tree/master
