// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/signal
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]
#![allow(dead_code)]
extern crate libc;

pub type sighandler_t = fn(c_int);
pub type c_int = libc::types::os::arch::c95::c_int;
pub type pid_t = libc::types::os::arch::posix88::pid_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Sig {
    HUP    =  0, // Hangup (POSIX).
    INT    =  1, // Interrupt (ANSI).
    QUIT   =  3, // Quit (POSIX).
    ILL    =  4, // Illegal instruction (ANSI).
    TRAP   =  5, // Trace trap (POSIX).
    IOT    =  6, // Abort (POSIX).
    BUS    =  7, // IOT trap (4.2 BSD).
    FPE    =  8, // BUS error (4.2 BSD).
    KILL   =  9, // Kill, unblockable (POSIX).
    USR1   = 10, // User-defined signal 1 (POSIX).
    SEGV   = 11, // Segmentation violation (ANSI).
    USR2   = 12, // User-defined signal 2 (POSIX).
    PIPE   = 13, // Broken pipe (POSIX).
    ALRM   = 14, // Alarm clock (POSIX).
    TERM   = 15, // Termination (ANSI).
    STKFLT = 16, // Stack fault.
    CHLD   = 17, // Child status has changed (POSIX).
    CONT   = 18, // Continue (POSIX).
    STOP   = 19, // Stop, unblockable (POSIX).
    TSTP   = 20, // Keyboard stop (POSIX).
    TTIN   = 21, // Background read from tty (POSIX).
    TTOU   = 22, // Background write to tttty (POSIX).
    URG    = 23, // Urgent condition on socket (4.2 BSD).
    XCPU   = 24, // CPU limit exceeded (4.2 BSD).
    XFS2   = 25, // File size limit exceeded (4.2 BSD).
    VTALRM = 26, // Virtual alarm clock (4.2 BSD).
    PROF   = 27, // Profiling alarm clock (4.2 BSD).
    WINCH  = 28, // Window size change (4.3 BSD, Sun).
    POLL   = 29, // Pollable event occured (System V).
    PWR    = 30, // Power failure restart (System V).
    SYS    = 31, // Bad system call.
    RTMIN  = 32,
    RTMAX  = 64,
}

extern "C" {
    pub fn signal(sig: c_int, handler: sighandler_t) -> sighandler_t;
    pub fn kill(pid: pid_t, sig: c_int) -> c_int;
}
