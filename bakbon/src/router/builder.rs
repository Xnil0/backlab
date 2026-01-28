use crate::{
    Balancer,
    Registry,
    Router,
};

/// Builder for constructing a [`Router`] with a [`Registry`] and a
/// [`Balancer`].
///
/// Used to set up a the [`Registry`] and balancing strategy before
/// creating an immutable [`Router`].
#[derive(Default)]
pub struct RouterBuilder {
    registry: Registry,
    balancer: Balancer,
}

impl RouterBuilder {
    /// Sets the service [`Registry`] used by the [`Router`].
    pub fn registry(mut self, registry: Registry) -> Self {
        self.registry = registry;
        self
    }

    /// Sets the balancing `Strategy` by name.
    ///
    /// See `Strategy` for supported values
    /// such as `"round_robin"`, `"least_connections"`, or `"random"`.
    pub fn balancer(mut self, strategy: &str) -> Self {
        self.balancer = Balancer::new(strategy);
        self
    }

    /// Finalizes the builder and returns a [`Router`].
    pub fn build(self) -> Router {
        Router {
            registry: self.registry,
            balancer: self.balancer,
        }
    }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use crate::{
        Router,
        router::RouterBuilder,
    };

    #[test]
    fn default_router_builder() {
        let builder = RouterBuilder::default();
        assert!(builder.registry.0.is_empty());
        assert_eq!(builder.balancer.strategy(), "round_robin");
    }

    #[test]
    fn build_default_router() {
        let builder = Router::builder();
        assert_eq!(builder.balancer.strategy(), "round_robin");
        assert!(
            builder
                .registry
                .list()
                .is_empty()
        );

        let router = builder.build();
        assert_eq!(router.balancing_strategy(), "round_robin");
        assert!(
            router
                .registry()
                .list()
                .is_empty()
        );
    }
}
