mod builder;

use {
    crate::{
        Error,
        Result,
        ServiceMap,
        ServiceVec,
    },
    builder::RegistryBuilder,
};

/// Immutable registry of services keyed by address.
///
/// The `Registry` stores one or more instances for each logical
/// [`Service`](crate::Service). It is used by [`Router`](crate::Router) to
/// lookup candidate instances before delegating selection go
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
    pub fn get(&self, address: &str) -> Option<&ServiceVec> {
        self.0
            .get(address)
            .map(|s| s.as_ref())
    }

    /// Returns a list of all registered [`Service`]
    /// [`Address`](crate::Address)es string representation.
    pub fn list(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|k| k.as_str())
            .collect()
    }
}

impl From<ServiceVec> for Registry {
    /// Builds a `Registry` from a flat list of [`Service`] instances.
    ///
    /// Instances are grouped by their [`Address`](crate::Address)] string
    /// representation.
    fn from(services: ServiceVec) -> Self {
        let mut service_map: ServiceMap = ServiceMap::new();

        for service in services {
            let key = service
                .address()
                .authority()
                .to_string();

            service_map
                .entry(key)
                .or_insert_with(ServiceVec::new)
                .push(service);
        }

        Self(service_map)
    }
}

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
            Service,
            ServiceBox,
        },
    };

    const ADDRESS: &str = "http://no-service.com";

    #[derive(Debug, Clone)]
    struct NilService(Address);

    impl Service for NilService {
        fn address(&self) -> &Address { &self.0 }

        fn duplicate(&self) -> ServiceBox { Box::new(self.clone()) }

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
        let instance = NilService(address);
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
        let instance = NilService(address);
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
        let instance = NilService(address);
        let mut registry = Registry::builder()
            .register(instance)
            .build();

        registry.add_instance(ADDRESS)?;
        let instances = registry.get(ADDRESS);
        assert!(instances.is_some());
        assert_eq!(instances.unwrap().len(), 2);

        Ok(())
    }

    #[test]
    fn registry_from_vector_of_same_service() {
        let address = Address::parse(ADDRESS).unwrap();
        let service = NilService(address.clone());
        let instances: ServiceVec = vec![
            service.duplicate(),
            service.duplicate(),
            service.duplicate(),
        ];
        let registry = Registry::from(instances);

        assert_eq!(registry.list().len(), 1);
        assert_eq!(
            registry
                .get(address.authority())
                .unwrap()
                .len(),
            3
        );
    }

    #[test]
    fn registry_from_vector_of_different_services() {
        let address1 = Address::parse(ADDRESS).unwrap();
        let address2 = Address::parse("http://example.com").unwrap();
        let instances: ServiceVec = vec![
            Box::new(NilService(address1.clone())),
            Box::new(NilService(address2.clone())),
            Box::new(NilService(address1.clone())),
        ];
        let registry = Registry::from(instances);

        assert_eq!(registry.list().len(), 2);
    }
}
