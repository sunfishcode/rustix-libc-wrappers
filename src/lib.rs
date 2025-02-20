//! rustix-style libc wrappers
//!
//! Some functions in libc can only be implemented in libc.

#![no_std]

// Re-export our public dependency on rustix.
pub use rustix;

// Re-export our public dependency on libc.
pub use libc;

#[cfg(any(all(linux_kernel, feature = "io_uring"), feature = "process"))]
mod signal_ext;

#[cfg(all(linux_kernel, feature = "io_uring"))]
pub mod io_uring {
    pub use super::signal_ext::*;
}
#[cfg(feature = "process")]
pub mod process {
    pub use super::signal_ext::*;
}
