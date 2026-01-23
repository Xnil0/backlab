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
        crate::{
            Address,
            Result,
        },
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
    fn new_cache_from_store() -> Result<()> {
        let src = Address::parse(SRC)?;
        let dst = Address::parse(DST)?;
        let payload = Bytes::default();
        let k = "service";
        let msg = Envelope::new(src, dst, payload.clone());

        let mut store = Store::default();
        store.insert(k.to_string(), msg);

        let cache = Cache::from(store);
        assert!(!cache.store.is_empty());

        let msg = cache.get(k);
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().payload(), &payload);
        Ok(())
    }

    #[test]
    fn store_into_cache() -> Result<()> {
        let src = Address::parse(SRC)?;
        let dst = Address::parse(DST)?;
        let payload = Bytes::default();
        let k = "msg";
        let msg = Envelope::new(src, dst, payload.clone());

        let mut store = Store::default();
        store.insert(k.to_string(), msg);

        let cache: Cache = store.into();
        assert!(!cache.store.is_empty());

        let msg = cache.get(k);
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().payload(), &payload);
        Ok(())
    }

    #[test]
    fn cache_store() -> Result<()> {
        let src = Address::parse(SRC)?;
        let dst = Address::parse(DST)?;
        let payload = Bytes::default();
        let msg = Envelope::new(src.clone(), dst.clone(), payload.clone());

        let k = "default";
        let mut cache = Cache::default();
        cache.set(k, msg);

        let msg = cache.get(k);
        assert!(msg.is_some());

        let msg = msg.unwrap();
        assert_eq!(msg.source(), &src);
        assert_eq!(msg.destination(), &dst);
        assert_eq!(msg.payload(), &payload);
        Ok(())
    }

    #[test]
    fn get_from_empty_cache() {
        let k = "phantom";
        let cache = Cache::default();
        let msg = cache.get(k);
        assert!(msg.is_none());
    }

    #[test]
    fn clear_cache() -> Result<()> {
        let src = Address::parse(SRC)?;
        let dst = Address::parse(DST)?;
        let payload = Bytes::default();

        let msg1 = Envelope::new(src.clone(), dst.clone(), payload.clone());
        let msg2 = Envelope::new(src.clone(), dst.clone(), payload.clone());
        let msg3 = Envelope::new(src.clone(), dst.clone(), payload.clone());
        let msg4 = Envelope::new(src.clone(), dst.clone(), payload.clone());
        let msg5 = Envelope::new(src.clone(), dst.clone(), payload.clone());

        let store: Store = [
            ("msg1".to_string(), msg1),
            ("msg2".to_string(), msg2),
            ("msg3".to_string(), msg3),
            ("msg4".to_string(), msg4),
            ("msg5".to_string(), msg5),
        ]
        .into_iter()
        .collect();

        let mut cache = Cache::from(store);
        assert_eq!(cache.store.len(), 5);

        cache.clear();
        assert!(cache.store.is_empty());
        Ok(())
    }
}
