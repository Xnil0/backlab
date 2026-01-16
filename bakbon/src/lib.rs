//! Bakbon is a infrastructure microkernel created to help configure and
//! compose different infrastructure components/services such as gateways,
//! load balancers, caching systems, etc. into custom distributed systems.
mod balancer;
mod cache;
mod error;
mod gateway;
mod message;
mod middleware;
mod protocol;
mod queue;
mod routing;
mod service;

pub use {
    error::{
        MyErr,
        MyResult,
    },
    gateway::Gateway,
    message::{
        Address,
        Envelope,
        Headers,
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
