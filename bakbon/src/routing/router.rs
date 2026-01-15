use {
    super::Registry,
    crate::{
        Envelope,
        MyErr,
        MyResult,
        Reply,
    },
};

pub struct Router {
    registry: Registry,
}

impl Router {
    pub fn new(registry: Registry) -> Self { Self { registry } }

    pub fn route(&self, message: Envelope) -> MyResult<Reply> {
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
        let router = Router::new(Registry::builder().build());
        assert!(
            router
                .registry
                .list()
                .is_empty()
        );
    }
}
