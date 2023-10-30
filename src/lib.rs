//! Non-blocking IO abstractions for Rust.
//!
//! This crates uses the following Systems APIs:
//!
//! - Linux: epoll
//! - Macos: kqueue

/// TCP, UDP non-blocking abstractions.
pub mod net;

mod poller;

pub use poller::Poll;
