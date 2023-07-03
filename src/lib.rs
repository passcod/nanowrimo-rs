//! A crate providing access to the NanoWrimo API, in its public and private forms.
//!
//! Currently, there is no public API. As such, this crate may break at any time. Please
//! direct any issues to [The Issue Tracker](https://github.com/CraftSpider/nanowrimo-rs)

mod enums;
mod kind;
mod utils;

pub mod client;
pub mod data;
pub mod error;

pub use client::NanoClient;
pub use data::*;
pub use enums::*;
pub use error::Error;
pub use kind::NanoKind;
