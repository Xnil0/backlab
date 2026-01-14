//! Bakbon is a infrastructure microkernel created to help configure and
//! compose different infrastructure components/services such as gateways,
//! load balancers, caching systems, etc. into custom distributed systems.
mod balancer;
mod cache;
mod channel;
mod config;
mod error;
mod gateway;
mod message;
mod middleware;
mod queue;
mod registry;
mod router;
mod service;

pub use {
    channel::{
        Channel,
        Receiver,
        Sender,
    },
    error::{
        MyErr,
        MyResult,
    },
    message::{
        Endpoint,
        Envelope,
        Headers,
        Reply,
    },
    service::{
        ProcMap,
        Processor,
        Service,
    },
};
