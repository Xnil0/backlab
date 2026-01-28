mod processor;

pub use processor::{
    ProcMap,
    Processor,
};
use {
    super::{
        Envelope,
        Reply,
        Result,
    },
    crate::Address,
    std::{
        collections::HashMap,
        fmt::Debug,
    },
};

/// A service that processes envelopes and returns replies.
///
/// Implement this trait to create custom services in BakBon.
///
/// # Examples
///
/// ```rust
/// use bakbon::*;
///
/// #[derive(Debug)]
/// pub struct NilService(Address);
///
/// impl Service for NilService {
///     fn address(&self) -> &Address { &self.0 }
///
///     fn duplicate(&self) -> ServiceBox {
///         let address = self.address().clone();
///         Box::new(Self(address))
///     }
///
///     fn process(&self, message: Envelope) -> Result<Reply> {
///         Ok(None)
///     }
/// }
/// ```
pub trait Service: Debug {
    fn address(&self) -> &Address;
    fn duplicate(&self) -> ServiceBox;
    fn process(&self, msg: Envelope) -> Result<Reply>;
}

/// A boxed service that can be cloned and processed.
pub type ServiceBox = Box<dyn Service>;

/// A vector of boxed services.
pub type ServiceVec = Vec<ServiceBox>;
pub type ServiceMap = HashMap<String, ServiceVec>;
