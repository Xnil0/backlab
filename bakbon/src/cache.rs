use {
    crate::Envelope,
    std::collections::HashMap,
};

type Store = HashMap<String, Envelope>;

#[derive(Default)]
pub struct Cache {
    store: Store,
}

impl Cache {
    pub fn get(&self, k: &str) -> Option<&Envelope> { self.store.get(k) }
}
