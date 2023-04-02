//---------------------------------------------------------------------------------------------------- Time
#[macro_export]
/// Expands to [`std::time::Instant::now()`]
macro_rules! now {
	() => {
		::std::time::Instant::now()
	}
}
pub use now;

#[macro_export]
/// Calls [`std::time::Instant::elapsed`] and [`std::time::Duration::as_secs`]
macro_rules! secs {
	($t:expr) => {
		$t.elapsed().as_secs()
	}
}
pub use secs;

#[macro_export]
/// Calls [`std::time::Instant::elapsed`] and [`std::time::Duration::as_secs_f64()`]
macro_rules! secs_f64 {
	($t:expr) => {
		$t.elapsed().as_secs_f64()
	}
}
pub use secs_f64;

#[macro_export]
/// Calls [`std::time::Instant::elapsed`] and [`std::time::Duration::as_millis()`]
macro_rules! millis {
	($t:expr) => {
		$t.elapsed().as_millis()
	}
}
pub use millis;

#[macro_export]
/// Calls [`std::time::Instant::elapsed`] and [`std::time::Duration::as_micros()`]
macro_rules! micros {
	($t:expr) => {
		$t.elapsed().as_micros()
	}
}
pub use micros;

#[macro_export]
/// Calls [`std::time::Instant::elapsed`] and [`std::time::Duration::as_nanos()`]
macro_rules! nanos {
	($t:expr) => {
		$t.elapsed().as_nanos()
	}
}
pub use nanos;

#[macro_export]
/// Get the seconds elapsed [`std::time::UNIX_EPOCH`]
///
/// ```rust
/// # use benri::time::*;
/// let now = unix!();
///
/// assert!(now > 1680460287);
/// ```
///
/// # Panics
/// This calls [`std::time::SystemTime::now`] then [`std::time::SystemTime::duration_since`].
///
/// Since this macro calls `.unwrap()`, this will panic if the system clock is wrong.
macro_rules! unix {
	() => {
		::std::time::SystemTime::now().duration_since(::std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs()
	}
}
pub use unix;

#[macro_export]
/// Get the seconds elapsed since [`std::time::UNIX_EPOCH`]
///
/// ```rust
/// # use benri::time::*;
/// let now = unix_result!().unwrap();
///
/// assert!(now > 1680460287);
/// ```
///
/// # Errors
/// This calls [`std::time::SystemTime::now`] then [`std::time::SystemTime::duration_since`].
///
/// This macro will return a [`Option::None`] if the above function fails.
macro_rules! unix_result {
	() => {
		match ::std::time::SystemTime::now().duration_since(::std::time::SystemTime::UNIX_EPOCH) {
			::std::result::Result::Ok(u)  => ::std::option::Option::Some(u.as_secs()),
			::std::result::Result::Err(_) => ::std::option::Option::None,
		}
	}
}
pub use unix_result;
