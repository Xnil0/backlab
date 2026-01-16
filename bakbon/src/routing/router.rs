use {
    super::Registry,
    crate::{
        Envelope,
        MyErr,
        MyResult,
        Reply,
    },
    std::time::Duration,
};

#[derive(Default)]
pub struct RouterBuilder {
    registry:       Registry,
    timeout:        Option<Duration>,
    max_retries:    Option<u32>,
    max_concurrent: Option<usize>,
}

impl RouterBuilder {
    pub fn registry(mut self, registry: Registry) -> Self {
        self.registry = registry;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    pub fn max_concurrent(mut self, concurrent: usize) -> Self {
        self.max_concurrent = Some(concurrent);
        self
    }

    pub fn build(self) -> Router {
        Router {
            registry:       self.registry,
            timeout:        self.timeout,
            max_retries:    self.max_retries,
            max_concurrent: self.max_concurrent,
        }
    }
}

#[allow(unused)]
pub struct Router {
    pub(super) registry:       Registry,
    pub(super) timeout:        Option<Duration>,
    pub(super) max_retries:    Option<u32>,
    pub(super) max_concurrent: Option<usize>,
}

impl Router {
    pub fn builder() -> RouterBuilder { RouterBuilder::default() }

    pub fn route(&self, message: Envelope) -> MyResult<Reply> {
        // Note -> Timeout, Retries, and Max Concurrent will be implemented.

        let address = message
            .destination()
            .authority();

        self.registry
            .get(address)
            .ok_or(MyErr::ServiceNotFound)?
            .process(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_router() {
        let registry = Registry::builder().build();
        let router = Router::builder()
            .registry(registry)
            .build();

        assert!(
            router
                .registry
                .list()
                .is_empty()
        );
    }
}
