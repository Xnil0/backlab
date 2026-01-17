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

    pub fn set(&mut self, k: String, v: Envelope) { self.store.insert(k, v); }

    pub fn clear(&mut self) { self.store.clear(); }
}
