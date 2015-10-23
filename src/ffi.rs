// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sig
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Sig {
    HUP    =  1, // Hangup (POSIX).
    INT    =  2, // Interrupt (ANSI).
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

#[allow(improper_ctypes)]
extern "C" {
    pub fn signal(sig: i32, handler: fn(i32)) -> fn(i32);
    pub fn kill(pid: i32, sig: i32) -> i32;
    pub fn getpid() -> i32;
}
