//---------------------------------------------------------------------------------------------------- Mem
#[macro_export]
/// Moves all inputs in a scope `{}` and immediately exits it.
///
/// This implicitly calls [`std::mem::drop()`] but on an infinite amount of arguments:
/// ```rust,compile_fail
/// # use benri::*;
/// let a = vec![0_u8];
/// let b = vec![0_u8];
/// let c = vec![0_u8];
///
/// drop!(a, b, c);
/// assert!(a == b); // Can't use `a, b, c` anymore.
/// ```
/// This is the same as:
/// ```rust,compile_fail
/// # use benri::*;
/// let a = vec![0_u8];
/// let b = vec![0_u8];
/// let c = vec![0_u8];
///
/// drop! {
///     a,
///     b,
///     c,
/// }
/// assert!(a == b); // Can't use `a, b, c` anymore.
/// ```
/// Which is the same as:
/// ```rust,compile_fail
/// # use benri::*;
/// let a = vec![0_u8];
/// let b = vec![0_u8];
/// let c = vec![0_u8];
///
/// {
///     a;
///     b;
///     c;
/// }
/// assert!(a == b); // Can't use `a, b, c` anymore.
/// ```
/// Which is the same as:
/// ```rust,compile_fail
/// # use benri::*;
/// let a = vec![0_u8];
/// let b = vec![0_u8];
/// let c = vec![0_u8];
///
/// drop(a);
/// drop(b);
/// drop(c);
/// assert!(a == b); // Can't use `a, b, c` anymore.
/// ```
macro_rules! drop {
	($($data:tt),*) => {
		{
			$(
				$data;
			)*
		}
	}
}
pub use drop;

#[macro_export]
/// Calls [`std::mem::size_of()`] on the type given
macro_rules! size {
	($type:ty) => {
		::std::mem::size_of::<$type>()
	}
}
pub use size;

#[macro_export]
/// Calls [`std::mem::swap()`]. This automatically appends `&mut` to `dest`.
macro_rules! replace {
	($dest:ident, $src:ident) => {
		::std::mem::replace(&mut $dest, $src)
	}
}
pub use replace;

#[macro_export]
/// Calls [`std::mem::swap()`]. This automatically appends `&mut` to both inputs.
macro_rules! swap {
	($x:ident, $y:ident) => {
		::std::mem::swap(&mut $x, &mut $y)
	}
}
pub use swap;

#[macro_export]
/// Calls [`std::mem::take()`]. This automatically appends `&mut` on the input.
macro_rules! take {
	($T:ident) => {
		::std::mem::take(&mut $T)
	}
}
pub use take;

#[macro_export]
/// Calls [`std::mem::forget()`]
macro_rules! forget {
	($T:ident) => {
		::std::mem::forget($T)
	}
}
pub use forget;
