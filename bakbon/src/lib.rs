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
    balancer::LoadBalancer,
    cache::Cache,
    config::Config,
    error::Error,
    gateway::Gateway,
    message::{
        Category,
        Message,
        Scope,
    },
    registry::Registry,
    route::Route,
    service::Service,
};
