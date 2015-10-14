# Signal

[![Build Status](https://travis-ci.org/adjivas/signal.svg)](https://travis-ci.org/adjivas/signal)
[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)

#### How to build:
```shell
git clone https://github.com/adjivas/signal.git signal && cd signal
cargo build
```

#### How to use:
```shell
cargo run --example usr1
```

#### Cargo'git-Dependencies:
```shell
 Libc
  |
Signal
```

#### Directory-Tree:
```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
|__ example/usr1.rs
\__ src
    |__ ffi.rs
    |__ lib.rs
    \__ macros.rs
```

#### License:
*signal*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/signal/blob/master/LICENSE).
