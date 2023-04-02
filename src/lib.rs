//! # `benri`
//! Convenient macros wrapping the standard library.
//!
//! This library provides convenient `macros` around [`std`] functionality.
//!
//! ## Modules
//! Due to how Rust macros are currently exported, all macros
//! will show up in the global namespace: `benri::*`.
//!
//! To use specific types of macros, you can import the module instead:
//! ```rust
//! use benri::time::*;
//!
//! let now = now!();
//! ```
//!
//! ## Feature flags
//! - `log` - Enable [`log`](https://docs.rs/log) usage in certain places
//!
//! ### Example 1 - Flip a bool:
//! ```rust
//! # use benri::ops::*;
//! let mut a = false;
//! flip!(a);
//! assert!(a == true);
//! flip!(a);
//! assert!(a == false);
//! ```
//!
//! ### Example 2 - Get the current `Instant`:
//! ```rust
//! # use benri::time::*;
//! let now = now!();
//!
//! std::thread::sleep(std::time::Duration::from_secs(1));
//!
//! assert!(now.elapsed().as_secs() >= 1);
//!```
//!
//! ### Example 3 - Get elapsed `Instant` time:
//! ```rust
//! # use benri::time::*;
//! let now = now!();
//!
//! std::thread::sleep(std::time::Duration::from_secs(1));
//! assert!(secs!(now)     >= 1);
//! assert!(secs_f64!(now) >= 1.0);
//! assert!(millis!(now)   >= 1000);
//! assert!(micros!(now)   >= 10000);
//! assert!(nanos!(now)    >= 100000);
//!```
//!
//! ### Example 4 - Sleep a thread:
//! ```rust
//! # use benri::time::*;
//! # use benri::thread::*;
//! let now = now!();
//!
//! // This sleeps the current thread for 1 second.
//! sleep!(1000);
//!
//! assert!(secs!(now) >= 1);
//! ```
//!
//! ### Example 5 - Exit _all_ threads:
//! ```rust,ignore
//! std::thread::spawn(|| {
//!     mass_panic!();
//! }).join().unwrap();
//!
//! // The program will has already exited.
//! // The below statement will never be reached.
//! unsafe { /* do bad things */ }
//! ```
//!
//! ### Example 6 - Send/receive a channel message or `mass_panic!()`:
//! This works with any channel (like [`crossbeam_channel`](https://github.com/crossbeam-rs/crossbeam)) that
//! have the same method names as the `std` channels since the inner macro is calling `.send()` and `.recv()`.
//!
//! ```rust
//! # use benri::sync::*;
//! # use benri::thread::*;
//! let (tx, rx) = std::sync::mpsc::channel::<u8>();
//!
//! std::thread::spawn(move || {
//!     send!(tx, 255);
//! }).join().unwrap();
//!
//! assert!(recv!(rx) == 255);
//! ```

#[cfg(feature = "log")]
#[macro_export]
/// Logging functionality using [`log`](https://docs.rs/log)
pub mod log;

/// Operators
pub mod ops;

/// Panic macros
pub mod panic;

/// `std::sync::*`
pub mod sync;

/// `std::thread::*`
pub mod thread;

/// `std::time::*`
pub mod time;
