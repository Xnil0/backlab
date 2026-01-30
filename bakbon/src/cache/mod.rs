mod builder;
mod eviction;

pub use eviction::Eviction;
use {
    crate::Envelope,
    builder::Builder,
    std::collections::HashMap,
};

type Store = HashMap<String, Envelope>;

#[derive(Default)]
pub struct Cache {
    store:    Store,
    eviction: Eviction,
}

impl Cache {
    pub fn builder() -> Builder { Builder::default() }

    pub fn get(&self, k: &str) -> Option<&Envelope> { self.store.get(k) }

    pub fn set(&mut self, k: &str, v: Envelope) {
        self.store
            .insert(k.to_string(), v);
    }

    pub fn clear(&mut self) { self.store.clear(); }

    pub fn eviction(&self) -> Eviction { self.eviction }
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
            Payload,
            Result,
        },
    };
    const SRC: &str = "http://source.com";
    const DST: &str = "http://destination.com";

    #[test]
    fn default_cache() {
        let cache = Cache::default();
        assert!(cache.store.is_empty());
    }

    #[test]
    fn cache_store() -> Result<()> {
        let src = Address::parse(SRC)?;
        let dst = Address::parse(DST)?;
        let payload = Payload::default();
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
        let payload = Payload::default();

        let msg1 = Envelope::new(src.clone(), dst.clone(), payload.clone());
        let msg2 = Envelope::new(src.clone(), dst.clone(), payload.clone());
        let msg3 = Envelope::new(src.clone(), dst.clone(), payload.clone());
        let msg4 = Envelope::new(src.clone(), dst.clone(), payload.clone());
        let msg5 = Envelope::new(src.clone(), dst.clone(), payload.clone());

        let mut cache = Cache::default();
        cache.set("msg1", msg1);
        cache.set("msg2", msg2);
        cache.set("msg3", msg3);
        cache.set("msg4", msg4);
        cache.set("msg5", msg5);

        assert_eq!(cache.store.len(), 5);

        cache.clear();
        assert!(cache.store.is_empty());
        Ok(())
    }
}
