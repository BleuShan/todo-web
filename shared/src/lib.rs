#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(format_args_capture, trait_alias, box_patterns, box_syntax)]

mod logger;
pub mod prelude;
pub mod views;

pub use logger::{
    Logger,
    LoggerConfig,
};
