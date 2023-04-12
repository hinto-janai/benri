//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};

//---------------------------------------------------------------------------------------------------- Thread
#[macro_export]
/// Sleep the current thread for `x` milliseconds
///
/// ```rust
/// # use benri::time::*;
/// # use benri::thread::*;
/// let now = now!();
///
/// // This sleeps the current thread for 1 second.
/// sleep!(1000);
///
/// assert!(secs!(now) >= 1);
/// ```
macro_rules! sleep {
    ($millis:expr) => {
		::std::thread::sleep(::std::time::Duration::from_millis($millis))
    }
}
pub use sleep;

#[macro_export]
/// Sleep the current thread for `x` seconds
///
/// ```rust
/// # use benri::time::*;
/// # use benri::thread::*;
/// let now = now!();
///
/// // This sleeps the current thread for 1 second.
/// sleep_secs!(1);
///
/// assert!(secs!(now) >= 1);
/// ```
macro_rules! sleep_secs {
    ($seconds:expr) => {
		::std::thread::sleep(::std::time::Duration::from_secs($seconds))
    }
}
pub use sleep_secs;

#[macro_export]
/// Calls [`std::thread::park`]
macro_rules! park {
    () => {
		::std::thread::park()
    }
}
pub use park;

#[macro_export]
/// Get the total available amount of threads as a [`usize`]
///
/// Calls [`std::thread::available_parallelism`] and converts to a [`usize`]
///
/// If the above function returns [`Err`], `1` will be returned.
macro_rules! threads {
    () => {
		match ::std::thread::available_parallelism() {
			Ok(t)  => usize::from(t),
			Err(e) => {
				#[cfg(feature = "log")]
				::log::error!("std::thread::available_parallelism() failed, returning 1!");
				1_usize
			},
		}
    }
}
pub use threads;

#[macro_export]
/// Get `25%` of the available threads as a `usize`.
///
/// This returns `1` on either `1|2|3` threads, else
/// it multiplies by `0.25` and rounds down.
macro_rules! quarter_threads {
    () => {{
		let threads = $crate::threads!();
		match threads {
			// Special cases (low thread-count).
			1|2 => 1,

			// Around 50%.
			_ => (threads as f64 * 0.5).floor() as usize,
		}
    }}
}
pub use quarter_threads;


#[macro_export]
/// Get `50%` the available threads as a `usize`.
///
/// This returns `1` on either `1|2` threads, else
/// it multiplies by `0.5` and rounds down.
macro_rules! half_threads {
    () => {{
		let threads = $crate::threads!();
		match threads {
			// Special cases (low thread-count).
			1|2 => 1,

			// Around 50%.
			_ => (threads as f64 * 0.5).floor() as usize,
		}
    }}
}
pub use half_threads;

#[macro_export]
/// Gets `80%` of the available threads as a `usize`.
///
/// This returns:
/// - `1` on either `1|2` threads
/// - `2` on `3` threads
/// - `3` on `4` threads
///
/// else, it multiplies by `0.8` and rounds down.
macro_rules! most_threads {
    () => {{
		let threads = $crate::threads!();
		match threads {
			// Special cases (low thread-count).
			1|2 => 1,
			3   => 2,
			4   => 3,

			// Around 80%.
			_ => (threads as f64 * 0.8).floor() as usize,
		}
    }}
}
pub use most_threads;
