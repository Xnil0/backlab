//! Bakbon is a infrastructure microkernel created to help configure and
//! compose different infrastructure components/services such as gateways,
//! load balancers, caching systems, etc. into custom distributed systems.
mod balancer;
mod cache;
mod config;
mod error;
mod gateway;
mod message;
mod registry;
mod route;
mod service;

pub use {
    error::{
        MyErr,
        MyResult,
    },
    message::{
        ContentType,
        Encoding,
        Message,
        MessageKind,
        Method,
        Scope,
    },
};
