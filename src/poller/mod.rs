//! Non-blocking IO abstractions for Rust.

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::Waiter;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::Waiter;

use std::os::fd::AsRawFd;

/// An IO Event Poller.
///
/// It is powered by epoll on linux and kqueue on OS X.
pub struct Poll {
    /// internal epoll instance.
    waiter: Waiter,
}

impl Poll {
    /// Create a new instance of `Poll`.
    pub fn new() -> Self {
        Self {
            waiter: Waiter::new(),
        }
    }
}

impl AsRawFd for Poll {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        self.waiter.as_raw_fd()
    }
}
