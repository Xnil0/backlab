//! Bakbon is a infrastructure microkernel created to help configure and
//! compose different infrastructure components/services such as gateways,
//! load balancers, caching systems, etc. into custom distributed systems.
mod core;
mod infra;
mod message;
mod queue;
mod routing;
mod service;

pub use {
    core::{
        Address,
        MyErr,
        MyResult,
        Protocol,
    },
    infra::{
        Cache,
        Gateway,
        Middleware,
    },
    message::{
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
