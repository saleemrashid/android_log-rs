//! A logger which writes to the Android logging subsystem. It must be compiled
//! with the Android NDK in order to link to `liblog`.
//!
//! ## Example
//!
//! ```
//! #[macro_use] extern crate log;
//! extern crate android_log;
//!
//! fn main() {
//!     android_log::init("MyApp");
//!
//!     trace!("Initialized Rust");
//!     debug!("Address is {:p}", main as *const ());
//!     info!("Did you know? {} = {}", "1 + 1", 2);
//!     warn!("Don't log sensitive information!");
//!     error!("Nothing more to say");
//! }
//! ```
//!
//! ```{.bash}
//! $ logcat
//! 12-25 12:00:00.000  1234  1234 V MyApp: Initialized Rust
//! 12-25 12:00:00.000  1234  1234 D MyApp: Address is 0xdeadbeef
//! 12-25 12:00:00.000  1234  1234 I MyApp: Did you know? 1 + 1 = 2
//! 12-25 12:00:00.000  1234  1234 W MyApp: Don't log sensitive information!
//! 12-25 12:00:00.000  1234  1234 E MyApp: Nothing more to say
