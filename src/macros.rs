// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sig
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `getpid` macro returns the PID of
/// program.

#[macro_export]
macro_rules! getpid {
  () => ({
    unsafe { sig::ffi::getpid() }
  });
}

/// The `getpid` macro returns the PID of
/// program.

#[macro_export]
macro_rules! signal {
    ($sig: expr, $fnc: expr) => ({
        unsafe {
            sig::ffi::signal (
                $sig as i32,
                $fnc
            )
        }
    });
}

#[macro_export]
macro_rules! kill {
    ($pid: expr) => ({
        kill!($pid, sig::ffi::Sig::KILL)
    });
    ($pid: expr, $sig: expr) => ({
        match unsafe {
            sig::ffi::kill (
                $pid as i32,
                $sig as i32,
            )
        } {
            -1i32 => false,
            _ => true,
        }
    });
}
