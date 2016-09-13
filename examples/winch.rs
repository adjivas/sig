// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sig
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate sig;

static mut signal: Option<i32> = None;

unsafe extern "C" fn event(sig: i32) {
    signal = Some(sig);
}

fn main() {
    signal!(sig::ffi::Sig::WINCH, event);
    unsafe {
        loop {
            match signal {
                Some(sig) => {
                    println!("{}", sig);
                    signal = None;
                }
                None => {},
            }
        }
    }
}
