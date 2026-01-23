//! Core types for BakBon addressing and errors.
//!
//! - [`Protocol`] models the transport scheme (e.g. "tcp", "http"...).
//! - [`Address`] is a URI-like endpoint built on top of or [`Protocol`].
//! - [`Error`] and [`Result`] are used for BakBon error handling.

mod address;
mod error;
mod protocol;

pub use {
    address::Address,
    error::{
        Error,
        Result,
    },
    protocol::Protocol,
};
