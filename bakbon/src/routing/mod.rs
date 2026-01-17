mod registry;
mod router;
mod balancer;

pub use {
    registry::Registry,
    router::Router,
};

use crate::Service;
