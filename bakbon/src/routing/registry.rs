use {
    super::Service,
    crate::{
        Error,
        Result,
    },
    std::collections::HashMap,
};

type Services = Vec<Box<dyn Service>>;
type ServiceMap = HashMap<String, Services>;

/// Builder for construction of registry of services.
///
/// `RegistryBuilder` let's you register one or more [`Service`] instances
/// and then freeze the configuration into an immutable [`Registry`].
#[derive(Default)]
pub struct RegistryBuilder(ServiceMap);

impl RegistryBuilder {
    /// Registers a new service instance in the builder.
    ///
    /// [`Service`]s are grouped in their adrress string representation.
    /// Multiple instances with the same address can be added later via
    /// [`Registry::add_instance()`]
    pub fn register(mut self, service: impl Service + 'static) -> Self {
        let name = service.address().to_string();

        self.0
            .insert(name, vec![Box::new(service)]);

        self
    }

    /// Finalizes the builder and returns an immutable [`Registry`].
    pub fn build(self) -> Registry { Registry(self.0) }
}

/// Immutable registry of services keyed by address.
///
/// The `Registry` stores one or more instances for each logical
/// [`Service`]. It is used by [`Router`](crate::Router) to lookup
/// candidate instances before delegating selection go
/// [`Balancer`](super::balancer::Balancer).
#[derive(Default)]
pub struct Registry(pub(super) ServiceMap);

impl Registry {
    /// Returns an empty [`RegistryBuilder`].
    pub fn builder() -> RegistryBuilder { RegistryBuilder::default() }

    /// Adds a new instance for an existing service address.
    ///
    /// The new instance is created by calling [`Service::duplicate()`] on
    /// the last registered instance. Returns [`Error::ServiceNotFound`] if
    /// the address is unknown.
    pub fn add_instance(&mut self, address: &str) -> Result<()> {
        let instances = self
            .0
            .get_mut(address)
            .ok_or(Error::ServiceNotFound)?;

        let new_instance = instances
            .last()
            .unwrap()
            .duplicate();

        instances.push(new_instance);
        Ok(())
    }

    /// Returns a list of all instances registered for a given address
    /// string representation, if any.
    pub fn get(&self, address: &str) -> Option<&Vec<Box<dyn Service>>> {
        self.0
            .get(address)
            .map(|s| s.as_ref())
    }

    /// Returns a list of all registered [`Service`] addresses string
    /// representation.
    pub fn list(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|k| k.as_str())
            .collect()
    }
}

// impl From<Vec<Box<dyn Service>>> for Registry {
//     /// Builds a Registry from a flat list of service instances.
//     ///
//     /// Instances are grouped by their address string representation.
//     fn from(services: Vec<Box<dyn Service>>) -> Self {
//         let service_map = services
//             .into_iter()
//             .map(|service| {
//                 let address = service.address().to_string();
//                 (address, vec![service])
//             })
//             .collect::<ServiceMap>();

//         Self(service_map)
//     }
// }

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Address,
            Envelope,
            Reply,
        },
    };

    const ADDRESS: &str = "http://no-service.com";

    #[derive(Debug, Clone)]
    struct NoService(Address);

    impl Service for NoService {
        fn address(&self) -> &Address { &self.0 }

        fn duplicate(&self) -> Box<dyn Service> { Box::new(self.clone()) }

        fn process(&self, _message: Envelope) -> Result<Reply> { Ok(None) }
    }

    #[test]
    fn default_registry() {
        let registry = Registry::default();
        assert!(registry.list().is_empty());
    }

    #[test]
    fn build_registry() -> Result<()> {
        let address = Address::parse(ADDRESS)?;
        let instance = NoService(address);
        let registry = Registry::builder()
            .register(instance)
            .build();

        let list = registry.list();
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], ADDRESS);
        Ok(())
    }

    #[test]
    fn get_instances_from_registry() -> Result<()> {
        let address = Address::parse(ADDRESS)?;
        let instance = NoService(address);
        let registry = Registry::builder()
            .register(instance)
            .build();

        let instances = registry.get(ADDRESS);
        assert!(instances.is_some());
        assert_eq!(instances.unwrap().len(), 1);
        Ok(())
    }

    #[test]
    fn new_service_instance() -> Result<()> {
        let address = Address::parse(ADDRESS)?;
        let instance = NoService(address);
        let mut registry = Registry::builder()
            .register(instance)
            .build();

        registry.add_instance(ADDRESS)?;
        let instances = registry.get(ADDRESS);
        assert!(instances.is_some());
        assert_eq!(instances.unwrap().len(), 2);

        Ok(())
    }

    // #[test]
    // fn registry_from_vector_with_same_address() {
    //     let address = Address::parse(ADDRESS).unwrap();
    //     let instances = vec![
    //         Box::new(NoService(address.clone())),
    //         Box::new(NoService(address.clone())),
    //         Box::new(NoService(address.clone())),
    //     ];
    //     let registry = Registry::from(instances);

    //     let instances = registry.get(address.authority());
    //     assert!(instances.is_some());
    //     assert_eq!(instances.unwrap().len(), 3);
    // }
}
