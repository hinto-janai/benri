# `benri`
[![Windows](https://github.com/hinto-janai/benri/actions/workflows/windows.yml/badge.svg)](https://github.com/hinto-janai/benri/actions/workflows/windows.yml) [![macOS](https://github.com/hinto-janai/benri/actions/workflows/macos.yml/badge.svg)](https://github.com/hinto-janai/benri/actions/workflows/macos.yml) [![Linux](https://github.com/hinto-janai/benri/actions/workflows/linux.yml/badge.svg)](https://github.com/hinto-janai/benri/actions/workflows/linux.yml) [![crates.io](https://img.shields.io/crates/v/benri.svg)](https://crates.io/crates/benri) [![docs.rs](https://docs.rs/benri/badge.svg)](https://docs.rs/benri)

Convenient macros wrapping the standard library.

This library provides convenient `macros!()` around [`std`](https://doc.rust-lang.org/stable/std/) functionality.

## Feature flags
| Flag             | Purpose |
|------------------|---------|
| `log`            | Enable [`log`](https://docs.rs/log) usage in certain places

### Example 1 - Flip a bool:
```rust
let mut a = false;
flip!(a);
assert!(a == true);
flip!(a);
assert!(a == false);
```

### Example 2 - Get the current `Instant`:
```rust
let now = now!();

std::thread::sleep(std::time::Duration::from_secs(1));

assert!(now.elapsed().as_secs() >= 1);
```

### Example 3 - Get elapsed `Instant` time:
```rust
let now = now!();

std::thread::sleep(std::time::Duration::from_secs(1));
assert!(secs!(now)     >= 1);
assert!(secs_f64!(now) >= 1.0);
assert!(millis!(now)   >= 1000);
assert!(micros!(now)   >= 10000);
assert!(nanos!(now)    >= 100000);
```

### Example 4 - Sleep a thread:
```rust
let now = now!();

// This sleeps the current thread for 1 second.
sleep!(1000);

assert!(secs!(now) >= 1);
```

### Example 5 - Exit _all_ threads:
```rust
std::thread::spawn(|| {
    mass_panic!();
}).join().unwrap();

// The program will has already exited.
// The below statement will never be reached.
unsafe { /* do bad things */ }
```

### Example 6 - Send/receive a channel message or `mass_panic!()`:
This works with any channel (like [`crossbeam_channel`](https://github.com/crossbeam-rs/crossbeam)) that
have the same method names as the `std` channels since the inner macro is calling `.send()` and `.recv()`.

```rust
let (tx, rx) = std::sync::mpsc::channel::<u8>();

std::thread::spawn(move || {
    send!(tx, 255);
}).join().unwrap();

assert!(recv!(rx) == 255);
```
