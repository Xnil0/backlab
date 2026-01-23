use {
    super::{
        Registry,
        balancer::Balancer,
    },
    crate::{
        Envelope,
        Error,
        Result,
        Reply,
    },
};

#[derive(Default)]
pub struct RouterBuilder {
    registry: Registry,
    balancer: Balancer,
}

impl RouterBuilder {
    pub fn registry(mut self, registry: Registry) -> Self {
        self.registry = registry;
        self
    }

    pub fn balancer(mut self, strategy: &str) -> Self {
        self.balancer = Balancer::new(strategy);
        self
    }

    pub fn build(self) -> Router {
        Router {
            registry: self.registry,
            balancer: self.balancer,
        }
    }
}

/// Routes envelopes to registered services with load balancing.
///
/// Router validates destination addresses and selects service instances
/// using a configurable balancing strategy (round-robin,
/// least-connections, etc.).
///
/// # Example s
///
/// ```rust
/// pub struct NilService(Address); 
/// 
/// impl Service for NilService {
///     fn address(&self) -> &Address { &self.address }
///
///     fn duplicate(&self) -> Box<dyn Service> {
///         let address = self.address.clone();
///         Box::new(Self(address))
///     }
///
///     fn process(&self, message: Envelope) -> Result<Reply> {
///         Ok(None)
///     }
/// }
/// 
/// let service = NilService(address);
/// let mut router = Router::builder()
///     .registry(registry)
///     .balancer("random")
///     .build();
///
/// let reply = router.route(envelope)?;
/// ```
pub struct Router {
    registry: Registry,
    balancer: Balancer,
}

impl Router {
    pub fn builder() -> RouterBuilder { RouterBuilder::default() }

    pub fn route(&mut self, msg: Envelope) -> Result<Reply> {
        let instances = self
            .registry
            .get(msg.destination())
            .ok_or(Error::ServiceNotFound)?;

        let service = self
            .balancer
            .select(instances);

        service.process(msg)
    }

    pub fn registry(&self) -> &Registry { &self.registry }

    pub fn balancing_strategy(&self) -> &str { self.balancer.strategy() }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_default_router() {
        let router = Router::builder().build();
        let list = router.registry.list();

        assert_eq!(router.balancer.strategy(), "round_robin");
        assert!(list.is_empty());
    }

    #[test]
    fn build_router_with_balancer() {
        let strategy = "least_connections";
        let router = Router::builder()
            .balancer(strategy)
            .build();

        let list = router.registry.list();

        assert_eq!(router.balancer.strategy(), strategy);
        assert!(list.is_empty());
    }

    #[test]
    fn build_router_with_registry() {
        let registry = Registry::builder().build();
        let router = Router::builder()
            .registry(registry)
            .build();

        let list = router.registry.list();

        assert!(list.is_empty());
        assert_eq!(router.balancer.strategy(), "round_robin");
    }
}
