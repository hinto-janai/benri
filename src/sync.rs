//---------------------------------------------------------------------------------------------------- Use
use std::sync::atomic::*;
use std::sync::mpsc::{
	Sender,
	Receiver,
};
use std::sync::{
	Arc,
	Mutex,
	RwLock,
};
use crate::mass_panic;

//---------------------------------------------------------------------------------------------------- Creation.
#[macro_export]
/// Calls an [`Arc::new`]
///
/// This is the same as:
/// ```rust
/// let a = std::sync::Arc::new(0);
/// ```
macro_rules! arc {
	($b:tt) => {
		::std::sync::Arc::new($b)
	}
}
pub use arc;

#[macro_export]
/// Calls [`Arc::new`] and [`Mutex::new`]
///
/// This is the same as:
/// ```rust
/// let a = std::sync::Arc::new(std::sync::Mutex::new(0));
/// ```
macro_rules! arc_mut {
	($b:tt) => {
		::std::sync::Arc::new(::std::sync::Mutex::new($b))
	}
}
pub use arc_mut;

#[macro_export]
/// Calls [`Arc::new`] and [`RwLock::new`]
///
/// This is the same as:
/// ```rust
/// let a = std::sync::Arc::new(std::sync::RwLock::new(0));
/// ```
macro_rules! arc_rw {
	($b:tt) => {
		::std::sync::Arc::new(::std::sync::RwLock::new($b))
	}
}
pub use arc_rw;

//---------------------------------------------------------------------------------------------------- Atomics
#[macro_export]
/// Flip an [`AtomicBool`] in place with [`Ordering::SeqCst`].
///
/// ```rust
/// # use std::sync::atomic::*;
/// # use benri::sync::*;
/// let mut a = AtomicBool::new(false);
///
/// atomic_flip!(a);
/// assert!(atomic_load!(a) == true);
///
/// atomic_flip!(a);
/// assert!(atomic_load!(a) == false);
/// ```
macro_rules! atomic_flip {
	($b:expr) => {
		$b.fetch_xor(true, ::std::sync::atomic::Ordering::SeqCst)
	}
}
pub use atomic_flip;

#[macro_export]
/// `load` a [`std::sync::atomic`] type with [`Ordering::SeqCst`]
///
/// ```rust
/// # use std::sync::atomic::*;
/// # use benri::sync::*;
/// let a = AtomicUsize::new(5);
///
/// assert!(atomic_load!(a) == 5);
/// ```
macro_rules! atomic_load {
	($b:expr) => {
		$b.load(::std::sync::atomic::Ordering::SeqCst)
	}
}
pub use atomic_load;

#[macro_export]
/// `fetch_add()` a [`std::sync::atomic`] type with [`Ordering::SeqCst`]
///
/// ```rust
/// # use std::sync::atomic::*;
/// # use benri::sync::*;
/// let a = AtomicUsize::new(5);
/// atomic_add!(a, 5);
///
/// assert!(atomic_load!(a) == 10);
/// ```
macro_rules! atomic_add {
	($atomic:expr, $i:expr) => {
		$atomic.fetch_add($i, ::std::sync::atomic::Ordering::SeqCst)
	}
}
pub use atomic_add;

#[macro_export]
/// `fetch_sub()` a [`std::sync::atomic`] type with [`Ordering::SeqCst`]
///
/// ```rust
/// # use std::sync::atomic::*;
/// # use benri::sync::*;
/// let a = AtomicUsize::new(5);
/// atomic_sub!(a, 5);
///
/// assert!(atomic_load!(a) == 0);
/// ```
macro_rules! atomic_sub {
	($atomic:expr, $i:expr) => {
		$atomic.fetch_sub($i, ::std::sync::atomic::Ordering::SeqCst)
	}
}
pub use atomic_sub;

#[macro_export]
/// `store()` a [`std::sync::atomic`] type with [`Ordering::SeqCst`]
///
/// ```rust
/// # use std::sync::atomic::*;
/// # use benri::sync::*;
/// let a = AtomicUsize::new(0);
/// atomic_store!(a, 123);
///
/// assert!(atomic_load!(a) == 123);
/// ```
macro_rules! atomic_store {
	($atomic:expr, $i:expr) => {
		$atomic.store($i, ::std::sync::atomic::Ordering::SeqCst)
	}
}
pub use atomic_store;

//---------------------------------------------------------------------------------------------------- Channels
#[macro_export]
/// `.send()` a channel message and `.unwrap()`
///
/// ```rust
/// # use benri::thread::*;
/// # use benri::sync::*;
/// let (tx, rx) = std::sync::mpsc::channel::<u8>();
///
/// std::thread::spawn(move || {
///     send!(tx, 255);
/// });
///
/// sleep!(1000);
/// assert!(recv!(rx) == 255);
/// ```
macro_rules! send {
	($channel:expr, $($msg:tt)*) => {
		$channel.send($($msg)*).unwrap()
	}
}
pub use send;

#[macro_export]
/// `.recv()` a channel message and `.unwrap()`
///
/// ```rust
/// # use benri::thread::*;
/// # use benri::sync::*;
/// let (tx, rx) = std::sync::mpsc::channel::<u8>();
///
/// std::thread::spawn(move || {
///     send!(tx, 255);
/// });
///
/// sleep!(1000);
/// assert!(recv!(rx) == 255);
/// ```
macro_rules! recv {
	($channel:expr) => {
		$channel.recv().unwrap()
	}
}
pub use recv;

#[macro_export]
/// `.send()` a channel message, [`mass_panic!`] on failure
///
/// ```rust
/// # use benri::thread::*;
/// # use benri::sync::*;
/// let (tx, rx) = std::sync::mpsc::channel::<u8>();
///
/// std::thread::spawn(move || {
///     send_or_mass!(tx, 255);
/// });
///
/// sleep!(1000);
/// assert!(recv_or_die!(rx) == 255);
/// ```
macro_rules! send_or_mass {
	($channel:expr, $($msg:tt)*) => {
		if let ::std::result::Result::Err(e) = $channel.send($($msg)*) {
			#[cfg(feature = "log")]
			::log::error!("THREAD PANIC - FAILED TO SEND: {}", e);
			#[cfg(not(feature = "log"))]
			::std::eprintln!("THREAD PANIC - FAILED TO SEND: {}", e);
			$crate::mass_panic!(e);
		}
	}
}
pub use send_or_mass;

#[macro_export]
/// `.recv()` a channel message, [`mass_panic!`] on failure
///
/// ```rust
/// # use benri::thread::*;
/// # use benri::sync::*;
/// let (tx, rx) = std::sync::mpsc::channel::<u8>();
///
/// std::thread::spawn(move || {
///     send_or_mass!(tx, 255);
/// });
///
/// sleep!(1000);
/// assert!(recv_or_die!(rx) == 255);
/// ```
macro_rules! recv_or_mass {
	($channel:expr) => {
		match $channel.recv() {
			::std::result::Result::Ok(msg) => msg,
			::std::result::Result::Err(e)  => {
				#[cfg(feature = "log")]
				::log::error!("THREAD PANIC - FAILED TO RECEIVE: {}", e);
				#[cfg(not(feature = "log"))]
				::std::eprintln!("THREAD PANIC - FAILED TO RECEIVE: {}", e);
				$crate::mass_panic!(e);
			},
		}
	}
}
pub use recv_or_mass;

//---------------------------------------------------------------------------------------------------- Mutex/RwLock
#[macro_export]
/// `.lock()` a [`Mutex`] and `.unwrap()`
///
/// ```rust
/// # use std::sync::Mutex;
/// # use benri::sync::*;
/// let a = Mutex::new(0);
///
/// assert!(*lock!(a) == 0);
/// ```
macro_rules! lock {
	($lock:expr) => {
		$lock.lock().unwrap()
	}
}
pub use lock;

#[macro_export]
/// `.read()` a [`RwLock`] and `.unwrap()`
///
/// ```rust
/// # use std::sync::RwLock;
/// # use benri::sync::*;
/// let a = RwLock::new(0);
///
/// assert!(*lockr!(a) == 0);
/// ```
macro_rules! lockr {
	($lock:expr) => {
		$lock.read().unwrap()
	}
}
pub use lockr;

#[macro_export]
/// `.write()` to a [`RwLock`] and .`unwrap()`
///
/// ```rust
/// # use std::sync::RwLock;
/// # use benri::sync::*;
/// let a = RwLock::new(0);
/// *lockw!(a) = 1;
///
/// assert!(*lockw!(a) == 1);
/// ```
macro_rules! lockw {
	($lock:expr) => {
		$lock.write().unwrap()
	}
}
pub use lockw;
