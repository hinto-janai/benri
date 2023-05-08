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
/// Calls [`std::time::Instant::elapsed`] and [`std::time::Duration::as_secs_f32()`]
macro_rules! secs_f32 {
	($t:expr) => {
		$t.elapsed().as_secs_f32()
	}
}
pub use secs_f32;

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
/// Expands to [`std::time::Duration::from_secs(1)`]
macro_rules! second {
	() => {
		::std::time::Duration::from_secs(1)
	}
}
pub use second;

#[macro_export]
/// Expands to [`std::time::Duration::from_millis(500)`]
macro_rules! half_second {
	() => {
		::std::time::Duration::from_millis(500)
	}
}
pub use half_second;

#[macro_export]
/// Expands to [`std::time::Duration::from_millis(333)`]
macro_rules! third_second {
	() => {
		::std::time::Duration::from_millis(333)
	}
}
pub use third_second;

#[macro_export]
/// Expands to [`std::time::Duration::from_millis(250)`]
macro_rules! quarter_second {
	() => {
		::std::time::Duration::from_millis(250)
	}
}
pub use quarter_second;

#[macro_export]
/// Expands to [`std::time::Duration::from_secs(60)`]
macro_rules! minute {
	() => {
		::std::time::Duration::from_secs(60)
	}
}
pub use minute;

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
/// # Error
/// This calls [`std::time::SystemTime::now`] then [`std::time::SystemTime::duration_since`].
///
/// Those calls might error if the system clock is wrong.
///
/// On error, this macro will silently return `0`.
macro_rules! unix {
	() => {
		::std::time::SystemTime::now()
			.duration_since(::std::time::SystemTime::UNIX_EPOCH)
			.unwrap_or(::std::time::Duration::ZERO)
			.as_secs()
	}
}
pub use unix;
