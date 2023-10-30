//! Poller impl for Linux.
//!
//! Uses epoll.

use std::os::fd::{AsFd, AsRawFd};

use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollFlags};

/// Internal impl of a Poller.
///
/// Powered by epoll.
pub struct Waiter {
    poller: Epoll,
}

impl Waiter {
    /// Create a new waiter
    pub fn new() -> Waiter {
        Self {
            poller: Epoll::new(EpollCreateFlags::empty()).unwrap(),
        }
    }

    /// Watch for events on the given file descriptor.
    pub fn register<Fd: AsFd>(&self, fd: Fd) {}
}

impl AsRawFd for Waiter {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        self.poller.0.as_raw_fd()
    }
}
