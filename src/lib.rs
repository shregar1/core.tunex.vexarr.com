#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! Core orchestration and abstraction contracts for `TuneX`.

pub mod cancellation;
pub mod channel;
pub mod checkpoint;
pub mod config;
pub mod distributed;
pub mod error;
pub mod events;
pub mod memory;
pub mod pipeline;
pub mod plugin;
pub mod prelude;
pub mod resources;
pub mod runtime;
pub mod scheduler;
pub mod serialization;
pub mod topology;
pub mod traits;
pub mod types;

pub use error::{TunexError, TunexResult};
