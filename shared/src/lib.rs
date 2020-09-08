#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(format_args_capture)]

mod logger;
pub mod prelude;
pub mod views;

pub use askama;
pub use logger::Logger;
