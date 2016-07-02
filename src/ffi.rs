// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sig
//
// This file may not be copied, modified, or distributed
// except according to those terms.

pub use libc::{pid_t, c_int, getpid, kill};

#[allow(non_snake_case)]
pub mod Sig {
    use libc::{self, c_int};

    pub const HUP    : c_int = libc::SIGHUP;    // Hangup (POSIX).
    pub const INT    : c_int = libc::SIGINT;    // Interrupt (ANSI).
    pub const QUIT   : c_int = libc::SIGQUIT;   // Quit (POSIX).
    pub const ILL    : c_int = libc::SIGILL;    // Illegal instruction (ANSI).
    pub const TRAP   : c_int = libc::SIGTRAP;   // Trace trap (POSIX).
    pub const ABRT   : c_int = libc::SIGABRT;   // Abort (POSIX).
    pub const IOT    : c_int = libc::SIGIOT;    // IOT trap (4.2 BSD).
    pub const BUS    : c_int = libc::SIGBUS;    // BUS error (4.2 BSD).
    pub const FPE    : c_int = libc::SIGFPE;
    pub const KILL   : c_int = libc::SIGKILL;   // Kill unblockable (POSIX).
    pub const USR1   : c_int = libc::SIGUSR1;   // User-defined signal 1 (POSIX).
    pub const SEGV   : c_int = libc::SIGSEGV;   // Segmentation violation (ANSI).
    pub const USR2   : c_int = libc::SIGUSR2;   // User-defined signal 2 (POSIX).
    pub const PIPE   : c_int = libc::SIGPIPE;   // Broken pipe (POSIX).
    pub const ALRM   : c_int = libc::SIGALRM;   // Alarm clock (POSIX).
    pub const TERM   : c_int = libc::SIGTERM;   // Termination (ANSI).
    pub const STKFLT : c_int = libc::SIGSTKFLT; // Stack fault.
    pub const CHLD   : c_int = libc::SIGCHLD;   // Child status has changed (POSIX).
    pub const CONT   : c_int = libc::SIGCONT;   // Continue (POSIX).
    pub const STOP   : c_int = libc::SIGSTOP;   // Stop unblockable (POSIX).
    pub const TSTP   : c_int = libc::SIGTSTP;   // Keyboard stop (POSIX).
    pub const TTIN   : c_int = libc::SIGTTIN;   // Background read from tty (POSIX).
    pub const TTOU   : c_int = libc::SIGTTOU;   // Background write to tttty (POSIX).
    pub const URG    : c_int = libc::SIGURG;    // Urgent condition on socket (4.2 BSD).
    pub const XCPU   : c_int = libc::SIGXCPU;   // CPU limit exceeded (4.2 BSD).
    pub const PROF   : c_int = libc::SIGPROF;   // Profiling alarm clock (4.2 BSD).
    pub const WINCH  : c_int = libc::SIGWINCH;  // Window size change (4.3 BSD Sun).
    pub const POLL   : c_int = libc::SIGPOLL;   // Pollable event occured (System V).
    pub const PWR    : c_int = libc::SIGPWR;    // Power failure restart (System V).
    pub const SYS    : c_int = libc::SIGSYS;    // Bad system call.
}
