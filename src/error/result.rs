//! Result alias for fallible `TuneX` operations.

use super::TunexError;

/// Convenient result alias for fallible `TuneX` core operations.
pub type TunexResult<T> = Result<T, TunexError>;
