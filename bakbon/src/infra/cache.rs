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

    pub fn set(&mut self, k: &str, v: Envelope) {
        self.store
            .insert(k.to_string(), v);
    }

    pub fn clear(&mut self) { self.store.clear(); }
}

impl From<Store> for Cache {
    fn from(value: Store) -> Self {
        Self {
            store: value,
        }
    }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use {
        super::*,
        bytes::Bytes,
    };

    const SRC: &str = "http://source.com";
    const DST: &str = "http://destination.com";

    #[test]
    fn default_cache() {
        let cache = Cache::default();
        assert!(cache.store.is_empty());
    }

    #[test]
    fn new_cache_from_store() {
        let payload = Bytes::default();
        let k = "msg";
        let msg = Envelope::new(SRC, DST, payload.clone());

        let mut store = Store::default();
        store.insert(k.to_string(), msg);

        let cache = Cache::from(store);
        assert!(!cache.store.is_empty());

        let msg = cache.get(k);
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().payload(), &payload)
    }

    #[test]
    fn store_into_cache() {
        let payload = Bytes::default();
        let k = "msg";
        let msg = Envelope::new(SRC, DST, payload.clone());

        let mut store = Store::default();
        store.insert(k.to_string(), msg);

        let cache: Cache = store.into();
        assert!(!cache.store.is_empty());

        let msg = cache.get(k);
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().payload(), &payload)
    }

    #[test]
    fn cache_store() {
        let payload = Bytes::default();
        let msg = Envelope::new(SRC, DST, payload.clone());

        let k = "default";
        let mut cache = Cache::default();
        cache.set(k, msg.clone());

        let msg = cache.get(k);
        assert!(msg.is_some());

        let msg = msg.unwrap();
        assert_eq!(msg.source(), SRC);
        assert_eq!(msg.destination(), DST);
        assert_eq!(msg.payload(), &payload);
    }

    #[test]
    fn get_from_empty_cache() {
        let k = "phantom";
        let cache = Cache::default();
        let msg = cache.get(k);
        assert!(msg.is_none());
    }

    #[test]
    fn clear_cache() {
        let payload = Bytes::default();
        let msg = Envelope::new(SRC, DST, payload);

        let store: Store = [
            ("msg1".to_string(), msg.clone()),
            ("msg2".to_string(), msg.clone()),
            ("msg3".to_string(), msg.clone()),
            ("msg4".to_string(), msg.clone()),
            ("msg5".to_string(), msg.clone()),
        ]
        .into_iter()
        .collect();

        let mut cache = Cache::from(store);
        assert_eq!(cache.store.len(), 5);

        cache.clear();
        assert!(cache.store.is_empty());
    }
}
