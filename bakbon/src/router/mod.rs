//! Routing layer: registry of services plus a pluggable load balancer
//! driving the [`Router`], which turns [`Envelope`] destinations into
//! concrete [`Service`](crate::Service) instances.

mod builder;

pub use builder::RouterBuilder;

use crate::{
    Balancer,
    Envelope,
    Error,
    Registry,
    Reply,
    Result,
};

/// Routes [`Envelope`]s to registered [`Service`](crate::Service) with
/// load balancing.
///
/// The `Router` looks up [`Service`](crate::Service) instances in the
/// [`Registry`] based on the [`Envelope`] destination
/// [`Address`](crate::Address) string representation, then delegates
/// instance selection to the internal [`Balancer`] before calling
/// [`process()`](crate::Service::process) on the chosen
/// instance.
///
/// # Examples
///
/// ```ignore
/// let mut router = Router::builder()
///     .registry(registry)
///     .balancer("least_connections")
///     .build();
///
/// let reply = router.route(envelope)?;
/// ```
pub struct Router {
    registry: Registry,
    balancer: Balancer,
}

impl Router {
    /// Returns a new [`RouterBuilder`] with default configuration.
    pub fn builder() -> RouterBuilder { RouterBuilder::default() }

    /// Routes a [`message`](Envelope) to a registered
    /// [`Service`](crate::Service) and returns its [`Reply`].
    ///
    /// This method:
    /// 1. Looks up instances for
    ///    [`msg.destination()`](Envelope::destination) in the
    ///    [`Registry`].
    /// 2. Uses the [`Balancer`] to select one instance.
    /// 3. Calls [`process()`](crate::Service::process) on that instance.
    ///
    /// Returns [`Error::ServiceNotFound`] if no
    /// [`Service`](crate::Service) is registered under the destination
    /// [`Address`](crate::Address) string representation.
    pub fn route(&mut self, msg: Envelope) -> Result<Reply> {
        let instances = self
            .registry
            .get(
                msg.destination()
                    .to_string()
                    .as_str(),
            )
            .ok_or(Error::ServiceNotFound)?;

        let service = self
            .balancer
            .select(instances)?;

        service.process(msg)
    }

    /// Returns a reference to the underlying [`Service`](crate::Service)
    /// [`Registry`].
    pub fn registry(&self) -> &Registry { &self.registry }

    /// Returns the active balancing `Strategy` as a string.
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
