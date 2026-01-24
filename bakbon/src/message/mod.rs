//! Message primitives for BakBon.
//!
//! This module defines the core message types used inside the system:
//!
//! - [`Envelope`] represents an application-level message with payload and
//!   routing  metadata.
//! - [Headers] is a map of string key/value pairs attached to an
//!   [`Envelope`].
//! - [Reply] models a optional reply message returned by
//!   [`Processor`](crate::Processor)
//!
//! High level components such as [`Gateway`](crate::Gateway),
//! [`Service`](crate::Service), [`Router`](crate::Router) build on top of
//! these primitives to exchange data between each other.

mod envelope;
mod route;

pub use envelope::Envelope;
use {
    bytes::Bytes,
    std::collections::HashMap,
};

/// Optional reply message returned by a [`Processor`](crate::Processor)
pub type Reply = Option<Envelope>;

/// Message metadata attached to an [`Envelope`](super::Envelope)
///
/// Headers are arbitrary key/value pairs. Some examples include:
/// - `content-type`
/// - `encoding`
/// - `x-correlation-id`
pub type Headers = HashMap<String, String>;

pub type Payload = Bytes;
