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
        core::Address,
    },
};

#[derive(Default)]
struct RouterBuilder {
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

    pub fn route(&mut self, message: Envelope) -> MyResult<Reply> {
        let address: Address = message
            .destination()
            .try_into()?;

        let instances = self
            .registry
            .get(address.authority())
            .ok_or(MyErr::ServiceNotFound)?;

        let service = self
            .balancer
            .select(instances);

        service.process(message)
    }
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
