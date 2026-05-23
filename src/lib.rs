#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! Core orchestration and abstraction contracts for `TuneX`.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(feature = "std", feature = "serde"))]
pub mod api;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod benchmark;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod cancellation;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod channel;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod checkpoint;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod config;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod distributed;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod error;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod events;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod memory;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod pipeline;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod plugin;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod prelude;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod resources;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod runtime;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod scheduler;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod serialization;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod testing;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod topology;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod traits;
#[cfg(all(feature = "std", feature = "serde"))]
pub mod types;

#[cfg(all(feature = "std", feature = "serde"))]
pub use error::{TunexError, TunexResult};
