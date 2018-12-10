//! Dynamic smart pointers for abstracting over different ownership models.
//!
//! This library allows you to dynamically abstract over three different kinds of "shared
//! ownership" in Rust: `Rc`, `Arc` and `&'static`. The two pointers defined in this crate can be
//! constructed from multiple different kinds of shared ownership pointers, *dynamically*
//! dispatching their `Clone` and `Drop` implementations.
//!
//! A SharedPointer can be constructed from any of the three pointer types, but does not implement 
//! Send or Sync. A SyncPointer can only be constructed from `Arc` or `&'static`, and is
//! threadsafe. Converting a SharedPointer to a SyncPointer is allowed, but panics if the
//! SharedPointer was constructed from an Rc.
#![deny(warnings, missing_docs)]

mod pointer;
mod shared;
mod sync;
mod tests;

pub use crate::shared::SharedPointer;
pub use crate::sync::SyncPointer;
