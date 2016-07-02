// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sig
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate sig;

unsafe extern "C" fn event(sig: i32) {
    println!("{} was intercepted", sig);
}

fn main() {
    let at: i32 = getpid!();

    signal!(sig::ffi::Sig::USR1, event);
    kill!(at, sig::ffi::Sig::USR1);
}
