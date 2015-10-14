// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/signal
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate signal;

fn event(sig: i32) {
    println!("{} intercepted", sig);
}

fn main() {
    signal!(signal::ffi::Sig::USR1 as i32, event);

    loop {
    }
}
