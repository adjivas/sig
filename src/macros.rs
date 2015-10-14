
/// The `getpid` macro returns the PID of
/// program.

#[macro_export]
macro_rules! signal {
  ($sig: expr, $fnc: expr) => ({
    unsafe {
        signal::ffi::signal($sig as i32, $fnc);
    }
  });
}
