# Signal

[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)
[![Build Status](https://travis-ci.org/adjivas/sig.svg)](https://travis-ci.org/adjivas/sig)
[![Circle CI](https://circleci.com/gh/adjivas/sig/tree/master.svg?style=svg)](https://circleci.com/gh/adjivas/sig/tree/master)

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
*sig*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/sig/blob/master/LICENSE).
