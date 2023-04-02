//---------------------------------------------------------------------------------------------------- Ops
#[macro_export]
/// Flip a [`bool`] in place
///
/// This does a bitwise XOR assign, aka: `bool ^= true`.
///
/// ```rust
/// # use benri::ops::*;
/// let mut b = true;
///
/// flip!(b);
/// assert!(b == false);
///
/// flip!(b);
/// assert!(b == true);
///
/// flip!(b);
/// assert!(b == false);
/// ```
macro_rules! flip {
	($b:expr) => {
		$b ^= true
	}
}
pub use flip;
