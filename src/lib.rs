//! ## Retry
//!
//! Retry any sync / async function with 

#[cfg(feature = "builder")]
pub mod builder;
#[cfg(feature = "futures")]
pub mod future;
pub(crate) mod tryable;

mod oneshot;
pub mod prelude {
    pub use crate::oneshot::*;
}
