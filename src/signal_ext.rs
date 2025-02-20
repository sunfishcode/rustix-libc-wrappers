#[cfg(any(linux_like, solarish, target_os = "hurd"))]
use core::num::NonZeroI32;
#[cfg(all(linux_kernel, feature = "io_uring"))]
use rustix::io_uring::Signal;
#[cfg(all(feature = "process", not(all(linux_kernel, feature = "io_uring"))))]
use rustix::process::Signal;

pub trait SignalExt: private::Sealed + Sized {
    /// `SIGRTMIN + n`—Convert a “real-time” signal offset into a `Signal`.
    ///
    /// This function adds `n` to `rt_min()` to construct the raw signal value.
    /// If the result is greater than `rt_max()`, it returns `None`. This
    /// prevents it from returning `Signal` values which would be reserved by
    /// the libc for internal use.
    #[doc(alias = "SIGRTMIN", alias = "SIGRTMAX")]
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn rt(n: i32) -> Option<Self>;

    /// `SIGRTMIN`—Return the minimum “real-time” signal value.
    ///
    /// This returns the libc `SIGRTMIN` value, which may be different from the
    /// OS kernel `SIGRTMIN` value to allow libc to reserve some signals for
    /// internal use.
    ///
    /// Use [`Signal::rt`] to construct `SIGRTMIN + n` values.
    #[doc(alias = "SIGRTMIN")]
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn rt_min() -> Self;

    /// `SIGRTMAX`—Return the maximum “real-time” signal value.
    ///
    /// This returns the libc `SIGRTMAX` value, which may be different from the
    /// OS kernel `SIGRTMAX` value to allow libc to reserve some signals for
    /// internal use.
    #[doc(alias = "SIGRTMAX")]
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn rt_max() -> Self;

    /// Convert a raw signal number into a `Signal`.
    ///
    /// Returns `None` if the signal value is unrecognized, out of range, or
    /// reserved for libc.
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn from_raw(sig: i32) -> Option<Self>;

    /// Convert a raw non-zero signal number into a `Signal`.
    ///
    /// Returns `None` if the signal value is unrecognized, out of range, or
    /// reserved for libc.
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn from_raw_nonzero(non_zero: NonZeroI32) -> Option<Self>;
}

impl SignalExt for Signal {
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn rt(n: i32) -> Option<Self> {
        let min = libc::SIGRTMIN();
        let sig = min.wrapping_add(n);
        if sig >= min && sig <= libc::SIGRTMAX() {
            // SAFETY: Values at least `SIGRTMIN` will never be zero.
            unsafe { Some(Self::from_raw_unchecked(sig)) }
        } else {
            None
        }
    }

    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn rt_min() -> Self {
        // SAFETY: The libc is telling us this is the value it wants us to use.
        unsafe { Self::from_raw_unchecked(libc::SIGRTMIN()) }
    }

    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn rt_max() -> Self {
        // SAFETY: The libc is telling us this is the value it wants us to use.
        unsafe { Self::from_raw_unchecked(libc::SIGRTMAX()) }
    }

    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn from_raw(sig: i32) -> Option<Self> {
        if let Some(non_zero) = NonZeroI32::new(sig) {
            Self::from_raw_nonzero(non_zero)
        } else {
            None
        }
    }

    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    fn from_raw_nonzero(non_zero: NonZeroI32) -> Option<Self> {
        #[cfg(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "hermit",
            all(
                linux_kernel,
                any(
                    target_arch = "mips",
                    target_arch = "mips32r6",
                    target_arch = "mips64",
                    target_arch = "mips64r6",
                    target_arch = "sparc",
                    target_arch = "sparc64"
                )
            )
        ))]
        const SIGEMT: i32 = Signal::EMT.as_raw();

        let sig = non_zero.get();
        match sig {
            libc::SIGHUP => Some(Self::HUP),
            libc::SIGINT => Some(Self::INT),
            libc::SIGQUIT => Some(Self::QUIT),
            libc::SIGILL => Some(Self::ILL),
            libc::SIGTRAP => Some(Self::TRAP),
            libc::SIGABRT => Some(Self::ABORT),
            libc::SIGBUS => Some(Self::BUS),
            libc::SIGFPE => Some(Self::FPE),
            libc::SIGKILL => Some(Self::KILL),
            #[cfg(not(target_os = "vita"))]
            libc::SIGUSR1 => Some(Self::USR1),
            libc::SIGSEGV => Some(Self::SEGV),
            #[cfg(not(target_os = "vita"))]
            libc::SIGUSR2 => Some(Self::USR2),
            libc::SIGPIPE => Some(Self::PIPE),
            libc::SIGALRM => Some(Self::ALARM),
            libc::SIGTERM => Some(Self::TERM),
            #[cfg(not(any(
                bsd,
                solarish,
                target_os = "aix",
                target_os = "haiku",
                target_os = "hurd",
                target_os = "nto",
                target_os = "vita",
                all(
                    linux_kernel,
                    any(
                        target_arch = "mips",
                        target_arch = "mips32r6",
                        target_arch = "mips64",
                        target_arch = "mips64r6",
                        target_arch = "sparc",
                        target_arch = "sparc64"
                    ),
                )
            )))]
            libc::SIGSTKFLT => Some(Self::STKFLT),
            #[cfg(not(target_os = "vita"))]
            libc::SIGCHLD => Some(Self::CHILD),
            #[cfg(not(target_os = "vita"))]
            libc::SIGCONT => Some(Self::CONT),
            #[cfg(not(target_os = "vita"))]
            libc::SIGSTOP => Some(Self::STOP),
            #[cfg(not(target_os = "vita"))]
            libc::SIGTSTP => Some(Self::TSTP),
            #[cfg(not(target_os = "vita"))]
            libc::SIGTTIN => Some(Self::TTIN),
            #[cfg(not(target_os = "vita"))]
            libc::SIGTTOU => Some(Self::TTOU),
            #[cfg(not(target_os = "vita"))]
            libc::SIGURG => Some(Self::URG),
            #[cfg(not(target_os = "vita"))]
            libc::SIGXCPU => Some(Self::XCPU),
            #[cfg(not(target_os = "vita"))]
            libc::SIGXFSZ => Some(Self::XFSZ),
            #[cfg(not(target_os = "vita"))]
            libc::SIGVTALRM => Some(Self::VTALARM),
            #[cfg(not(target_os = "vita"))]
            libc::SIGPROF => Some(Self::PROF),
            #[cfg(not(target_os = "vita"))]
            libc::SIGWINCH => Some(Self::WINCH),
            #[cfg(not(any(target_os = "haiku", target_os = "vita")))]
            libc::SIGIO => Some(Self::IO),
            #[cfg(not(any(bsd, target_os = "haiku", target_os = "hurd", target_os = "vita")))]
            libc::SIGPWR => Some(Self::POWER),
            libc::SIGSYS => Some(Self::SYS),
            #[cfg(any(
                bsd,
                solarish,
                target_os = "aix",
                target_os = "hermit",
                all(
                    linux_kernel,
                    any(
                        target_arch = "mips",
                        target_arch = "mips32r6",
                        target_arch = "mips64",
                        target_arch = "mips64r6",
                        target_arch = "sparc",
                        target_arch = "sparc64"
                    )
                )
            ))]
            SIGEMT => Some(Self::EMT),
            #[cfg(bsd)]
            libc::SIGINFO => Some(Self::INFO),
            #[cfg(target_os = "freebsd")]
            libc::SIGTHR => Some(Self::THR),
            #[cfg(target_os = "freebsd")]
            libc::SIGLIBRT => Some(Self::LIBRT),
            _ => {
                if sig >= libc::SIGRTMIN() && sig <= libc::SIGRTMAX() {
                    // SAFETY: We just checked that `sig` is in a valid range.
                    unsafe { Some(Self::from_raw_unchecked(sig)) }
                } else {
                    None
                }
            }
        }
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for super::Signal {}
}

#[cfg(test)]
mod tests {
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    use super::*;

    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    #[test]
    fn test_sigrt() {
        assert_eq!(libc::SIGRTMIN(), Signal::rt_min().as_raw());
        assert_eq!(libc::SIGRTMIN(), Signal::rt_min().as_raw_nonzero().get());
        assert_eq!(libc::SIGRTMAX(), Signal::rt_max().as_raw());
        assert_eq!(libc::SIGRTMAX(), Signal::rt_max().as_raw_nonzero().get());
        assert_eq!(Signal::rt(0).unwrap(), Signal::rt_min());
        // POSIX guarantees at least 8 values.
        assert_ne!(Signal::rt(7).unwrap(), Signal::rt_min());
        assert_ne!(Signal::rt(7).unwrap(), Signal::rt_max());
        assert_eq!(Signal::rt(7).unwrap().as_raw(), libc::SIGRTMIN() + 7);
        assert_eq!(
            Signal::rt(7).unwrap().as_raw_nonzero().get(),
            libc::SIGRTMIN() + 7
        );
        assert!(Signal::from_raw(0).is_none());
        for raw in libc::SIGRTMIN()..=libc::SIGRTMAX() {
            assert!(Signal::from_raw(raw).is_some());
        }
        assert!(Signal::from_raw(libc::SIGRTMIN() - 1).is_none());
        assert!(Signal::from_raw(libc::SIGRTMAX() + 1).is_none());
    }
}
