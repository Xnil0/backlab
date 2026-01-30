use super::{
    Cache,
    Eviction,
    Store,
};

#[derive(Default)]
pub struct Builder {
    store:    Store,
    eviction: Eviction,
}

impl Builder {
    pub fn eviction_policy(mut self, policy: Eviction) -> Self {
        self.eviction = policy;
        self
    }

    pub fn build(self) -> Cache {
        Cache {
            store:    self.store,
            eviction: self.eviction,
        }
    }
}
