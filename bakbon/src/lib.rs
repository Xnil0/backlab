//! BakBon - Infrastructure Microkernel for Distributed Systems
//!
//! BakBon provides protocol-agnostic building blocks for message-driven
//! distributed systems. Build microservices, IoT networks, or blockchain
//! infrastructure with composable, type-safe components.
//!
//! # Modules
//!
//! - `Balancer`: [`Balancer`] for load balancing.
//! - `Core`: [`Address`], [`Protocol`], [`Error`], [`Result`].
//! - `Discovery`: [`Registry`] for service discovery.
//! - `Gateway`: [`Gateway`] for network communication.
//! - `Message`: [`Envelope`], [`Headers`], [`Payload`], [`Reply`].
//! - `Infra`: [`Cache`], [`Middleware`].
//! - `Queue`: [`Queue`] and delivery semantics.
//! - `Routing`: [`Router`] for message routing.
//! - `Service`: [`Service`] and [`Processor`] traits.
//!
//! # Quick Example
//!
//! ```rust
//! use bakbon::prelude::*;
//!
//!     // Create addresses.
//!     let source = Address::parse("http://client-address.com");
//!     let destination = Address::parse("grpc://service.com/echo");
//!     assert!(source.is_ok());
//!     assert!(destination.is_ok());
//!     let src = source.unwrap();
//!     let dst = destination.unwrap();
//!
//!     // Create a payload from the bytes crate.
//!     let payload = Payload::from("Hello there");
//!
//!     // Create a message envelope.
//!     let message = Envelope::new(src, dst.clone() , payload);
//!
//!     // Create a service.
//!     #[derive(Debug)]
//!     struct NilService(Address);
//!
//!     impl Service for NilService {
//!         fn address(&self) -> &Address { &self.0 }
//!         fn duplicate(&self) -> ServiceBox { Box::new(Self(self.0.clone())) }
//!         fn process(&self, msg: Envelope) -> Result<Reply> { Ok(None) }
//!     }
//!
//!     let service = NilService(dst);
//!
//!     // Create a service registry.
//!     let registry = Registry::builder()
//!         .register(service)
//!         .build();
//!
//!     // Create a router.
//!     let mut router = Router::builder()
//!         .registry(registry)
//!         .build();
//!
//!     // Route the created message.
//!     let reply = router.route(message);
//!     assert!(reply.is_ok());
//!     let reply = reply.unwrap();
//!     assert!(reply.is_none());
//! ```
mod balancer;
mod core;
mod gateway;
mod infra;
mod message;
mod queue;
mod registry;
mod router;
mod service;

pub use {
    balancer::Balancer,
    core::{
        Address,
        Error,
        Protocol,
        Result,
    },
    gateway::Gateway,
    infra::{
        Cache,
        Middleware,
    },
    message::{
        Envelope,
        Headers,
        Payload,
        Reply,
    },
    queue::Queue,
    registry::Registry,
    router::Router,
    service::{
        ProcMap,
        Processor,
        Service,
        ServiceBox,
        ServiceMap,
        ServiceVec,
    },
};

pub mod prelude {
    pub use crate::{
        Address,
        Balancer,
        Cache,
        Envelope,
        Error,
        Gateway,
        Headers,
        Middleware,
        Payload,
        ProcMap,
        Processor,
        Protocol,
        Queue,
        Registry,
        Reply,
        Result,
        Router,
        Service,
        ServiceBox,
        ServiceMap,
        ServiceVec,
    };
}
