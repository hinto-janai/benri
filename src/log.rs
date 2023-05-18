//---------------------------------------------------------------------------------------------------- Use
use log::{info,error,warn,trace,debug};

//---------------------------------------------------------------------------------------------------- Log
#[macro_export]
/// Forward input to [`log::info`], prefixed with green `[ OK ]`
macro_rules! ok {
	($($tts:tt)*) => {
		::log::info!("\x1b[1;92m[ OK ]\x1b[0m {}", ::std::format_args!($($tts)*))
	}
}
pub use ok;

#[macro_export]
/// Forward input to [`log::debug`], prefixed with green `[ OK ]`
macro_rules! ok_debug {
	($($tts:tt)*) => {
		::log::debug!("\x1b[1;92m[ OK ]\x1b[0m {}", ::std::format_args!($($tts)*))
	}
}
pub use ok_debug;

#[macro_export]
/// Forward input to [`log::trace`], prefixed with green `[ OK ]`
macro_rules! ok_trace {
	($($tts:tt)*) => {
		::log::trace!("\x1b[1;92m[ OK ]\x1b[0m {}", ::std::format_args!($($tts)*))
	}
}
pub use ok_trace;

#[macro_export]
/// Forward input to [`log::info`], prefixed with white `[SKIP]`
macro_rules! skip {
	($($tts:tt)*) => {
		::log::info!("\x1b[1;97m[SKIP]\x1b[0m {}", ::std::format_args!($($tts)*))
	}
}
pub use skip;

#[macro_export]
/// Forward input to [`log::warn`], prefixed with white `[SKIP]`
macro_rules! skip_warn {
	($($tts:tt)*) => {
		::log::warn!("\x1b[1;97m[SKIP]\x1b[0m {}", ::std::format_args!($($tts)*))
	}
}
pub use skip_warn;

#[macro_export]
/// Forward input to [`log::debug`], prefixed with white `[SKIP]`
macro_rules! skip_debug {
	($($tts:tt)*) => {
		::log::debug!("\x1b[1;97m[SKIP]\x1b[0m {}", ::std::format_args!($($tts)*))
	}
}
pub use skip_debug;

#[macro_export]
/// Forward input to [`log::trace`], prefixed with white `[SKIP]`
macro_rules! skip_trace {
	($($tts:tt)*) => {
		::log::trace!("\x1b[1;97m[SKIP]\x1b[0m {}", ::std::format_args!($($tts)*))
	}
}
pub use skip_trace;

#[macro_export]
/// Forward input to [`log::error!`], prefixed with red `[FAIL]`
macro_rules! fail {
	($($tts:tt)*) => {
		::log::error!("\x1b[1;91m[FAIL]\x1b[0m {}", ::std::format_args!($($tts)*))
	}
}
pub use fail;
