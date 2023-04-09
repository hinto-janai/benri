//---------------------------------------------------------------------------------------------------- Panic
#[macro_export]
/// Terminate _all_ threads.
///
/// If the `log` feature flag is enabled, this emits a
/// message with [`log::error`] before exiting.
macro_rules! mass_panic {
	($($tts:tt)*) => {{
		// Log.
		#[cfg(feature = "log")]
		{
			::log::error!("");
			::log::error!("");
			::log::error!("");
			::log::error!("----- MASS PANIC: {} @ {} -----", file!(), line!());
			::log::error!("{}", $($tts)*);
			::log::error!("----- EXITING ALL THREADS -----");
			::log::error!("");
			::log::error!("");
			::log::error!("");
		}
		#[cfg(not(feature = "log"))]
		{
			::std::eprintln!("");
			::std::eprintln!("");
			::std::eprintln!("");
			::std::eprintln!("----- MASS PANIC: {} @ {} -----", file!(), line!());
			::std::eprintln!("{}", $($tts)*);
			::std::eprintln!("----- EXITING ALL THREADS -----");
			::std::eprintln!("");
			::std::eprintln!("");
			::std::eprintln!("");
		}

		// Exit all threads.
		::std::process::exit(111)
	}}
}
pub use mass_panic;

#[macro_export]
/// `match` a [`Result`], [`mass_panic!`] on [`Result::Err`]
macro_rules! unwrap_or_mass {
	($var:expr) => {
		match $var {
			::std::result::Result::Ok(o)  => o,
			::std::result::Result::Err(e) => $crate::panic::mass_panic!(e),
		}
	}
}
pub use unwrap_or_mass;
