use crate::Result;

pub trait Storage {
    type Key: Send + Sync;
    type Value: Send + Sync;

    fn put(&mut self, key: &Self::Key, value: &Self::Value) -> Result<()>;
    fn get(&self, key: &Self::Key) -> Result<Option<Self::Value>>;
    fn delete(&mut self, key: &Self::Key) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
}
