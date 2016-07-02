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
    unsafe { $crate::ffi::getpid() }
  });
}

/// The `signal` macro receives a signal to
/// a PID program.

#[macro_export]
macro_rules! signal {
    ($sig: expr, $fnc: expr) => ({
        unsafe {
            $crate::set_signal_handler (
                $sig as $crate::ffi::c_int,
                $fnc
            )
        }
    });
}

/// The `signal` macro sends a signal to
/// a PID program.

#[macro_export]
macro_rules! kill {
    ($pid: expr) => ({
        kill!($pid, sig::ffi::Sig::KILL)
    });
    ($pid: expr, $sig: expr) => ({
        match unsafe {
            sig::ffi::kill (
                $pid as $crate::ffi::pid_t,
                $sig as $crate::ffi::c_int,
            )
        } {
            -1 => true,
            _ => false,
        }
    });
}
