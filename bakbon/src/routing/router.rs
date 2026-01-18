use {
    super::{
        Registry,
        balancer::Balancer,
    },
    crate::{
        Envelope,
        MyErr,
        MyResult,
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

pub struct Router {
    registry: Registry,
    balancer: Balancer,
}

impl Router {
    pub fn builder() -> RouterBuilder { RouterBuilder::default() }

    pub fn route(&mut self, msg: Envelope) -> MyResult<Reply> {
        let instances = self
            .registry
            .get(msg.destination())
            .ok_or(MyErr::ServiceNotFound)?;

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
