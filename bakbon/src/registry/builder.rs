use {
    super::Registry,
    crate::{
        Service,
        ServiceMap,
    },
};

/// Builder for construction of registry of services.
///
/// `RegistryBuilder` let's you register one or more [`Service`] instances
/// and then freeze the configuration into an immutable [`Registry`].
#[derive(Default)]
pub struct RegistryBuilder(ServiceMap);

impl RegistryBuilder {
    /// Registers a new service instance in the builder.
    ///
    /// [`Service`]s are grouped in their [`Adrress`](crate::Address)
    /// string representation. Multiple instances with the same
    /// [`Address`](crate::Address) can be added later via
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
            Result,
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
    fn default_registry_builder() {
        let builder = RegistryBuilder::default();
        assert!(builder.0.is_empty());
    }

    #[test]
    fn build_default_registry() {
        let builder = Registry::builder();
        assert!(builder.0.is_empty());

        let registry = builder.build();
        assert!(registry.list().is_empty());
    }

    #[test]
    fn build_registry_with_service() -> Result<()> {
        let address = Address::parse(ADDRESS)?;
        let service = NilService(address);

        let builder = Registry::builder().register(service);
        assert!(!builder.0.is_empty());
        assert_eq!(builder.0.len(), 1);

        let registry = builder.build();
        assert!(!registry.list().is_empty());
        assert_eq!(registry.list().len(), 1);

        Ok(())
    }
}
