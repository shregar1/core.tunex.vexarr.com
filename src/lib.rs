#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! Core orchestration and abstraction contracts for `TuneX`.

pub mod config;
pub mod error;
pub mod prelude;
pub mod traits;
pub mod types;

pub use error::{TunexError, TunexResult};
