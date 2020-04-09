// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sig
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # Sig
//!
//! [![Crate][crate-badge]][crate] [![license-badge][]][license] [![travis-badge][]][travis] [![circle-badge][]][circle]
//!
//! [crate-badge]: https://img.shields.io/badge/crates.io-v0.1.1-orange.svg?style=flat-square
//! [crate]: https://crates.io/crates/sig
//! [license-badge]: https://img.shields.io/crates/l/cublas.svg?style=flat-square
//! [license]: https://github.com/adjivas/sig/blob/master/README.md#license
//! [travis-badge]: https://travis-ci.org/adjivas/sig.svg?style=flat-square
//! [travis]: https://travis-ci.org/adjivas/sig
//! [circle-badge]: https://circleci.com/gh/adjivas/sig/tree/master.svg?style=svg
//! [circle]: https://circleci.com/gh/adjivas/sig/tree/master

extern crate libc;

#[macro_use]
mod macros;
pub mod ffi;

use libc::sighandler_t;
#[cfg(unix)] use libc::{sigaction, sigfillset};
#[cfg(windows)] use libc::signal;
use std::{mem, ptr};

#[cfg(unix)]
#[inline]
pub unsafe fn set_signal_handler(sig: ffi::c_int,
                                 handler: unsafe extern "C" fn(ffi::c_int)) {
    let mut sigset = mem::uninitialized();

    // Block all signals during the handler. This is the expected behavior, but
    // it's not guaranteed by `signal()`.
    if sigfillset(&mut sigset) != -1 {
        // Done because sigaction has private members.
        // This is safe because sa_restorer and sa_handlers are pointers that
        // might be null (that is, zero).
        let mut action: sigaction = mem::zeroed();

        // action.sa_flags = 0;
        action.sa_mask = sigset;
        action.sa_sigaction = handler as sighandler_t;

        sigaction(sig, &action, ptr::null_mut());
    }
}

#[cfg(windows)]
#[inline]
pub unsafe fn set_signal_handler(sig: ffi::c_int,
                                 handler: unsafe extern "C" fn(ffi::c_int)) {
    signal(sig, handler as sighandler_t);
}
