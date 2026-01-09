//! Bakbon is a infrastructure microkernel created to help configure and
//! compose different infrastructure components/services such as gateways,
//! load balancers, caching systems, etc. into custom distributed systems.
mod balancer;
mod cache;
mod channel;
mod config;
mod envelope;
mod error;
mod gateway;
mod middleware;
mod registry;
mod route;
mod service;

pub use {
    channel::{
        Channel,
        Endpoint,
        Receiver,
        Sender,
    },
    envelope::{
        Envelope,
        Message,
    },
    error::{
        MyErr,
        MyResult,
    },
};
