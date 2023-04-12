//---------------------------------------------------------------------------------------------------- Hint
#[macro_export]
/// Calls [`std::hint::spin_loop()`]
macro_rules! spin {
	() => {
		::std::hint::spin_loop()
	}
}
pub use spin;

#[macro_export]
/// Calls [`std::hint::black_box()`] on the given function
macro_rules! black_box {
	($function:$tt) => {
		::std::hint::black_box($function)
	}
}
pub use black_box;
