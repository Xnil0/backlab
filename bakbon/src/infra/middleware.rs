use crate::{
    Envelope,
    Result,
};

/// Example:
/// ```ignore
/// struct LogMiddleware;
/// impl Middleware for LogMiddleware {
///     fn intercept(&self, msg: Envelope) -> Result<Envelope> {
///         println!("incoming: {:?}", msg);
///         Ok(msg)
///     }
/// }
/// ```
pub trait Middleware {
    fn intercept(&self, message: Envelope) -> Result<Envelope>;
}
