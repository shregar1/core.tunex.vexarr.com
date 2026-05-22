//! Cancellation and shutdown contracts.

pub mod guard;
pub mod reason;
pub mod scope;
pub mod token;

pub use guard::ShutdownGuard;
pub use reason::{CancellationReason, DrainPolicy, ShutdownMode};
pub use scope::CancellationScope;
pub use token::CancellationToken;
