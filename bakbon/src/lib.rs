//! BakBon - Infrastructure Microkernel for Distributed Systems
//!
//! BakBon provides protocol-agnostic building blocks for message-driven
//! distributed systems. Build microservices, IoT networks, or blockchain
//! infrastructure with composable, type-safe components.
//!
//! # Quick Example
//!
//! ```rust
//! use {
//!     bakbon::*,
//! };
//!
//! // Create an address.
//! let source = Address::parse("http://client-address.com").unwrap();
//! let destination = "grpc://service.com/echo";
//!
//! // Create a payload from the bytes crate.
//! let payload = Payload::from("Hello there");
//!
//! // Create a message envelope.
//! let message = Envelope::new(source, destination , payload);
//!
//! // EchoService is a custom struct implementing Service from tests/
//! let dst_addr = Address::parse(destination).unwrap();
//! let service = EchoService::new(dst_addr);
//! let registry = Registry::builder()
//!     .register(service)
//!     .build();
//!
//! let mut router = Router::builder()
//!     .registry(registry)
//!     .build();
//!
//! let reply = router.route(message).unwrap();
//! ```
mod core;
mod infra;
mod message;
mod queue;
mod routing;
mod service;

pub use {
    core::{
        Address,
        Error,
        Protocol,
        Result,
    },
    infra::{
        Cache,
        Gateway,
        Middleware,
    },
    message::{
        Envelope,
        Headers,
        Payload,
        Reply,
    },
    queue::Queue,
    routing::{
        Registry,
        Router,
    },
    service::{
        ProcMap,
        Processor,
        Service,
    },
};
