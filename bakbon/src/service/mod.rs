mod processor;

pub use processor::{
    ProcMap,
    Processor,
};
use {
    super::{
        Envelope,
        Result,
        Reply,
    },
    crate::Address,
};

/// A service that processes envelopes and returns replies.
///
/// Implement this trait to create custom services in Bakbon.
///
/// # Examples
///
/// ```rust
/// pub struct NilService(Address); 
/// 
/// impl Service for NilService {
///     fn address(&self) -> &Address { &self.address }
///
///     fn duplicate(&self) -> Box<dyn Service> {
///         let address = self.address.clone();
///         Box::new(Self(address))
///     }
///
///     fn process(&self, message: Envelope) -> Result<Reply> {
///         Ok(None)
///     }
/// }
/// ```
pub trait Service {
    fn address(&self) -> &Address;
    fn process(&self, message: Envelope) -> Result<Reply>;
    fn duplicate(&self) -> Box<dyn Service>;
}
